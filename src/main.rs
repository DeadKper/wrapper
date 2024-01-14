use anyhow::{anyhow, bail, Result};
use clap::{
    error::{ContextKind, ContextValue, ErrorKind},
    ArgMatches, CommandFactory, FromArgMatches, Parser, Subcommand,
};
use std::env::args;

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Known<T>
where
    T: FromArgMatches,
{
    matches: T,
    rest: Vec<String>,
}

impl<T> Known<T>
where
    T: FromArgMatches,
{
    pub fn new<I, S>(matches: ArgMatches, rest: I) -> Result<Self>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        Ok(Self {
            matches: T::from_arg_matches(&matches)?,
            rest: rest.into_iter().map(|a| a.as_ref().to_string()).collect(),
        })
    }
}

trait ParseKnown: FromArgMatches {
    fn parse_known() -> Result<Known<Self>>;
}

impl<T> ParseKnown for T
where
    T: CommandFactory + FromArgMatches,
{
    fn parse_known() -> Result<Known<T>> {
        let command = Self::command();
        let mut rest = Vec::new();
        let mut args = args().collect::<Vec<_>>();

        loop {
            match command.clone().try_get_matches_from(&args) {
                Ok(matches) => {
                    return Known::new(matches, rest);
                }
                Err(e) if matches!(e.kind(), ErrorKind::UnknownArgument) => {
                    if let Some(ContextValue::String(v)) = e
                        .context()
                        .find_map(|(k, v)| matches!(k, ContextKind::InvalidArg).then_some(v))
                    {
                        let unknown = args
                            .iter()
                            .find(|a| a.starts_with(v))
                            .cloned()
                            .ok_or_else(|| anyhow!("No argument starts with {}", v))?;

                        if args.contains(&unknown) {
                            args.retain(|a| a != &unknown);
                            rest.push(unknown);
                        } else {
                            bail!("Got unknown argument {} but it is not in args at all.", v);
                        }
                    } else {
                        bail!("No string value found for unknown argument: {}", e);
                    }
                }
                Err(e) => bail!("Error getting matches from args '{:?}': {}", args, e),
            }
        }
    }
}

#[derive(Subcommand, Debug, Clone)]
#[clap(
    allow_hyphen_values = true,
    trailing_var_arg = true,
    disable_help_flag = true,
    disable_version_flag = true,
    disable_help_subcommand = true,
)]
#[command(author, version, about, long_about = None)]
enum SubCmd {
    /// Install packages
    Install {
        /// Print help
        #[arg(short, long)]
        help: bool,
        /// Packages to install
        packages: Option<Vec<String>>
    },
    /// Search for packages
    Search {
        /// Print help
        #[arg(short, long)]
        help: bool,
    },
    /// Updates packages
    Update {
        /// Print help
        #[arg(short, long)]
        help: bool,
    },
    /// Remove packages
    Remove {
        /// Print help
        #[arg(short, long)]
        help: bool,
    },
    /// List installed packages
    List {
        /// Print help
        #[arg(short, long)]
        help: bool,
    },
}

#[derive(Parser, Debug, Clone)]
#[clap(
    allow_hyphen_values = true,
    trailing_var_arg = true,
    disable_help_flag = true,
    disable_version_flag = true,
    disable_help_subcommand = true
)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Install programs
    #[command(subcommand)]
    cmd: Option<SubCmd>,
    /// Print help
    #[arg(short, long)]
    help: bool,
    /// Print version
    #[arg(short = 'V', long)]
    version: bool,
}

fn main() -> Result<()> {
    let args = Args::parse_known()?.matches;
    let mut cmd = Args::command();
    cmd.build();
    if args.help {
        cmd.print_help()?;
        std::process::exit(0);
    }

    if args.version {
        print!("{}", cmd.render_version());
        std::process::exit(0);
    }

    match args.cmd {
        Some(SubCmd::Install { help, packages }) => {
            if help || packages.is_none() {
                cmd.find_subcommand_mut("install").unwrap().print_help()?;
                std::process::exit(0);
            }
        }
        Some(SubCmd::Search { help }) => {
            if help {
                cmd.find_subcommand_mut("search").unwrap().print_help()?;
                std::process::exit(0);
            }
        }
        Some(SubCmd::Update { help }) => {
            if help {
                cmd.find_subcommand_mut("update").unwrap().print_help()?;
                std::process::exit(0);
            }
        }
        Some(SubCmd::Remove { help }) => {
            if help {
                cmd.find_subcommand_mut("remove").unwrap().print_help()?;
                std::process::exit(0);
            }
        }
        Some(SubCmd::List { help }) => {
            if help {
                cmd.find_subcommand_mut("list").unwrap().print_help()?;
                std::process::exit(0);
            }
        }
        None => {
            // if args.rest.len() == 0 {
            //     cmd.print_help()?;
            //     std::process::exit(0);
            // }
        }
    }

    // println!("{:#?}", args);
    Ok(())
}
