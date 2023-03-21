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
$ curl https://raw.githubusercontent.com/allanger/dumb-downloader/main/scripts/download_dudo.sh | bash
$ dudo --help
```
### Docker

You can use the `latest` or a `tagged` docker image
```
$ docker pull ghcr.io/allanger/dumb-downloader:latest
$ docker run ghcr.io/allanger/dumb-downloader:latest dudo -h
```

### Build from source
1. Build binary
```
$ cargo build --release
``` 
2. Run `dudo --help`

# How to use?
## Custom configurations

In case the default config is not doing the trick for you, you can pass a custom configuration, for example, you need to download a package "package-linux-amd64_x86_64_intel_v1.0.3" and this kind of name for an architecture is not supported by the `dudo`, then you can create a config file like 
```yaml
# config-example.yaml
---
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
    - amd64_x86_64_intel
  aarch64:
    - aarch64
    - arm64
    - m1

```

And execute `dudo -l "package-{{ os }}-{{ arch }}-{{ version}}" -p v1.0.3 -d /tmp/package` and dudo will download the package to the `/tmp/package` then, 

## Dockerfile

The initial intetion for developing this was to use it for writing multi-architecture Dockerfiles for my another projects. I needed to download `helm` and `helmfile` for `arm64` and `amd64`. And I couldn't come up with good simple script for settings environment variables that would point to the the correct url, because `uname -m` wasn't giving me results that I would need. I was thinkg about writing a script to create some kind of map for different architectures, but then I thought that is was already not the first time I was having that problem and I decided to come up with a tool. And here is example, how one could use it in a `Dockerfile`

```DOCKERFILE
ARG BASE_VERSION=latest
FROM ghcr.io/allanger/dumb-downloader as builder
RUN apt-get update -y && apt-get install tar -y
ARG HELM_VERSION=v3.10.3
ARG HELMFILE_VERSION=0.151.0
ENV RUST_LOG=info
RUN dudo -l "https://github.com/helmfile/helmfile/releases/download/v{{ version }}/helmfile_{{ version }}_{{ os }}_{{ arch }}.tar.gz" -i /tmp/helmfile.tar.gz -p $HELMFILE_VERSION
RUN dudo -l "https://get.helm.sh/helm-{{ version }}-{{ os }}-{{ arch }}.tar.gz" -i /tmp/helm.tar.gz -p $HELM_VERSION
RUN tar -xf /tmp/helm.tar.gz  -C /tmp && rm -f /tmp/helm.tar.gz 
RUN tar -xf /tmp/helmfile.tar.gz  -C /tmp && rm -f /tmp/helmfile.tar.gz 
RUN mkdir /out && for bin in `find /tmp | grep helm`; do cp $bin /out/; done
RUN chmod +x /out/helm
RUN chmod +x /out/helmfile

FROM ghcr.io/allanger/check-da-helm-base:${BASE_VERSION} 
COPY --from=builder /out/ /usr/bin
RUN apk update --no-cache && apk add --no-cache jq bash
ENTRYPOINT ["cdh"]
```

In the builder it is downloading dependencies that are needed in my final docker image.