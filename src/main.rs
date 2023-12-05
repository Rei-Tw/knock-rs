use std::fmt;
use std::net::{TcpStream, UdpSocket};
use std::{thread, time};

use clap::Parser;
use regex::Regex;

#[derive(Debug, Clone)]
enum Proto {
    TCP,
    UDP,
}

impl From<&str> for Proto {
    fn from(value: &str) -> Self {
        match value {
            "tcp" => Self::TCP,
            "udp" => Self::UDP,
            &_ => Self::TCP, // else we can consider TCP
        }
    }
}

#[derive(Debug, Clone)]
struct Port {
    number: u16,
    proto: Proto,
}

impl fmt::Display for Port {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}/{}",
            self.number,
            match self.proto {
                Proto::TCP => "tcp",
                Proto::UDP => "udp",
            }
        )
    }
}

impl From<&str> for Port {
    fn from(value: &str) -> Self {
        let re = Regex::new(r"^(\d+)[/]?(tcp|udp)?$").unwrap();

        let captures = re.captures(value).expect(
            format!(
            "{} is invalid. Format: <port>/<tcp|udp>. Protocol is optional and will default to tcp.",
            &value
        )
            .as_str(),
        );
        Port {
            number: captures
                .get(1)
                .map_or("0", |m| m.as_str())
                .parse::<u16>()
                .expect("Invalid port number. Must be within 1-65535."),
            proto: Proto::from(captures.get(2).map_or("tcp", |m| m.as_str())),
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, required = true)]
    remote: String,

    #[arg(short, long, value_delimiter = ' ', num_args = 1.., required = true, value_parser = clap::value_parser!(Port))]
    ports: Vec<Port>,

    #[arg(short, long, default_value_t = 0)]
    delay: u64,
}

fn main() {
    let args: Args = Args::parse();

    println!("Attempting to knock on {}.", args.remote);

    for port in args.ports {
        let target: String = format!("{}:{}", &args.remote, &port.number);

        println!("Knock {}...", port);

        // Since knockd captures traffic using libpcap, it is silently listening.
        // That means knockd won't answer our request. That's the reason we won't manage
        // errors while attempting to connect to remote.
        match port.proto {
            Proto::TCP => {
                let _ = TcpStream::connect(&target);
            }
            Proto::UDP => {
                let socket = UdpSocket::bind("[::]:0").expect("Couldn't bind to address");
                socket
                    .send_to(&[0, 0], &target)
                    .expect("Couldn't send empty packet to target.");
            }
        };

        thread::sleep(time::Duration::from_millis(args.delay));
    }
}
