#pragma once
#include <string>

#include "ffi.hpp"

namespace rust
{
inline void init()
{
    rust_ffi::init();
}

template<typename ... Args>
void trace(const std::string& format, Args ... args)
{
    auto msg = std::vformat(format, std::make_format_args(args...));
    rust_ffi::info(msg.c_str());
}

template<typename ... Args>
void debug(const std::string& format, Args ... args)
{
    auto msg = std::vformat(format, std::make_format_args(args...));
    rust_ffi::info(msg.c_str());
}

template<typename ... Args>
void info(std::string_view format, Args&&... args)
{
    auto msg = std::vformat(format, std::make_format_args(args...));
    rust_ffi::info(msg.c_str());
}

template<typename ... Args>
void warn(const std::string& format, Args ... args)
{
    auto msg = std::vformat(format, std::make_format_args(args...));
    rust_ffi::info(msg.c_str());
}

template<typename ... Args>
void error(const std::string& format, Args ... args)
{
    auto msg = std::vformat(format, std::make_format_args(args...));
    rust_ffi::info(msg.c_str());
}
}