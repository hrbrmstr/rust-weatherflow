use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
  
  let mut buf = [0; 1024];
  let s = UdpSocket::bind("0.0.0.0:50222").expect(r#"{"message":"Could not bind to address/port."}"#);
  
  loop {
    
    let (n, _) = s.recv_from(&mut buf).expect(r#"{"message":"No broadcasts received."}"#);
    
    println!("{}", String::from_utf8(buf[..n].to_vec()).unwrap())
    
  }
  
}
