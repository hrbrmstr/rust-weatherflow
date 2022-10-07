use std::net::UdpSocket;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {

}

fn main() -> std::io::Result<()> {

  let _args = Args::parse();

  let mut buf = [0; 1024];
  let s = UdpSocket::bind("0.0.0.0:50222").expect(r#"{"message":"Could not bind to address/port."}"#);
  
  loop {
    
    let (n, _) = s.recv_from(&mut buf).expect(r#"{"message":"No broadcasts received."}"#);
    
    println!("{}", String::from_utf8(buf[..n].to_vec()).unwrap())
    
  }
  
}
