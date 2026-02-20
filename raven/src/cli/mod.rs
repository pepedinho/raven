use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use clap::Parser;

use crate::mock_engine::{MockBlock, MockDefinition};

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    #[arg[long, default_value = "8080"]]
    pub port: u16,

    #[arg(long, default_value = "127.0.0.1")]
    pub host: String,

    #[arg(long, conflicts_with = "mock")]
    pub mocks: Option<PathBuf>,

    #[arg(long, conflicts_with = "mocks")]
    pub mock: Option<PathBuf>,

    #[arg(long)]
    pub watch: bool,
}

impl Cli {
    pub fn load_mock_file(path: &PathBuf) -> anyhow::Result<HashMap<String, MockBlock>> {
        let content = fs::read_to_string(path)?;

        let mocks: MockDefinition = toml::from_str(&content)?;

        Ok(mocks.mock)
    }

    pub fn load_mock_dir(
        _root: &Path,
        dir: &PathBuf,
    ) -> anyhow::Result<HashMap<String, MockBlock>> {
        println!("load mock from {}", dir.display());
        let mut all = HashMap::new();

        let to_canonical = |mocks: HashMap<String, MockBlock>| {
            mocks.into_values().map(|mock| {
                let canonical = canonical_key_from_path(&mock.r#match.path);
                (canonical, mock)
            })
        };

        if !dir.is_dir() {
            let mocks = Self::load_mock_file(dir)?;
            all.extend(to_canonical(mocks));
            return Ok(all);
        }

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                let tmp = Self::load_mock_dir(_root, &path)?;
                all.extend(tmp);
                continue;
            }
            let mock = Self::load_mock_file(&path)?;

            // let namespace = path
            //     .parent()
            //     .unwrap()
            //     .strip_prefix(root)?
            //     .iter()
            //     .map(|os| os.to_string_lossy())
            //     .collect::<Vec<_>>()
            //     .join("::");

            println!("mock size: {}", mock.len());
            all.extend(to_canonical(mock));
        }

        Ok(all)
    }
}

fn canonical_key_from_path(path: &str) -> String {
    path.split('/')
        .filter(|s| !s.is_empty())
        .map(|segment| {
            if segment.starts_with('{') && segment.ends_with('}') {
                format!("{{{}}}", &segment[1..segment.len() - 1])
            } else {
                segment.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("::")
}
