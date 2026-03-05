use std::io::stdout;

use ferris_says::say;

fn main() {
    let stdout = stdout();
    let message = "Hello from Fellow Rustaceans";
    let width = message.len();
    say(message, width, stdout).unwrap();
}
