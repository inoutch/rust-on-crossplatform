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
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    common();
}

#[cfg(target_os = "ios")]
pub fn launch_from_ios() -> std::os::raw::c_int {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    common();
    0
}

#[cfg(target_os = "ios")]
#[no_mangle]
pub extern "C" fn main_rs() -> std::os::raw::c_int {
    stop_unwind(launch_from_ios)
}

#[cfg(target_os = "ios")]
fn stop_unwind<F: FnOnce() -> T + std::panic::UnwindSafe, T>(f: F) -> T {
    match std::panic::catch_unwind(f) {
        Ok(t) => t,
        Err(_) => {
            println!("Attempt to Unwind out of rust code");
            std::process::abort()
        }
    }
}

pub fn common() {
    // If you can build on all platforms, all the code in the following sections will work.
    info!("hello world");
}
