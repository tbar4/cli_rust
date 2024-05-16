use clap::{Arg, ArgAction, Command};

fn main() {
    let matches = Command::new("echor")
        .version("0.1.0")
        .author("Trevor Barnes")
        .about("The Rust version of Echo")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .num_args(1..),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .action(ArgAction::SetTrue)
                .help("Do not print new line"),
        )
        .get_matches();

    let text: Vec<String> = matches
        .get_many("text")
        .expect("text is required")
        .cloned()
        .collect();
    
    let omit_newline = matches.get_flag("omit_newline");
    
    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });

}
