cmake_minimum_required(VERSION 3.19)
project(gtk C)

set(CMAKE_C_STANDARD 11)

find_package(PkgConfig REQUIRED)
pkg_check_modules(GTK REQUIRED gtk+-3.0)
#pkg_check_modules(GTK REQUIRED gtk4)
if (NOT GTK_FOUND)
    message(FATAL_ERROR "GTK not found")
endif()
message("gtk_found = ${GTK_FOUND} ")
include_directories(${GTK_INCLUDE_DIRS})
link_directories(${GTK_LIBRARY_DIRS})
add_definitions(${GTK_CFLAGS_OTHER})

# gcc gtk1.c `pkgconf --cflags --libs gtk4`
# or
# gcc `pkg-config --cflags gtk+-3.0` -o gtk1 gtk1.c `pkg-config --libs gtk+-3.0`
add_executable(gtk_1 gtk_1_window.c)
target_link_libraries(gtk_1 ${GTK_LIBRARIES})

add_executable(gtk_2 gtk_2_button_signal_callback.c)
target_link_libraries(gtk_2 ${GTK_LIBRARIES})
