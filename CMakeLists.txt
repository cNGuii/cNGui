cmake_minimum_required(VERSION 3.20)
project(cNGui VERSION 0.1.0 LANGUAGES C CXX)

set(CMAKE_C_STANDARD 11)
set(CMAKE_CXX_STANDARD 17)

# --- 1. Build the Rust Core ---
# We define a custom target that triggers `cargo build` and outputs the static library.
set(RUST_LIB_PATH "${CMAKE_CURRENT_SOURCE_DIR}/target/debug/libcngui.a")

add_custom_command(
    OUTPUT ${RUST_LIB_PATH}
    COMMAND cargo build
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
    COMMENT "Building Rust core..."
    VERBATIM
)

add_custom_target(rust_core DEPENDS ${RUST_LIB_PATH})

# Import the Rust static library as an interface we can link against
add_library(cngui_rust STATIC IMPORTED GLOBAL)
set_target_properties(cngui_rust PROPERTIES IMPORTED_LOCATION ${RUST_LIB_PATH})
add_dependencies(cngui_rust rust_core)

# --- 2. Build the C Platform Layer ---
# This contains the Win32/X11/Headless window creation code.
file(GLOB PLATFORM_SOURCES "platform/*.c")
add_library(cngui_platform STATIC ${PLATFORM_SOURCES})
target_include_directories(cngui_platform PUBLIC include)

# --- 3. Build the Examples ---
option(CNGUI_BUILD_EXAMPLES "Build cNGui examples" ON)

if(CNGUI_BUILD_EXAMPLES)
    # C Example
    add_executable(example_c examples/c/main.c)
    target_link_libraries(example_c PRIVATE cngui_platform cngui_rust)
    
    # C++ Example (uses the RAII wrapper)
    add_executable(example_cpp examples/cpp/main.cpp)
    target_link_libraries(example_cpp PRIVATE cngui_platform cngui_rust)
endif()
