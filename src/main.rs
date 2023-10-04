use cc_core::{JobNumber, JobURL};

fn main() {
    let results = std::env::args()
        .skip(1)
        .map(|input| input.trim().parse::<JobNumber>())
        .map(|input| input.map(|num| JobURL::from(num)))
        .map(|input| input.map(|url| open::that(url.as_str()).map(|_| url)));

    for result in results {
        match result {
            Ok(ok) => match ok {
                Ok(ok) => println!("Opened: {}", ok),
                Err(err) => eprintln!("Error: {}", err),
            },
            Err(err) => eprintln!("Error: {}", err),
        }
    }
}
