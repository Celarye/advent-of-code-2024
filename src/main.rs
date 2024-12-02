use std::process::ExitCode;

mod days;
mod utils;

fn main() -> ExitCode {
    if let Err(exitcode) = utils::env::load_session_token() {
        eprintln!("Exiting the program");
        return ExitCode::from(exitcode);
    }

    // TODO: Allow giving a range
    let all_results = days::results();

    let mut day: u8 = 1;
    println!("\nResults:\n");
    for day_results in all_results.iter() {
        println!("Day {}:\n", &day);
        day += 1;

        let mut part: u8 = 1;
        for part_result in day_results.iter() {
            print!("  Part {}: ", &part);
            part += 1;

            println!("{}\n", &part_result);
        }
    }

    println!("Exiting the program");
    return ExitCode::from(0x0);
}
