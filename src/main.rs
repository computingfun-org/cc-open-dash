use cc_core::{JobNumber, JobURL};

fn main() {
    let results = std::env::args()
        .skip(1)
        .map(|args| args.trim().parse::<JobNumber>())
        .map(|num| num.map(|num| JobURL::from(num)))
        .map(|url| url.map(|url| open::that(url.as_str()).map(|_| url)));

    for result in results {
        match result {
            Ok(parsed) => match parsed {
                Ok(opened) => println!("Opened: {}", opened),
                Err(opened_err) => eprintln!("Error: {}", opened_err),
            },
            Err(parsed_err) => eprintln!("Error: {}", parsed_err),
        }
    }
}
