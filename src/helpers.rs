pub fn download(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get(url)?.text()?;
    Ok(resp)
}
