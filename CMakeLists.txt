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
set(EQSAT_RS_LIB_WIN_REL ${BUILD_DIR}/lib/windows/release/eqsat_rs.lib)
set(EQSAT_RS_LIB_WIN_DBG ${BUILD_DIR}/lib/windows/debug/eqsat_rs.lib)
set(EQSAT_RS_LIB_MACOS_REL ${BUILD_DIR}/lib/macos/release/libeqsat_rs.a)
set(EQSAT_RS_LIB_MACOS_DBG ${BUILD_DIR}/lib/macos/debug/libeqsat_rs.a)
set(EQSAT_RS_LIB_LINUX_REL ${BUILD_DIR}/lib/linux/release/libeqsat_rs.a)
set(EQSAT_RS_LIB_LINUX_DBG ${BUILD_DIR}/lib/linux/debug/libeqsat_rs.a)

if (WIN32)
  set(EQSAT_RS_LIBS
    ${EQSAT_RS_LIB_WIN_REL}
    ${EQSAT_RS_LIB_WIN_DBG}
  )
elseif(APPLE)
  set(EQSAT_RS_LIBS
    ${EQSAT_RS_LIB_MACOS_REL}
    ${EQSAT_RS_LIB_MACOS_DBG}
  )
else()
  set(EQSAT_RS_LIBS
    ${EQSAT_RS_LIB_LINUX_REL}
    ${EQSAT_RS_LIB_LINUX_DBG}
  )
endif()

add_custom_command(
    OUTPUT ${EQSAT_RS_INC} ${EQSAT_RS_SRC} ${EQSAT_RS_LIBS}
    COMMAND bash ${CMAKE_CURRENT_LIST_DIR}/build.sh
    DEPENDS ${CMAKE_CURRENT_LIST_DIR}/build.sh
    WORKING_DIRECTORY ${CMAKE_CURRENT_LIST_DIR}
)
add_custom_target(eqsat-rs-build ALL
    DEPENDS ${EQSAT_RS_INC} ${EQSAT_RS_SRC} ${EQSAT_RS_LIBS}
)

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
    IMPORTED_LOCATION_DEBUG ${EQSAT_RS_LIB_WIN_DBG}
    IMPORTED_LOCATION_RELEASE ${EQSAT_RS_LIB_WIN_REL}
    IMPORTED_LOCATION_RELWITHDEBINFO ${EQSAT_RS_LIB_WIN_REL}
    IMPORTED_LOCATION_MINSIZEREL ${EQSAT_RS_LIB_WIN_REL}
  )

  target_link_libraries(eqsat-rs INTERFACE
    ws2_32 userenv advapi32 bcrypt ntdll
  )

elseif(APPLE)
  set_target_properties(eqsat-rs PROPERTIES
    IMPORTED_LOCATION_DEBUG ${EQSAT_RS_LIB_MACOS_DBG}
    IMPORTED_LOCATION_RELEASE ${EQSAT_RS_LIB_MACOS_REL}
    IMPORTED_LOCATION_RELWITHDEBINFO ${EQSAT_RS_LIB_MACOS_REL}
    IMPORTED_LOCATION_MINSIZEREL ${EQSAT_RS_LIB_MACOS_REL}
  )

  target_link_libraries(eqsat-rs INTERFACE
    "-framework Security"
    "-framework SystemConfiguration"
  )

else()
  set_target_properties(eqsat-rs PROPERTIES
    IMPORTED_LOCATION_DEBUG ${EQSAT_RS_LIB_LINUX_DBG}
    IMPORTED_LOCATION_RELEASE ${EQSAT_RS_LIB_LINUX_REL}
    IMPORTED_LOCATION_RELWITHDEBINFO ${EQSAT_RS_LIB_LINUX_REL}
    IMPORTED_LOCATION_MINSIZEREL ${EQSAT_RS_LIB_LINUX_REL}
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
add_dependencies(mim_eqsat eqsat-rs-build)
target_include_directories(mim_eqsat
    PUBLIC
        $<BUILD_INTERFACE:${CMAKE_CURRENT_LIST_DIR}/include>
        $<INSTALL_INTERFACE:include>
)
