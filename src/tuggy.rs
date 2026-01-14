//! CLI tuggy tool

extern crate getopts;
extern crate regex;
extern crate tuggy;

use die::{Die, die};
use std::env;
use std::fs;
use std::process;

/// CLI entrypoint
fn main() {
    let brief: String = format!(
        "Usage: {} [OPTIONS] [<DOCKER WORKING DIRECTORY>] [-- <DOCKER BUILDX OPTIONS>]",
        env!("CARGO_PKG_NAME")
    );

    let mut opts: getopts::Options = getopts::Options::new();
    opts.optflag(
        "",
        "list",
        "list operation. enumerate buildx cache for the given image name, of the form name[:tag]",
    );
    opts.optflag(
        "",
        "load",
        "load operation. copy buildx image of the given platform into local Docker registry as a side effect. mutually exclusive of --push.",
    );
    opts.optflag(
        "",
        "push",
        "push operation. publish all Docker image artifacts to Docker registry as a side effect. mutually exclusive of --load.",
    );
    opts.optflag(
        "",
        "get-platforms",
        "get platforms operation. enumerate available buildx platforms",
    );
    opts.optopt(
        "C",
        "directory",
        "Docker working directory (default: current working directory)",
        "<dir>",
    );
    opts.optopt(
        "a",
        "aliases",
        "create tag aliases as a side effect (comma separated). requires --push.",
        "<aliases>",
    );
    opts.optopt(
        "c",
        "configuration",
        "customize configuration file path (default: tuggy.toml)",
        "<path>",
    );
    opts.optflag(
        "",
        "clean",
        "cleanup operatino. remove tuggy buildx builder",
    );
    opts.optflag("d", "debug", "enable additional logging");
    opts.optopt(
        "f",
        "file",
        "Dockerfile source file path (default: Dockerfile)",
        "<path>",
    );
    opts.optopt(
        "j",
        "jobs",
        &format!(
            "Concurrent build job limit. Zero indicates no restriction. (default: {})",
            tuggy::DEFAULT_JOBS_LIMIT
        ),
        "<limit>",
    );
    opts.optflag("h", "help", "usage operation. print usage info");
    opts.optopt(
        "t",
        "tag",
        "docker image name, of the form name[:tag]",
        "<tag>",
    );
    opts.optflag("v", "version", "version operation. print version info");

    let usage: String = opts.usage(&brief);
    let arguments: Vec<String> = env::args().collect();
    let optmatches: getopts::Matches = opts.parse(&arguments[1..]).die(&usage);

    if optmatches.opt_present("h") {
        die!(0; usage);
    }

    if optmatches.opt_present("v") {
        die!(0; format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")));
    }

    let mut debug: Option<bool> = None;

    if optmatches.opt_present("d") {
        debug = Some(true);
    }

    if optmatches.opt_present("clean") {
        match tuggy::clean(debug) {
            Err(e) => die!(1; format!("error: {e}")),
            _ => die!(0),
        };
    }

    let mut ty = tuggy::Tuggy::default();

    if optmatches.opt_present("c") {
        match optmatches.opt_str("c") {
            Some(pth) => {
                ty = match tuggy::Tuggy::load(&pth) {
                    Err(e) => die!(1; format!("error: {e}")),
                    Ok(e) => e,
                }
            }
            _ => {
                eprintln!("missing value for -c <path>");
                die!(1; usage);
            }
        };
    } else {
        let config_path_exists = match fs::exists(tuggy::CONFIGURATION_FILENAME) {
            Err(e) => die!(1; format!("error: {e}")),
            Ok(e) => e,
        };

        if config_path_exists {
            ty = match tuggy::Tuggy::load(tuggy::CONFIGURATION_FILENAME) {
                Err(e) => die!(1; format!("error: {e}")),
                Ok(e) => e,
            }
        };
    };

    if let Some(true) = debug {
        ty.debug = debug;
    }

    if optmatches.opt_present("C") {
        ty.directory = match optmatches.opt_str("C") {
            None => {
                eprintln!("error: missing value in -C <dir>");
                die!(1; usage);
            }
            e => e,
        };
    }

    let buildx_args: Vec<String> = if !arguments.contains(&"--".to_string()) {
        vec![]
    } else {
        optmatches.free.clone()
    };

    if !buildx_args.is_empty() {
        ty.buildx_args = Some(buildx_args);
    }

    if let Some(true) = ty.debug {
        eprintln!("debug: configuration: {:?}", ty);
    }

    if optmatches.opt_present("get-platforms") {
        match ty.get_platforms() {
            Err(e) => die!(1; "error: {}", e),
            Ok(l) => {
                for platform in l {
                    println!("{}", platform);
                }

                println!();
                println!(
                    "niche (disabled by default): {}",
                    &tuggy::DEFAULT_SKIP_PLATFORMS.join(",")
                );
                process::exit(0);
            }
        }
    }

    let tag: String = match optmatches.opt_str("t") {
        Some(e) => e,
        _ => {
            eprintln!("error: missing flag -t <tag>");
            die!(1; usage);
        }
    };

    if optmatches.opt_present("list") {
        match ty.list_image_cache(&tag) {
            Err(e) => die!(1; format!("error: {e}")),
            _ => die!(0),
        };
    }

    if optmatches.opt_present("j") {
        match optmatches.opt_str("j") {
            Some(e) => match e.parse::<usize>() {
                Err(_) => die!(1; format!("error: invalid limit: {e}")),
                Ok(e) => ty.jobs_limit = Some(e),
            },
            _ => {
                eprintln!("error: missing value in -j <limit>");
                die!(1; usage);
            }
        };
    }

    if optmatches.opt_present("f") {
        ty.dockerfile = match optmatches.opt_str("f") {
            None => {
                eprintln!("error: missing value in -f <dockerfile>");
                die!(1; usage);
            }
            e => e,
        };
    }

    if optmatches.opt_present("a") {
        if !optmatches.opt_present("push") {
            eprintln!("aliasing requires pushing");
            die!(1; usage);
        }

        ty.aliases = match optmatches.opt_str("a") {
            Some(e) => Some(e.split(",").map(|e| e.to_string()).collect::<Vec<String>>()),
            _ => {
                eprintln!("error: missing value for -a <aliases>");
                die!(1; usage);
            }
        };
    }

    if optmatches.opt_present("load") && optmatches.opt_present("push") {
        eprintln!("--load, --push are mutually exclusive operations");
        die!(usage);
    }

    if optmatches.opt_present("load") {
        ty.load = Some(true);
    }

    if optmatches.opt_present("push") {
        ty.push = Some(true);
    }

    if let Err(e) = ty.build(&tag) {
        die!(1; format!("error: {e}"));
    }
}
