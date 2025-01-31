project(
    'billion_loops',
    ['c', 'cpp', 'rust'],
    version: '0.0.0',
    default_options: [
        'buildtype=plain',
        'debug=false',
        'warning_level=0',
        'rust_std=2024',
        'cpp_std=c++11',
        'c_std=c99',
        'b_ndebug=true',
    ],
    meson_version: '>= 1.7.0',
)

bl_c = executable('bl_c', sources: 'c/speedometer.c')
bl_cpp = executable('bl_cpp', sources: 'cpp/speedometer.cpp')
bl_rs = executable(
    'bl_rs',
    sources: 'rust/src/main.rs',
    rust_args: ['-Cpanic=abort'],
)
bl_rs_no_core = executable(
    'bl_rs-no-core',
    sources: 'rust-no-core/src/main.rs',
    rust_args: ['-Cpanic=abort'],
)

benchmark('Bench C', bl_c)
benchmark('Bench C++', bl_cpp)
benchmark('Bench Rust', bl_rs)
benchmark('Bench Rust with no_core', bl_rs_no_core)
