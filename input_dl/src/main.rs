use reqwest::{Client, StatusCode};
use std::process::Command;

fn main() {
    let day_nr: u8 = match std::env::args()
        .skip(1)
        .next()
        .and_then(|day| day.parse().ok())
    {
        Some(x) => x,
        None => {
            eprintln!("expected 1 argument with the day number");
            std::process::exit(-1);
        }
    };

    let url = format!("https://adventofcode.com/2019/day/{}/input", day_nr);
    let token = match std::fs::read_to_string("./token.txt") {
        Ok(token) => token,
        Err(err) => {
            eprintln!("cannot read token\n{:?}", err);
            std::process::exit(-1);
        }
    };

    let client = Client::new();
    let (resp, text) = match client
        .get(&url)
        .header("cookie", format!("session={}", token))
        .send()
        .and_then(|mut resp| resp.text().map(|text| (resp, text)))
    {
        Ok(x) => x,
        Err(err) => {
            eprintln!("http error:\n{:?}", err);
            std::process::exit(-1);
        }
    };
    if resp.status() != StatusCode::OK {
        eprintln!("expected HTTP 200 OK status, got {:?}", resp.status());
        std::process::exit(-1);
    }

    let file_path = format!("./data/day{:0>2}.txt", day_nr);
    match std::fs::create_dir_all("./data/").and_then(|_| std::fs::write(&file_path, text)) {
        Ok(()) => {}
        Err(err) => {
            eprintln!("couldn't write downloaded input to file:\n{:?}", err);
            std::process::exit(-1);
        }
    }

    let _ = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .arg("/C")
            .arg(&format!("code {}", file_path))
            .spawn()
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(&format!("code {}", file_path))
            .spawn()
    };
}
