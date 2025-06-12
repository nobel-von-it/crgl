use clap::{Arg, ArgAction, Command};

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
}

fn command() -> Command {
    let am = ArgManager::default();
    Command::new("crgl").subcommand(SubcommandManager::cargo_command(am.clone()))
}

fn main() {
    let matches = command().get_matches();

    println!("{:#?}", matches);
}
