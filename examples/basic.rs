//! Basic example of using VF3 for node-induced subgraph isomorphism.

use std::path::PathBuf;

use vf3lib_rs::{RunOptions, run_vf3};

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
        repetition_time_limit: 0.5,
        ..Default::default()
    };

    match run_vf3(pattern.as_str(), target.as_str(), opts) {
        Ok(res) => println!(
            "Solutions: {}, Time to first: {:.3}s, Total time: {:.3}s",
            res.solutions, res.time_first, res.time_all
        ),
        Err(e) => eprintln!("Error: {e}"),
    }
}
