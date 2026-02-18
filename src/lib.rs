//! tuggy provides predicates for building multiplatform Docker images.

extern crate regex;
extern crate toml;

use serde::{Deserialize, Serialize};

use std::fmt;
use std::fs;
use std::process;
use std::sync;

/// CONFIGURATION_FILENAME denotes the file path to an optional TOML configuration file,
/// relative to the current working directory.
pub static CONFIGURATION_FILENAME: &str = "tuggy.toml";

/// BUILDER_NAME identifies the tuggy buildx builder.
pub static BUILDER_NAME: &str = "tuggy";

/// NODE_NAME identifies the tuggy buildx node.
pub static NODE_NAME: &str = "tuggy0";

/// BUILDX_AVAILABLE_PLATFORMS_PATTERN parses a platform list string from `docker buildx inspect` output.
pub static BUILDX_AVAILABLE_PLATFORMS_PATTERN: sync::LazyLock<regex::Regex> =
    sync::LazyLock::new(|| regex::Regex::new(r"Platforms:\W+(?P<platforms>.+)$").unwrap());

/// DEFAULT_PLATFORMS_SKIP collects fringe Docker platforms.
pub static DEFAULT_PLATFORMS_SKIP: sync::LazyLock<Vec<&str>> = sync::LazyLock::new(|| {
    vec![
        "linux/loong64",
        "linux/mips64",
        "linux/mips64le",
        "linux/ppc64le",
        "linux/riscv64",
        "linux/s390x",
    ]
});

/// PLATFORMS_SKIP_PATTERN_REPLACE_TEMPLATE combines `platforms_skip` and a pipe (|) delimited platform string to form a pattern matching skippable platforms.
pub static PLATFORMS_SKIP_PATTERN_REPLACE_TEMPLATE: &str = r"^(platforms_skip)$";

/// generate_skip_platform_pattern builds a platform matching pattern from a collection of platforms.
pub fn generate_skip_platform_pattern(platforms: &[&str]) -> Result<regex::Regex, regex::Error> {
    regex::Regex::new(
        &PLATFORMS_SKIP_PATTERN_REPLACE_TEMPLATE.replace("platforms_skip", &platforms.join("|")),
    )
}

#[test]
fn test_default_platform_exclusion_pattern() {
    let pattern = generate_skip_platform_pattern(&DEFAULT_PLATFORMS_SKIP).unwrap();
    assert!(pattern.is_match("linux/mips64"));
    assert!(!pattern.is_match("linux/amd64"));
}

/// DEFAULT_JOBS_LIMIT restricts the number of concurrent Docker builds.
pub static DEFAULT_JOBS_LIMIT: usize = 4;

/// TuggyError models bad computer states.
#[derive(Debug)]
pub enum TuggyError {
    IOError(String),
    UnsupportedPathError(String),
    PathRenderError(String),
    UnknownMimetypeError(String),
    RegexParseError(String),
    TOMLParseError(String),
}

impl fmt::Display for TuggyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TuggyError::IOError(e) => write!(f, "{e}"),
            TuggyError::UnknownMimetypeError(e) => write!(f, "{e}"),
            TuggyError::UnsupportedPathError(e) => write!(f, "{e}"),
            TuggyError::PathRenderError(e) => write!(f, "{e}"),
            TuggyError::RegexParseError(e) => write!(f, "{e}"),
            TuggyError::TOMLParseError(e) => write!(f, "{e}"),
        }
    }
}

impl die::PrintExit for TuggyError {
    fn print_exit(&self) -> ! {
        eprintln!("{}", self);
        process::exit(die::DEFAULT_EXIT_CODE);
    }
}

/// Platform models Docker platforms.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, Eq, PartialOrd, Ord)]
pub struct Platform {
    /// os denotes an operating system.
    pub os: String,

    /// arch denotes an architecture.
    pub arch: String,
}

pub static PLATFORM_PATTERN: sync::LazyLock<regex::Regex> =
    sync::LazyLock::new(|| regex::Regex::new("^(?P<os>[^/]+)/(?P<arch>.+)$").unwrap());

impl Platform {
    /// from_string parses platforms.
    pub fn from_string(s: &str) -> Result<Platform, TuggyError> {
        if !PLATFORM_PATTERN.is_match(s) {
            return Err(TuggyError::IOError(format!("invalid platform: {}", s)));
        }

        match PLATFORM_PATTERN.captures(s) {
            Some(e) => Ok(Platform {
                os: e["os"].to_string(),
                arch: e["arch"].to_string(),
            }),
            _ => Err(TuggyError::IOError(format!("invalid platform: {s}"))),
        }
    }
}

impl fmt::Display for Platform {
    /// fmt renders a Platform to consoles.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.os, self.arch)
    }
}

#[test]
fn test_platform_from_string() {
    assert!(
        Platform::from_string("linux/amd64").unwrap()
            == Platform {
                os: "linux".to_string(),
                arch: "amd64".to_string()
            }
    );
    assert!(
        Platform::from_string("linux/amd64/v2").unwrap()
            == Platform {
                os: "linux".to_string(),
                arch: "amd64/v2".to_string()
            }
    );
    assert!(Platform::from_string("").is_err());
}

/// Tuggy conducts Docker buildx image operations.
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Tuggy {
    /// debug enables additional logging.
    pub debug: Option<bool>,

    /// driver overrides the default buildx driver.
    pub driver: Option<String>,

    /// platforms_skip match skippable platforms.
    ///
    /// Syntax is Rust [regex](https://crates.io/crates/regex).
    pub platforms_skip: Option<Vec<String>>,

    /// platforms_allow restricts platforms to only those specifically requested.
    ///
    /// Syntax is exact match Docker [platform](https://docs.docker.com/build/building/multi-platform/) identifier (e.g. `linux/amd64`, `linux/arm64`, etc.)
    pub platforms_allow: Option<Vec<String>>,

    /// load enables a side effect of loading a given buildx image platform into the local Docker cache.
    pub load: Option<bool>,

    /// push enables a side effect of pushing buildx images to remote Docker registries.
    pub push: Option<bool>,

    /// tag denotes a Docker image name.
    pub tag: Option<String>,

    /// aliases collects additional tag names for this image.
    pub aliases: Option<Vec<String>>,

    /// dockerfile denotes a Dockerfile source file path (default: "Dockerfile").
    pub dockerfile: Option<String>,

    /// jobs_limit restricts the number of concurrent Docker builds (default: DEFAULT_JOB_LIMIT).
    /// Zero indicates no limit.
    pub jobs_limit: Option<usize>,

    /// buildx_args collects custom flags to forward to `docker buildx`... commands.
    pub buildx_args: Option<Vec<String>>,

    /// directory denotes a Docker working directory (default: current working directory).
    pub directory: Option<String>,

    /// wd caches the Docker current working directory.
    wd: Option<String>,

    /// enabled_platforms caches unskipped platform names.
    enabled_platforms: Option<Vec<Platform>>,

    /// platform_group caches the current platform group.
    platform_group: Option<Vec<Platform>>,

    /// batch_size caches job limits.
    batch_size: Option<usize>,
}

impl Tuggy {
    /// load generates a Tuggy.
    pub fn load(pth: &str) -> Result<Self, TuggyError> {
        let toml_string = fs::read_to_string(pth)
            .map_err(|_| TuggyError::IOError(format!("unable to read file: {pth}")))?;
        let tuggy: Tuggy = toml::from_str(&toml_string)
            .map_err(|e| TuggyError::TOMLParseError(e.message().to_string()))?;
        Ok(tuggy)
    }

    /// ensure_buildx_builder allocates the tuggy buildx builder.
    pub fn ensure_buildx_builder(&self) -> Result<(), TuggyError> {
        let mut cmd = process::Command::new("docker");
        let mut base_args: Vec<String> = [
            "buildx",
            "create",
            "--bootstrap",
            "--name",
            BUILDER_NAME,
            "--node",
            NODE_NAME,
        ]
        .iter()
        .map(|e| e.to_string())
        .collect();

        if let Some(driver) = self.driver.clone() {
            base_args.push("--driver".to_string());
            base_args.push(driver);
        }

        let args: Vec<&str> = base_args.iter().map(|e| e.as_ref()).collect::<Vec<&str>>();
        cmd.args(args.as_slice());
        cmd.stderr(process::Stdio::piped());

        if let Some(true) = self.debug {
            eprintln!("debug: running command: {:?}", cmd);
        }

        let output: process::Output = cmd
            .output()
            .map_err(|e| TuggyError::IOError(e.to_string()))?;

        if !output.status.success() {
            let stderr_utf8: String =
                String::from_utf8(output.stderr).map_err(|e| TuggyError::IOError(e.to_string()))?;
            eprintln!("{}", stderr_utf8);
            return Err(TuggyError::IOError(
                "unable to provision buildx builder".to_string(),
            ));
        }

        Ok(())
    }

    pub fn get_platforms(&self) -> Result<Vec<Platform>, TuggyError> {
        self.ensure_buildx_builder()?;
        let mut cmd = process::Command::new("docker");
        let base_args: Vec<String> = ["buildx", "inspect", BUILDER_NAME]
            .iter()
            .map(|e| e.to_string())
            .collect();
        let args: Vec<&str> = base_args.iter().map(|e| e.as_ref()).collect::<Vec<&str>>();
        cmd.args(args.as_slice());
        cmd.stderr(process::Stdio::inherit());

        if let Some(true) = self.debug {
            eprintln!("debug: running command: {:?}", cmd);
        }

        let output: process::Output = cmd
            .output()
            .map_err(|e| TuggyError::IOError(e.to_string()))?;

        let success: bool = output.status.success();

        if !success {
            return Err(TuggyError::IOError(format!(
                "unable to query buildx builder: {BUILDER_NAME}"
            )));
        }

        let stdout_utf8: String =
            String::from_utf8(output.stdout).map_err(|e| TuggyError::IOError(e.to_string()))?;

        let mut platforms: Vec<Platform> = Vec::new();

        for line in stdout_utf8.lines() {
            if !BUILDX_AVAILABLE_PLATFORMS_PATTERN.is_match(line) {
                continue;
            }

            let platforms_comma_delimited: String =
                match &BUILDX_AVAILABLE_PLATFORMS_PATTERN.captures(line) {
                    Some(e) => Ok(e["platforms"].to_string()),
                    _ => Err(TuggyError::IOError(format!(
                        "invalid platforms list: {line}"
                    ))),
                }?;

            let platforms_raw: Vec<&str> = platforms_comma_delimited.split(", ").collect();

            for platform_raw in platforms_raw {
                let platform = Platform::from_string(platform_raw)?;
                platforms.push(platform);
            }
        }

        if platforms.is_empty() {
            return Err(TuggyError::IOError("no platforms detected".to_string()));
        }

        platforms.sort();
        Ok(platforms)
    }

    /// run_batch processes Docker builds.
    fn run_batch(&self, tag: &str) -> Result<(), TuggyError> {
        let mut cmd_create = process::Command::new("docker");

        // Work around spurious buildx warnings
        cmd_create.env("BUILDX_NO_DEFAULT_LOAD", "true");

        let mut base_args_create: Vec<String> = ["buildx", "build", "--builder", BUILDER_NAME]
            .iter()
            .map(|e| e.to_string())
            .collect();

        if let Some(true) = &self.load
            && let Some(true) = &self.push
        {
            return Err(TuggyError::IOError(
                "load, push are mutually exclusive operations".to_string(),
            ));
        }

        if let Some(true) = &self.load {
            base_args_create.push("--load".to_string());
        } else {
            base_args_create.push("--platform".to_string());
            base_args_create.push(
                self.platform_group
                    .clone()
                    .unwrap()
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
            );
        }

        if let Some(true) = self.push {
            base_args_create.push("--push".to_string());
        }

        base_args_create.extend(["-t".to_string(), tag.to_string()]);

        if let Some(dockerfile) = &self.dockerfile {
            base_args_create.extend(["-f".to_string(), dockerfile.to_string()]);
        }

        let extra_args = self.buildx_args.clone().unwrap_or_default();
        let args_create_strings: Vec<String> = [
            base_args_create,
            extra_args.clone(),
            vec![self.wd.clone().unwrap()],
        ]
        .concat();
        let args_create: Vec<&str> = args_create_strings
            .iter()
            .map(|e| e.as_ref())
            .collect::<Vec<&str>>();
        cmd_create.args(args_create.as_slice());
        cmd_create.stdout(process::Stdio::inherit());
        cmd_create.stderr(process::Stdio::inherit());

        if let Some(true) = self.debug {
            eprintln!("debug: running command: {:?}", cmd_create);
        }

        let output_create: process::Output = cmd_create
            .output()
            .map_err(|e| TuggyError::IOError(e.to_string()))?;

        if !output_create.status.success() {
            let stderr_utf8: String = String::from_utf8(output_create.stderr)
                .map_err(|e| TuggyError::IOError(e.to_string()))?;
            eprintln!("{}", stderr_utf8);
            return Err(TuggyError::IOError("unable to build image".to_string()));
        }

        if let Some(aliases) = &self.aliases {
            for alias in aliases {
                let mut cmd_retag = process::Command::new("docker");

                // Work around spurious buildx warnings
                cmd_retag.env("BUILDX_NO_DEFAULT_LOAD", "true");

                let base_args_retag: Vec<String> = [
                    "buildx",
                    "imagetools",
                    "--builder",
                    BUILDER_NAME,
                    "create",
                    "-t",
                    alias,
                    tag,
                ]
                .iter()
                .map(|e| e.to_string())
                .collect();

                let args_retag: Vec<&str> = base_args_retag
                    .iter()
                    .map(|e| e.as_ref())
                    .collect::<Vec<&str>>();
                cmd_retag.args(args_retag.as_slice());
                cmd_retag.stdout(process::Stdio::inherit());
                cmd_retag.stderr(process::Stdio::inherit());

                if let Some(true) = self.debug {
                    eprintln!("debug: running command: {:?}", cmd_retag);
                }

                let output_retag: process::Output = cmd_retag
                    .output()
                    .map_err(|e| TuggyError::IOError(e.to_string()))?;

                if !output_retag.status.success() {
                    let stderr_utf8: String = String::from_utf8(output_retag.stderr)
                        .map_err(|e| TuggyError::IOError(e.to_string()))?;
                    eprintln!("{}", stderr_utf8);
                    return Err(TuggyError::IOError("unable to tag image".to_string()));
                }
            }
        }

        Ok(())
    }

    /// build generates Docker images.
    pub fn build(&mut self, tag: &str) -> Result<(), TuggyError> {
        self.wd = match &self.directory {
            None => Some(".".to_string()),
            e => e.clone(),
        };

        let base_platforms = self.get_platforms()?;
        let mut platforms: Vec<Platform> = Vec::new();
        let platforms_skip: Vec<&str> = match &self.platforms_skip {
            Some(e) => e.iter().map(|e| e.as_ref()).collect(),
            _ => DEFAULT_PLATFORMS_SKIP.clone(),
        };
        let pattern = generate_skip_platform_pattern(&platforms_skip).unwrap();
        let platforms_allow = self.platforms_allow.clone();

        for platform in base_platforms {
            let platform_string: String = platform.to_string();

            if pattern.is_match(&platform_string) {
                if let Some(true) = self.debug {
                    eprintln!("debug: skip platform: {platform}");
                }

                continue;
            }

            if let Some(allowlist) = &platforms_allow
                && !allowlist.contains(&platform_string)
            {
                if let Some(true) = self.debug {
                    eprintln!("debug: deny platform: {platform}");
                }

                continue;
            }

            platforms.push(platform);
        }

        if platforms.is_empty() {
            return Err(TuggyError::IOError(
                "all platforms unavailable, skipped, or denied".to_string(),
            ));
        }

        self.enabled_platforms = Some(platforms);
        self.batch_size = Some(self.jobs_limit.unwrap_or(DEFAULT_JOBS_LIMIT));

        if let Some(0) = self.batch_size {
            return self.run_batch(tag);
        }

        for platform_group in self
            .enabled_platforms
            .clone()
            .unwrap()
            .chunks(self.batch_size.unwrap())
        {
            self.platform_group = Some(platform_group.to_vec());
            self.run_batch(tag)?;
        }

        if let Some(true) = self.push {
            self.platform_group = self.enabled_platforms.clone();
            return self.run_batch(tag);
        }

        Ok(())
    }

    /// list_image_cache describes buildx images by tag.
    pub fn list_image_cache(&self, tag: &str) -> Result<(), TuggyError> {
        let mut cmd = process::Command::new("docker");
        let base_args: Vec<String> = [
            "buildx",
            "imagetools",
            "inspect",
            "--builder",
            BUILDER_NAME,
            tag,
        ]
        .iter()
        .map(|e| e.to_string())
        .collect();
        let args: Vec<&str> = base_args.iter().map(|e| e.as_ref()).collect::<Vec<&str>>();
        cmd.args(args.as_slice());
        cmd.stdout(process::Stdio::inherit());
        cmd.stderr(process::Stdio::inherit());

        if let Some(true) = self.debug {
            eprintln!("debug: running command: {:?}", cmd);
        }

        let status = cmd
            .status()
            .map_err(|e| TuggyError::IOError(format!("unable to query image cache: {e}")))?;

        if !status.success() {
            return Err(TuggyError::IOError(format!(
                "unable to list image cache for builder: {BUILDER_NAME}"
            )));
        }

        Ok(())
    }
}

/// remove_buildx_image_cache resets buildx image caches.
pub fn remove_buildx_image_cache(debug: Option<bool>) -> Result<(), TuggyError> {
    let mut cmd = process::Command::new("docker");
    cmd.args(["buildx", "prune", "--force"]);
    cmd.stdout(process::Stdio::inherit());
    cmd.stderr(process::Stdio::inherit());

    if let Some(true) = debug {
        eprintln!("debug: running command: {:?}", cmd);
    }

    let status = cmd
        .status()
        .map_err(|e| TuggyError::IOError(e.to_string()))?;

    if !status.success() {
        return Err(TuggyError::IOError(
            "unable to prune image cache".to_string(),
        ));
    }

    Ok(())
}

/// remove_tuggy_builder deletes the tuggy buildx builder.
pub fn remove_tuggy_builder(debug: Option<bool>) -> Result<(), TuggyError> {
    let mut cmd = process::Command::new("docker");
    cmd.args(["buildx", "rm", BUILDER_NAME]);
    cmd.stdout(process::Stdio::inherit());
    cmd.stderr(process::Stdio::inherit());

    if let Some(true) = debug {
        eprintln!("debug: running command: {:?}", cmd);
    }

    let status = cmd
        .status()
        .map_err(|e| TuggyError::IOError(e.to_string()))?;

    if !status.success() {
        return Err(TuggyError::IOError(
            "unable to remove buildx builder".to_string(),
        ));
    }

    Ok(())
}

/// clean resets buildx components.
pub fn clean(debug: Option<bool>) -> Result<(), TuggyError> {
    remove_buildx_image_cache(debug)?;
    remove_tuggy_builder(debug)
}
