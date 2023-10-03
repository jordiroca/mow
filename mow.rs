/*
 * DESCRIPTION: mow, command line program in Rust to remove leading and trailing
 *                   whitespace from input lines based on the script name.
 *
 * VERSION: 23.10.2
 *
 * OPTIONS:
 * -h, --help    Display this message.
 *
 * AUTHOR: Jordi Roca
 * CREATED: 2023/10/02
 *
 * GITHUB: https://github.com/jordiroca/mow
 *
 * LICENSE: See LICENSE file.
 *
 */

use std::env;
use std::io::{self, BufRead, Write};

/*
##     ##    ###    #### ##    ##
###   ###   ## ##    ##  ###   ##
#### ####  ##   ##   ##  ####  ##
## ### ## ##     ##  ##  ## ## ##
##     ## #########  ##  ##  ####
##     ## ##     ##  ##  ##   ###
##     ## ##     ## #### ##    ##
*/

fn main() {
    let script_name = env::args().nth(0).unwrap_or_else(|| "mow".to_string());
    let script_name = std::path::Path::new(&script_name)
        .file_stem()
        .and_then(std::ffi::OsStr::to_str)
        .unwrap_or("mow")
        .to_string();

    if env::args().any(|arg| arg == "-h" || arg == "--help") {
        print_help(&script_name);
        return;
    }

    let mow_action = match script_name.as_str() {
        "mow" | "trim" => MowAction::Both,
        "lmow" | "ltrim" => MowAction::Leading,
        "rmow" | "rtrim" => MowAction::Trailing,
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", script_name).unwrap();
            std::process::exit(1);
        }
    };

    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdout_lock = stdout.lock();

    for line_result in stdin.lock().lines() {
        match line_result {
            Ok(line) => {
                let mowed_line = match mow_action {
                    MowAction::Both => line.trim(),
                    MowAction::Leading => line.trim_start(),
                    MowAction::Trailing => line.trim_end(),
                };
                writeln!(stdout_lock, "{}", mowed_line).unwrap();
            }
            Err(err) => {
                writeln!(io::stderr(), "Error reading line: {}", err).unwrap();
                std::process::exit(1);
            }
        }
    }
}

enum MowAction {
    Both,
    Leading,
    Trailing,
}

/*
##     ## ######## ##       ########
##     ## ##       ##       ##     ##
##     ## ##       ##       ##     ##
######### ######   ##       ########
##     ## ##       ##       ##
##     ## ##       ##       ##
##     ## ######## ######## ##
*/

fn print_help(script_name: &str) {
    println!("Usage: {} [-h | --help]", script_name);
    println!("Removes leading and/or trailing whitespace from each line of input.");
    println!("Determine the action based on the script name:");
    println!(
        " - {} or trim: remove both leading and trailing whitespace",
        script_name
    );
    println!(" - lmow or ltrim: remove leading whitespace");
    println!(" - rmow or rtrim: remove trailing whitespace");
    println!("\nOptions:");
    println!(" -h, --help   Display this help message");
}
