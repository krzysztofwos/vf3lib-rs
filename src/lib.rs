//! Rust bindings to VF3/VF3L/VF3P subgraph isomorphism algorithms via CXX.
//!
//! This crate provides efficient subgraph isomorphism detection using the VF3 family of algorithms.
//! The underlying implementation is a C++11 library from MIVIA Lab.
//!
//! # Examples
//!
//! Node-induced subgraph isomorphism:
//! ```no_run
//! use vf3lib_rs::{run_vf3, RunOptions};
//! let result = run_vf3("pattern.grf", "target.grf", RunOptions::default())?;
//! # Ok::<(), vf3lib_rs::VF3Error>(())
//! ```
//!
//! Edge-induced (monomorphism):
//! ```no_run
//! use vf3lib_rs::{run_vf3, RunOptions};
//! let opts = RunOptions { edge_induced: true, ..Default::default() };
//! let result = run_vf3("pattern.grf", "target.grf", opts)?;
//! # Ok::<(), vf3lib_rs::VF3Error>(())
//! ```

use thiserror::Error;

/// Errors that can occur during VF3 algorithm execution.
#[derive(Error, Debug)]
pub enum VF3Error {
    /// The VF3 algorithm execution failed with a non-zero status code.
    #[error("VF3 execution failed with status code {code}")]
    ExecutionFailed {
        /// The status code returned by the C++ implementation.
        code: i32,
    },

    /// An error occurred in the FFI layer.
    #[error("FFI error: {message}")]
    FfiError {
        /// Description of the FFI error.
        message: String,
    },

    /// The specified graph format is not supported.
    #[error("Unsupported graph format: {format}")]
    UnsupportedFormat {
        /// The format string that was provided.
        format: String,
    },
}

// Skip C++ compilation on docs.rs to avoid build failures.
#[cfg(not(docsrs))]
#[cxx::bridge(namespace = "vf3ffi")]
#[allow(clippy::too_many_arguments)]
mod vf3ffi {
    /// Execution result from C++ VF3 algorithms.
    #[derive(Debug, Clone)]
    pub struct VF3Result {
        /// Status code: 0 on success, non-zero on error.
        pub status: i32,
        /// Number of isomorphic mappings found.
        pub solutions: u64,
        /// Time to first solution in seconds.
        pub time_first: f64,
        /// Average total execution time in seconds.
        pub time_all: f64,
    }

    unsafe extern "C++" {
        include!("vf3_bridge.hpp");

        /// VF3 algorithm with all heuristics (best for medium/large dense graphs).
        fn run_vf3(
            pattern: &str,
            target: &str,
            format: &str,
            undirected: bool,
            store_solutions: bool,
            first_only: bool,
            verbose: bool,
            repetition_time_limit: f32,
            edge_induced: bool,
        ) -> VF3Result;

        /// VF3L lightweight variant without look-ahead (best for small/sparse graphs).
        fn run_vf3l(
            pattern: &str,
            target: &str,
            format: &str,
            undirected: bool,
            store_solutions: bool,
            first_only: bool,
            verbose: bool,
            repetition_time_limit: f32,
            edge_induced: bool,
        ) -> VF3Result;

        /// VF3P parallel variant for multi-threaded execution.
        fn run_vf3p(
            pattern: &str,
            target: &str,
            format: &str,
            undirected: bool,
            store_solutions: bool,
            verbose: bool,
            repetition_time_limit: f32,
            edge_induced: bool,
            algo: i8,
            cpu: i16,
            num_threads: i16,
            lock_free: bool,
            ssr_high_limit: i16,
            ssr_local_stack_limit: i16,
        ) -> VF3Result;
    }
}

#[cfg(docsrs)]
mod vf3ffi {
    #[derive(Debug, Clone)]
    pub struct VF3Result {
        pub status: i32,
        pub solutions: u64,
        pub time_first: f64,
        pub time_all: f64,
    }
}

/// Graph file format for loading.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphFormat {
    /// VF text/binary format used by MIVIA datasets (.grf files).
    VFLegacy,
    /// Simple edge list format (one edge per line as "u v").
    EdgeList,
}

impl GraphFormat {
    fn as_str(self) -> &'static str {
        match self {
            GraphFormat::VFLegacy => "vf",
            GraphFormat::EdgeList => "edge",
        }
    }
}

/// Configuration options for VF3 algorithm execution.
#[derive(Debug, Clone)]
pub struct RunOptions {
    /// Graph file format.
    pub format: GraphFormat,
    /// Treat graphs as undirected.
    pub undirected: bool,
    /// Store all solution mappings in memory (may use significant memory for large result sets).
    pub store_solutions: bool,
    /// Stop after finding the first solution (sequential algorithms only).
    pub first_only: bool,
    /// Enable verbose output.
    pub verbose: bool,
    /// Minimum execution time in seconds for averaging multiple runs.
    pub repetition_time_limit: f32,
    /// Use edge-induced isomorphism (monomorphism) instead of node-induced.
    pub edge_induced: bool,
}

impl Default for RunOptions {
    fn default() -> Self {
        Self {
            format: GraphFormat::VFLegacy,
            undirected: false,
            store_solutions: false,
            first_only: false,
            verbose: false,
            repetition_time_limit: 1.0,
            edge_induced: false,
        }
    }
}

/// Configuration options for parallel VF3P execution.
#[derive(Debug, Clone)]
pub struct ParallelOptions {
    /// Algorithm variant: 1 = GSS (Global State Stack), 2 = WLS (Work-stealing with Local Stack).
    pub algo: i8,
    /// Starting CPU for thread pinning (-1 disables pinning).
    pub cpu: i16,
    /// Number of worker threads (must be >= 1).
    pub num_threads: i16,
    /// Use lock-free stack implementation.
    pub lock_free: bool,
    /// Depth limit for forcing states into global stack (WLS only).
    pub ssr_high_limit: i16,
    /// Maximum size of thread-local stack (WLS only).
    pub ssr_local_stack_limit: i16,
}

impl Default for ParallelOptions {
    fn default() -> Self {
        Self {
            algo: 1,
            cpu: -1,
            num_threads: 1,
            lock_free: false,
            ssr_high_limit: 3,
            ssr_local_stack_limit: 10,
        }
    }
}

/// Results from VF3 algorithm execution.
#[derive(Debug, Clone)]
pub struct ResultData {
    /// Number of isomorphic mappings found.
    pub solutions: u64,
    /// Time to first solution in seconds.
    pub time_first: f64,
    /// Average total execution time in seconds.
    pub time_all: f64,
}

#[cfg(not(docsrs))]
fn convert_result(res: vf3ffi::VF3Result) -> Result<ResultData, VF3Error> {
    if res.status == 0 {
        Ok(ResultData {
            solutions: res.solutions,
            time_first: res.time_first,
            time_all: res.time_all,
        })
    } else {
        Err(VF3Error::ExecutionFailed { code: res.status })
    }
}

/// Run VF3 algorithm with full heuristics.
///
/// Best suited for medium to large dense graphs.
///
/// # Errors
///
/// Returns [`VF3Error::ExecutionFailed`] if the C++ algorithm fails.
pub fn run_vf3(pattern: &str, target: &str, opts: RunOptions) -> Result<ResultData, VF3Error> {
    #[cfg(not(docsrs))]
    {
        let res = vf3ffi::run_vf3(
            pattern,
            target,
            opts.format.as_str(),
            opts.undirected,
            opts.store_solutions,
            opts.first_only,
            opts.verbose,
            opts.repetition_time_limit,
            opts.edge_induced,
        );
        convert_result(res)
    }
    #[cfg(docsrs)]
    {
        let _ = (pattern, target, opts);
        Err(VF3Error::FfiError {
            message: "VF3 not available in docs.rs build".into(),
        })
    }
}

/// Run VF3L lightweight variant without look-ahead heuristic.
///
/// Best suited for small or sparse graphs.
///
/// # Errors
///
/// Returns [`VF3Error::ExecutionFailed`] if the C++ algorithm fails.
pub fn run_vf3l(pattern: &str, target: &str, opts: RunOptions) -> Result<ResultData, VF3Error> {
    #[cfg(not(docsrs))]
    {
        let res = vf3ffi::run_vf3l(
            pattern,
            target,
            opts.format.as_str(),
            opts.undirected,
            opts.store_solutions,
            opts.first_only,
            opts.verbose,
            opts.repetition_time_limit,
            opts.edge_induced,
        );
        convert_result(res)
    }
    #[cfg(docsrs)]
    {
        let _ = (pattern, target, opts);
        Err(VF3Error::FfiError {
            message: "VF3L not available in docs.rs build".into(),
        })
    }
}

/// Run VF3P parallel variant with multi-threading support.
///
/// Best suited for computationally hard instances that benefit from parallelization.
///
/// # Errors
///
/// Returns [`VF3Error::ExecutionFailed`] if the C++ algorithm fails.
pub fn run_vf3p(
    pattern: &str,
    target: &str,
    opts: RunOptions,
    par: ParallelOptions,
) -> Result<ResultData, VF3Error> {
    #[cfg(not(docsrs))]
    {
        let res = vf3ffi::run_vf3p(
            pattern,
            target,
            opts.format.as_str(),
            opts.undirected,
            opts.store_solutions,
            opts.verbose,
            opts.repetition_time_limit,
            opts.edge_induced,
            par.algo,
            par.cpu,
            par.num_threads,
            par.lock_free,
            par.ssr_high_limit,
            par.ssr_local_stack_limit,
        );
        convert_result(res)
    }
    #[cfg(docsrs)]
    {
        let _ = (pattern, target, opts, par);
        Err(VF3Error::FfiError {
            message: "VF3P not available in docs.rs build".into(),
        })
    }
}

/// Builder for configuring and executing VF3 subgraph isomorphism queries.
///
/// Provides a fluent API for setting options and choosing algorithm variants.
///
/// # Examples
///
/// ```no_run
/// use vf3lib_rs::VF3Query;
///
/// // Simple usage with default settings
/// let result = VF3Query::new("pattern.grf", "target.grf")
///     .run()?;
///
/// // Edge-induced matching with VF3L variant
/// let result = VF3Query::new("pattern.grf", "target.grf")
///     .edge_induced()
///     .undirected()
///     .run_light()?;
///
/// // Parallel execution with custom thread count
/// let result = VF3Query::new("pattern.grf", "target.grf")
///     .with_threads(4)
///     .run_parallel()?;
/// # Ok::<(), vf3lib_rs::VF3Error>(())
/// ```
pub struct VF3Query<'a> {
    pattern: &'a str,
    target: &'a str,
    options: RunOptions,
    parallel: ParallelOptions,
}

impl<'a> VF3Query<'a> {
    /// Create a new query with the given pattern and target graph files.
    pub fn new(pattern: &'a str, target: &'a str) -> Self {
        Self {
            pattern,
            target,
            options: RunOptions::default(),
            parallel: ParallelOptions::default(),
        }
    }

    /// Set the graph file format.
    pub fn format(mut self, format: GraphFormat) -> Self {
        self.options.format = format;
        self
    }

    /// Treat graphs as undirected.
    pub fn undirected(mut self) -> Self {
        self.options.undirected = true;
        self
    }

    /// Treat graphs as directed (default).
    pub fn directed(mut self) -> Self {
        self.options.undirected = false;
        self
    }

    /// Use edge-induced isomorphism (monomorphism) instead of node-induced.
    pub fn edge_induced(mut self) -> Self {
        self.options.edge_induced = true;
        self
    }

    /// Use node-induced isomorphism (default).
    pub fn node_induced(mut self) -> Self {
        self.options.edge_induced = false;
        self
    }

    /// Store all solution mappings in memory.
    ///
    /// Warning: This may use significant memory for large result sets.
    pub fn store_solutions(mut self) -> Self {
        self.options.store_solutions = true;
        self
    }

    /// Stop after finding the first solution (sequential algorithms only).
    pub fn first_only(mut self) -> Self {
        self.options.first_only = true;
        self
    }

    /// Enable verbose output.
    pub fn verbose(mut self) -> Self {
        self.options.verbose = true;
        self
    }

    /// Set minimum execution time in seconds for averaging multiple runs.
    pub fn repetition_time_limit(mut self, seconds: f32) -> Self {
        self.options.repetition_time_limit = seconds;
        self
    }

    /// Set the number of worker threads for parallel execution.
    pub fn with_threads(mut self, num_threads: i16) -> Self {
        self.parallel.num_threads = num_threads;
        self
    }

    /// Set the parallel algorithm variant.
    ///
    /// * `1` - GSS (Global State Stack)
    /// * `2` - WLS (Work-stealing with Local Stack)
    pub fn parallel_algorithm(mut self, algo: i8) -> Self {
        self.parallel.algo = algo;
        self
    }

    /// Enable lock-free stack implementation for parallel execution.
    pub fn lock_free(mut self) -> Self {
        self.parallel.lock_free = true;
        self
    }

    /// Run the VF3 algorithm with full heuristics.
    ///
    /// Best suited for medium to large dense graphs.
    ///
    /// # Errors
    ///
    /// Returns [`VF3Error::ExecutionFailed`] if the algorithm fails.
    pub fn run(self) -> Result<ResultData, VF3Error> {
        run_vf3(self.pattern, self.target, self.options)
    }

    /// Run the VF3L lightweight variant without look-ahead heuristic.
    ///
    /// Best suited for small or sparse graphs.
    ///
    /// # Errors
    ///
    /// Returns [`VF3Error::ExecutionFailed`] if the algorithm fails.
    pub fn run_light(self) -> Result<ResultData, VF3Error> {
        run_vf3l(self.pattern, self.target, self.options)
    }

    /// Run the VF3P parallel variant with multi-threading support.
    ///
    /// Best suited for computationally hard instances.
    ///
    /// # Errors
    ///
    /// Returns [`VF3Error::ExecutionFailed`] if the algorithm fails.
    pub fn run_parallel(self) -> Result<ResultData, VF3Error> {
        run_vf3p(self.pattern, self.target, self.options, self.parallel)
    }
}
