use std::{env, process};

use clap::{Arg, ArgAction, ArgMatches, Command};

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

    #[allow(dead_code)]
    fn form_arg_vec(&mut self, name: &'static str, req: bool, help: &'static str) -> Arg {
        self.form_arg(name, req, help).num_args(0..)
    }
}

fn new_subcommand(am: &mut ArgManager) -> Command {
    Command::new("new")
        .about("Create a new project")
        .arg(am.form_arg("name", true, "The name of the project"))
        .arg(am.form_arg_flags("template", false, "The template to use"))
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
        .arg(am.form_arg_flags(
            "edition",
            false,
            "The edition of the project (defaults to 2024)",
        ))
        .arg(
            am.form_bool_arg("quiet", false, "Quiet mode (defaults to true)")
                .default_value("true"),
        )
}

fn add_subcommand(am: &mut ArgManager) -> Command {
    Command::new("add")
        .about("Add a dependency to the project")
        .arg(am.form_arg("name", true, "The name of the dependency"))
        .arg(am.form_arg_flags("version", false, "The version of the dependency"))
        .arg(am.form_arg_flags(
            "features",
            false,
            "Features to add to the dependency (comma separated)",
        ))
}

fn init_subcommand(am: &mut ArgManager) -> Command {
    Command::new("init")
        .about("Initialize a new project")
        .arg(am.form_arg("path", false, "The path to the project"))
        .arg(am.form_bool_arg("git", false, "Initialize a git repository in the project"))
        .arg(
            am.form_bool_arg("bin", false, "Create a binary project")
                .default_value("true"),
        )
        .arg(
            am.form_bool_arg("lib", false, "Create a library project")
                .default_value("false"),
        )
        .arg(am.form_arg_flags("edition", false, "The edition of the project"))
        .arg(
            am.form_bool_arg("quiet", false, "Quiet mode (defaults to true)")
                .default_value("true"),
        )
}

fn command(am: &mut ArgManager) -> Command {
    Command::new("crgl")
        .subcommand_required(true)
        .about("Cargo Limp Like Templater")
        .subcommand(new_subcommand(am))
        .subcommand(add_subcommand(am))
        .subcommand(init_subcommand(am))
}

trait Parser {
    fn parse_args(matches: &ArgMatches) -> Self;
}

enum CrglCommand {
    New(NewCommand),
    Add(AddCommand),
    Init(InitCommand),
}

impl Parser for CrglCommand {
    fn parse_args(matches: &ArgMatches) -> Self {
        match matches.subcommand() {
            Some(("new", sub_matches)) => CrglCommand::New(NewCommand::parse_args(sub_matches)),
            Some(("add", sub_matches)) => CrglCommand::Add(AddCommand::parse_args(sub_matches)),
            _ => todo!(),
        }
    }
}

impl CrglCommand {
    fn execute(&self) {
        match self {
            CrglCommand::New(new_command) => {
                // TODO: Cargo wrapper
                // Transfer args to cargo
                let mut cargo_command = process::Command::new("cargo");

                cargo_command.arg("new");

                if new_command.quiet {
                    cargo_command.arg("--quiet");
                }

                if !new_command.git {
                    cargo_command.arg("--vcs=none");
                }

                if new_command.bin {
                    cargo_command.arg("--bin");
                }

                if new_command.lib {
                    cargo_command.arg("--lib");
                }

                if let Some(edition) = &new_command.edition {
                    cargo_command.arg("--edition").arg(edition);
                }

                cargo_command.arg(&new_command.name);

                cargo_command.spawn().unwrap().wait().unwrap();

                // TODO:
            }
            CrglCommand::Add(add_command) => {
                let mut cargo_command = process::Command::new("cargo");

                cargo_command.arg("add");

                if let Some(version) = &add_command.version {
                    cargo_command.arg("--version").arg(version);
                }

                if let Some(features) = &add_command.features {
                    cargo_command.arg("--features").arg(features);
                }

                cargo_command.arg(&add_command.name);

                cargo_command.spawn().unwrap().wait().unwrap();
            }
            CrglCommand::Init(init_command) => {
                let mut cargo_command = process::Command::new("cargo");

                cargo_command.arg("init");

                if let Some(path) = &init_command.path {
                    cargo_command.arg("--path").arg(path);
                }

                if let Some(edition) = &init_command.edition {
                    cargo_command.arg("--edition").arg(edition);
                }

                if !init_command.git {
                    cargo_command.arg("--vcs=none");
                }

                if init_command.bin {
                    cargo_command.arg("--bin");
                }

                if init_command.lib {
                    cargo_command.arg("--lib");
                }

                if init_command.quiet {
                    cargo_command.arg("--quiet");
                }

                cargo_command.spawn().unwrap().wait().unwrap();
            }
        }
    }
}

struct NewCommand {
    name: String,
    edition: Option<String>,

    bin: bool,
    lib: bool,
    git: bool,
    quiet: bool,

    template_name: Option<String>,
}

impl Parser for NewCommand {
    fn parse_args(matches: &ArgMatches) -> Self {
        let name = matches.get_one::<String>("name").unwrap();
        let template_name = matches.get_one::<String>("template");
        let edition = matches.get_one::<String>("edition");
        let bin = matches.get_flag("bin");
        let lib = matches.get_flag("lib");
        let git = matches.get_flag("git");
        let quiet = matches.get_flag("quiet");

        Self {
            name: name.clone(),
            template_name: template_name.cloned(),
            edition: edition.cloned(),
            bin,
            lib,
            git,
            quiet,
        }
    }
}

struct AddCommand {
    name: String,
    version: Option<String>,
    features: Option<String>,
}

impl Parser for AddCommand {
    fn parse_args(matches: &ArgMatches) -> Self {
        let name = matches.get_one::<String>("name").unwrap();
        let version = matches.get_one::<String>("version");
        let features = matches.get_one::<String>("features");
        Self {
            name: name.clone(),
            version: version.cloned(),
            features: features.cloned(),
        }
    }
}

struct InitCommand {
    path: Option<String>,
    edition: Option<String>,

    git: bool,
    bin: bool,
    lib: bool,

    quiet: bool,
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
    let mut am = ArgManager::default();
    let matches = command(&mut am).get_matches();

    let crgl_command = CrglCommand::parse_args(&matches);
    crgl_command.execute();

    // minimal test
    // let args = env::args().skip(1).collect::<Vec<_>>();
    // let mut cargo_test = process::Command::new("cargo").args(args).spawn().unwrap();
    // cargo_test.wait().unwrap();
}
