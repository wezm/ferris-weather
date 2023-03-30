#    Copyright 2015 Wolfgang Thaller.
#
#    This file is part of Retro68.
#
#    Retro68 is free software: you can redistribute it and/or modify
#    it under the terms of the GNU General Public License as published by
#    the Free Software Foundation, either version 3 of the License, or
#    (at your option) any later version.
#
#    Retro68 is distributed in the hope that it will be useful,
#    but WITHOUT ANY WARRANTY; without even the implied warranty of
#    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
#    GNU General Public License for more details.
#
#    You should have received a copy of the GNU General Public License
#    along with Retro68.  If not, see <http://www.gnu.org/licenses/>.

# To use this example as a standalone project using CMake:
# mkdir build
# cd build
# cmake .. -DCMAKE_TOOLCHAIN_FILE=path/to/Retro68-build/toolchain/m68k-apple-macos/cmake/retro68.toolchain.cmake
# make
cmake_minimum_required(VERSION 3.24)

project(TootClassic)

find_library(OPENTRANSPORT_LIBRARY NAMES libOpenTransportLib.a)

add_application(TootClassic
    trampoline.c
    dialog.c
    dialog.r
    )

target_link_libraries(TootClassic ${CMAKE_SOURCE_DIR}/target/powerpc-apple-macos/release/libclassic_mac_rust.obj)
target_link_libraries(TootClassic -lOpenTransportAppPPC -lOpenTransportLib -lOpenTptInternetLib)

# Enable -ffunction-sections and -gc-sections to make the app as small as possible
# On 68K, also enable --mac-single to build it as a single-segment app (so that this code path doesn't rot)
set_target_properties(TootClassic PROPERTIES COMPILE_OPTIONS -ffunction-sections)
if(CMAKE_SYSTEM_NAME MATCHES Retro68)
    set_target_properties(TootClassic PROPERTIES LINK_FLAGS "-Wl,-gc-sections -Wl,--mac-single")

else()
    set_target_properties(TootClassic PROPERTIES LINK_FLAGS "-Wl,-gc-sections")
endif()
