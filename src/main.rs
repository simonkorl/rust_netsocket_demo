use std::{net::{SocketAddr, TcpStream, TcpListener, UdpSocket}, io::{Write, Read}};

fn tcp_connection_test() -> std::io::Result<()> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 7736));
    let listener = TcpListener::bind(&addr)?;
    let mut client = TcpStream::connect(&addr)?;
    let mut server = match listener.accept() {
        Ok((socket, addr)) => {
            println!("new tcp client: {addr:?}");
            Some(socket)
        },
        Err(e) => { 
            println!("couldn't get tcp client: {e:?}");
            None
        },
    }.unwrap();

    assert_eq!(client.write(b"hello world from tcp").unwrap(), b"hello world from tcp".len());
    let mut buf = [0u8;64];
    if let Ok(read_size) = server.read(&mut buf) {
        let content = buf[..read_size].to_vec();
        println!("{:?}", String::from_utf8(content).unwrap());
        // println!("{:?}", )
    }
    return Ok(());
}

fn udp_socket_test() -> std::io::Result<()> {
    let saddr = SocketAddr::from(([127, 0, 0, 1], 7381));
    let caddr = SocketAddr::from(([127, 0, 0, 1], 7971));
    let server = UdpSocket::bind(saddr)?;
    let client = UdpSocket::bind(caddr)?;
    client.connect(saddr)?;
    
    client.send(b"hello world from udp")?;
    client.send_to(b"hello world from udp sent to", saddr)?;
    let mut buf = [0u8; 64];
    if let Ok(recv_len) = server.recv(&mut buf) {
        println!("udp server recv: {} bytes, {:?}", recv_len, String::from_utf8(buf[..recv_len].to_vec()).unwrap());
    }
    if let Ok((recv_len, addr)) = server.recv_from(&mut buf) {
        println!("udp server recv_from {} bytes from {:?}, {:?}", recv_len, addr, 
            String::from_utf8(buf[..recv_len].to_vec()).unwrap()
        );
    }
    return Ok(());
}

fn main() -> std::io::Result<()>{
    tcp_connection_test()?;
    udp_socket_test()?;
    return Ok(());
}
