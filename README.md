# Rust on cross-platform
## What is this?
A reference repository for development of cross-platform applications and libraries in Rust.

## Setup

### iOS setup
 1. Install toolchain for iOS
 
        rustup target add aarch64-apple-ios

 1. Install [xcodegen](https://github.com/yonaskolb/XcodeGen) and generate xcode project

        cd ios
        xcodegen

### Android setup
 1. Install toolchain for Android
 
        rustup target add aarch64-linux-android

 1. Install cargo-apk
 
        cargo install cargo-apk

 1. Install android-sdk and set $ANDROID_SDK_ROOT environment variable
 1. Install android-ndk and set $ANDROID_NDK_ROOT environment variable
 1. ./deploy.sh

### WebAssembly setup
 1. Install wasm-pack
 
        cargo install wasm-pack

 1. Install node_modules by npm
 
        npm install

 1. Run dev server
 
        npm start
