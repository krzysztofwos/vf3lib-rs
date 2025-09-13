// C++ header for VF3 algorithm bridge functions (CXX interop).

#ifndef VF3_BRIDGE_HPP
#define VF3_BRIDGE_HPP

#include "rust/cxx.h"

namespace vf3ffi {

struct VF3Result;

VF3Result run_vf3(
    rust::Str pattern,
    rust::Str target,
    rust::Str format,
    bool undirected,
    bool store_solutions,
    bool first_only,
    bool verbose,
    float repetition_time_limit,
    bool edge_induced);

VF3Result run_vf3l(
    rust::Str pattern,
    rust::Str target,
    rust::Str format,
    bool undirected,
    bool store_solutions,
    bool first_only,
    bool verbose,
    float repetition_time_limit,
    bool edge_induced);

VF3Result run_vf3p(
    rust::Str pattern,
    rust::Str target,
    rust::Str format,
    bool undirected,
    bool store_solutions,
    bool verbose,
    float repetition_time_limit,
    bool edge_induced,
    std::int8_t algo,
    std::int16_t cpu,
    std::int16_t num_threads,
    bool lock_free,
    std::int16_t ssr_high_limit,
    std::int16_t ssr_local_stack_limit);

}  // namespace vf3ffi

#endif  // VF3_BRIDGE_HPP
