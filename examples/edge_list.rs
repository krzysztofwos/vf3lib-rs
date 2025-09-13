//! Example of using edge list format for graph input.

use std::{
    fs,
    path::{Path, PathBuf},
};

use vf3lib_rs::{GraphFormat, RunOptions, run_vf3};

fn write_file(dir: &Path, name: &str, content: &str) -> PathBuf {
    let path = dir.join(name);
    fs::write(&path, content).expect("Failed to write test graph");
    path
}

fn main() {
    // Create edge-list graphs in temp directory.
    // Format: one-based node IDs, "u v" per line, '#' for comments.
    let mut dir = std::env::temp_dir();
    dir.push("vf3_edge_list_example");
    fs::create_dir_all(&dir).expect("Failed to create temp directory");

    // Pattern: triangle (nodes 1-2-3).
    let pattern_txt = "# Triangle pattern\n1 2\n2 3\n1 3\n";
    // Target: triangle with additional edge (3-4).
    let target_txt = "# Triangle plus tail\n1 2\n2 3\n1 3\n3 4\n";

    let pattern = write_file(&dir, "pattern.edgelist", pattern_txt);
    let target = write_file(&dir, "target.edgelist", target_txt);

    let opts = RunOptions {
        format: GraphFormat::EdgeList,
        undirected: true,
        repetition_time_limit: 0.25,
        ..Default::default()
    };

    match run_vf3(
        pattern.to_string_lossy().as_ref(),
        target.to_string_lossy().as_ref(),
        opts,
    ) {
        Ok(res) => println!(
            "Solutions: {}, Time to first: {:.3}s, Total time: {:.3}s",
            res.solutions, res.time_first, res.time_all
        ),
        Err(e) => eprintln!("Error: {e}"),
    }
}
