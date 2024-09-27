use reqwest::blocking::Client;
use reqwest::Url;
use scraper::{Html, Selector};
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::PathBuf;


fn download_page(url: &str, client: &Client) -> Result<String, Box<dyn std::error::Error>> {
    let response = client.get(url).send()?.text()?;
    Ok(response)
}

fn save_page(file_path: &PathBuf, content: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(file_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn parse_links(content: &str, base_url: &str) -> Vec<String> {
    let document = Html::parse_document(content);
    let selector = Selector::parse("a").unwrap();
    let mut links = Vec::new();

    for element in document.select(&selector) {
        if let Some(link) = element.value().attr("href") {
            let full_url = Url::parse(link).unwrap_or_else(|_| {
                let base = Url::parse(base_url).unwrap();
                base.join(link).unwrap()
            });
            links.push(full_url.to_string());
        }
    }
    links
}

fn download_site(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    // Скачиваем главную страницу
    let content = download_page(url, &client)?;
    let parsed_url = Url::parse(url)?;
    let domain = parsed_url.domain().unwrap_or("");
    let path = parsed_url.path();

    // Создание директории для сохранения
    let dir_path = PathBuf::from(format!("{}{}", domain, path));
    create_dir_all(&dir_path)?;

    // Сохранение страницы
    let file_path = dir_path.join("index.html");
    save_page(&file_path, &content)?;

    // Парсим ссылки и скачиваем другие страницы
    let links = parse_links(&content, url);
    for link in links {
        let page_content = download_page(&link, &client)?;
        let link_parsed = Url::parse(&link)?;
        let link_path = PathBuf::from(format!("{}{}", domain, link_parsed.path()));
        create_dir_all(link_path.parent().unwrap())?; // Создаем директорию
        let link_file_path = link_path.with_extension("html");
        save_page(&link_file_path, &page_content)?;
    }

    Ok(())
}

fn main() {
    let url = "https://tech.wildberries.ru/cabinet/courses/rust";
    if let Err(err) = download_site(url) {
        eprintln!("Error downloading site: {}", err);
    }
}
