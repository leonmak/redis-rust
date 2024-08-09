use clap::{Arg, Command};

pub struct Args {
    pub ipaddr: String,
    pub port: u16,
}

pub fn parse_args() -> Args {
    let port_arg = Arg::new("port")
        .long("port")
        .short('p')
        .value_name("PORT")
        .value_parser(clap::value_parser!(u16))
        .help("The port number to use");

    let matches = Command::new("Redis TCP Server").arg(port_arg).get_matches();

    let port: u16 = *matches.get_one::<u16>("port").unwrap_or(&6379);
    Args {
        port,
        ipaddr: "127.0.0.1".to_string(),
    }
}
