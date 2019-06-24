use hw::named::Term as NamedTerm;
use hw::unnamed::Term as UnnamedTerm;

use ron::de;
use std::io::stdin;
use structopt::StructOpt;

#[derive(StructOpt)]
enum Command {
    /// Perform substitution on a named term
    #[structopt(name = "subs-named")]
    SubstituteNamed,

    /// Perfrom substitution on an unnamed term
    #[structopt(name = "subs-unnamed")]
    SubstituteUnnamed,

    /// Convert a named term to unnamed
    #[structopt(name = "conv-named")]
    ConvertNamed,

    /// Convert an unnamed term to named
    #[structopt(name = "conv-unnamed")]
    ConvertUnnamed,
}

fn main() {
    let cmd = Command::from_args();

    match cmd {
        Command::SubstituteNamed => {
            let term = {
                let mut line = String::new();
                stdin().read_line(&mut line).unwrap();
                de::from_str::<NamedTerm>(&line).expect("parse error")
            };

            let var = {
                let mut line = String::new();
                stdin().read_line(&mut line).unwrap();
                line.trim().to_string()
            };

            let subs = {
                let mut line = String::new();
                stdin().read_line(&mut line).unwrap();
                de::from_str::<NamedTerm>(&line).expect("parse error")
            };

            println!("{}", term.substitute(&var, &subs));
        },
        Command::SubstituteUnnamed => {
            let term = {
                let mut line = String::new();
                stdin().read_line(&mut line).unwrap();
                de::from_str::<UnnamedTerm>(&line).expect("parse error")
            };

            let var = {
                let mut line = String::new();
                stdin().read_line(&mut line).unwrap();
                line.trim().parse::<usize>().expect("parse error")
            };

            let subs = {
                let mut line = String::new();
                stdin().read_line(&mut line).unwrap();
                de::from_str::<UnnamedTerm>(&line).expect("parse error")
            };

            println!("{}", term.substitute(var, &subs));
        },
        Command::ConvertNamed => {
            let mut line = String::new();
            stdin().read_line(&mut line).unwrap();

            match de::from_str::<NamedTerm>(&line) {
                Ok(term) => println!("{}", UnnamedTerm::from_named(&term).0),
                Err(err) => println!("Parse error: {}", err),
            }
        },
        Command::ConvertUnnamed => {
            let mut line = String::new();
            stdin().read_line(&mut line).unwrap();

            match de::from_str::<UnnamedTerm>(&line) {
                Ok(term) => println!("{}", NamedTerm::from_unnamed(&term)),
                Err(err) => println!("Parse error: {}", err),
            }
        },
    }
}
