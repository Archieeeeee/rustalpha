use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::net::{SocketAddrV4, Ipv4Addr};
use std::str::FromStr;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};

async fn copy_stream(source: &mut OwnedReadHalf, dst: &mut OwnedWriteHalf) {
    let mut buf = [0;1024];
    loop {
        let len = source.read(&mut buf).await;
        if let Err(e) = len {
            eprintln!("copy_stream read err {}", e);
            break;
        }
        let len = len.unwrap();
        if len == 0 {
            break;
        }
        match dst.write_all(&buf[0..len]).await {
            Ok(_) => {}
            Err(e) => {
                eprintln!("copy_stream write err {}", e);
                break;
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:27018").await?;
    println!("TcpListener started");

    loop {
        let (mut socket, _) = listener.accept().await?;
        println!("TcpListener accepted");

        tokio::spawn(async move {
            let mut buf = [0; 1024];
            let mut wbuf: [u8; 1024] = [0; 1024];

            let _n = socket.read(&mut buf).await.unwrap();

            let ver = &buf[0];
            let nmethods = &buf[1];
            let mut method_idx = 1;
            for _ in 0..(*nmethods as i32) {
                method_idx += 1;
                let method = &buf[method_idx];
                println!("client auth method {}", method);
            }
            println!("server now choose socks method");
            wbuf[0] = *ver;
            wbuf[1] = 0; // no need auth
            if let Err(e) = socket.write_all(&wbuf[0..2]).await {
                eprintln!("server write auth method err {:?}", e);
                return;
            }


            let _n = socket.read(&mut buf).await.unwrap();
            let ver = &buf[0];
            let _cmd = &buf[1];
            let _rsv = &buf[2];
            let addr_type = &buf[3];
            let mut sock_addr: SocketAddrV4 = SocketAddrV4::from_str("127.0.0.1:12345").unwrap();
            if *addr_type == 1 {
                let addr = &buf[4..8];
                // let _port1 = buf[8];
                // let _port2 = buf[9];
                let port: u16 = buf[8] as u16 * 256u16 + buf[9] as u16;
                println!("client connect {:?} {}", addr, port);
                //connect
                sock_addr = SocketAddrV4::new(Ipv4Addr::new(addr[0], addr[1], addr[2], addr[3]), port);

                //send connect result
                wbuf[0] = *ver;
                wbuf[1] = 0; // 0=success
                wbuf[2] = 0; // rsv
                wbuf[3..10].copy_from_slice(&buf[3..10]);
                if let Err(e) = socket.write_all(&wbuf[0..10]).await {
                    eprintln!("server write connect result err {:?}", e);
                    return;
                }
            }


            let stream = TcpStream::connect(sock_addr).await.unwrap();
            let (mut sr, mut sw) = stream.into_split();
            let (mut or, mut ow) = socket.into_split();
            tokio::spawn(async move {
                copy_stream(&mut sr, &mut ow).await;
            });
            copy_stream(&mut or, &mut sw).await;
            println!("tcp accept finished");
        });
    }
}

