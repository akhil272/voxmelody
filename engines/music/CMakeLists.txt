cmake_minimum_required(VERSION 3.24...3.28)
project(voxmelody-music LANGUAGES C CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
# No Visual C++ runtime install is required on the user's computer.
set(CMAKE_MSVC_RUNTIME_LIBRARY "MultiThreaded")
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)

# Portable builds: never tune for the build machine.
set(GGML_NATIVE OFF CACHE BOOL "" FORCE)
set(GGML_OPENMP OFF CACHE BOOL "" FORCE)
set(GGML_CCACHE OFF CACHE BOOL "" FORCE)
if(WIN32)
  # Backends load at runtime, so a PC without Vulkan still starts and each CPU gets its best variant.
  set(BUILD_SHARED_LIBS ON CACHE BOOL "" FORCE)
  set(GGML_BACKEND_DL ON CACHE BOOL "" FORCE)
  set(GGML_CPU_ALL_VARIANTS ON CACHE BOOL "" FORCE)
  set(GGML_VULKAN ON CACHE BOOL "" FORCE)
elseif(APPLE)
  set(BUILD_SHARED_LIBS OFF CACHE BOOL "" FORCE)
  set(GGML_METAL ON CACHE BOOL "" FORCE)
  set(GGML_METAL_EMBED_LIBRARY ON CACHE BOOL "" FORCE)
endif()

include(FetchContent)
# SOURCE_SUBDIR skips the upstream build, which assumes it is the top-level project.
FetchContent_Declare(
  acestep
  GIT_REPOSITORY https://github.com/ServeurpersoCom/acestep.cpp.git
  GIT_TAG 973517be024ff8b6bbd50e0adc9e308ab2ed71b3
  GIT_SUBMODULES ggml
  SOURCE_SUBDIR none
)
FetchContent_MakeAvailable(acestep)

add_compile_definitions(GGML_MAX_NAME=128 _CRT_SECURE_NO_WARNINGS NOMINMAX)
add_subdirectory(${acestep_SOURCE_DIR}/ggml ggml)

find_package(Threads REQUIRED)
add_executable(
  voxmelody-music
  src/main.cpp
  ${acestep_SOURCE_DIR}/src/request.cpp
  ${acestep_SOURCE_DIR}/src/model-store.cpp
  ${acestep_SOURCE_DIR}/src/pipeline-lm.cpp
  ${acestep_SOURCE_DIR}/src/pipeline-synth.cpp
  ${acestep_SOURCE_DIR}/src/pipeline-synth-ops.cpp
  ${acestep_SOURCE_DIR}/vendor/yyjson/yyjson.c
)
target_include_directories(
  voxmelody-music SYSTEM PRIVATE ${acestep_SOURCE_DIR} ${acestep_SOURCE_DIR}/src ${acestep_SOURCE_DIR}/vendor/yyjson
                                 ${acestep_SOURCE_DIR}/ggml/include
)
target_link_libraries(voxmelody-music PRIVATE ggml ggml-base Threads::Threads)
if(APPLE)
  target_link_libraries(voxmelody-music PRIVATE ggml-cpu ggml-metal)
endif()
