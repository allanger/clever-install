use clap::Parser;
use handlebars::Handlebars;
use http::StatusCode;
use log::{error, info};
use serde::{Deserialize, Serialize};

use std::{
    collections::HashMap,
    env::consts::{ARCH, OS},
    fmt::Display,
    fs::{File, OpenOptions},
    io::{self},
    process::exit,
};

type Result<T> = std::result::Result<T, DudoError>;
#[derive(Debug)]
enum DudoError {
    IoError(io::Error),
    SerdeYamlError(serde_yaml::Error),
}
impl From<io::Error> for DudoError {
    fn from(error: io::Error) -> Self {
        DudoError::IoError(error)
    }
}

impl From<serde_yaml::Error> for DudoError {
    fn from(error: serde_yaml::Error) -> Self {
        DudoError::SerdeYamlError(error)
    }
}

impl Display for DudoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DudoError::SerdeYamlError(err) => write!(f, "{}", err),
            DudoError::IoError(err) => write!(f, "{}", err),
        }
    }
}

static CONFIG: &str = "
---
os:
  macos:
    - macos
    - darwin
    - mac
    - apple
  linux:
    - linux
  windows:
    - windows
  freebsd:
    - freebsd
arch:
  x86_64:
    - x86_64
    - amd64
    - amd
    - intel
  aarch64:
    - aarch64
    - arm64
    - m1
";

/// Maybe not that clever, but at least not dumb. Download binaries for defferent architectures easier
#[derive(Parser)]
#[clap(author = "allanger <allanger@zohomail.com>", version, about, long_about = None, arg_required_else_help(true))]
struct Args {
    /// A templated link for downloading
    #[clap(short, long, env = "DUDO_LINK_TEMPLATE")]
    link_template: String,
    /// Version that you want to download
    #[clap(short, long, env = "DUDO_PACKAGE_VERSION")]
    package_version: String,
    /// Path to download
    #[clap(short, long, env = "DUDO_DOWNLOADPATH")]
    download_path: String,
    /// Path to dudo config file
    #[clap(short, long, default_value = "", env = "DUDO_CONFIG")]
    config: String,
}

#[derive(Clone, Serialize, Deserialize)]
struct SystemValues {
    version: String,
    os: String,
    arch: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
struct Config {
    os: HashMap<String, Vec<String>>,
    arch: HashMap<String, Vec<String>>,
}

fn main() {
    // Initial steps
    env_logger::init();
    let args = Args::parse();

    // Register download url template
    let mut reg = Handlebars::new();
    match reg.register_template_string("download_link", args.link_template) {
        Ok(_) => info!("Your template is successfully registered"),
        Err(err) => error!("{}", err),
    };

    // Set system aliases
    let config = match parse_config(args.config) {
        Ok(config) => config,
        Err(err) => {
            error!("{}", err);
            exit(1);
        }
    };

    info!("Running on {} {}", OS, ARCH);
    let oss = config.os.get(&OS.clone().to_string()).unwrap();
    let archs = config.arch.get(&ARCH.clone().to_string()).unwrap();

    for arch in archs {
        for os in oss {
            let version = args.package_version.clone();
            let values = SystemValues {
                arch: arch.clone(),
                os: os.clone(),
                version,
            };

            let link = reg.render("download_link", &values).unwrap();
            info!("Trying to download from {}", link.clone());
            let mut resp = reqwest::blocking::get(link).unwrap();
            if resp.status() == StatusCode::OK {
                info!("Response is 200, I'll try to download");
                let mut out =
                    File::create(args.download_path.clone()).expect("failed to create file");
                io::copy(&mut resp, &mut out).expect("failed to copy content");
                exit(0);
            }
            info!("Will try another name for arch, because response is not 200");
        }
    }
}

fn parse_config(config_path: String) -> Result<Config> {
    let config_res: std::result::Result<Config, _>;
    if config_path.is_empty() {
        config_res = serde_yaml::from_str(CONFIG);
    } else {
        let f = OpenOptions::new().write(false).read(true).open(config_path);
        let f = match f {
            Ok(file) => file,
            Err(err) => {
                return Err(err.into());
            }
        };
        config_res = serde_yaml::from_reader(f);
    }
    match config_res {
        Ok(config) => Ok(config),
        Err(err) => Err(err.into()),
    }
}

#[cfg(test)]
mod tests {
    use crate::parse_config;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn parse_config_default() {
        let config = parse_config("".to_owned()).unwrap();
        assert_eq!(
            config.os.get("linux").unwrap().clone(),
            vec!["linux".to_string()]
        );
        assert_eq!(
            config.os.get("windows").unwrap().clone(),
            vec!["windows".to_string()]
        );
        assert_eq!(
            config.os.get("macos").unwrap().clone(),
            vec![
                "macos".to_string(),
                "darwin".to_string(),
                "mac".to_string(),
                "apple".to_string(),
            ]
        );
        assert_eq!(
            config.arch.get("x86_64").unwrap().clone(),
            vec![
                "x86_64".to_string(),
                "amd64".to_string(),
                "amd".to_string(),
                "intel".to_string(),
            ]
        );
        assert_eq!(
            config.arch.get("aarch64").unwrap().clone(),
            vec!["aarch64".to_string(), "arm64".to_string(), "m1".to_string(),]
        )
    }

    #[test]
    fn parse_config_custom() {
        let config = "
---
os:
  macos:
    - macos
  linux:
    - linux
  windows:
    - windows
  freebsd:
    - freebsd
arch:
  x86_64:
    - x86_64
  aarch64:
    - aarch64
        ";
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "{}", config).unwrap();
        let path = file.into_temp_path();
        // It's looking damn not right
        let config = parse_config(path.to_str().unwrap().clone().to_string()).unwrap();
        assert_eq!(
            config.os.get("linux").unwrap().clone(),
            vec!["linux".to_string()]
        );
        assert_eq!(
            config.os.get("windows").unwrap().clone(),
            vec!["windows".to_string()]
        );
        assert_eq!(
            config.os.get("macos").unwrap().clone(),
            vec!["macos".to_string()]
        );
        assert_eq!(
            config.arch.get("x86_64").unwrap().clone(),
            vec!["x86_64".to_string()]
        );
        assert_eq!(
            config.arch.get("aarch64").unwrap().clone(),
            vec!["aarch64".to_string()]
        )
    }
}
