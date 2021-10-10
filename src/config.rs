use async_recursion::async_recursion;
use serde::Deserialize;
use std::{
    ops::Add,
    path::{Path, PathBuf},
};
use thiserror::Error;

impl From<ConfigError> for crate::MaidenError {
    fn from(c: ConfigError) -> Self {
        crate::MaidenError::Config(c)
    }
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("io error: {0}: {1}")]
    IoError(std::io::Error, String),

    #[error("yaml error: {0}: {1}")]
    YamlError(serde_yaml::Error, String),
}

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    imports: Option<Vec<String>>,
    fixture: Option<crate::fixture::Fixture>,
    service: Option<crate::service::Service>,
}

impl Config {
    pub async fn new(config_file: Option<PathBuf>) -> Result<Self, ConfigError> {
        Config::parse(config_file.unwrap_or_else(|| PathBuf::from("maiden.yaml"))).await
    }

    #[async_recursion]
    async fn parse(config_file: PathBuf) -> Result<Self, ConfigError> {
        let config =
            serde_yaml::from_reader::<_, Self>(std::fs::File::open(config_file.clone()).map_err(
                |err| ConfigError::IoError(err, config_file.clone().to_string_lossy().to_string()),
            )?)
            .map_err(|err| {
                ConfigError::YamlError(err, config_file.clone().to_string_lossy().to_string())
            })?
            .resolve(
                config_file
                    .parent()
                    .unwrap_or_else(|| Path::new("."))
                    .to_path_buf(),
            )
            .await?;
        Ok(config)
    }

    async fn resolve(mut self, parent: PathBuf) -> Result<Self, ConfigError> {
        if let Some(imports) = self.imports.as_mut() {
            *imports = imports
                .iter()
                .map(|import| {
                    let mut parent = parent.clone();
                    parent.push(import);
                    parent.to_string_lossy().to_string()
                })
                .collect();
        }
        if let Some(imports) = self.imports.clone() {
            for import in imports {
                let config = Config::parse(PathBuf::from(import.as_str())).await?;
                self = self.add(config);
            }
        }
        Ok(self)
    }
}

impl std::ops::Add for Config {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        if let Some(import) = rhs.imports {
            self.imports = self.imports.map(|mut val| {
                val.extend(import);
                val
            });
        }
        if rhs.fixture.is_some() {
            unimplemented!("fixtures is currently unimplemented");
            // self.fixture = self.fixture.and_then(|val| Some(val.add(fixture)));
        }
        if let Some(service) = rhs.service {
            self.service = self.service.map(|val| val.add(service));
        }
        self
    }
}

/* // InputFixture corresponds with the data structure of unmarshalled config values.
// It shouldn't be used directly and instead marshalled via it's parse method.
type InputFixture struct {
    DockerCompose *struct {
        Output string `yaml:"output"`
    } `yaml:"docker-compose"`
    Imports   interface{} `yaml:"imports"`
    Cassandra *struct {
        Sources []struct {
            Keyspace   string `yaml:"keyspace"`
            Definition string `yaml:"definition"`
            Files      string `yaml:"files"`
        } `yaml:"src"`
        Destination string `yaml:"dest"`
    }
    Elasticsearch *struct {
        Sources []struct {
            Index       string `yaml:"index"`
            Mapping     string `yaml:"mapping"`
            MappingType string `yaml:"mapping-type"`
            Files       string `yaml:"files"`
        } `yaml:"src"`
        Destination string `yaml:"dest"`
    } `yaml:"elasticsearch"`
    PostgreSQL *struct {
        Sources []struct {
            Database   string `yaml:"database"`
            Definition string `yaml:"definition"`
            Files      string `yaml:"files"`
        } `yaml:"src"`
        Destination string `yaml:"dest"`
    } `yaml:"postgresql"`
    Redis *struct {
        Source      string `yaml:"src"`
        Destination string `yaml:"dest"`
    } `yaml:"redis"`
} */
