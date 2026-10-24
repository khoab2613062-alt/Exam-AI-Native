# Exam AI Native — Rust Android source

**Native Rust + Slint**, no HTML/WebView. Prototype for a question bank and future hybrid AI agent.

Read [BUILD_ANDROID.md](BUILD_ANDROID.md) to build. See `src/agent.rs` for online/offline routing and structured question validation. Actual AI inference is not wired yet.

Input document format:
```
Câu 1: Nội dung câu hỏi?
A. Đáp án A
B. Đáp án B
C. Đáp án C
D. Đáp án D
Đáp án: B
```
DOCX and TXT only. Imported answers must be reviewed. Android file paths may not be accessible without SAF integration.
