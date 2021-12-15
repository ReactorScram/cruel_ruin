cmake_minimum_required (VERSION 3.1.0)

project (cruel_ruin)

add_executable (cruel_ruin
src/main.cpp
)

target_compile_options (cruel_ruin PRIVATE -std=c++17 -Wall -pedantic -effc++)

target_link_libraries (cruel_ruin PRIVATE 
${CMAKE_SOURCE_DIR}/../cruel_ruin_rs/target/release/libcruel_ruin_rs.a
curl
dl
pthread
)
