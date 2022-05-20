use chrono::prelude::*;

use rand::Rng;
use rand::thread_rng;
use rand::seq::SliceRandom;

use fake::Fake;
use fake::locales::EN;
use fake::faker::internet::raw::IPv4;

use clap::Parser;

/// Generate fake apache logs (CLF Format)
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
   /// Number of logs to generate
   #[clap(short, long, default_value = &"10")]
   number: usize 
}

fn main() {
    let args = Args::parse();

    let mut rng = thread_rng();
    let verbs = ["GET", "POST", "PUT", "DELETE"];
    let urls = ["/update", "/create", "/search", "/delete"];
    let codes = ["200", "301", "404", "500"];

    for _ in 0 .. args.number {
        let ip: String = IPv4(EN).fake();
        let date: String = Utc::now().format("%d/%b/%Y:%H:%M:%S").to_string();
        let verb = verbs.choose(&mut rng).unwrap();
        let url = urls.choose(&mut rng).unwrap();
        let id: usize = rng.gen_range(1 .. 1000);
        // let ua: String = UserAgent(EN).fake();
        let code = codes.choose(&mut rng).unwrap();
        let size: usize = rng.gen_range(200 .. 20000);

        println!("{} - - [{}] \"{} +0000 {}/{} HTTP/1.0\" {} {}", ip, date, verb, url, id, code, size);
    }
}
