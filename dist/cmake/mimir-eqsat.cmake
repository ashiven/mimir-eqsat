add_library(mimir-eqsat STATIC IMPORTED)

set_target_properties(mimir-eqsat PROPERTIES
  INTERFACE_INCLUDE_DIRECTORIES "${CMAKE_CURRENT_LIST_DIR}/../include"
  INTERFACE_SOURCES "${CMAKE_CURRENT_LIST_DIR}/../src/mimir_eqsat.cc"
)

if (WIN32)
  set_target_properties(mimir-eqsat PROPERTIES
    IMPORTED_LOCATION_DEBUG "${CMAKE_CURRENT_LIST_DIR}/../lib/windows/debug/mimir_eqsat.lib"
    IMPORTED_LOCATION_RELEASE "${CMAKE_CURRENT_LIST_DIR}/../lib/windows/release/mimir_eqsat.lib"
    IMPORTED_LOCATION_RELWITHDEBINFO "${CMAKE_CURRENT_LIST_DIR}/../lib/windows/release/mimir_eqsat.lib"
    IMPORTED_LOCATION_MINSIZEREL "${CMAKE_CURRENT_LIST_DIR}/../lib/windows/release/mimir_eqsat.lib"
  )

  target_link_libraries(mimir-eqsat INTERFACE
    ws2_32 userenv advapi32 bcrypt ntdll
  )

else()
  set_target_properties(mimir-eqsat PROPERTIES
    IMPORTED_LOCATION_DEBUG "${CMAKE_CURRENT_LIST_DIR}/../lib/linux/debug/libmimir_eqsat.a"
    IMPORTED_LOCATION_RELEASE "${CMAKE_CURRENT_LIST_DIR}/../lib/linux/release/libmimir_eqsat.a"
    IMPORTED_LOCATION_RELWITHDEBINFO "${CMAKE_CURRENT_LIST_DIR}/../lib/linux/release/libmimir_eqsat.a"
    IMPORTED_LOCATION_MINSIZEREL "${CMAKE_CURRENT_LIST_DIR}/../lib/linux/release/libmimir_eqsat.a"
  )
endif()
