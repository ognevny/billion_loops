project(
    'billion_loops',
    ['c', 'cpp'],
    version: '0.0.0',
    default_options: [
        'buildtype=plain',
        'debug=false',
        'warning_level=0',
        'rust_std=2021',
        'cpp_std=c++11',
        'c_std=c99',
    ],
)

cc_id = meson.get_compiler('c').get_id()

bl_c = executable('bl_c', sources: 'c/speedometer.c')
bl_cpp = executable('bl_cpp', sources: 'cpp/speedometer.cpp')

if cc_id != 'msvc'
  add_languages('rust')
  bl_rs = executable(
      'bl_rs',
      sources: 'rust/src/main.rs',
      rust_args: ['-Cpanic=abort', '-Coverflow-checks=false'],
  )
  bl_rs_no_core = executable(
      'bl_rs-no-core',
      sources: 'rust-no-core/src/main.rs',
      rust_args: ['-Cpanic=abort', '-Coverflow-checks=false'],
  )
endif

benchmark('Bench C', bl_c)
benchmark('Bench C++', bl_cpp)

if cc_id != 'msvc'
  benchmark('Bench Rust', bl_rs)
  benchmark('Bench Rust with no_core', bl_rs_no_core)
endif
