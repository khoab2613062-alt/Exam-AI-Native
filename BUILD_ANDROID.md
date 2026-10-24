# Build Android APK (Rust native)

> **Status:** Source prototype; not compiled or tested in this environment. No APK binary is included.

## Requirements
- Rust stable + rustup
- JDK 17
- Android SDK (platform-tools + build-tools + platform 35)
- Android NDK 26+ (configure `ANDROID_NDK_HOME`)
- `ANDROID_HOME` / `ANDROID_SDK_ROOT`

## Build on Windows (PowerShell)
```powershell
rustup target add aarch64-linux-android
cargo install cargo-apk
$env:ANDROID_HOME = 'C:\Users\YOUR_USER\AppData\Local\Android\Sdk'
$env:ANDROID_NDK_HOME = "$env:ANDROID_HOME\ndk\YOUR_NDK_VERSION"
cargo apk build --release --target aarch64-linux-android
```

Find the APK under `target/aarch64-linux-android/release/apk/` (cargo-apk may choose a version-dependent name).

If cargo-apk or Slint dependency APIs have changed, check their current Android integration documentation. This project has not been compiled here; build errors are possible.

## Implemented
- Native Slint UI and question entry
- DOCX/TXT parser (only readable filesystem paths)
- Local JSON bank
- Hybrid backend routing logic + validation schema/tests

## Not yet implemented (do not mistake for working features)
- Actual OpenAI backend / secure server
- llama.cpp integration / GGUF model download
- 30-question AI generation, explanations and source verification
- Android Storage Access Framework file picker
- SQLite, edit/delete UI, complete exam player
- Liquid Glass refraction shader

The `agent.rs` module defines contracts and routing; it **does not** call AI APIs. Avoid embedding OpenAI API keys in an APK.
