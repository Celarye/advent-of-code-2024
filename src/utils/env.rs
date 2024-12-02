use std::{env, fs};

pub fn load_session_token() -> Result<(), u8> {
    println!("Loading all environment variables from the dotenv file");

    let p_dotenv_vec = fs::read(".env");

    let dotenv_vec;
    match p_dotenv_vec {
        Ok(r_dotenv_vec) => dotenv_vec = r_dotenv_vec,
        Err(err) => {
            eprintln!("Failed to load the dotenv file, error: {}", &err);
            return Err(0x1);
        }
    }

    let p_dotenv_string = String::from_utf8(dotenv_vec);

    let dotenv_string;
    match p_dotenv_string {
        Ok(r_dotenv_string) => dotenv_string = r_dotenv_string,
        Err(err) => {
            eprintln!(
                "Failed to parse the dotenv vec<u8> into a String, error: {}",
                &err
            );
            return Err(0x1);
        }
    }

    let dotenv_str_lines = dotenv_string.split("\n");

    for dotenv_str in dotenv_str_lines {
        if dotenv_str.len() == 0 {
            continue;
        }

        let key_value: Vec<&str> = dotenv_str.split("=").collect();

        if key_value.len() != 2 {
            eprintln!(
                "A line from the dotenv file was ignored because of it not being a valid key=value pair, line: \"{}\"",
                &dotenv_str
            );
            continue;
        }

        env::set_var(&key_value[0], &key_value[1]);
    }

    println!("All valid environment variables from the dotenv file have been loaded");
    Ok(())
}
