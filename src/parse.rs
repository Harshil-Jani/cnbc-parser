use scraper::{Html, Selector};
use serde_json::Value;

pub fn extract_gallery_data(html_content: &str) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
    let document = Html::parse_document(html_content);
    let script_selector = Selector::parse("script#\\__NEXT_DATA__").unwrap();

    let script_tag = document.select(&script_selector).next();
    if script_tag.is_none() {
        return Err("No script tag with id '__NEXT_DATA__' found.".into());
    }

    let script_content = script_tag.unwrap().inner_html();
    let json_data: Value = serde_json::from_str(&script_content)?;

    let gallery_data = json_data["props"]["pageProps"]["article"]["gallery"].as_array();
    if gallery_data.is_none() {
        return Err("Gallery object not found in __NEXT_DATA__.".into());
    }

    Ok(gallery_data.unwrap().clone())
}
