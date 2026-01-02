# CONFIGURATION

tuggy loads an optional `tuggy.toml` file in the current working directory.

## Example

```toml
# debug = true

# skip_platforms = [
#     "linux/mips64",
# ]

# dockerfile = "Dockerfile"

# jobs_limit = 4

# buildx_args = []

# directory = "."
```

# debug

Default: `false`

Enables additional logging.

# skip_platforms

Default:

```toml
[
    "linux/mips64"
]
```

Collects patterns of exclusions to skip image builds.

Patterns use Rust [regex](https://crates.io/crates/regex) notation.

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
