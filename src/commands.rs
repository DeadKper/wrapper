use clap::Subcommand;

#[derive(Subcommand, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub enum Command {
    /// Install packages
    Install {
        /// Packages to install
        packages: Vec<String>,
    },
    /// Search for packages
    Search {},
    /// Updates packages
    Update {},
    /// Remove packages
    Remove {},
    /// List installed packages
    List {},
}
