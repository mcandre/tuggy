# CONFIGURATION

tuggy loads an optional `tuggy.toml` file in the current working directory.

## Example

```toml
# debug = true

# platforms_skip = [
#     "linux/loong64",
#     "linux/mips64",
#     "linux/mips64le",
#     "linux/ppc64le",
#     "linux/riscv64",
#     "linux/s390x",
# ]

# platforms_allow = []

# dockerfile = "Dockerfile"

# jobs_limit = 4

# buildx_args = []

# directory = "."
```

# debug

Default: `false`

Enables additional logging.

# driver

Override custom buildx driver.

# platforms_skip

Default:

```toml
[
    "linux/loong64",
    "linux/mips64",
    "linux/mips64le",
    "linux/ppc64le",
    "linux/riscv64",
    "linux/s390x"
]
```

Collects patterns of exclusions to skip image builds.

Patterns use Rust [regex](https://crates.io/crates/regex) notation.

# platforms_allow

Default: (Allow all)

Restricts platforms to only those specificially requested.

Example:

```toml
[
    "linux/amd64",
    "linux/arm64",
]
```

Syntax is exact match Docker buildx [platform](https://docs.docker.com/build/building/multi-platform/) identifier (e.g. `linux/amd64`, `linux/arm64`, etc.)

# dockerfile

Default: `Dockerfile`

Customize the file path to the Docker manifest.

# jobs_limit

Default: `4`

Customize the number of concurrent operations.

Push operations are not batched, in order to work around glitches with Docker Hub multiplatform image pushes.

# buildx_args

Default:

```toml
[]
```

Supply additional command line arguments to `docker buildx` commands.

# directory

Default: `.` (current working directory)

Customize the Docker working directory.
