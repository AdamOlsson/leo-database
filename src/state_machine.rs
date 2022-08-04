use std::{net::TcpStream, io::{Write, Read}};

use crate::req_headers::req_headers::get_content_length;

use self::state_machine::{Session, Event, State, build_key, build_value, build_state_machine, StateMachine};

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
fn write(to: &mut TcpStream, buffer: &[u8]) -> Result<(), std::io::Error> {
    match to.write(buffer) {
        Ok(_) => {
            return Ok(());
        },
        Err(e) => {
            println!("Failed to write to stream.");
            return Err(e);
        },
    }
}

fn read_body(mut stream: &TcpStream, payload_size: usize) -> Result<Vec<u8>, std::io::Error> {
    /* Read the body from a stream. Returns a vector with the body. */
    let mut buffer = [0; 4096];
    let mut data: Vec<u8> = Vec::new();
    let mut bytes_read = 0;

    loop {
        match stream.read(&mut buffer) {
            Ok(i) => {
                data.extend_from_slice(&buffer);
                bytes_read += i;

                buffer.fill(0); // TODO remove this line
            }
            Err(e) => return Err(e),
        };

        if bytes_read >= payload_size {
            break;
        }
    }
    return Ok(data);
}

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

pub mod state_machine {
    use std::{collections::HashMap, net::TcpStream};

    use crate::req_headers::req_headers::ReqHeaders;

    pub type Event = str;
    pub type State = str;
    pub type StateMachine<'a> = HashMap<Key<'a>, Value>;
    pub type Function<'a> = fn(&mut Session) -> Result<&'static Event, std::io::Error>;

    #[derive(PartialEq, Eq, Hash)]
    pub struct Key<'a>{
        state: &'a State,
        event: &'a Event
    }

    pub struct Value {
        pub fun: Function<'static>,
        pub state: &'static State,
    }
    pub struct Session<'a>{
        pub state: &'a State,
        pub client: TcpStream,
        pub req_headers: ReqHeaders<'a>
    }
    

    pub fn build_key<'a>(state: &'a State, event: &'a Event) -> Key<'a> {
        return Key {state, event}
    }
    
    pub fn build_session<'a>(state: &'a State, client: TcpStream, req_headers: ReqHeaders<'a>) -> Session<'a> {
        return Session { state , client, req_headers };
    }

    pub fn build_value(fun: Function<'static>, state: &'static State) -> Value {
        return Value {fun, state};
    }
    pub fn build_state_machine<'a>() -> StateMachine<'a> {
        return HashMap::new();
    }

    pub fn start<'a>(sm: &'a StateMachine, session: &'a mut Session, event: &Event) {
        next(sm, session, event);
    }

    fn next<'a>(sm: &'a StateMachine, session: &'a mut Session, event: &'a Event) {
        let key: Key = build_key(session.state, event);
        let value: &Value = match sm.get(&key) {
            Some(v) => v,
            None => return, // Is this a bad way to exit recursion?
        };
        let new_event: &Event = (value.fun)(session).unwrap();
        session.state = value.state;

        next(sm, session, new_event);
    }
}

// #[cfg(test)]
// mod state_machine_test{

//     use super::{*, state_machine::start};

//     static STATE_INIT: &State = "init";
//     static STATE_1: &State = "state_1";
//     static STATE_2: &State = "state_2";
//     static STATE_3: &State = "state_3";
//     static GOTO_STATE_1: &Event = "goto_state_1";
//     static GOTO_STATE_2: &Event = "goto_state_2";
//     static GOTO_STATE_3: &Event = "goto_state_3";
//     static GOTO_STATE_DONE: &Event = "goto_state_done";
    
//     fn fun_in_state_1<'a>(session: &Session) -> &'static Event {
//         println!("fun_in_state_1");
//         return GOTO_STATE_2;
//     }

//     fn fun_in_state_2<'a>(session: &Session) -> &'static Event {
//         println!("fun_in_state_2");
//         return GOTO_STATE_3;
//     }

//     fn fun_in_state_3<'a>(session: &Session) -> &'static Event {
//         println!("fun_in_state_3");
//         return GOTO_STATE_DONE;
//     }

//     #[test]
//     fn transition_test() {
//         // TODO: Find something like a void* in C to providethe SM. Then wrap this type in the state_machine
//         // and unwrap it for the function calls in each state
    
//         let mut sm: StateMachine = build_state_machine();
//         sm.insert(build_key(STATE_INIT, GOTO_STATE_1), build_value(fun_in_state_1, STATE_1));
//         sm.insert(build_key(STATE_1, GOTO_STATE_2), build_value(fun_in_state_2, STATE_2));
//         sm.insert(build_key(STATE_2, GOTO_STATE_3), build_value(fun_in_state_3, STATE_3));

//         // let mut session = build_session(STATE_INIT, &client, &rh);

//         // start(&mut sm, &mut session, GOTO_STATE_1);
//     }
// }