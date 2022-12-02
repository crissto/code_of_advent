use std::{
    env,
    fs::{self, OpenOptions},
    io::Write,
};

use aoc_2022::GenericResult;
use clap::Parser;
use reqwest::{cookie::Jar, Url};

const APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " (rust; 2022; gh: @crissto)"
);

const AOC_DOMAIN: &str = "adventofcode.com";

const AOC_YEAR: &str = "2022";

fn fetch_input(day: &str) -> GenericResult<String> {
    let session =
        std::fs::read_to_string(".session").expect("Missing .session file with AOC session cookie");
    let cookie = format!("session={session}; Domain=.{AOC_DOMAIN}");
    let url = format!("https://{AOC_DOMAIN}").parse::<Url>()?;

    let cookie_store = Jar::default();
    cookie_store.add_cookie_str(&cookie, &url);

    let client = reqwest::blocking::Client::builder()
        .user_agent(APP_USER_AGENT)
        .cookie_store(true)
        .cookie_provider(cookie_store.into())
        .build()?;

    let day_url = format!("https://{AOC_DOMAIN}/{AOC_YEAR}/day/{day}/input");
    let response = client.get(&day_url).send()?.text()?;
    let stripped_trailing_new_line = response
        .strip_suffix("\r\n")
        .or(response.strip_suffix("\n"))
        .unwrap_or(&response);
    Ok(stripped_trailing_new_line.to_string())
}

fn write_input_to_day(day: &str, contents: String) {
    fs::write(format!("./src/day{day}/input.txt"), contents).expect("Error writing input");
}

fn create_day_folder(day: &str) {
    fs::create_dir_all(format!("./src/day{day}")).expect("Error creating folder");
}

fn write_stub(day: &str) {
    let stub = std::fs::read_to_string("./src/create-new-day/solution.rs.stub").unwrap();
    let replaced = stub.replace("$DAY_REPLACER$", day);
    fs::write(format!("./src/day{day}/solution.rs"), replaced).expect("Error writing stub");
}

fn add_to_bins(day: &str) {
    let stub: String = format!(
        "

[[bin]]
name = \"day{day}\"
path = \"src/day{day}/solution.rs\""
    );

    let mut cargo_toml = OpenOptions::new()
        .write(true)
        .append(true)
        .open("Cargo.toml")
        .expect("Error openning cargo");

    cargo_toml
        .write(stub.as_bytes())
        .expect("Unable to write to file");
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    day: String,
}

fn main() {
    let args = Args::parse();
    let contents = fetch_input(&args.day).unwrap();

    create_day_folder(&args.day);
    write_input_to_day(&args.day, contents);
    write_stub(&args.day);
    add_to_bins(&args.day);
}
