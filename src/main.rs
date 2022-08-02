use std::{net::{TcpListener, TcpStream}, io::Read};
mod ReqHeaders;
mod state_machine;



fn main() {
    let address: &str = "0.0.0.0:50505";
    let listener: TcpListener = TcpListener::bind(address).unwrap();
    println!("Listening to {}", address);

    let mut clients: Vec<TcpStream> = Vec::new();
    for stream in listener.incoming() {
        let mut client: TcpStream = stream.unwrap();

        // Read req from connection
        let mut buffer = [0; 1024];
        match client.read(&mut buffer){
            Ok(_) => (),
            Err(e) => panic!("Failed to read from stream: {:?}", e),
        };

        let req_headers: ReqHeaders::ReqHeaders = ReqHeaders::parse_req_headers(&mut buffer);
        println!("{}", req_headers);

        match (ReqHeaders::get_method(&req_headers), ReqHeaders::get_uri(&req_headers)) {
            ("POST", "/video") => todo!(),
            _ => todo!()
        }
    }
}

fn do_post_video(){
    // Video is being uploaded to db

    state_machine::build_post_video_sm();

}