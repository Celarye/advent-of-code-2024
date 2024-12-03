// TODO: Replace the Curl subprocess with a basic HTTP/2 request using std::io::net
// TODO: Cache the response
// TODO: Prevent early fetching
// TODO: Improve input parsing

use std::{env, process::Command};

pub fn request(day: u8, part: Option<u8>) -> Result<String, ()> {
    match part {
        Some(part_number) => {
            println!(
                "Requesting the input of day {}, part {}",
                &day, &part_number
            )
        }
        None => println!("Requesting the input of day {}", &day),
    }

    let p_request = Command::new("/usr/bin/curl")
        .arg("-H")
        .arg(format!(
            "Cookie: session={}",
            env::var("SESSION").unwrap_or_else(|_| "".to_string())
        ))
        .arg(format!("https://adventofcode.com/2024/day/{}/input", day))
        .output();

    match p_request {
        Ok(p_input) => match String::from_utf8(p_input.stdout) {
            Ok(input) => match input.as_str() {
                "Puzzle inputs differ by user.  Please log in to get your puzzle input.\n" => {
                    eprintln!("An error occurred while requesting the input: the provided session token is invalid");
                    Err(())
                }
                "Please don't repeatedly request this endpoint before it unlocks! The calendar countdown is synchronized with the server time; the link will be enabled on the calendar the instant this puzzle becomes available.\n" => {
                    eprintln!("An error occurred while requesting the input: the puzzle has not been released yet");
                    Err(())
                }
                _ => {
                    println!("Successfully requested the input");
                    Ok(input)
                }
            }
            Err(err) => {
                eprintln!("An error occurred while requesting the input: {}", &err);
                Err(())
            },
        },
        Err(err) => {
            eprintln!("An error occurred while requesting the input: {}", &err);
            Err(())
        },
    }
}
