cmake_minimum_required(VERSION 3.19)
project(qt CXX)
set(CMAKE_CXX_STANDARD 20)

find_package(Qt5 COMPONENTS Widgets REQUIRED)

add_executable(qt_01 qt_01_window.cpp)
target_link_libraries(qt_01 Qt5::Widgets)
