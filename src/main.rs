use color_eyre::eyre::WrapErr;
use dialoguer::{theme::ColorfulTheme, Password as DPassword};
use structopt::StructOpt;

mod password;
use password::*;
mod secure_eq;
pub use secure_eq::SecureEq;

fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;
    install_tracing();

    let password = Password::from(
        DPassword::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter the password you'd like to memorize")
            .interact()
            .wrap_err("Failed to get password")?,
    );

    loop {
        match password
            .test_interactive()
            .wrap_err("Failed to get password")?
        {
            Matches::Correct => {
                println!("Good job, that was correct!");
            }
            Matches::Incorrect => {
                println!("That's not the password you gave at the start -- try again?");
            }
        }
    }
}
/// A command-line application to assist in memorizing passwords by prompting for
/// you them over, and over, and over again...
#[derive(StructOpt)]
struct Opt {}

// Boilerplate: https://github.com/yaahc/color-eyre/blob/master/examples/usage.rs
fn install_tracing() {
    use tracing_error::ErrorLayer;
    use tracing_subscriber::prelude::*;
    use tracing_subscriber::{fmt, EnvFilter};

    let fmt_layer = fmt::layer().with_target(false);
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .with(ErrorLayer::default())
        .init();
}
