use hw::named::Term as NamedTerm;
use hw::unnamed::Term as UnnamedTerm;

use ron::de;
use std::io::stdin;
use structopt::StructOpt;

#[derive(StructOpt)]
enum Command {
    /// Субституция върху именуван ламбда терм
    ///
    /// Приема три агумента от стандартния вход, всеки на нов ред - съответно
    /// ламбда терм, променлива и субституция за тази променлива
    ///
    /// # Пример
    ///
    /// Вход
    /// ```ignore
    /// Lambda("x", Var("y"))
    /// "y"
    /// Lambda("z", Var("z"))
    /// ```
    ///
    /// Изход
    /// ```ignore
    /// λ x λ z z
    /// ```
    #[structopt(name = "subs-named")]
    SubstituteNamed,

    /// Субституция върху безименен ламбда терм
    ///
    /// Приема три агумента от стандартния вход, всеки на нов ред - съответно
    /// ламбда терм, променлива и субституция за тази променлива
    ///
    /// # Пример
    ///
    /// Вход
    /// ```ignore
    /// Lambda(Var(1))
    /// 0
    /// Lambda(Var(0))
    /// ```
    ///
    /// Изход
    /// ```ignore
    /// λ λ 0
    /// ```
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
                de::from_str::<String>(&line).expect("parse error")
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
                de::from_str::<usize>(&line).expect("parse error")
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

            let term = de::from_str::<NamedTerm>(&line).expect("parse error");
            println!("{}", UnnamedTerm::from_named(&term).0);
        },
        Command::ConvertUnnamed => {
            let mut line = String::new();
            stdin().read_line(&mut line).unwrap();

            let term = de::from_str::<UnnamedTerm>(&line).expect("parse error");
            println!("{}", NamedTerm::from_unnamed(&term));
        },
    }
}
