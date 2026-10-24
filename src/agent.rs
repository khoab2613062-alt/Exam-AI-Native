//! Hybrid AI routing and validated exam plans. Network/model adapters are intentionally explicit.
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Mode { Auto, Online, Offline }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Backend { OpenAi, LlamaCpp }

pub fn select_backend(mode: Mode, internet: bool, local_model_ready: bool) -> Result<Backend, String> {
    match mode {
        Mode::Online if internet => Ok(Backend::OpenAi),
        Mode::Offline if local_model_ready => Ok(Backend::LlamaCpp),
        Mode::Auto if internet => Ok(Backend::OpenAi),
        Mode::Auto if local_model_ready => Ok(Backend::LlamaCpp),
        Mode::Online => Err("Không có kết nối Internet".into()),
        Mode::Offline => Err("Chưa cài mô hình GGUF".into()),
        Mode::Auto => Err("Không có mạng và chưa cài mô hình offline".into()),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedQuestion {
    pub chapter: u8,
    pub question: String,
    pub options: [String; 4],
    pub correct_answer: usize,
    pub explanation: String,
    pub source_ids: Vec<String>,
}

pub fn validate(questions: &[GeneratedQuestion], chapter1: usize, chapter2: usize) -> Result<(), String> {
    if questions.len() != chapter1 + chapter2 { return Err("Sai số lượng câu hỏi".into()); }
    if questions.iter().filter(|q| q.chapter == 1).count() != chapter1 ||
       questions.iter().filter(|q| q.chapter == 2).count() != chapter2 {
        return Err("Sai phân bổ chương".into());
    }
    let mut seen = std::collections::HashSet::new();
    for (i, q) in questions.iter().enumerate() {
        if q.question.trim().is_empty() || q.options.iter().any(|x| x.trim().is_empty()) ||
           q.correct_answer >= 4 || q.explanation.trim().is_empty() || q.source_ids.is_empty() {
            return Err(format!("Câu {} thiếu dữ liệu hoặc nguồn", i+1));
        }
        if !seen.insert(q.question.trim().to_lowercase()) { return Err(format!("Câu {} trùng nội dung", i+1)); }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn routing() {
        assert_eq!(select_backend(Mode::Auto, false, true).unwrap(), Backend::LlamaCpp);
        assert!(select_backend(Mode::Offline, true, false).is_err());
    }
}
