use std::process;

use clap::{Arg, ArgAction, ArgMatches, Command};

#[derive(Debug, Default, Clone)]
struct ArgManager {
    shorts: Vec<char>,
}

impl ArgManager {
    fn add_short(&mut self, name: &'static str) -> char {
        let mut names = name.chars();
        while let Some(short) = names.next() {
            if self.shorts.contains(&short) {
                continue;
            }
            self.shorts.push(short);
            return short;
        }

        return name.chars().next().unwrap();
    }

    fn form_arg(&mut self, name: &'static str, req: bool, help: &'static str) -> Arg {
        Arg::new(name).required(req).help(help)
    }

    fn form_arg_flags(&mut self, name: &'static str, req: bool, help: &'static str) -> Arg {
        let short = self.add_short(name);
        self.form_arg(name, req, help).short(short).long(name)
    }

    fn form_bool_arg(&mut self, name: &'static str, req: bool, help: &'static str) -> Arg {
        self.form_arg_flags(name, req, help)
            .action(ArgAction::SetTrue)
    }

    #[allow(dead_code)]
    fn form_arg_vec(&mut self, name: &'static str, req: bool, help: &'static str) -> Arg {
        self.form_arg(name, req, help).num_args(0..)
    }
}

struct SubcommandManager;
impl SubcommandManager {
    fn cargo_command(mut am: ArgManager) -> Command {
        Command::new("cargo")
            .about("Cargo API wrapper")
            .arg(am.form_arg_vec("commands", true, "The commands to run with Cargo"))
    }
    fn template_command(mut am: ArgManager) -> Command {
        Command::new("template")
            .about("Template for a new project")
            .arg(am.form_arg_flags("name", true, "The name of the project"))
            .arg(am.form_arg_flags("path", true, "The path to the template"))
    }
}

fn command() -> Command {
    let am = ArgManager::default();
    Command::new("crgl")
        .subcommand(SubcommandManager::cargo_command(am.clone()))
        .subcommand(SubcommandManager::template_command(am.clone()))
}

trait Parser {
    fn parse_args(matches: &ArgMatches) -> Self;
}

#[derive(Debug)]
enum CrglCommand {
    Cargo(CargoCommand),
    Template(TemplateCommand),
}

impl Parser for CrglCommand {
    fn parse_args(matches: &ArgMatches) -> Self {
        match matches.subcommand() {
            Some(("cargo", sub_matches)) => {
                CrglCommand::Cargo(CargoCommand::parse_args(sub_matches))
            }
            Some(("template", sub_matches)) => {
                CrglCommand::Template(TemplateCommand::parse_args(sub_matches))
            }
            _ => todo!(),
        }
    }
}

impl CrglCommand {
    fn execute(&self) {
        match self {
            CrglCommand::Cargo(c) => {
                let mut cargo_command = process::Command::new("cargo");
                for command in &c.commands {
                    cargo_command.arg(command);
                }
                cargo_command.spawn().unwrap().wait().unwrap();
            }
            CrglCommand::Template(t) => println!("{:#?}", t),
        }
    }
}

#[derive(Debug)]
struct CargoCommand {
    commands: Vec<String>,
}

impl Parser for CargoCommand {
    fn parse_args(matches: &ArgMatches) -> Self {
        let commands = matches
            .get_many::<String>("commands")
            .unwrap()
            .cloned()
            .collect();
        Self { commands }
    }
}

#[derive(Debug)]
struct TemplateCommand {
    name: String,
    path: String,
}

impl Parser for TemplateCommand {
    fn parse_args(matches: &ArgMatches) -> Self {
        let name = matches.get_one::<String>("name").unwrap();
        let path = matches.get_one::<String>("path").unwrap();
        Self {
            name: name.clone(),
            path: path.clone(),
        }
    }
}

fn main() {
    let matches = command().get_matches();

    let command = CrglCommand::parse_args(&matches);
    command.execute();
}
