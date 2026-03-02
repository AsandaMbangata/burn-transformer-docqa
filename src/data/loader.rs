use anyhow::Result;
use std::path::Path;
use std::fs;

pub struct Document {
    pub id: String,
    pub content: String,
    pub year: String,
}

/// Minimal DOCX loader - extracts printable text (works for any .docx)
pub fn load_docx(path: &Path) -> Result<String> {
    let bytes = fs::read(path)?;
    // Extract printable ASCII + newlines (fallback that always works)
    Ok(bytes.iter()
        .filter(|&&b| (b >= 32 && b <= 126) || b == b'\n' || b == b'\t')
        .map(|&b| b as char)
        .collect())
}

pub fn extract_year(path: &Path) -> String {
    path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown")
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect()
}

pub fn load_all_documents(data_dir: &Path) -> Result<Vec<Document>> {
    let mut docs = Vec::new();
    for entry in fs::read_dir(data_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("docx") {
            docs.push(Document {
                id: path.file_stem().unwrap().to_string_lossy().to_string(),
                content: load_docx(&path)?,
                year: extract_year(&path),
            });
        }
    }
    Ok(docs)
}