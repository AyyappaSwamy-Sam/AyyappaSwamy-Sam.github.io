use std::{fs, io};
fn escape(s: &str) -> String { s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;") }
fn main() -> io::Result<()> {
    let template = fs::read_to_string("src/template.html")?;
    let data = fs::read_to_string("content/projects.tsv")?;
    let mut cards = String::new();
    for (line_no, line) in data.lines().enumerate() {
        if line.trim().is_empty() { continue; }
        let fields: Vec<&str> = line.split('\t').collect();
        if fields.len() != 8 { return Err(io::Error::new(io::ErrorKind::InvalidData, format!("project line {} must have 8 fields", line_no + 1))); }
        let f: Vec<String> = fields.iter().map(|v| escape(v)).collect();
        let link = if f[6].is_empty() { String::new() } else { format!("<a class=\"project-link\" href=\"{}\" target=\"_blank\" rel=\"noopener noreferrer\">{} <span aria-hidden=\"true\">↗</span></a>",f[6],f[7]) };
        cards.push_str(&format!("<article class=\"project\" data-category=\"{}\"><div class=\"project-num\">{}</div><div class=\"project-main\"><p class=\"eyebrow\">{}</p><h3>{}</h3><p class=\"project-description\">{}</p><p class=\"project-tech\">{}</p>{}</div><span class=\"project-arrow\" aria-hidden=\"true\">↗</span></article>\n",f[1],f[0],f[2],f[3],f[4],f[5],link));
    }
    if !template.contains("{{PROJECTS}}") { return Err(io::Error::new(io::ErrorKind::InvalidData,"missing projects placeholder")); }
    fs::write("index.html", template.replace("{{PROJECTS}}", &cards))
}
