use tokio::net::UdpSocket;
use quiche::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:27020";
    let socket = UdpSocket::bind(addr).await?;
    let mut buf = [0;1024];

    let mut quic_config = quiche::Config::new(quiche::PROTOCOL_VERSION).unwrap();
    // let quic_name = "quic-aio";
    let quic_server_addr = addr.parse().unwrap();
    let scid = quiche::ConnectionId::from_ref(&[0xba;16]);
    let mut conn = quiche::accept(&scid, None, quic_server_addr, &mut quic_config).unwrap();
    println!("QuicServer started");

    loop {
        let (len, addr) = socket.recv_from(&mut buf).await.unwrap();
        let recv_info = quiche::RecvInfo{from: addr};
        match conn.recv(&mut buf[0..len], recv_info) {
            Ok(n) => {println!("socket recv data {:?}", &mut buf[0..n])}
            Err(e) => {eprintln!("quic recv err {}", e); break;}
        }
    }

    Ok(())
}