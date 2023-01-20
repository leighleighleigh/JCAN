cmake_minimum_required(VERSION 3.22)
project(jorzacan)

# ROS Foxy / Ubuntu 20 targets C++14
set(CMAKE_CXX_STANDARD 14)

set(CARGO_MANIFEST ${CMAKE_SOURCE_DIR}/Cargo.toml)
set(CARGO_TARGET_DIR ${CMAKE_SOURCE_DIR}/target)

set(JORZACAN_SOURCE_FILE ${CMAKE_SOURCE_DIR}/src/lib.rs)
set(JORZACAN_BRIDGE_CPP ${CARGO_TARGET_DIR}/cxxbridge/jorzacan/src/lib.rs.cc)
set(JORZACAN_LIB ${CARGO_TARGET_DIR}/debug/${CMAKE_STATIC_LIBRARY_PREFIX}jorzacan${CMAKE_STATIC_LIBRARY_SUFFIX})

# Add a custom command that builds the rust crate and generates C++ bridge code
add_custom_command(
        OUTPUT ${JORZACAN_BRIDGE_CPP} ${JORZACAN_LIB}
        COMMAND cargo build --manifest-path ${CARGO_MANIFEST}
        DEPENDS ${JORZACAN_SOURCE_FILE}
        USES_TERMINAL
        COMMENT "Running cargo..."
)

add_executable(jorzacan src/main.cpp ${JORZACAN_BRIDGE_CPP})

target_include_directories(
        jorzacan
        PRIVATE
        include/
        ${CARGO_TARGET_DIR}/cxxbridge/jorzacan/src/
)

target_link_libraries(jorzacan ${JORZACAN_LIB})

set_target_properties(
        jorzacan
        PROPERTIES ADDITIONAL_CLEAN_FILES ${CARGO_TARGET_DIR}
)

# Windows-only configuration
if(WIN32)
    target_link_libraries(jorzacan userenv ws2_32 bcrypt)
    set_target_properties(
            jorzacan
            PROPERTIES
            MSVC_RUNTIME_LIBRARY "MultiThreadedDLL"
            RUNTIME_OUTPUT_DIRECTORY_DEBUG ${CMAKE_BINARY_DIR}
            RUNTIME_OUTPUT_DIRECTORY_RELEASE ${CMAKE_BINARY_DIR}
    )
endif()
