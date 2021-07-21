cmake_minimum_required(VERSION 3.19)
project(gtk C)
set(CMAKE_C_STANDARD 11)

find_package(PkgConfig REQUIRED)
pkg_check_modules(GTK3 REQUIRED gtk+-3.0)
if (NOT GTK3_FOUND)
    message(FATAL_ERROR "GTK3 not found")
else()
    message("gtk_found = ${GTK3_FOUND}")
endif()
include_directories(${GTK3_INCLUDE_DIRS})
#add_definitions(${GTK3_CFLAGS_OTHER})

# gcc gtk1.c `pkgconf --cflags --libs gtk4`
# or
# gcc `pkg-config --cflags gtk+-3.0` -o gtk1 gtk1.c `pkg-config --libs gtk+-3.0`
add_executable(gtk_1 gtk_1_window.c)
target_link_libraries(gtk_1 ${GTK3_LIBRARIES})

add_executable(gtk_2 gtk_2_button_signal_callback.c)
target_link_libraries(gtk_2 ${GTK3_LIBRARIES})

add_executable(gtk_3 gtk_3_container.c)
target_link_libraries(gtk_3 ${GTK3_LIBRARIES})

add_executable(gtk_4 gtk_4_entry_password_input.c)
target_link_libraries(gtk_4 ${GTK3_LIBRARIES})

add_executable(gtk_5 gtk_5_spin_button_input_number.c)
target_link_libraries(gtk_5 ${GTK3_LIBRARIES})

add_executable(gtk_temp gtk_temp.c)
target_link_libraries(gtk_temp ${GTK3_LIBRARIES})
