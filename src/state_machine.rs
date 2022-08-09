
mod sm;
mod session;

use std::{net::TcpStream, str::{from_utf8, Split}, fs, process::exit, path::{PathBuf, Path}};

use crate::{util::{read_body}, util::{write}, shared::req_headers::req_headers::{get_content_length, ReqHeaders}};

use sm::sm::{Event, State, build_key, build_value, build_state_machine, StateMachine};

use self::session::session::{Session, build_session};

static STATE_INIT: &State = "STATE_INIT";
static EVENT_START: &Event = "EVNET_START";

static STATE_SEND_CONTINUE: &State = "STATE_SEND_CONTINUE";
static EVENT_CONTINUE_SENT: &Event = "EVENT_CONTINUE_SENT";

static STATE_RECEIVE_PAYLOAD: &State = "STATE_RECEIVE_PAYLOAD";
static EVENT_PAYLOAD_RECEIVED: &Event = "EVENT_PAYLOAD_RECEIVED";

static STATE_PARSE_AND_STORE_PAYLOAD: &State = "STATE_PARSE_AND_STORE_PAYLOAD";
static EVENT_PAYLOAD_PARSED_AND_STORED: &Event = "EVENT_PAYLOAD_PARSED_AND_STORED";

static STATE_SEND_OK: &State = "STATE_SEND_OK";
static EVENT_OK_SENT: &Event = "EVENT_OK_SENT";

static STATE_DONE: &State = "STATE_DONE";
static EVENT_EXIT: &Event = "EVENT_EXIT";

static EVENT_ERROR: &Event = "EVENT_ERROR";


const HTTPCONTINUE: &[u8]           = b"HTTP/1.1 100 CONTINUE\r\nContent-Type: text/html; encoding=utf8\r\nContent-Length: 0\r\nConnection: keep-alive\r\n\r\n";
const HTTPOK: &[u8]                 = b"HTTP/1.1 200 OK\r\nContent-Type: text/html; encoding=utf8\r\nContent-Length: 0\r\nConnection: close\r\n\r\n";

fn do_send_continue<'a>(session: &mut Session) -> Result<&'static Event, std::io::Error> {
    let client = &mut session.client;
    match write(client, HTTPCONTINUE) {
        Ok(_) => {
            println!("db --> client: continue to send video.");
            return Ok(EVENT_CONTINUE_SENT);
        },
        Err(e) => return Err(e),
    }
}

fn do_receive_payload<'a>(session: &mut Session) -> Result<&'static Event, std::io::Error> {
    let client = &mut session.client;
    let payload_size = get_content_length(&session.req_headers);
    match read_body(client, payload_size) {
        Ok(pl) => {
            session.payload = Some(pl);
            println!("client --> db: video.");
            return Ok(EVENT_PAYLOAD_RECEIVED);
        },
        Err(e) => return Err(e),
    }
}

fn do_parse_and_store_payload<'a>(session: &mut Session) -> Result<&'static Event, std::io::Error>{
    let payload: &Vec<u8> = session.payload.as_ref().unwrap();

    // Find where payload splits head and data
    let needle = b"\r\n\r\n";
    let mut idx = 0;
    loop {
        let slice: &[u8] = &payload[idx..idx+needle.len()];
        if slice == needle {
            break;
        } else {
            idx += 1;
        }
    }
    let payload_data_and_tail: &[u8] = &payload[idx+4..]; // TODO remove trail
    let payload_head: &[u8] = &payload[..idx+4];

    // Find where payload splits data and tail
    let tail = b"\r\n???--\r\n"; // TODO Make this not hardcoded
    idx = payload_data_and_tail.len()-tail.len();
    loop {
        let slice: &[u8] = &payload_data_and_tail[idx..idx+tail.len()];
        if slice == tail {
            break;
        } else {
            idx -= 1;
        }
    }
    let payload_data = &payload_data_and_tail[..idx];

    // Get filename from payload_head
    let payload_head_str = match from_utf8(payload_head) {
        Ok(s) => s,
        Err(_) => {
            println!("Failed to parse payload head:\n{:?}", payload_head);
            exit(1);
        }
    };
    let filename = get_filename_from_payload_head(payload_head_str);
    
    // Write payload_data to disc with name specified in payload_head
    let path = Path::new("videos").join(filename);
    match fs::write(path, payload_data) {
        Ok(_) => println!("New database entry: {}", filename),
        Err(_) => todo!(),
    };

    return Ok(EVENT_PAYLOAD_PARSED_AND_STORED);
}

fn get_filename_from_payload_head(payload_head: &str) -> &str {
    let vec: Vec<&str> = payload_head.split("filename=").collect();
    let filename_and_tail = vec[1];
    let vec2: Vec<&str> = filename_and_tail.split("\n").collect();
    let filename_with_qoutes = vec2[0];
    let filename = &filename_with_qoutes[1..(filename_with_qoutes.len() - 2)]; // "filename" -> filename
    return filename;
}

fn do_send_ok<'a>(session: &mut Session) -> Result<&'static Event, std::io::Error> {
    let client = &mut session.client;
    match write(client, HTTPOK) {
        Ok(_) => {
            println!("db --> client: ok.");
            return Ok(EVENT_OK_SENT);
        },
        Err(e) => return Err(e),
    }
}

fn do_done<'a>(_session: &mut Session) -> Result<&'static Event, std::io::Error> {
    println!("done");
    return Ok(EVENT_EXIT);
}

pub fn build_post_video_sm() -> StateMachine<'static> {
    let mut sm: StateMachine = build_state_machine();
    sm.insert(build_key(STATE_INIT, EVENT_START), build_value(do_send_continue, STATE_SEND_CONTINUE));
    sm.insert(build_key(STATE_SEND_CONTINUE, EVENT_CONTINUE_SENT), build_value(do_receive_payload, STATE_RECEIVE_PAYLOAD));
    sm.insert(build_key(STATE_RECEIVE_PAYLOAD, EVENT_PAYLOAD_RECEIVED), build_value(do_parse_and_store_payload, STATE_PARSE_AND_STORE_PAYLOAD));
    sm.insert(build_key(STATE_PARSE_AND_STORE_PAYLOAD, EVENT_PAYLOAD_PARSED_AND_STORED), build_value(do_send_ok, STATE_SEND_OK));
    sm.insert(build_key(STATE_SEND_OK, EVENT_OK_SENT), build_value(do_done, STATE_DONE));

    return sm;
}

pub fn start<'a>(sm: &'a StateMachine, client: TcpStream, req_headers: ReqHeaders) {
    let mut session = build_session(STATE_INIT, client, req_headers);
    sm::sm::start(sm, &mut session, EVENT_START);
}