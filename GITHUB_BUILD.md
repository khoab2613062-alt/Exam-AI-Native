# Build APK on GitHub Actions (from Android phone)

1. Extract the ZIP. Upload all project files to the **root** of a GitHub repository, including hidden `.github/workflows/android-apk.yml`.
2. Open the repository -> **Actions** -> **Build Android APK** -> **Run workflow**.
3. Wait for the build job. If green, open the run and download `exam-ai-android-arm64-apk` under **Artifacts**.
4. Extract the artifact ZIP and install the APK on an ARM64 Android device. The APK may be debug-signed by cargo-apk; do not distribute as a production release without proper signing.

A push to main/master also starts the workflow. The source is a prototype and has not been compiled or verified in this environment. A failed workflow means the compiler or Android packaging errors must be fixed before an APK exists. Open the failing step and share the log.

No OpenAI keys are required because the AI service integration is NOT implemented. This project does not yet generate 30 questions with AI or run llama.cpp.
