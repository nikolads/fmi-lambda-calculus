use hw::named::Term as NamedTerm;
use hw::unnamed::Term as UnnamedTerm;

use ron::de;
use std::io::stdin;
use structopt::StructOpt;

/// Интерактивна демонстрация на функционалността.
///
/// Програмата приема команда на командния ред.
/// Командата приема допълнителни аргументи от стандартния вход - виж примерите
/// в документацията.
///
/// Ламбда термовете се въвеждат в RON формат, но се извеждат в математическа
/// нотация. (RON формата изглежда най-близко до вътрешното представяне на
/// термовете, а десериализацията от вече дефиниран формат за данни ми беше
/// значително по-лесно от имплементиране на въвеждане наръка.)
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
    /// ```
    /// Lambda("x", Var("y"))
    /// "y"
    /// Lambda("z", Var("z"))
    /// ```
    ///
    /// Изход
    /// ```
    /// λ x. λ z. z
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
    /// ```
    /// Lambda(Var(1))
    /// 0
    /// Lambda(Var(0))
    /// ```
    ///
    /// Изход
    /// ```
    /// λ λ 0
    /// ```
    #[structopt(name = "subs-unnamed")]
    SubstituteUnnamed,

    /// Превръща именуван терм в безименен
    ///
    /// # Пример
    ///
    /// Вход
    /// ```
    /// Lambda("x", Lambda("y", Apply(Var("y"), Var("x"))))
    /// ```
    ///
    /// Изход
    /// ```
    /// λ λ 0 1
    /// ```
    #[structopt(name = "conv-named")]
    ConvertNamed,

    /// Превръша безимемен терм в именуван
    ///
    /// Вход
    /// ```
    /// Lambda(Lambda(Apply(Var(0), Var(1))))
    /// ```
    ///
    /// Изход
    /// ```
    /// λ x. λ y. y x
    /// ```
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
