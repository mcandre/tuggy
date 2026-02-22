# CONFIGURATION

tuggy loads an optional `tuggy.toml` file in the current working directory.

# debug

Default: `false`

Enables additional logging.

# platforms

Required.

Example:

```toml
platforms = [
    "linux/386",
    "linux/amd64",
    "linux/amd64/v2",
    "linux/arm/v6",
    "linux/arm/v7",
    "linux/arm64",
    "linux/ppc64le",
    "linux/riscv64",
    "linux/s390x",
]
```

Enumerates target Docker buildx [platforms](https://docs.docker.com/build/building/multi-platform/).

# load_platform

Default: (`DOCKER_DEFAULT_PLATFORM` environment variable)

Example:

```toml
load_platform = "linux/amd64"
```

Override the default loading platform.

# directory

Default: `.` (current working directory)

Customize the Docker working directory.

# dockerfile

Default: `Dockerfile`

Customize the file path to the Docker manifest.

# jobs_limit

Default: `4`

Customize the number of concurrent operations.

Push operations are not batched, in order to work around glitches with Docker Hub multiplatform image pushes.

# driver

Override custom buildx driver.

# buildx_args

Default:

```toml
[]
```

Supply additional command line arguments to `docker buildx` commands.
