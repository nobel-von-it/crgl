use clap::{Arg, ArgAction, Command};

#[derive(Debug, Default)]
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

        unreachable!();
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
}

fn new_subcommand(am: &mut ArgManager) -> Command {
    Command::new("new")
        .about("Create a new project")
        .arg(am.form_arg("name", true, "The name of the project"))
        .arg(am.form_arg_flags("template", false, "The template to use"))
        .arg(am.form_arg_flags(
            "path",
            false,
            "The path to create the project in (defaults to current directory)",
        ))
        .arg(
            am.form_bool_arg(
                "git",
                false,
                "Initialize a git repository in the project (defaults to true)",
            )
            .default_value("true"),
        )
        .arg(
            am.form_bool_arg("bin", false, "Create a binary project (defaults to true)")
                .default_value("true"),
        )
        .arg(
            am.form_bool_arg("lib", false, "Create a library project (defaults to false)")
                .default_value("false"),
        )
}

fn command(am: &mut ArgManager) -> Command {
    Command::new("crgl")
        .subcommand_required(true)
        .about("Cargo Limp Like Templater")
        .subcommand(new_subcommand(am))
}

#[derive(Debug)]
struct Project {
    name: String,
    template_name: Option<String>,
    path: Option<String>,
    git: bool,
    bin: bool,
    lib: bool,
}

impl Project {
    fn parse_args() -> Self {
        let mut am = ArgManager::default();
        let matches = command(&mut am).get_matches();

        match matches.subcommand() {
            Some(("new", sub_matches)) => {
                let name = sub_matches.get_one::<String>("name").unwrap();
                let template_name = sub_matches.get_one::<String>("template");
                let path = sub_matches.get_one::<String>("path");
                let git = sub_matches.get_flag("git");
                let bin = sub_matches.get_flag("bin");
                let lib = sub_matches.get_flag("lib");

                Self {
                    name: name.clone(),
                    template_name: template_name.cloned(),
                    path: path.cloned(),
                    git,
                    bin,
                    lib,
                }
            }
            _ => todo!(),
        }
    }
}

fn main() {
    let project = Project::parse_args();
    println!("{:?}", project);
}
