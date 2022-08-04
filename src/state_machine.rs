
mod sm;
mod session;

use std::net::TcpStream;

use crate::{util::read_body, util::{write}, shared::req_headers::req_headers::{get_content_length, ReqHeaders}};

use sm::sm::{Event, State, build_key, build_value, build_state_machine, StateMachine};

use self::session::session::{Session, build_session};

pub static STATE_INIT: &State = "STATE_INIT";
pub static EVENT_START: &Event = "EVNET_START";

pub static STATE_SEND_CONTINUE: &State = "STATE_SEND_CONTINUE";
pub static EVENT_CONTINUE_SENT: &Event = "EVENT_CONTINUE_SENT";

pub static STATE_RECEIVE_PAYLOAD: &State = "STATE_RECEIVE_PAYLOAD";
pub static EVENT_PAYLOAD_RECEIVED: &Event = "EVENT_RECEIVE_PAYLOAD";

pub static STATE_SEND_OK: &State = "STATE_SEND_OK";
pub static EVENT_OK_SENT: &Event = "EVENT_OK_SENT";

pub static STATE_DONE: &State = "STATE_DONE";
pub static EVENT_EXIT: &Event = "EVENT_EXIT";

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
        Ok(_) => {
            println!("client --> db: video.");
            return Ok(EVENT_PAYLOAD_RECEIVED);
        },
        Err(e) => return Err(e),
    }
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
    sm.insert(build_key(STATE_RECEIVE_PAYLOAD, EVENT_PAYLOAD_RECEIVED), build_value(do_send_ok, STATE_SEND_OK));
    sm.insert(build_key(STATE_SEND_OK, EVENT_OK_SENT), build_value(do_done, STATE_DONE));

    return sm;
}

pub fn start<'a>(sm: &'a StateMachine, client: TcpStream, req_headers: ReqHeaders) {
    let mut session = build_session(STATE_INIT, client, req_headers);
    sm::sm::start(sm, &mut session, EVENT_START);
}