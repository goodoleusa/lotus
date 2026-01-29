use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct ServeOpts {
    /// Port to run the web server on
    #[structopt(
        short = "p",
        long = "port",
        default_value = "8080",
        help = "Port number for the web server",
        value_name = "PORT"
    )]
    pub port: u16,

    /// Host address to bind to
    #[structopt(
        short = "H",
        long = "host",
        default_value = "127.0.0.1",
        help = "Host address to bind to (use 0.0.0.0 for all interfaces)",
        value_name = "HOST"
    )]
    pub host: String,

    /// Open browser automatically
    #[structopt(
        short = "o",
        long = "open",
        help = "Open the web UI in the default browser"
    )]
    pub open_browser: bool,
}
