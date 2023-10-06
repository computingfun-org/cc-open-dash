use cc_core::{JobNumber, JobURL};

fn main() {
    let args = std::env::args().skip(1);
    match args.len() == 0 {
        true => open_jobs_from_clipboard(),
        false => args.for_each(|s| open_jobs(&s)),
    }
}

fn open_jobs_from_clipboard() {
    let clipboard = match cli_clipboard::get_contents() {
        Ok(contents) => contents,
        Err(err) => return eprintln!("{err}"),
    };
    open_jobs(clipboard.as_str());
}

fn open_jobs(job_nums: &str) {
    job_nums.split_whitespace().for_each(open_job)
}

fn open_job(job_num: &str) {
    use std::str::FromStr;
    let url = match JobNumber::from_str(job_num) {
        Ok(num) => JobURL::from(num),
        Err(err) => return eprintln!("{err}"),
    };
    match open::that(url.as_str()) {
        Ok(_) => println!("{url}"),
        Err(err) => eprintln!("{err}"),
    }
}
