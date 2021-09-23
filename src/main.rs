use clap::{Arg, App};
use std::net::ToSocketAddrs;

// fn main1() {
//     println!("Hello, world!");
//     let config = quiche::Config::new(quiche::PROTOCOL_VERSION)?;
//
//     let matches = App::new("rust alpha")
//         .arg(Arg::new("local").short('l').long("local").value_name("localmode").takes_value(true))
//         .arg(Arg::new("remote").short('r').long("remote").value_name("remotemode").takes_value(false))
//         .get_matches();
//
//     if let Some(local) = matches.value_of("localmode") {
//         println!("Value for input: {}", local);
//     }
//
//     if let Some(remote) = matches.value_of("remotemode") {
//         println!("Value for input: {}", remote);
//     }
//
// }

fn main() {
    println!("Hello, world start!");


    let matches = App::new("rust alpha")
    .arg(Arg::new("LOCAL").short('l').long("local").value_name("LOCAL").takes_value(true))
    .arg(Arg::new("REMOTE").short('r').long("remote").value_name("REMOTE").takes_value(true))
    .get_matches();

    let mut is_local:bool = false;
    if let Some(local) = matches.value_of("LOCAL") {
        println!("Value for local: {}", local);
        is_local = true;
    }

    let mut is_remote:bool = false;
    if let Some(remote) = matches.value_of("REMOTE") {
        println!("Value for remote: {}", remote);
        is_remote = true;
    }

    println!("local {} remote {}", is_local, is_remote);

    let mut config = quiche::Config::new(quiche::PROTOCOL_VERSION).unwrap();
    let scid = [0; quiche::MAX_CONN_ID_LEN];
    let scid = quiche::ConnectionId::from_ref(&scid);

    let url = url::Url::parse("https://example.net/").unwrap();
    let addrs = url.to_socket_addrs().unwrap().next().unwrap();

    let mut conn =
        quiche::connect(url.domain(), &scid, addrs, &mut config).unwrap();

    let mut out = [0; 1350];
    let (write, send_info) = conn.send(&mut out).expect("initial send failed");

    println!("Hello, world done!");
}
