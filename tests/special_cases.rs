//! Special cases, configurations, and error handling tests.
//!
//! Tests edge-induced mode, alternative graph formats, configuration options
//! (first_only, store_solutions), parallel variants, and error conditions.

mod common;

use std::{
    fs,
    path::{Path, PathBuf},
};

use common::fixture_pair;
use vf3lib_rs::{GraphFormat, RunOptions, run_vf3, run_vf3l};
#[cfg(target_os = "linux")]
use vf3lib_rs::{ParallelOptions, run_vf3p};

fn write_tmp(dir: &Path, name: &str, content: &str) -> PathBuf {
    let path = dir.join(name);
    fs::write(&path, content).expect("Failed to write temp file");
    path
}

fn default_bvg_pair() -> (String, String) {
    fixture_pair("bvg1.sub.grf", "bvg1.grf")
}

fn bvg2_pair() -> (String, String) {
    fixture_pair("bvg2.sub.grf", "bvg2.grf")
}

fn rand1_pair() -> (String, String) {
    fixture_pair("rand1.sub.grf", "rand1.grf")
}

#[test]
fn edge_list_undirected_triangle() {
    // Use a more unique directory name to avoid collisions
    let mut dir = std::env::temp_dir();
    let unique_name = format!(
        "vf3_edge_list_tests_{}_{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    );
    dir.push(unique_name);

    // Clean up any existing directory first
    if dir.exists() {
        let _ = fs::remove_dir_all(&dir);
    }
    fs::create_dir_all(&dir).expect("Failed to create test directory");

    // Match the format that works in the example - include comments
    let pattern_content = "# Triangle pattern\n1 2\n2 3\n1 3\n";
    let target_content = "# Triangle with tail\n1 2\n2 3\n1 3\n3 4\n";

    let patt = write_tmp(&dir, "pattern.edgelist", pattern_content);
    let targ = write_tmp(&dir, "target.edgelist", target_content);

    let opts = RunOptions {
        format: GraphFormat::EdgeList,
        undirected: true,
        repetition_time_limit: 0.02,
        ..Default::default()
    };
    let res = run_vf3(
        patt.to_string_lossy().as_ref(),
        targ.to_string_lossy().as_ref(),
        opts,
    )
    .expect("Edge list execution failed");
    assert!(
        res.solutions >= 1,
        "Expected at least one triangle, got {} solutions. Pattern: {:?}, Target: {:?}",
        res.solutions,
        patt,
        targ
    );

    // Clean up temp directory
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn vf3_edge_induced_smoke() {
    let (pattern, target) = default_bvg_pair();
    let opts = RunOptions {
        edge_induced: true,
        repetition_time_limit: 0.02,
        ..Default::default()
    };
    let res = run_vf3(&pattern, &target, opts).expect("VF3 edge-induced failed");
    assert!(res.solutions >= 1);
}

#[test]
fn vf3l_edge_induced_smoke() {
    let (pattern, target) = default_bvg_pair();
    let opts = RunOptions {
        edge_induced: true,
        repetition_time_limit: 0.02,
        ..Default::default()
    };
    let res = run_vf3l(&pattern, &target, opts).expect("VF3L edge-induced failed");
    assert!(res.solutions >= 1);
}

#[test]
fn vf3_first_only_time_equals_all() {
    let (pattern, target) = default_bvg_pair();
    // With first_only=true, time_first should equal time_all.
    let opts = RunOptions {
        first_only: true,
        repetition_time_limit: 0.02,
        ..Default::default()
    };
    let res = run_vf3(&pattern, &target, opts).expect("VF3 first_only failed");
    assert!(res.solutions >= 1);
    assert!(
        (res.time_first - res.time_all).abs() < 1e-12,
        "Expected time_first == time_all, got {} vs {}",
        res.time_first,
        res.time_all
    );
}

#[test]
fn vf3_store_solutions_no_count_change() {
    let (pattern, target) = default_bvg_pair();
    let base = RunOptions {
        repetition_time_limit: 0.02,
        ..Default::default()
    };
    let a = run_vf3(&pattern, &target, base.clone()).expect("VF3 run A failed");
    let mut bopts = base;
    bopts.store_solutions = true;
    let b = run_vf3(&pattern, &target, bopts).expect("VF3 run B failed");
    assert_eq!(
        a.solutions, b.solutions,
        "store_solutions flag must not change solution count"
    );
}

#[cfg(target_os = "linux")]
#[test]
fn vf3p_wls_lockfree_smoke() {
    let (pattern, target) = default_bvg_pair();
    let opts = RunOptions {
        repetition_time_limit: 0.02,
        ..Default::default()
    };
    let par = ParallelOptions {
        algo: 2, // WLS algorithm.
        num_threads: 2,
        lock_free: true,
        ..Default::default()
    };
    let res = run_vf3p(&pattern, &target, opts, par).expect("VF3P WLS failed");
    assert!(res.solutions >= 1);
}

#[test]
fn vf3_bvg2_node_induced() {
    let (pattern, target) = bvg2_pair();
    let opts = RunOptions {
        repetition_time_limit: 0.05,
        ..Default::default()
    };
    let res = run_vf3(&pattern, &target, opts).expect("VF3 bvg2 failed");
    assert!(res.solutions >= 1);
}

#[test]
fn vf3l_rand1_sparse_graph() {
    let (pattern, target) = rand1_pair();
    let opts = RunOptions {
        repetition_time_limit: 0.05,
        ..Default::default()
    };
    let res = run_vf3l(&pattern, &target, opts).expect("VF3L rand1 failed");
    assert!(res.solutions >= 1);
}

#[test]
fn vf3_rand1_edge_induced() {
    let (pattern, target) = rand1_pair();
    let opts = RunOptions {
        edge_induced: true,
        repetition_time_limit: 0.05,
        ..Default::default()
    };
    let res = run_vf3(&pattern, &target, opts).expect("VF3 rand1 edge-induced failed");
    assert!(res.solutions >= 1);
}

#[test]
fn bad_paths_return_error() {
    let opts = RunOptions {
        repetition_time_limit: 0.01,
        ..Default::default()
    };
    let err = run_vf3("/no/such/file/sub.grf", "/no/such/file/grf", opts).unwrap_err();
    // Check that the error is an ExecutionFailed variant
    match err {
        vf3lib_rs::VF3Error::ExecutionFailed { .. } => {
            // Expected error type
        }
        _ => panic!("Unexpected error type: {err:?}"),
    }
}
