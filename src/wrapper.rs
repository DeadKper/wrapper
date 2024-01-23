use crate::Command;
use clap::Parser;
use std::env::args;

#[derive(Parser, Debug, Clone)]
#[clap(subcommand_required = true)]
#[command(author, version, about, long_about = None)]
pub struct Wrapper {
    #[command(subcommand)]
    pub command: Command,
    #[clap(skip)]
    pub managers: Vec<String>,
}

impl Wrapper {
    pub fn parse_managers() -> Self {
        // TODO: detect managers
        let detected_managers = vec!["nix-env", "flatpak", "dnf/dnf5", "dnf/dnf4"];
        let args = args().collect::<Vec<_>>();

        let mut remaining: Vec<String> = vec![];
        let mut managers: Vec<String> = vec![];
        let mut curr_managers: Vec<String> = vec![];
        let mut base_managers: Vec<String> = vec![];
        let mut ignore;

        for flag in args {
            ignore = false;
            if flag.starts_with("--") && flag.len() > 2 {
                let flag_value = &flag[2..flag.len()];
                for manager in &detected_managers {
                    let split_man = if manager.contains("/") {
                        manager.split("/").collect::<Vec<_>>()[0].to_string()
                    } else {
                        manager.to_string()
                    };
                    if &split_man != manager && base_managers.contains(&split_man) {
                        ignore = true;
                        continue;
                    }
                    if manager.starts_with(flag_value)
                        || manager.contains(&format!("/{flag_value}"))
                    {
                        if &split_man != manager {
                            base_managers.push(split_man);
                        }
                        ignore = true;
                        curr_managers.push(manager.to_string());
                    }
                }
            }

            if curr_managers.len() > 0 {
                managers.append(&mut curr_managers);
            } else if !ignore {
                remaining.push(flag);
            }
        }

        let mut wrp = Self::parse_from(remaining);
        wrp.managers = managers;
        wrp
    }
}
