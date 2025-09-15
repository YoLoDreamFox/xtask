# Multibuild Script for Rust Project (xtask)
This project provides a Cargo build wrapper that automates cross-compiling a Rust application for multiple operating systems and architectures. It simplifies the process of multi-target builds and packagin

# 🔧 Installation
**Windows (from Linux host)**

```
sudo pacman -S mingw-w64-gcc
rustup target add x86_64-pc-windows-gnu
```
**Android**

Install Android NDK, then add the Rust target:
```
rustup target add aarch64-linux-android
```
**MacOS & iOS (requires macOS host)**
```
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-ios
```

# ⚙️ Cargo Configuration
In your main project, add a ***.cargo/config.toml*** file with:
```
[alias]
multibuild = "run -p xtask --"

[target.x86_64-pc-windows-gnu]
linker = "x86_64-w64-mingw32-gcc"

[target.aarch64-linux-android]
linker = "aarch64-linux-android21-clang"
```
This enables the ***cargo multibuild*** alias and configures the proper linkers for cross-compilation.

# 📦 Workspace Setup
Make sure your ***Cargo.toml*** includes the ***xtask*** helper project in the workspace:
```
[workspace]
members = [
    ".",
    "xtask",
]
```

# 📂 Project Structure Example
A typical project layout with this script looks like:
```
my-project/
├── Cargo.toml
├── src/
│   └── main.rs
├── assets/             # optional resources copied into dist/
│   ├── icons/
│   └── config.json
├── xtask/              # helper project with the build script
│   ├── Cargo.toml
│   └── src/main.rs     # your multibuild script
└── .cargo/
    └── config.toml     # cargo alias & linker configuration
```
After running ***cargo multibuild***, results will appear in:
```
target/dist/<platform>/
```

# 🛠 Features per Platform
The script automatically enables ***--features <platform>*** during the build.

Your ***Cargo.toml*** should define them like this:
```
[features]
windows = []
linux   = []
macos   = []
android = []
ios     = []
```
This allows platform-specific conditional compilation.

# 🚀 Usage
Build a single target
```
cargo multibuild --release --target windows
cargo multibuild --release --target linux
cargo multibuild --release --target android
cargo multibuild --release --target ios   # macOS only
cargo multibuild --release --target macos # macOS only
```
Build all targets
```
cargo multibuild --release
```

# ✅ Summary
1) Automates cross-compilation for Windows, Linux, macOS, iOS, and Android.
2) Copies build artifacts into ***target/dist/platform***.
3) Supports platform-specific ***[features]***.
4) Provides a ***cargo multibuild*** alias for convenience.
