mod clean;
mod fetch;
mod parse;
mod save;

use chrono::Local;
use clean::clean_html;
use fetch::fetch_html;
use parse::extract_gallery_data;
use save::save_to_file;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();

    // Ensure at least the URL is provided
    if args.len() < 2 {
        eprintln!("Usage: {} <URL> [filename-to-save]", args[0]);
        std::process::exit(1);
    }

    let url = &args[1];
    let file_name = if args.len() > 2 {
        args[2].clone()
    } else {
        Local::now().format("%Y-%m-%d").to_string() // Default to today's date
    };

    // Fetch HTML content
    let html_content = fetch_html(url)?;

    // Parse gallery data
    let gallery_items = extract_gallery_data(&html_content)?;

    // Clean and process data
    let mut company_news = std::collections::HashMap::new();
    for item in gallery_items {
        if let Some(caption) = item["caption"].as_str() {
            let cleaned_caption = clean_html(caption);
            if cleaned_caption.contains(" | ") {
                let parts: Vec<&str> = cleaned_caption.splitn(2, " | ").collect();
                if parts.len() == 2 {
                    company_news.insert(parts[0].trim().to_string(), parts[1].trim().to_string());
                }
            }
        }
    }

    // Save data to the specified file
    save_to_file(&file_name, &company_news)?;
    Ok(())
}
