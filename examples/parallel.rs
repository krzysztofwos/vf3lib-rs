//! Example of VF3P parallel variant for multi-threaded execution.

use std::path::PathBuf;

use vf3lib_rs::{ParallelOptions, RunOptions, run_vf3p};

const FIXTURE_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data");

fn fixture_path(name: &str) -> String {
    let path = PathBuf::from(FIXTURE_DIR).join(name);
    if !path.exists() {
        panic!("missing bundled fixture: {}", path.display());
    }
    path.to_string_lossy().into_owned()
}

fn main() {
    let pattern = fixture_path("bvg1.sub.grf");
    let target = fixture_path("bvg1.grf");

    let opts = RunOptions {
        repetition_time_limit: 0.25,
        ..Default::default()
    };

    // Configure parallel execution.
    let par = ParallelOptions {
        num_threads: 4,
        ..Default::default()
    };

    match run_vf3p(pattern.as_str(), target.as_str(), opts, par) {
        Ok(res) => println!(
            "Solutions: {}, Time to first: {:.3}s, Total time: {:.3}s",
            res.solutions, res.time_first, res.time_all
        ),
        Err(e) => eprintln!("Error: {e}"),
    }
}
