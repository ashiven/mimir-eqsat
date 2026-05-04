cmake_minimum_required(VERSION 3.25 FATAL_ERROR)
project(eqsat)

if(NOT COMMAND add_mim_plugin)
  find_package(mim REQUIRED)
endif()

set(BUILD_DIR ${CMAKE_CURRENT_LIST_DIR}/build)
file(MAKE_DIRECTORY "${BUILD_DIR}/include")
file(MAKE_DIRECTORY "${BUILD_DIR}/src")
file(MAKE_DIRECTORY "${BUILD_DIR}/lib")

set(EQSAT_RS_INC ${BUILD_DIR}/include/eqsat_rs.h)
set(EQSAT_RS_SRC ${BUILD_DIR}/src/eqsat_rs.cc)
add_custom_command(
    OUTPUT ${EQSAT_RS_INC} ${EQSAT_RS_SRC}
    COMMAND bash ${CMAKE_CURRENT_LIST_DIR}/build.sh
    DEPENDS ${CMAKE_CURRENT_LIST_DIR}/build.sh
    WORKING_DIRECTORY ${CMAKE_CURRENT_LIST_DIR}
)
add_custom_target(eqsat-rs-build DEPENDS ${EQSAT_RS_INC} ${EQSAT_RS_SRC})

set(EQSAT_RS_INC_OUT "${CMAKE_BINARY_DIR}/include/rust/eqsat_rs.h")
add_custom_command(
  OUTPUT "${EQSAT_RS_INC_OUT}"
  COMMAND ${CMAKE_COMMAND} -E make_directory "${CMAKE_BINARY_DIR}/include/rust"
  COMMAND ${CMAKE_COMMAND} -E copy_if_different "${EQSAT_RS_INC}" "${EQSAT_RS_INC_OUT}"
  DEPENDS eqsat-rs-build "${EQSAT_RS_INC}"
  VERBATIM
)
add_custom_target(eqsat-rs-stage-headers DEPENDS "${EQSAT_RS_INC_OUT}")

add_library(eqsat-rs STATIC IMPORTED)
add_dependencies(eqsat-rs eqsat-rs-build)

set_target_properties(eqsat-rs PROPERTIES
  INTERFACE_INCLUDE_DIRECTORIES "${BUILD_DIR}/include"
  INTERFACE_SOURCES "${BUILD_DIR}/src/eqsat_rs.cc"
)

if (WIN32)
  set_target_properties(eqsat-rs PROPERTIES
    IMPORTED_LOCATION_DEBUG "${BUILD_DIR}/lib/windows/debug/eqsat_rs.lib"
    IMPORTED_LOCATION_RELEASE "${BUILD_DIR}/lib/windows/release/eqsat_rs.lib"
    IMPORTED_LOCATION_RELWITHDEBINFO "${BUILD_DIR}/lib/windows/release/eqsat_rs.lib"
    IMPORTED_LOCATION_MINSIZEREL "${BUILD_DIR}/lib/windows/release/eqsat_rs.lib"
  )

  target_link_libraries(eqsat-rs INTERFACE
    ws2_32 userenv advapi32 bcrypt ntdll
  )

elseif(APPLE)
  set_target_properties(eqsat-rs PROPERTIES
    IMPORTED_LOCATION_DEBUG "${BUILD_DIR}/lib/macos/debug/libeqsat_rs.a"
    IMPORTED_LOCATION_RELEASE "${BUILD_DIR}/lib/macos/release/libeqsat_rs.a"
    IMPORTED_LOCATION_RELWITHDEBINFO "${BUILD_DIR}/lib/macos/release/libeqsat_rs.a"
    IMPORTED_LOCATION_MINSIZEREL "${BUILD_DIR}/lib/macos/release/libeqsat_rs.a"
  )

  target_link_libraries(eqsat-rs INTERFACE
    "-framework Security"
    "-framework SystemConfiguration"
  )

else()
  set_target_properties(eqsat-rs PROPERTIES
    IMPORTED_LOCATION_DEBUG "${BUILD_DIR}/lib/linux/debug/libeqsat_rs.a"
    IMPORTED_LOCATION_RELEASE "${BUILD_DIR}/lib/linux/release/libeqsat_rs.a"
    IMPORTED_LOCATION_RELWITHDEBINFO "${BUILD_DIR}/lib/linux/release/libeqsat_rs.a"
    IMPORTED_LOCATION_MINSIZEREL "${BUILD_DIR}/lib/linux/release/libeqsat_rs.a"
  )

endif()

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
