cmake_minimum_required(VERSION 3.22)
project(cpp_receive_id_filter)

# ROS Foxy / Ubuntu 20 targets C++14
set(CMAKE_CXX_STANDARD 14)

set(JCAN_BRIDGE_CPP ${CMAKE_SOURCE_DIR}/include/jcan/jcan.cc)
set(JCAN_LIB ${CMAKE_SOURCE_DIR}/include/jcan/${CMAKE_STATIC_LIBRARY_PREFIX}jcan${CMAKE_STATIC_LIBRARY_SUFFIX})

add_executable(cpp_receive_id_filter src/main.cpp ${JCAN_BRIDGE_CPP})

target_include_directories(
        cpp_receive_id_filter
        PRIVATE
        include/
        ${CMAKE_SOURCE_DIR}/include/jcan/
)

target_link_libraries(cpp_receive_id_filter ${JCAN_LIB})

# Windows-only configuration
if(WIN32)
    target_link_libraries(cpp_receive_id_filter userenv ws2_32 bcrypt)
    set_target_properties(
            cpp_receive_id_filter
            PROPERTIES
            MSVC_RUNTIME_LIBRARY "MultiThreadedDLL"
            RUNTIME_OUTPUT_DIRECTORY_DEBUG ${CMAKE_BINARY_DIR}
            RUNTIME_OUTPUT_DIRECTORY_RELEASE ${CMAKE_BINARY_DIR}
    )
endif()
