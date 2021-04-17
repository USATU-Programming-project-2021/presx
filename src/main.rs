use iced::{Application, Settings};
use presx::analyzer;
use presx::app::{Flags, Pres};
use std::env;

fn main() -> iced::Result {
    let arg: Vec<String> = env::args().collect();
    match analyzer::presentation::PresYml::from_file(arg.get(1).unwrap().to_string()) {
        Ok(pres) => Pres::run(Settings::with_flags(Flags { pres })),
        Err(err) => {
            println!("{:?}", err.to_string());
            Ok(())
        }
    }
}
