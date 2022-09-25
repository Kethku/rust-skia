use super::prelude::*;

pub fn args(config: &BuildConfiguration, builder: &mut ArgBuilder) {
    let features = &config.features;

    builder
        .skia("cc", quote("emcc"))
        .skia("cxx", quote("em++"))
        .skia("skia_gl_standard", quote("webgl"))
        .skia("skia_use_freetype", yes())
        .skia("skia_use_system_freetype2", no())
        .skia("skia_use_webgl", yes_if(features.gpu()))
        .skia("target_cpu", quote("wasm"));

    // The custom embedded font manager is enabled by default on WASM, but depends
    // on the undefined symbol `SK_EMBEDDED_FONTS`. Enable the custom empty font
    // manager instead so typeface creation still works.
    // See https://github.com/rust-skia/rust-skia/issues/648
    builder
        .skia("skia_enable_fontmgr_custom_embedded", no())
        .skia("skia_enable_fontmgr_custom_empty", yes());

    // visibility=default, otherwise some types may be missing:
    // <https://github.com/rust-lang/rust-bindgen/issues/751#issuecomment-555735577>
    builder.clang_arg("-fvisibility=default");

    let emsdk_base_dir = match std::env::var("EMSDK") {
        Ok(val) => val,
        Err(_e) => panic!(
            "please set the EMSDK environment variable to the root of your Emscripten installation"
        ),
    };

    builder.clang_arg("-nobuiltininc");

    // Add C++ includes (otherwise build will fail with <cmath> not found)
    let mut add_sys_include = |path: &str| {
        builder.clang_arg(format!(
            "-isystem{emsdk_base_dir}/upstream/emscripten/system/{path}",
        ));
    };

    add_sys_include("lib/libc/musl/arch/emscripten");
    add_sys_include("lib/libc/musl/arch/generic");
    add_sys_include("lib/libcxx/include");
    add_sys_include("lib/libc/musl/include");
    add_sys_include("include");
}

pub fn link_libraries(features: &Features, builder: &mut LinkLibrariesBuilder) {
    if features.gl {
        builder.link_library("GL");
    }
}
