use clap::{Arg, Command};

pub struct Args {
    pub ipaddr: String,
    pub port: u16,
    pub master_addr: String,
}

pub fn parse_args() -> Args {
    let port_arg = Arg::new("port")
        .long("port")
        .short('p')
        .value_name("PORT")
        .value_parser(clap::value_parser!(u16))
        .help("The port number to use");

    let replicaof_arg = Arg::new("replicaof")
        .long("replicaof")
        .value_name("HOST_PORT")
        .help("Set the replica of another Redis server");

    let matches = Command::new("Redis TCP Server")
        .arg(port_arg)
        .arg(replicaof_arg)
        .get_matches();

    let port: u16 = *matches.get_one::<u16>("port").unwrap_or(&6379);
    let mut arg_res = Args {
        port,
        ipaddr: "127.0.0.1".to_string(),
        master_addr: "".to_owned(),
    };

    if let Some(replicaof_values) = matches.get_one::<String>("replicaof") {
        let mut replicaof_values = replicaof_values.split(' ').into_iter();
        if let Some(host) = replicaof_values.next() {
            if let Some(port) = replicaof_values.next() {
                let port: u16 = port.parse().unwrap_or_else(|_| {
                    eprintln!("Invalid port value: {}", port);
                    std::process::exit(1);
                });
                println!("Replica of: {}:{}", host, port);
                arg_res.master_addr = format!("{}:{}", host, port).to_owned();
            } else {
                eprintln!("Missing port value for --replicaof");
                std::process::exit(1);
            }
        } else {
            eprintln!("Missing host value for --replicaof");
            std::process::exit(1);
        }
    }

    arg_res
}
