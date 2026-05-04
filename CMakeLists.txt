cmake_minimum_required(VERSION 3.25 FATAL_ERROR)
project(eqsat)

if(NOT COMMAND add_mim_plugin)
  find_package(mim REQUIRED)
endif()

add_library(eqsat-rs STATIC IMPORTED)

set_target_properties(eqsat-rs PROPERTIES
  INTERFACE_INCLUDE_DIRECTORIES "${CMAKE_CURRENT_LIST_DIR}/dist/include"
  INTERFACE_SOURCES "${CMAKE_CURRENT_LIST_DIR}/dist/src/eqsat_rs.cc"
)

if (WIN32)
  set_target_properties(eqsat-rs PROPERTIES
    IMPORTED_LOCATION_DEBUG "${CMAKE_CURRENT_LIST_DIR}/dist/lib/windows/debug/eqsat_rs.lib"
    IMPORTED_LOCATION_RELEASE "${CMAKE_CURRENT_LIST_DIR}/dist/lib/windows/release/eqsat_rs.lib"
    IMPORTED_LOCATION_RELWITHDEBINFO "${CMAKE_CURRENT_LIST_DIR}/dist/lib/windows/release/eqsat_rs.lib"
    IMPORTED_LOCATION_MINSIZEREL "${CMAKE_CURRENT_LIST_DIR}/dist/lib/windows/release/eqsat_rs.lib"
  )

  target_link_libraries(eqsat-rs INTERFACE
    ws2_32 userenv advapi32 bcrypt ntdll
  )

elseif(APPLE)
  set_target_properties(eqsat-rs PROPERTIES
    IMPORTED_LOCATION_DEBUG "${CMAKE_CURRENT_LIST_DIR}/dist/lib/macos/debug/libeqsat_rs.a"
    IMPORTED_LOCATION_RELEASE "${CMAKE_CURRENT_LIST_DIR}/dist/lib/macos/release/libeqsat_rs.a"
    IMPORTED_LOCATION_RELWITHDEBINFO "${CMAKE_CURRENT_LIST_DIR}/dist/lib/macos/release/libeqsat_rs.a"
    IMPORTED_LOCATION_MINSIZEREL "${CMAKE_CURRENT_LIST_DIR}/dist/lib/macos/release/libeqsat_rs.a"
  )

  target_link_libraries(eqsat-rs INTERFACE
    "-framework Security"
    "-framework SystemConfiguration"
  )

else()
  set_target_properties(eqsat-rs PROPERTIES
    IMPORTED_LOCATION_DEBUG "${CMAKE_CURRENT_LIST_DIR}/dist/lib/linux/debug/libeqsat_rs.a"
    IMPORTED_LOCATION_RELEASE "${CMAKE_CURRENT_LIST_DIR}/dist/lib/linux/release/libeqsat_rs.a"
    IMPORTED_LOCATION_RELWITHDEBINFO "${CMAKE_CURRENT_LIST_DIR}/dist/lib/linux/release/libeqsat_rs.a"
    IMPORTED_LOCATION_MINSIZEREL "${CMAKE_CURRENT_LIST_DIR}/dist/lib/linux/release/libeqsat_rs.a"
  )

endif()

configure_file(
    "${CMAKE_CURRENT_LIST_DIR}/dist/include/eqsat_rs.h"
    "${CMAKE_BINARY_DIR}/include/rust/eqsat_rs.h"
)

add_mim_plugin(eqsat
    SOURCES
        plug/eqsat.cpp
        plug/phase/eqsat_phase.cpp
        plug/phase/egg_rewrite.cpp
        plug/phase/slotted_rewrite.cpp
    PRIVATE
        eqsat-rs
    INSTALL
)

target_include_directories(mim_eqsat
    PUBLIC
        $<BUILD_INTERFACE:${CMAKE_CURRENT_LIST_DIR}/include>
        $<INSTALL_INTERFACE:include>
)
