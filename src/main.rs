use std::{net::{TcpListener, TcpStream}, io::{Read}};

use state_machine::{state_machine::build_session, STATE_INIT, EVENT_START};
mod req_headers;
mod state_machine;



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

        let req_headers: req_headers::req_headers::ReqHeaders = req_headers::req_headers::parse_req_headers(&mut buffer);
        println!("{}", req_headers);

        match (req_headers::req_headers::get_method(&req_headers), req_headers::req_headers::get_uri(&req_headers)) {
            ("POST", "/video") => do_post_video(client, req_headers),
            _ => todo!()
        }
    }
}

fn do_post_video(client: TcpStream, rh: req_headers::req_headers::ReqHeaders){
    // Video is being uploaded to db

    let sm:state_machine::state_machine::StateMachine = state_machine::build_post_video_sm();
    let mut session = build_session(STATE_INIT, client, rh);

    state_machine::state_machine::start(&sm, &mut session, EVENT_START);
}