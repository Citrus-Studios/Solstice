use tokio::net::UdpSocket;

#[tokio::main]
async fn main() {
    let socket = UdpSocket::bind("127.0.0.1:34254")
        .await
        .expect("Couldn't bind to that address");
    let mut buf = [0; 10];
    let (number_of_bytes, src_addr) = socket
        .recv_from(&mut buf)
        .await
        .expect("Didn't receive data");
    let filled_buf = &mut buf[..number_of_bytes];
    println!("{:?}", filled_buf);
}
