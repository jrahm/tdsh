mod auth_main;
mod util;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = clap::App::new("Tdsh - TD Ameritrade Shell")
        .version("0.1.0")
        .author("J. Allen")
        .about("Provides a command line interface to TD Ameritrade.")
        .arg(
            clap::Arg::with_name("authenticate")
                .short("a")
                .long("authenticate")
                .takes_value(false)
                .help("Run the authentication flow to set up this client for use with TD Ameritrade."),
        )
        .get_matches();

    if matches.is_present("authenticate") {
        auth_main::authentication_main();
        Ok(())
    } else {
        Ok(())
    }
}
