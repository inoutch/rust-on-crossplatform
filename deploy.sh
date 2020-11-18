#!/bin/bash

set -eux

rm -r target/debug/apk
cargo apk run || echo "Suppress signing error"

# Issue for https://github.com/rust-windowing/android-ndk-rs/issues/76#issuecomment-698508327
java -jar uber-apk-signer-1.1.0.jar --apks ./target/debug/apk/

adb install -r /home/inoue/projects/github/rust-on-android/target/debug/apk/rust-on-android-aligned-debugSigned.apk
