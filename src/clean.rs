use regex::Regex;

pub fn clean_html(raw_html: &str) -> String {
    let cleanr = Regex::new(r"<.*?>").unwrap();
    let mut cleaned = cleanr.replace_all(raw_html, "").to_string();
    cleaned = cleaned.replace('\u{20b9}', ""); // Remove ₹ symbol
    cleaned = cleaned.replace('\u{2019}', ""); // Remove ’ symbol
    cleaned.replace('\\', "").trim().to_string()
}
