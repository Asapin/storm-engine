#pragma once

#include "ffi.hpp"

namespace rust
{
void init();

template<typename ... Args>
void trace(const std::string& format, Args ... args);

template<typename ... Args>
void debug(const std::string& format, Args ... args);

template<typename ... Args>
void info(const std::string& format, Args ... args);

template<typename ... Args>
void warn(const std::string& format, Args ... args);

template<typename ... Args>
void error(const std::string& format, Args ... args);
}