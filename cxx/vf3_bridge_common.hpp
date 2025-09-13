// Shared declarations for VF3 bridge C++ implementations.

#ifndef VF3_BRIDGE_COMMON_HPP
#define VF3_BRIDGE_COMMON_HPP

#include <cstdint>

namespace vf3ffi {

struct VF3Result {
    std::int32_t status;
    std::uint64_t solutions;
    double time_first;
    double time_all;
};

}  // namespace vf3ffi

#endif  // VF3_BRIDGE_COMMON_HPP
