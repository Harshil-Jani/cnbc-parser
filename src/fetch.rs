use reqwest;

pub fn fetch_html(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let response = client.get(url).header("Accept", "*/*").send()?;

    if !response.status().is_success() {
        return Err(format!("Failed to fetch the URL: {}", url).into());
    }

    Ok(response.text()?)
}
