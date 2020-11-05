mod cache;
mod calendar;
mod consts;
mod output;

use output::body_parser::body_parser as parser;
use output::printer::printer::*;
use cache::cache::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        go_with_cache().await?;
    }else if args.len() > 2 {
        println!("too many args! see 'shams -h.");
    }else {
        match args[1].as_str() {
            "-h" | "--help" => { print_help() }
            "-u" | "--update" => { request().await? }
            "-a" | "--about" => { print_about() }
            _ => { print_help() }
        }
    }

    Ok(())
}

async fn go_with_cache() -> Result<(), Box<dyn std::error::Error>> {
    let cache_result = restore().await;
    match cache_result {
        Ok(cache) => {
            print(&cache.calendar)
        }
        Err(_) => {
            request().await?
        }
    }

    Ok(())
}

async fn request() -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get("https://time.ir")
        .await?
        .text()
        .await?;
    let calendar = parser::process_the_body(response).await;
    print(&calendar);
    store(calendar).await?;

    Ok(())
}

