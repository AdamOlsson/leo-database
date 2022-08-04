use std::{net::{TcpListener, TcpStream}, io::{Read}};

use state_machine::{STATE_INIT, EVENT_START};

use crate::shared::req_headers::req_headers::{ReqHeaders, parse_req_headers, get_method, get_uri};
mod state_machine;
mod util;
mod shared;



fn main() {
    let address: &str = "0.0.0.0:50505";
    let listener: TcpListener = TcpListener::bind(address).unwrap();
    println!("Listening to {}", address);

    // let mut clients: Vec<TcpStream> = Vec::new();
    for stream in listener.incoming() {
        let mut client: TcpStream = stream.unwrap();

        // Read req from connection
        let mut buffer = [0; 1024];
        match client.read(&mut buffer){
            Ok(_) => (),
            Err(e) => panic!("Failed to read from stream: {:?}", e),
        };

        let req_headers: ReqHeaders = parse_req_headers(&mut buffer);
        println!("{}", req_headers);

        match (get_method(&req_headers), get_uri(&req_headers)) {
            ("POST", "/video") => do_post_video(client, req_headers),
            _ => todo!()
        }
    }
}

fn do_post_video(client: TcpStream, rh: ReqHeaders){
    // Video is being uploaded to db

    let sm = state_machine::build_post_video_sm();

    state_machine::start(&sm, client, rh);
}