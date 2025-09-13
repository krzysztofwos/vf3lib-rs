//! Correctness tests for VF3, VF3L, and VF3P algorithms.
//!
//! Tests the core algorithms with various graph sizes from small to large,
//! verifying that they find the expected isomorphisms.

use std::path::PathBuf;

use vf3lib_rs::{ParallelOptions, RunOptions, run_vf3, run_vf3l, run_vf3p};

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join(name)
}

#[test]
fn vf3_basic_smoke_test() {
    let pattern = fixture_path("bvg1.sub.grf");
    let target = fixture_path("bvg1.grf");

    let opts = RunOptions {
        repetition_time_limit: 0.05,
        ..Default::default()
    };

    let res = run_vf3(
        pattern.to_string_lossy().as_ref(),
        target.to_string_lossy().as_ref(),
        opts,
    )
    .expect("VF3 execution failed");

    assert!(
        res.solutions >= 1,
        "Expected at least one solution, got {}",
        res.solutions
    );
}

#[test]
fn vf3_multiple_small_graphs() {
    let test_cases = vec![
        ("bvg1.sub.grf", "bvg1.grf"),
        ("bvg2.sub.grf", "bvg2.grf"),
        ("m2d1.sub.grf", "m2d1.grf"),
        ("m2d2.sub.grf", "m2d2.grf"),
        ("rand1.sub.grf", "rand1.grf"),
        ("rand2.sub.grf", "rand2.grf"),
        ("rand3.sub.grf", "rand3.grf"),
    ];

    for (pattern_name, target_name) in test_cases {
        let pattern = fixture_path(pattern_name);
        let target = fixture_path(target_name);

        let opts = RunOptions::default();

        let res = run_vf3(
            pattern.to_string_lossy().as_ref(),
            target.to_string_lossy().as_ref(),
            opts,
        )
        .expect(&format!("VF3 failed for {}", pattern_name));
        assert!(
            res.solutions >= 1,
            "No solutions found for {}",
            pattern_name
        );
    }
}

#[test]
fn vf3l_sparse_graphs() {
    // VF3L is optimized for sparse graphs
    let test_cases = vec![
        ("rand1.sub.grf", "rand1.grf"),
        ("rand2.sub.grf", "rand2.grf"),
        ("rand3.sub.grf", "rand3.grf"),
    ];

    for (pattern_name, target_name) in test_cases {
        let pattern = fixture_path(pattern_name);
        let target = fixture_path(target_name);

        let opts = RunOptions::default();

        let res = run_vf3l(
            pattern.to_string_lossy().as_ref(),
            target.to_string_lossy().as_ref(),
            opts,
        )
        .expect(&format!("VF3L failed for {}", pattern_name));
        assert!(
            res.solutions >= 1,
            "No solutions found for {}",
            pattern_name
        );
    }
}

#[test]
fn vf3_medium_si2_datasets() {
    let test_cases = vec![
        ("si2_b03_m400_37.sub.grf", "si2_b03_m400_37.grf"),
        ("si2_b06m_m400_96.sub.grf", "si2_b06m_m400_96.grf"),
    ];

    for (pattern_name, target_name) in test_cases {
        let pattern = fixture_path(pattern_name);
        let target = fixture_path(target_name);

        let opts = RunOptions {
            repetition_time_limit: 0.05,
            first_only: true, // Stop after first solution for medium graphs
            ..Default::default()
        };

        let res = run_vf3(
            pattern.to_string_lossy().as_ref(),
            target.to_string_lossy().as_ref(),
            opts,
        )
        .expect(&format!("VF3 failed for {}", pattern_name));
        assert!(
            res.solutions >= 1,
            "No solutions found for {}",
            pattern_name
        );
    }
}

#[test]
fn vf3_larger_si2_dataset() {
    // Test the larger si2_b03m_m800_22 with sequential VF3
    let pattern = fixture_path("si2_b03m_m800_22.sub.grf");
    let target = fixture_path("si2_b03m_m800_22.grf");

    let opts = RunOptions {
        repetition_time_limit: 0.05,
        first_only: true, // Important for this larger graph
        ..Default::default()
    };

    let res = run_vf3(
        pattern.to_string_lossy().as_ref(),
        target.to_string_lossy().as_ref(),
        opts,
    )
    .expect("VF3 failed for si2_b03m_m800_22");
    assert!(res.solutions >= 1);
}

#[test]
fn vf3_non_isomorphic_pairs() {
    // Test cases where graphs should NOT be isomorphic
    let test_cases = vec![
        ("bvg1_3.sub.grf", "bvg1.grf"), // Different sized patterns
        ("bvg1_4.sub.grf", "bvg1.grf"),
    ];

    for (pattern_name, target_name) in test_cases {
        let pattern = fixture_path(pattern_name);
        let target = fixture_path(target_name);

        let opts = RunOptions::default();

        let _res = run_vf3(
            pattern.to_string_lossy().as_ref(),
            target.to_string_lossy().as_ref(),
            opts,
        )
        .expect(&format!("VF3 failed for {}", pattern_name));
        // These might still have subgraph isomorphisms, just checking it doesn't crash
    }
}

#[cfg(target_os = "linux")]
#[test]
fn vf3p_parallel_small_graphs() {
    let test_cases = vec![
        ("bvg3.sub.grf", "bvg3.grf"),
        ("m2d1.sub.grf", "m2d1.grf"),
        ("m2d2.sub.grf", "m2d2.grf"),
    ];

    let par = ParallelOptions {
        num_threads: 2,
        ..Default::default()
    };

    for (pattern_name, target_name) in test_cases {
        let pattern = fixture_path(pattern_name);
        let target = fixture_path(target_name);

        let opts = RunOptions::default();

        let res = run_vf3p(
            pattern.to_string_lossy().as_ref(),
            target.to_string_lossy().as_ref(),
            opts,
            par.clone(),
        )
        .expect(&format!("VF3P failed for {}", pattern_name));
        assert!(
            res.solutions >= 1,
            "No solutions found for {}",
            pattern_name
        );
    }
}

#[cfg(target_os = "linux")]
#[test]
fn vf3p_parallel_medium_datasets() {
    let pattern = fixture_path("si2_b03_m400_37.sub.grf");
    let target = fixture_path("si2_b03_m400_37.grf");

    let opts = RunOptions {
        repetition_time_limit: 0.05,
        first_only: true,
        ..Default::default()
    };
    let par = ParallelOptions {
        num_threads: 2,
        algo: 2, // WLS algorithm variant
        ..Default::default()
    };

    let res = run_vf3p(
        pattern.to_string_lossy().as_ref(),
        target.to_string_lossy().as_ref(),
        opts,
        par,
    )
    .expect("VF3P failed");
    assert!(res.solutions >= 1);
}

#[cfg(target_os = "linux")]
#[test]
fn vf3p_parallel_lockfree() {
    let pattern = fixture_path("bvg2.sub.grf");
    let target = fixture_path("bvg2.grf");

    let opts = RunOptions::default();
    let par = ParallelOptions {
        num_threads: 2,
        lock_free: true,
        ..Default::default()
    };

    let res = run_vf3p(
        pattern.to_string_lossy().as_ref(),
        target.to_string_lossy().as_ref(),
        opts,
        par,
    )
    .expect("VF3P lockfree failed");
    assert!(res.solutions >= 1);
}
