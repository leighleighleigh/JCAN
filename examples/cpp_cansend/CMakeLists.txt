cmake_minimum_required(VERSION 3.22)
project(cpp_cansend)

# ROS Foxy / Ubuntu 20 targets C++14
set(CMAKE_CXX_STANDARD 14)

set(JORZACAN_BRIDGE_CPP ${CMAKE_SOURCE_DIR}/include/jorzacan/jorzacan.cc)
set(JORZACAN_LIB ${CMAKE_SOURCE_DIR}/include/jorzacan/${CMAKE_STATIC_LIBRARY_PREFIX}jorzacan${CMAKE_STATIC_LIBRARY_SUFFIX})

add_executable(cpp_cansend src/main.cpp ${JORZACAN_BRIDGE_CPP})

target_include_directories(
        cpp_cansend
        PRIVATE
        include/
        ${CMAKE_SOURCE_DIR}/include/jorzacan/
)

target_link_libraries(cpp_cansend ${JORZACAN_LIB})

# Windows-only configuration
if(WIN32)
    target_link_libraries(cpp_cansend userenv ws2_32 bcrypt)
    set_target_properties(
            cpp_cansend
            PROPERTIES
            MSVC_RUNTIME_LIBRARY "MultiThreadedDLL"
            RUNTIME_OUTPUT_DIRECTORY_DEBUG ${CMAKE_BINARY_DIR}
            RUNTIME_OUTPUT_DIRECTORY_RELEASE ${CMAKE_BINARY_DIR}
    )
endif()
