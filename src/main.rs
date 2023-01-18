use clap::Parser;
use handlebars::Handlebars;
use http::{StatusCode};
use log::{error, info};
use serde::{Deserialize, Serialize};

use std::{
    env::consts::{ARCH, OS},
    fs::File,
    io,
    process::exit,
};

/// Maybe not that clever, but at least not dumb. Download binaries for defferent architectures easier
#[derive(Parser)]
#[clap(author = "allanger <allanger@zohomail.com>", version, about, long_about = None, arg_required_else_help(true))]
struct Args {
    /// A templated link for downloading
    #[clap(short, long, env = "CLIN_LINK")]
    link_template: String,
    /// Version that you want to download
    #[clap(short, long, env = "CLIN_VERSION")]
    package_version: String,
    /// Path to download
    #[clap(short, long, env = "CLIN_PATH")]
    install_path: String,
}

#[derive(Clone, Serialize, Deserialize)]
struct Values {
    version: String,
    os: String,
    arch: String,
}

fn main() {
    env_logger::init();
    let args = Args::parse();
    let mut reg = Handlebars::new();
    reg.register_template_string("download_link", args.link_template)
        .unwrap();

    let archs: Vec<String> = match ARCH {
        "x86_64" => vec!["x86_64".to_string(), "amd64".to_string()],
        "aarch64" => vec!["aarch64".to_string(), "arm64".to_string()],
        _ => {
            error!("Unknown architecture");
            exit(1);
        }
    };

    for arch in archs {
        let version = args.package_version.clone();
        let os = OS.to_string();
        let values = Values { arch, os, version };

        let link = reg.render("download_link", &values).unwrap();
        info!("Trying to download from {}", link.clone());
        let mut resp = reqwest::blocking::get(link).unwrap();
        if resp.status() == StatusCode::OK {
            info!("Response is 200, I'll try to download");
            let mut out = File::create(args.install_path).expect("failed to create file");
            io::copy(&mut resp, &mut out).expect("failed to copy content");
            break;
        }
        info!("Will try another name for arch, because response is not 200");
    }
}
