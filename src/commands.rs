use clap::Subcommand;

#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    /// Install packages
    Install {
        /// Interactive search + install mode
        #[arg(short, long)]
        interactive: bool,
        /// Packages to install
        packages: Vec<String>,
    },
    /// List installed packages
    List,
    /// Refresh metadata for valid package managers
    Refresh,
    /// Remove packages
    Remove {
        /// Remove unneeded packages
        #[arg(short, long)]
        unneeded: bool,
        /// Packages to remove
        packages: Vec<String>,
    },
    /// Search for package matching all specified strings
    Search {
        /// Package to search
        package: Vec<String>,
    },
    /// Updates packages
    Update {
        /// Packages to update
        packages: Vec<String>,
    },
}
