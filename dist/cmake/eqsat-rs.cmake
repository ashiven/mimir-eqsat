add_library(eqsat-rs STATIC IMPORTED)

set_target_properties(eqsat-rs PROPERTIES
  INTERFACE_INCLUDE_DIRECTORIES "${CMAKE_CURRENT_LIST_DIR}/../include"
  INTERFACE_SOURCES "${CMAKE_CURRENT_LIST_DIR}/../src/eqsat_rs.cc"
)

if (WIN32)
  set_target_properties(eqsat-rs PROPERTIES
    IMPORTED_LOCATION_DEBUG "${CMAKE_CURRENT_LIST_DIR}/../lib/windows/debug/eqsat_rs.lib"
    IMPORTED_LOCATION_RELEASE "${CMAKE_CURRENT_LIST_DIR}/../lib/windows/release/eqsat_rs.lib"
    IMPORTED_LOCATION_RELWITHDEBINFO "${CMAKE_CURRENT_LIST_DIR}/../lib/windows/release/eqsat_rs.lib"
    IMPORTED_LOCATION_MINSIZEREL "${CMAKE_CURRENT_LIST_DIR}/../lib/windows/release/eqsat_rs.lib"
  )

  target_link_libraries(eqsat-rs INTERFACE
    ws2_32 userenv advapi32 bcrypt ntdll
  )

elseif(APPLE)
  set_target_properties(eqsat-rs PROPERTIES
    IMPORTED_LOCATION_DEBUG "${CMAKE_CURRENT_LIST_DIR}/../lib/macos/debug/libeqsat_rs.a"
    IMPORTED_LOCATION_RELEASE "${CMAKE_CURRENT_LIST_DIR}/../lib/macos/release/libeqsat_rs.a"
    IMPORTED_LOCATION_RELWITHDEBINFO "${CMAKE_CURRENT_LIST_DIR}/../lib/macos/release/libeqsat_rs.a"
    IMPORTED_LOCATION_MINSIZEREL "${CMAKE_CURRENT_LIST_DIR}/../lib/macos/release/libeqsat_rs.a"
  )

  target_link_libraries(eqsat-rs INTERFACE
    "-framework Security"
    "-framework SystemConfiguration"
  )

else()
  set_target_properties(eqsat-rs PROPERTIES
    IMPORTED_LOCATION_DEBUG "${CMAKE_CURRENT_LIST_DIR}/../lib/linux/debug/libeqsat_rs.a"
    IMPORTED_LOCATION_RELEASE "${CMAKE_CURRENT_LIST_DIR}/../lib/linux/release/libeqsat_rs.a"
    IMPORTED_LOCATION_RELWITHDEBINFO "${CMAKE_CURRENT_LIST_DIR}/../lib/linux/release/libeqsat_rs.a"
    IMPORTED_LOCATION_MINSIZEREL "${CMAKE_CURRENT_LIST_DIR}/../lib/linux/release/libeqsat_rs.a"
  )

endif()
