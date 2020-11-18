use log::info;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn launch_from_wasm() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    wasm_logger::init(wasm_logger::Config::default());
    common();
}

#[cfg_attr(target_os = "android", ndk_glue::main(backtrace = "on"))]
pub fn launch_from_android() {
    env_logger::init();
    common();
}

pub fn common() {
    // If you can build on all platforms, all the code in the following sections will work.
    info!("hello world");
}
