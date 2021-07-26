cmake_minimum_required(VERSION 3.19)
project(gtk C CXX)
set(CMAKE_C_STANDARD 11)

find_package(PkgConfig REQUIRED)
pkg_check_modules(GTK3 REQUIRED gtk+-3.0)
if (NOT GTK3_FOUND)
    message(FATAL_ERROR "GTK3 not found")
else()
    message("gtk_found = ${GTK3_FOUND}")
endif()
include_directories(${GTK3_INCLUDE_DIRS})
#include_directories(/usr/include/qt) # bad example

# gcc gtk1.c `pkgconf --cflags --libs gtk4`
# or
# gcc `pkg-config --cflags gtk+-3.0` -o gtk1 gtk1.c `pkg-config --libs gtk+-3.0`
add_executable(gtk_01 gtk_01_window.c)
target_link_libraries(gtk_01 ${GTK3_LIBRARIES})

add_executable(gtk_02 gtk_02_button_signal_callback.c)
target_link_libraries(gtk_02 ${GTK3_LIBRARIES})

add_executable(gtk_03 gtk_03_container.c)
target_link_libraries(gtk_03 ${GTK3_LIBRARIES})

add_executable(gtk_04 gtk_04_entry_password_input.c)
target_link_libraries(gtk_04 ${GTK3_LIBRARIES})

add_executable(gtk_05 gtk_05_spin_button_input_number.c)
target_link_libraries(gtk_05 ${GTK3_LIBRARIES})

add_executable(gtk_06 gtk_06_radio_button.c)
target_link_libraries(gtk_06 ${GTK3_LIBRARIES})

add_executable(gtk_07 gtk_07_tree_view.c)
target_link_libraries(gtk_07 ${GTK3_LIBRARIES})

add_executable(gtk_08 gtk_08_message_dialog.c)
target_link_libraries(gtk_08 ${GTK3_LIBRARIES})

# bad example
#add_executable(temp temp.cpp)
#target_link_libraries(temp qt)
