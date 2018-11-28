extern crate tokio;

use tokio::codec::Framed;
use tokio::net::TcpStream;
use tokio::prelude::*;

mod codec;
use self::codec::ProtoCodec;

fn main() {
    let addr = "127.0.0.1:8000".parse().unwrap();

    let future = TcpStream::connect(&addr)
        .and_then(|socket| {
            let socket_clone = socket.try_clone().unwrap();

            let send_f = Framed::new(socket_clone, ProtoCodec)
                .send(4)
                .and_then(|_| {
                    println!("SENT");
                    Ok(())
                })
                .map_err(|err| eprintln!("ERROR sending : {:?}", err));
            tokio::spawn(send_f);

            Framed::new(socket, ProtoCodec).for_each(|number| {
                println!("Received number : {}", number);
                Ok(())
            })
        })
        .map_err(|err| eprintln!("ERROR : {:?}", err));

    tokio::run(future);
}
