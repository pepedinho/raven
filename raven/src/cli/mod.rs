use std::{collections::HashMap, fs, path::PathBuf};

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

    pub fn load_mock_dir(dir: &PathBuf) -> anyhow::Result<HashMap<String, MockBlock>> {
        let mut all = HashMap::new();

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                let tmp = Self::load_mock_dir(&path)?;
                all.extend(tmp);
                continue;
            }

            if path.extension().and_then(|s| s.to_str()) != Some("toml") {
                continue;
            }

            let filename = path.file_stem().unwrap().to_string_lossy();

            let mock = Self::load_mock_file(&path)?;

            for (mock_name, mock) in mock.iter() {
                let key = format!("{}::{}", filename, mock_name);
                all.insert(key, mock.clone());
            }
        }

        Ok(all)
    }
}
