cmake_minimum_required(VERSION 3.25 FATAL_ERROR)
project(eqsat)

if(NOT COMMAND add_mim_plugin)
  find_package(mim REQUIRED)
endif()

# ------------------------------------------------------------
# Paths
# ------------------------------------------------------------
set(BUILD_DIR ${CMAKE_CURRENT_LIST_DIR}/build)

set(EQSAT_RS_INCLUDE ${BUILD_DIR}/include/eqsat_rs.h)
set(EQSAT_RS_SRC ${BUILD_DIR}/src/eqsat_rs.cc)

set(EQSAT_RS_LIB_WIN_REL ${BUILD_DIR}/lib/windows/release/eqsat_rs.lib)
set(EQSAT_RS_LIB_WIN_DBG ${BUILD_DIR}/lib/windows/debug/eqsat_rs.lib)

set(EQSAT_RS_LIB_MACOS_REL ${BUILD_DIR}/lib/macos/release/libeqsat_rs.a)
set(EQSAT_RS_LIB_MACOS_DBG ${BUILD_DIR}/lib/macos/debug/libeqsat_rs.a)

set(EQSAT_RS_LIB_LINUX_REL ${BUILD_DIR}/lib/linux/release/libeqsat_rs.a)
set(EQSAT_RS_LIB_LINUX_DBG ${BUILD_DIR}/lib/linux/debug/libeqsat_rs.a)

# ------------------------------------------------------------
# Build static libs and ffi header
# ------------------------------------------------------------
add_custom_target(eqsat-rs-build
    COMMAND bash ${CMAKE_CURRENT_LIST_DIR}/build.sh
    WORKING_DIRECTORY ${CMAKE_CURRENT_LIST_DIR}
    DEPENDS ${CMAKE_CURRENT_LIST_DIR}/build.sh
    COMMENT "Building eqsat-rs via build.sh"
)

# ------------------------------------------------------------
# Stage ffi header into rust include dir
# ------------------------------------------------------------
set(STAGED_HEADER
    ${CMAKE_BINARY_DIR}/include/rust/eqsat_rs.h)

add_custom_command(
    OUTPUT ${STAGED_HEADER}
    COMMAND ${CMAKE_COMMAND} -E make_directory ${CMAKE_BINARY_DIR}/include/rust
    COMMAND ${CMAKE_COMMAND} -E copy_if_different ${EQSAT_RS_INCLUDE} ${STAGED_HEADER}
    DEPENDS eqsat-rs-build ${EQSAT_RS_INCLUDE}
    COMMENT "Staging Rust header"
)

add_custom_target(eqsat-rs-header
    DEPENDS ${STAGED_HEADER}
)

# ------------------------------------------------------------
# Configure eqsat-rs library
# ------------------------------------------------------------
add_library(eqsat-rs STATIC IMPORTED GLOBAL)

add_dependencies(eqsat-rs eqsat-rs-build)

set_target_properties(eqsat-rs PROPERTIES
  INTERFACE_INCLUDE_DIRECTORIES "${BUILD_DIR}/include"
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

# ------------------------------------------------------------
# Plugin
# ------------------------------------------------------------
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

add_dependencies(mim_eqsat eqsat-rs-header)

target_include_directories(mim_eqsat
    PUBLIC
        $<BUILD_INTERFACE:${CMAKE_CURRENT_LIST_DIR}/include>
        $<INSTALL_INTERFACE:include>
)

# ------------------------------------------------------------
# Valgrind override
# ------------------------------------------------------------
set(VALGRIND_COMMAND_OPTIONS "--leak-check=full --error-exitcode=1 --suppressions=${PROJECT_SOURCE_DIR}/extra/eqsat/valgrind.supp")
set(CTEST_MEMORYCHECK_COMMAND_OPTIONS "--leak-check=full --error-exitcode=1 --suppressions=${PROJECT_SOURCE_DIR}/extra/eqsat/valgrind.supp")
