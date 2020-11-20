# Rust on cross-platform
## What is this?
A reference repository for development of cross-platform applications and libraries in Rust.

## Setup and Run

### Desktop setup

 1. Build and run

        export RUST_LOG=info # To print hello world message
        cargo run --bin rust_on_crossplatform

### iOS setup
 1. Install toolchain for iOS.
 
        rustup target add aarch64-apple-ios

 1. Install [xcodegen](https://github.com/yonaskolb/XcodeGen) and generate xcode project

        cd ios
        xcodegen

 1. Run app on xcode.

### Android setup
 1. Install toolchain for Android.
 
        rustup target add aarch64-linux-android

 1. Install [cargo-apk](https://github.com/rust-windowing/android-ndk-rs).
 
        cargo install cargo-apk

 1. Install [android-sdk](https://developer.android.com/studio) (you can build with just the CLI tools) and set `$ANDROID_SDK_ROOT` environment variable.
 1. Install [android-ndk](https://developer.android.com/ndk) and set `$ANDROID_NDK_ROOT` environment variable.
 1. Deploy app to Android device.
 
        ./deploy.sh

 1. Run app on a device.

### WebAssembly setup
 1. Install node_modules by npm.
 
        npm install

 1. Run dev server.
 
        npm start

## How to work as app?
Rust supports cross-compilation of various platforms. Therefore, it is very easy to build and generate binary files.
However, the generation of an executable application requires configuration and build flow depending on the platform.

### Desktop
In an environment that can run Rust as-is, you can build an executable file by defining the main function.

### iOS
It generates a library (.a or .dylib) from Rust code and link it when building the executable iOS app.
To build to the library, you can specify `cdylib` or `staticlib` in the `crate-type` in Cargo.toml.
To call Rust from Objective-C, you can use [cbindgen](https://github.com/eqrion/cbindgen) to create an interface (.h) and include it.

### Android
The binaries built in Rust will be linked using the Android NDK.
With [cargo-apk](https://github.com/rust-windowing/android-ndk-rs/tree/master/cargo-apk), it is possible to build an apk file easily.
To make the application executable, it is convenient to use [ndk-macro](https://github.com/rust-windowing/android-ndk-rs/tree/master/ndk-macro). The macro applies the function directly to the main function.

### WebAssembly
[wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) makes it easy to communicate with JavaScript and Wasm modules.
[wasm-pack](https://github.com/rustwasm/wasm-pack) provides an interface that can be built into wasm and called in JavaScript.
In addition, since wasm-pack is highly compatible with webpack, it is possible to use the [@wasm-tool/wasm-pack-plugin](https://github.com/wasm-tool/wasm-pack-plugin) to incorporate it into webpack build.
