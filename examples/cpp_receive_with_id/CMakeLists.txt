cmake_minimum_required(VERSION 3.10)
project(cpp_receive_with_id)

# ROS Foxy / Ubuntu 20 targets C++14
set(CMAKE_CXX_STANDARD 14)

set(JCAN_CPP ${CMAKE_SOURCE_DIR}/include/jcan/jcan.cc)
set(JCAN_LIB ${CMAKE_SOURCE_DIR}/include/jcan/${CMAKE_STATIC_LIBRARY_PREFIX}jcan${CMAKE_STATIC_LIBRARY_SUFFIX})


find_package (Threads REQUIRED)

if (NOT TARGET jcan)
  add_library(jcan ${JCAN_CPP})
  target_link_libraries(jcan ${JCAN_LIB} pthread dl rt)
endif()



add_executable(cpp_receive_with_id src/main.cpp)

target_include_directories(
        cpp_receive_with_id
        PRIVATE
        include/
        ${CMAKE_SOURCE_DIR}/include/jcan/
)

target_link_libraries(cpp_receive_with_id jcan)

# Windows-only configuration
if(WIN32)
    target_link_libraries(cpp_receive_with_id userenv ws2_32 bcrypt jcan)
    set_target_properties(
            cpp_receive_with_id
            PROPERTIES
            MSVC_RUNTIME_LIBRARY "MultiThreadedDLL"
            RUNTIME_OUTPUT_DIRECTORY_DEBUG ${CMAKE_BINARY_DIR}
            RUNTIME_OUTPUT_DIRECTORY_RELEASE ${CMAKE_BINARY_DIR}
    )
endif()
