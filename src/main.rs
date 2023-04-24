pub mod daemon;
pub mod service;
pub mod standard;
pub mod xunlei_asset;
use std::io::Write;

use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, arg_required_else_help = true)]
struct Opt {
    /// Enable debug mode
    #[clap(short, long, global = true)]
    debug: bool,

    #[clap(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Install xunlei
    Install(Config),
    /// Uninstall xunlei
    Uninstall,
    /// Execute xunlei
    Execute(Config),
}

#[derive(Args)]
pub struct Config {
    /// Xunlei internal mode
    #[clap(short, long)]
    internal: bool,
    /// Xunlei web-ui port
    #[clap(short, long, default_value = "5055", value_parser = parser_port_in_range)]
    port: u16,
    /// Xunlei config directory
    #[clap(short, long, default_value = standard::SYNOPKG_PKGBASE)]
    config_path: PathBuf,
    /// Xunlei download directory
    #[clap(short, long, default_value = standard::TMP_DOWNLOAD_PATH)]
    download_path: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::parse();
    init_log(opt.debug);
    match opt.commands {
        Commands::Install(config) => {
            service::XunleiInstall::from(config).execute()?;
        }
        Commands::Uninstall => {
            service::XunleiUninstall {}.execute()?;
        }
        Commands::Execute(config) => {
            daemon::XunleiDaemon::from(config).execute()?;
        }
    }

    Ok(())
}

fn init_log(debug: bool) {
    match debug {
        true => std::env::set_var("RUST_LOG", "DEBUG"),
        false => std::env::set_var("RUST_LOG", "INFO"),
    };
    env_logger::builder()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} {}: {}",
                record.level(),
                //Format like you want to: <-----------------
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.args()
            )
        })
        .init();
}

const PORT_RANGE: std::ops::RangeInclusive<usize> = 1024..=65535;

// port range parser
pub(crate) fn parser_port_in_range(s: &str) -> anyhow::Result<u16> {
    let port: usize = s
        .parse()
        .map_err(|_| anyhow::anyhow!(format!("`{}` isn't a port number", s)))?;
    if PORT_RANGE.contains(&port) {
        return Ok(port as u16);
    }
    anyhow::bail!(format!(
        "Port not in range {}-{}",
        PORT_RANGE.start(),
        PORT_RANGE.end()
    ))
}

pub trait Running {
    fn execute(&self) -> anyhow::Result<()>;
}