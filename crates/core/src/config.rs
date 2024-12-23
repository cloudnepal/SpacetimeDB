use std::path::Path;
use std::{fmt, io};

use spacetimedb_lib::Address;
use spacetimedb_paths::cli::{ConfigDir, PrivKeyPath, PubKeyPath};
use spacetimedb_paths::server::{ConfigToml, MetadataTomlPath};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct MetadataFile {
    pub version: semver::Version,
    pub edition: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_address: Option<Address>,
}

pub fn parse_config<T: serde::de::DeserializeOwned>(path: &Path) -> anyhow::Result<Option<T>> {
    match std::fs::read_to_string(path) {
        Ok(contents) => Ok(Some(toml::from_str(&contents)?)),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(e.into()),
    }
}

impl MetadataFile {
    pub fn read(path: &MetadataTomlPath) -> anyhow::Result<Option<Self>> {
        parse_config(path.as_ref())
    }

    pub fn write(&self, path: &MetadataTomlPath) -> io::Result<()> {
        path.write(self.to_string())
    }

    pub fn version_compatible_with(&self, version: &semver::Version) -> bool {
        semver::Comparator {
            op: semver::Op::Caret,
            major: self.version.major,
            minor: Some(self.version.minor),
            patch: Some(self.version.patch),
            pre: self.version.pre.clone(),
        }
        .matches(version)
    }
}

impl fmt::Display for MetadataFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "# THIS FILE IS GENERATED BY SPACETIMEDB, DO NOT MODIFY!")?;
        writeln!(f)?;
        f.write_str(&toml::to_string(self).unwrap())
    }
}

pub fn current_version() -> semver::Version {
    env!("CARGO_PKG_VERSION").parse().unwrap()
}

#[derive(serde::Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct ConfigFile {
    #[serde(default)]
    pub certificate_authority: Option<CertificateAuthority>,
    #[serde(default)]
    pub logs: LogConfig,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct CertificateAuthority {
    pub jwt_priv_key_path: PrivKeyPath,
    pub jwt_pub_key_path: PubKeyPath,
}

impl CertificateAuthority {
    pub fn in_cli_config_dir(dir: &ConfigDir) -> Self {
        Self {
            jwt_priv_key_path: dir.jwt_priv_key(),
            jwt_pub_key_path: dir.jwt_pub_key(),
        }
    }

    pub fn get_or_create_keys(&self) -> anyhow::Result<crate::auth::JwtKeys> {
        crate::auth::get_or_create_keys(self)
    }
}

#[serde_with::serde_as]
#[derive(serde::Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct LogConfig {
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub level: Option<tracing_core::LevelFilter>,
    #[serde(default)]
    pub directives: Vec<String>,
}

impl ConfigFile {
    pub fn read(path: &ConfigToml) -> anyhow::Result<Option<Self>> {
        parse_config(path.as_ref())
    }
}
