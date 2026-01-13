# INSTALL

We support several installation methods.

# RUNTIME REQUIREMENTS

* [Docker](https://www.docker.com/) 28.0.1+

# PRECOMPILED BINARIES

https://github.com/mcandre/tuggy/releases

## Instructions

1. Download release archive.
2. Extract archive.
3. Select executables for your target platform.
4. Copy executabless to a convenient location, e.g. `$HOME/bin`.
5. Ensure location is registered in `$PATH`.

# DOCKER

## Instructions

```sh
docker pull n4jm4/tuggy
```

# BUILD FROM SOURCE

## Requirements

* [Rust](https://www.rust-lang.org/en-US/) 1.92.0+

## Instructions

```sh
cargo install --force --path .
```

For more details on developing tuggy itself, see [DEVELOPMENT.md](DEVELOPMENT.md).
