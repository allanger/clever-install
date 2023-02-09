# Dumb Downloader (dudo)

# What's it about?
It's just a tool to make downloading binaries for different platforms easier. 
## For example
If you want to build a docker image, but you want to make it available on different platforms. But your tool needs other tools as dependencies, e.g. `helm`.
To install helm on Alpine you need to use curl, wget, or something. You need to choose a version, an operating system, and an architecture. For me, it was obvious that you must be able to use `uname -m`...
```BASH
uname -m 
aarch64

uname -m
x86_64

uname -m
arm64
```

While release naming is also not very consecutive

- release_example.amd-macos.zip
- another-release-for-amd64-macos.zip
- linux-aarch64-release.zip

# How to install?
## Install 
### Download 

Get executable from github releases

Prebuilt binaries exist for **Linux x86_64** and **MacOS arm64** and **x86_64**

Don't forget to add the binary to $PATH
```
$ curl https://raw.githubusercontent.com/allanger/clever-install/main/scripts/download_dudo.sh | bash
$ dudo -h
```
### Docker

You can use the `latest` or a `tagged` docker image
```
$ docker pull ghcr.io/allanger/clever-install:latest
$ docker run ghcr.io/allanger/clever-install:latest dudo -h
```

### Build from source
1. Build binary
```
$ cargo build --release
``` 
2. Run `gum help`

# How to use?
To be done
