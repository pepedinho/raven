use std::{fs, sync::Arc};

use mlua::{Function, Lua, Table};
use tokio::{
    net::TcpListener,
    sync::{Mutex, mpsc, oneshot},
};

// #[derive(Clone)]
struct LuaJob {
    callback_name: String,
    arg: String,
    resp_tx: oneshot::Sender<String>,
}
pub struct Server {
    port: u16,
    lua_pool: Vec<Arc<Mutex<Lua>>>,
    job_tx: mpsc::Sender<LuaJob>,
}

impl Server {
    pub async fn new(script: String) -> anyhow::Result<Self> {
        let n_vms = 2;

        let lua = Lua::new();
        let lua_content = fs::read_to_string(script)?;
        lua.load(&lua_content).exec()?;

        let raven_ns: Table = lua.globals().get("raven")?;
        let init_fn: Function = raven_ns.get("init")?;
        let config: Table = init_fn.call(())?;
        let port: u16 = config.get("port")?;

        println!("Lua config loaded : port = {}", port);

        let (job_tx, job_rx) = mpsc::channel::<LuaJob>(100);
        let job_rx = Arc::new(Mutex::new(job_rx));

        let mut lua_pool = Vec::new();
        for _ in 0..n_vms {
            let lua_vm = Arc::new(Mutex::new(Lua::new()));
            {
                let lua_guard = lua_vm.lock().await;
                lua_guard.load(&lua_content).exec()?;
            }
            lua_pool.push(lua_vm.clone());

            let lua_clone = lua_vm.clone();
            let job_rx_clone = Arc::clone(&job_rx);

            tokio::spawn(async move {
                loop {
                    let job_opt = {
                        let mut rx = job_rx_clone.lock().await;
                        rx.recv().await
                    };
                    if let Some(job) = job_opt {
                        let lua_guard = lua_clone.lock().await;
                        if let Ok(raven_ns) = lua_guard.globals().get::<_, Table>("raven") {
                            if let Ok(callback) =
                                raven_ns.get::<_, Function>(job.callback_name.as_str())
                            {
                                let _ = callback.call::<_, ()>(job.arg.clone());
                            }
                        }
                    } else {
                        break;
                    }
                }
            });
        }

        Ok(Self {
            port,
            lua_pool,
            job_tx,
        })
    }

    pub async fn start(&self) -> anyhow::Result<()> {
        let listener = TcpListener::bind(("0.0.0.0", self.port)).await?;
        println!("Server listening on port {}", self.port);

        loop {
            let (socket, addr) = listener.accept().await?;

            println!("Client connected: {}", addr);

            // let job = LuaJob {
            //     callback_name: "on_client_connected".to_string(),
            //     arg: addr.to_string(),
            // };
            // let _ = self.job_tx.send(job).await;
        }
    }
}
