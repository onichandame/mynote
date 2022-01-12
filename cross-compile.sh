# requires ndk 22
rustup target add aarch64-linux-android
export NDK_HOME=$PWD/../android-ndk-r22b
export PATH=$PATH:$NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin
RUSTFLAGS=-Clinker=aarch64-linux-android30-clang cargo build --release --target aarch64-linux-android