# tuggy: Multiplatform Docker rescue ship

[![Docker Pulls](https://img.shields.io/docker/pulls/n4jm4/tuggy)](https://hub.docker.com/r/n4jm4/tuggy) [![Crates.io Downloads (recent)](https://img.shields.io/crates/dr/tuggy?label=crate%20downloads)](https://crates.io/crates/tuggy) [![GitHub Downloads](https://img.shields.io/github/downloads/mcandre/tuggy/total?logo=github)](https://github.com/mcandre/tuggy/releases) [![docs.rs](https://img.shields.io/docsrs/tuggy)](https://docs.rs/tuggy/latest/tuggy/) [![Test](https://github.com/mcandre/tuggy/actions/workflows/test.yml/badge.svg)](https://github.com/mcandre/tuggy/actions/workflows/test.yml) [![license](https://img.shields.io/badge/license-BSD-3)](LICENSE.md) [![Donate](https://img.shields.io/badge/-any?logo=gumroad&label=Donate&color=grey)](https://mcandre.gumroad.com/)

![logo](tuggy.png)

# SUMMARY

tuggy automates *multiplatform* Docker pipelines.

Spend less time managing buildx images. Enjoy more time developing your core application.

# EXAMPLE

```console
$ cd example

$ tuggy -t n4jm4/tuggy-demo

$ tuggy --list -t n4jm4/tuggy-demo
Platform:  linux/386
Platform:  linux/amd64
Platform:  linux/amd64/v2
...

$ tuggy -t n4jm4/tuggy-demo --load

$ docker run --rm n4jm4/tuggy-demo cat /banner
Hello World!
```

# MOTIVATION

buildx is hard. tuggy is easy.

When Docker introduced the buildx subsystem, their goals included making buildx operationally successful. But not necessarily as straightforward, consistent, and intuitive as single-platform `docker` commands. We have run extensive drills on what buildx has to offer, and wrapped this into a neat little package called tuggy.

We are not replacing buildx, we just provide a proven workflow for high level buildx operation. We hope tuggy helps you to jumpstart multiplatform projects and even learn some fundamental buildx commands along the way.

You can see more Docker gears turning, apply the `tuggy -d` flag. tuggy respects your time, but also rewards curiosity.

## Rate Limits

Docker Hub has austere rate limits for many operations. Recommend adopting a (sponsored) subscription and org tokens, in order to handle the additional HTTP requests involved in multiplatform image development.

Docker Hub rate limiting appears to be overly sensitive to repeated low level HTTP requests, interpreting these as attacks, when they are actually ordinary user operations.

Tip: Check for any duplicate tag names for the same (multiplatform) image. Deduplicate tags and/or aliases. Rate limiting appears to struggle with multiplatform tag aliases.

Tip: Alter the image checksum, such as by recompiling. Rate limiting appears to struggle with repeated attempts to push the same image contents.

# INSTALLATION

See [INSTALL.md](INSTALL.md).

## Recommended

* a host capable of running musl/Linux containers (e.g. a GNU/Linux, musl/Linux, macOS, or Windows host)
* a UNIX-like environment (e.g. [WSL](https://learn.microsoft.com/en-us/windows/wsl/))
* [Docker First Aid Kit](https://github.com/mcandre/docker-first-aid-kit)
* 256 GB of space allocated to Docker
* Apply `DOCKER_DEFAULT_PLATFORM` = `linux/amd64` environment variable
* [ASDF](https://asdf-vm.com/) 0.18 (run `asdf reshim` after each Rust application binary installation)
* [cargo-cache](https://crates.io/crates/cargo-cache)
* [direnv](https://direnv.net/) 2
* POSIX compliant [tar](https://pubs.opengroup.org/onlinepubs/7908799/xcu/tar.html)
* [tree](https://en.wikipedia.org/wiki/Tree_(command))
* [GNU](https://www.gnu.org/) [time](https://en.wikipedia.org/wiki/Time_(Unix))
* [Amphetamine](https://apps.apple.com/us/app/amphetamine/id937984704?mt=12) (macOS), [The Caffeine](https://www.microsoft.com/store/productId/9PJBW5SCH9LC) (Windows), [Caffeine](https://launchpad.net/caffeine) (Linux) can prevent hibernation during any long builds

Regardless of target application environment, we encourage an amd64 compatible build environment. This tends to improve build reliability.

In time, we may revisit this recommendation. For now, an amd64 compatible host affords better chances for successful cross-compilation than trying, for example, to build `mips64` targets from `s390x` hosts.

Note: Docker buildx tends to make less aggressive use of caching compared with normal Docker. Expect some builds to restart from the first layer each time. Tune `.dockerignore` in order to maximize the potential for faster, cached image builds.

# USAGE

## Operations

`tuggy --get-platforms` lists available platforms.

`tuggy -t <tag>` builds multiplatform images with the given image, of the format `name[:tag]`. This is the essential tuggy build command.

`tuggy --clean` resets Docker resources, including the buildx image cache and the tuggy buildx builder.

Notable options:

* `-d` / `--debug` enables additional logging.
* `-j` / `--jobs` `<limit>` customizes the number of concurrent, nonpush operations.
* `-C` / `--directory` `<directory>` customizes the Docker current working directory.
* `-c` / `--configuration` `<path>` overrides the default configuration file path (`tuggy.toml`).

See `tuggy -h` for more options.

## Load

`tuggy -t <tag> --load <platform>` copies an image from the Docker buildx cache to the main Docker cache.

This step is often necessary for running or analyzing images.

Warning: Docker often silently ignores unavailable platforms until requested to *push* images. Update your tuggy configuration accordingly.

## Push

`tuggy -t <tag> --push` uploads buildx cached images to the remote Docker registry, as a side effect of the build.

Note: Normally, multiplatform images cannot be pushed directly from the main local cache, because most platforms do not support loading into the main local cache.

## Tag Aliasing

The `-a` / `--alias` `<aliases>` option creates additional tags for the same image as a side effect.

Note: Docker buildx tag aliasing requires `--push`.

## List

`tuggy -t <tag> --list` lists cached buildx entries.

Warning: Docker buildx is often unable to provide descriptions for buildx images until the images are published to a remote image registry, such as Docker Hub.

Warning: Docker buildx is sloppy with many platforms, corrupting them to `unknown/unknown` in many cases. When in doubt, verify image platforms at a remote registry.

# CONFIGURATION

See [CONFIGURATION.md](CONFIGURATION.md).

# FAQ

## How do I get started?

Practice basic, single-platform [Docker](https://www.docker.com/). As you gain confidence with Docker, you can extend this work into the realm of multiplatform images.

See the [example](example/) project, which can be built with plain `docker`, or with `docker buildx`, or with `tuggy`.

Apply the `-d` option to see more commands. Follow the basic, low level [buildx documentation](https://docs.docker.com/buildx/working-with-buildx/). For a more advanced illustration, see how the [snek](https://github.com/mcandre/snek) project builds its Docker images.

### Warning

Docker often gets confused, and may present spurious errors about missing images that were just built.

Suggested solutions:

* Ensure the relevant images support your target Docker platforms.
* Tune `DOCKER_DEFAULT_PLATFORM` to a supported platform, such as `linux/amd64` or `linux/arm64`.
* Explicitly append `--load [<platform>]` to `docker build -t <tag>`... commands.
* Removing any buildx builders with `docker buildx rm <builder>`. Confirm with `docker buildx ls`.

## Unsupported platform?

Depends on your particular base image. Each base image on Docker Hub, for example, is a little platform snowflake. A base image usually supports some smaller subset of the universe of platform combinations. When in doubt, [configure](CONFIGURATION.md) tuggy to skip more platforms, and/or restrict with an allow list.

## tuggy-in-docker?

Running tuggy itself within a Docker context, such as for CI/CD, naturally requires Docker-in-Docker privileges. See the relevant documentation for your particular cluster environment, such as Kubernetes.

# DOCKER HUB COMMUNITY

[Docker Hub](https://hub.docker.com/) provides an exceptional variety of base images, everything from Debian to Ubuntu to RHEL to glibc to musl to uClibC. If your base image lacks support for a particular platform, try searching for alternative base images. Or, build a new base image from scratch and publish it back to Docker Hub! The more we refine our base images, the easier it is to extend and use them.

# SEE ALSO

* [chandler](https://github.com/mcandre/chandler) normalizes executable archives
* [crit](https://github.com/mcandre/crit) generates Rust ports
* [factorio](https://github.com/mcandre/factorio) generates multiplatform Go application binaries
* [LLVM](https://llvm.org/) bitcode offers an abstract assembler format for C/C++ code
* [rockhopper](https://github.com/mcandre/rockhopper) generates packages for many operating systems
* [snek](https://github.com/mcandre/snek) ports native C/C++ applications
* [WASM](https://webassembly.org/) provides a portable interface for C/C++ code
* [xgo](https://github.com/techknowlogick/xgo) ports Go projects with native cgo dependencies

🛥️
