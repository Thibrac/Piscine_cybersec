use reqwest::{blocking::Response, Url};
use scraper::Selector;
use std::fs;

pub fn check_url(url: &str) -> Result<Response, Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get(url)?;
    Ok(response)
}

fn get_full_url(base_url: &str, image_url: &str) -> String {
    if image_url.starts_with("http") {
        image_url.to_string()
    } else if image_url.starts_with("//") {
        format!("http:{}", image_url)
    } else {
        let base = Url::parse(base_url).unwrap();
        base.join(image_url).unwrap().to_string()
    }
}

fn dl_img(img_url: &str, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let filename = img_url.split("/").last().unwrap_or("image.jpg");
    let file_path = format!("{}/{}", path, filename);
    fs::create_dir_all(path)?;

    let mut file = fs::File::create(&file_path)?;
    reqwest::blocking::get(img_url)
        .unwrap()
        .copy_to(&mut file)
        .unwrap();
    Ok(())
}

pub fn extract_img(url: &str, path: &str, response: &str) {
    let doc = scraper::Html::parse_document(response);
    let selector = Selector::parse("img").unwrap();
    let extensions = [".jpg", ".jpeg", ".png", ".gif", ".bmp"];

    for el in doc.select(&selector) {
        if let Some(img_url) = el.value().attr("src") {
            if extensions.iter().any(|ext| img_url.ends_with(ext)) {
                let full_url = get_full_url(url, &img_url);
                if let Err(e) = dl_img(&full_url, path) {
                    println!("Bug: {}", e);
                }
            }
        }
    }
}

fn extract_links(url: &str, response: &str) -> Vec<String> {
    let mut links: Vec<String> = Vec::new();
    let doc = scraper::Html::parse_document(response);
    let selector = Selector::parse("a").unwrap();

    for el in doc.select(&selector) {
        if let Some(link) = el.value().attr("href") {
            let full_url = get_full_url(url, &link);
            links.push(full_url.to_string());
        }
    }
    links
}

pub fn extract_recursive(url: &str, path: &str, depth: u32) {
    if depth == 0 {
        return;
    }
    let mut links = Vec::new();
    match check_url(url) {
        Ok(response) => {
            let response_txt = response.text().unwrap();
            println!("URL = {url}");
            extract_img(url, path, &response_txt);
            links = extract_links(url, &response_txt);
        }
        Err(e) => {
            println!("Error = {e}");
        }
    }
    for link in links {
        extract_recursive(&link, path, depth - 1);
    }
}
