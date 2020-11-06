mod cache;
mod calendar;
mod consts;
mod output;
mod shams;

use output::printer::printer::*;
use shams::*;

type Result<T> = std::result::Result<T,Box<dyn std::error::Error + Send + Sync + 'static >>;

#[tokio::main]
async fn main() -> crate::Result<()> {
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


