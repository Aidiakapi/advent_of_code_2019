use reqwest::{Client, StatusCode};

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
    match client
        .get(&url)
        .header("cookie", format!("session={}", token))
        .send()
        .and_then(|mut resp| resp.text().map(|text| (resp, text)))
    {
        Ok((resp, text)) => {
            if resp.status() != StatusCode::OK {
                eprintln!("expected HTTP 200 OK status, got {:?}", resp.status());
                std::process::exit(-1);
            }

            match std::fs::create_dir_all("./data/")
                .and_then(|_| std::fs::write(format!("./data/day{:0>2}.txt", day_nr), text))
            {
                Ok(()) => {}
                Err(err) => {
                    eprintln!("couldn't write downloaded input to file:\n{:?}", err);
                    std::process::exit(-1);
                }
            }
        }
        Err(err) => {
            eprintln!("http error:\n{:?}", err);
            std::process::exit(-1);
        }
    };
}
