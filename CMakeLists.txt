cmake_minimum_required(VERSION 3.25 FATAL_ERROR)
project(eqsat)

if(NOT COMMAND add_mim_plugin)
  find_package(mim REQUIRED)
endif()

add_mim_plugin(eqsat
    SOURCES
        plug/eqsat.cpp
        plug/phase/eqsat_phase.cpp
        plug/phase/egg_rewrite.cpp
        plug/phase/slotted_rewrite.cpp
    INSTALL
)
