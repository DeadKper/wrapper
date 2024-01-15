use crate::Commands;
use clap::Parser;
use std::env::args;

#[derive(Parser, Debug, Clone)]
#[clap(subcommand_required = true)]
#[command(author, version, about, long_about = None)]
pub struct Wrapper {
    /// Install programs
    #[command(subcommand)]
    pub cmd: Commands,
    #[clap(skip)]
    pub managers: Vec<String>,
}

impl Wrapper {
    pub fn parse_managers() -> Self {
        // TODO: detect managers
        let detected_managers = vec!["dnf5", "flatpak", "nix-env"];
        let args = args().collect::<Vec<_>>();
        let mut remaining: Vec<String> = vec![];
        let mut managers: Vec<String> = vec![];
        let mut curr_managers: Vec<String> = vec![];
        let mut found;
        for flag in args {
            found = false;
            if flag.starts_with("--") {
                let flag_value = &flag[2..flag.len()];
                for manager in &detected_managers {
                    if manager.starts_with(flag_value) {
                        found = true;
                        curr_managers.push(manager.to_string());
                    }
                }

                if curr_managers.len() > 1 {
                    panic!("argument \"{flag}\" has more than 1 manager detected: {curr_managers:?}")
                } else if curr_managers.len() == 1 {
                    managers.append(&mut curr_managers);
                }
            }

            if !found {
                remaining.push(flag);
            }
        }

        let mut wrp = Self::parse_from(remaining);
        wrp.managers = managers;
        wrp
    }
}
