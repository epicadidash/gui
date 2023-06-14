use reqwest::{Client, Response};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, Write};
use regex::Regex;
use tokio;

async fn get_url(url: &str, input:String) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let response = client
        .post("https://www.getfvid.com/downloader")
        .form(&[("url", url)])
        .send()
        .await?;

    let body = response.text().await?;
    let nama_video = format!("noname_{}.mp4", rand::random::<u64>());

    let private_regex = Regex::new(r"Uh-Oh! This video might be private and not publi")?;
    if private_regex.is_match(&body) {
        println!("\x1b[31m[-] This Video Is Private\x1b[0m");
        return Ok(());
    }

    let nama_regex = Regex::new(r#"<p class="card-text">(.*?)<\/p>"#)?;
    if let Some(capture) = nama_regex.captures(&body) {
        if let Some(nama) = capture.get(1) {
            let nama_video = format!("{}.mp4", nama.as_str());
        }
    }

    let rgx = Regex::new(r#"<a href="(.+?)" target="_blank" class="btn btn-download"(.+?)>(.+?)<\/a>"#)?;
    let res_akhir: Vec<Vec<String>> = rgx
        .captures_iter(&body)
        .enumerate()
        .map(|(i, cap)| {
            let item = cap.get(3).map_or("", |m| m.as_str());
            let link = cap.get(1).map_or("", |m| m.as_str()).replace("amp;", "");
            vec![item.to_string(), link]
        })
        .collect();

    if res_akhir.is_empty() {
        println!("\x1b[31m[-] Invalid Video URL\n\x1b[0m");
    } else {
        let option: usize = input.trim().parse()?;

        if let Some(download) = res_akhir.get(option - 1) {
            let name = sanitize_filename::sanitize(&nama_video);
            download_file(name, &download[1]).await?;
        } else {
            println!("\x1b[31m[-] Invalid option\n\x1b[0m");
        }
    }

    Ok(())
}

async fn download_file(name: String, url: &str) -> Result<(), Box<dyn Error>> {
    if !std::path::Path::new("fb video").exists() {
        std::fs::create_dir("fb video")?;
    }

    let path = format!("fb video/{}", name);
    let mut file = File::create(&path)?;

    let client = Client::new();
    let mut response = client.get(url).send().await?;
    io::copy(&mut response.bytes().await?.as_ref(), &mut file)?;

    println!("\x1b[32m[+] Download Success: {}\x1b[0m", name);

    Ok(())
}

fn start() -> Result<(), Box<dyn Error>> {
    println!(
        "\x1b[32m\n================================================\n\
        Facebook Video Downloader\n\
        =================================================\n\x1b[0m"
    );

    let mut input = String::new();
    println!("\x1b[32mInput URL: \x1b[0m");
    io::stdin().read_line(&mut input)?;
    let url = input.trim();
    println!("\x1b[32m\n[+] Checking Video URL...\x1b[0m");
    let input:String= String::from("1");
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(get_url(url,input))?;

    Ok(())
}

fn main() {
    if let Err(err) = start() {
        eprintln!("Something happened: {}", err);
    }
}
