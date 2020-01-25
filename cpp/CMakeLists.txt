cmake_minimum_required(VERSION 3.2)
set(TARGET binding_example)
project(${TARGET})

include(${CMAKE_BINARY_DIR}/conanbuildinfo.cmake)
conan_basic_setup()

add_executable(${TARGET}
	src/BindingTest.cpp
	src/UtilsTest.cpp
	src/main.cpp
	)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

if ("${CMAKE_CXX_COMPILER_ID}" STREQUAL "GNU")
	add_compile_options (-fdiagnostics-color=always)
	set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -fdiagnostics-color=always")
elseif ("${CMAKE_CXX_COMPILER_ID}" STREQUAL "Clang")
	add_compile_options (-fcolor-diagnostics)
endif ()

set_target_properties(${TARGET} PROPERTIES
	CXX_STANDARD 20
	CXX_STANDARD_REQUIRED ON
	)
target_link_libraries(${TARGET} ${CONAN_LIBS})

enable_testing ()
add_test (NAME MyTest COMMAND ${TARGET})
