mod agent;
slint::include_modules!();
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, fs, io::Read, path::Path, rc::Rc};

#[derive(Clone, Serialize, Deserialize)]
struct Question { text: String, options: [String; 4], answer: usize }

fn paragraphs(path: &Path) -> Result<Vec<String>, String> {
    let ext = path.extension().and_then(|x| x.to_str()).unwrap_or("").to_ascii_lowercase();
    if ext == "txt" {
        return fs::read_to_string(path).map(|s| s.lines().map(str::trim).filter(|x| !x.is_empty()).map(str::to_string).collect()).map_err(|e| e.to_string());
    }
    if ext != "docx" { return Err("Chỉ hỗ trợ DOCX hoặc TXT".into()); }
    let file = fs::File::open(path).map_err(|e| e.to_string())?;
    let mut archive = zip::ZipArchive::new(file).map_err(|e| e.to_string())?;
    let mut xml = String::new();
    archive.by_name("word/document.xml").map_err(|e| e.to_string())?.read_to_string(&mut xml).map_err(|e| e.to_string())?;
    let mut reader = quick_xml::Reader::from_str(&xml);
    let mut result = Vec::new();
    let mut paragraph = String::new();
    let mut in_text = false;
    loop {
        match reader.read_event() {
            Ok(quick_xml::events::Event::Start(e)) => {
                if e.name().as_ref() == b"w:t" { in_text = true; }
            }
            Ok(quick_xml::events::Event::Text(e)) if in_text => {
                if let Ok(t) = e.unescape() { paragraph.push_str(&t); }
            }
            Ok(quick_xml::events::Event::End(e)) => match e.name().as_ref() {
                b"w:t" => in_text = false,
                b"w:p" => { if !paragraph.trim().is_empty() { result.push(paragraph.trim().to_string()); } paragraph.clear(); }
                _ => {}
            },
            Ok(quick_xml::events::Event::Eof) => break,
            Err(e) => return Err(e.to_string()),
            _ => {}
        }
    }
    Ok(result)
}

fn parse_questions(lines: &[String]) -> Vec<Question> {
    let mut questions = Vec::new();
    let mut current: Option<Question> = None;
    for line in lines {
        let s = line.trim();
        let upper = s.to_uppercase();
        let option = ["A.", "B.", "C.", "D.", "A)", "B)", "C)", "D)"].iter().position(|p| upper.starts_with(p));
        if let Some(i) = option {
            if let Some(q) = current.as_mut() { q.options[i % 4] = s[2..].trim().to_string(); }
        } else if upper.starts_with("ĐÁP ÁN:") || upper.starts_with("DAP AN:") {
            if let Some(q) = current.as_mut() {
                if let Some(c) = s.chars().last() { if ('A'..='D').contains(&c.to_ascii_uppercase()) { q.answer = (c.to_ascii_uppercase() as u8 - b'A') as usize; } }
            }
        } else if upper.starts_with("CÂU ") || upper.starts_with("CAU ") || upper.starts_with("QUESTION ") {
            if let Some(q) = current.take() { questions.push(q); }
            current = Some(Question { text: s.to_string(), options: Default::default(), answer: 0 });
        } else if let Some(q) = current.as_mut() { q.text.push(' '); q.text.push_str(s); }
    }
    if let Some(q) = current { questions.push(q); }
    questions
}

fn start() -> Result<(), slint::PlatformError> {
    let app = App::new()?;
    let bank = Rc::new(RefCell::new(Vec::<Question>::new()));
    let path = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    let save_path = Path::new(&path).join("exam_bank.json");
    if let Ok(data) = fs::read(&save_path) { if let Ok(saved) = serde_json::from_slice(&data) { *bank.borrow_mut() = saved; } }
    app.set_count_text(format!("{} câu hỏi", bank.borrow().len()).into());
    let weak = app.as_weak(); let store = bank.clone(); let dest = save_path.clone();
    app.on_save_question(move || {
        if let Some(ui) = weak.upgrade() {
            let text = ui.get_question_text().to_string();
            if text.trim().is_empty() { ui.set_status_text("Hãy nhập nội dung câu hỏi".into()); return; }
            let options = [ui.get_option_a().to_string(), ui.get_option_b().to_string(), ui.get_option_c().to_string(), ui.get_option_d().to_string()];
            let answer = match ui.get_answer().as_str() { "B" => 1, "C" => 2, "D" => 3, _ => 0 };
            store.borrow_mut().push(Question { text, options, answer });
            if let Ok(json) = serde_json::to_vec_pretty(&*store.borrow()) { let _ = fs::write(&dest, json); }
            ui.set_count_text(format!("{} câu hỏi", store.borrow().len()).into());
            ui.set_status_text("Đã lưu câu hỏi".into());
        }
    });
    let weak = app.as_weak(); let store = bank.clone(); let dest = save_path.clone();
    app.on_import_document(move |file| {
        if let Some(ui) = weak.upgrade() {
            match paragraphs(Path::new(file.as_str())) {
                Ok(lines) => {
                    let found = parse_questions(&lines);
                    let n = found.len(); store.borrow_mut().extend(found);
                    if let Ok(json) = serde_json::to_vec_pretty(&*store.borrow()) { let _ = fs::write(&dest, json); }
                    ui.set_count_text(format!("{} câu hỏi", store.borrow().len()).into());
                    ui.set_status_text(format!("Đã nhập {n} câu; hãy kiểm tra lại đáp án").into());
                }
                Err(e) => ui.set_status_text(format!("Không thể nhập: {e}").into()),
            }
        }
    });
    let weak = app.as_weak();
    app.on_export_bank(move || { if let Some(ui) = weak.upgrade() { ui.set_status_text(format!("Dữ liệu lưu tại {}", save_path.display()).into()); } });
    app.run()
}

#[cfg(target_os = "android")]
#[no_mangle]
pub fn android_main(app: android_activity::AndroidApp) {
    slint::android::init(app).expect("Slint Android init failed");
    start().expect("App failed");
}

#[cfg(not(target_os = "android"))]
pub fn run_desktop() { start().expect("App failed"); }
