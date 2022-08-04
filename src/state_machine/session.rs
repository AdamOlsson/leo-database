
pub mod session {
    use std::net::TcpStream;

    use crate::{state_machine::sm::sm::State, ReqHeaders};

    pub struct Session<'a>{
        pub state: &'a State,
        pub client: TcpStream,
        pub req_headers: ReqHeaders<'a>
    }

    pub fn build_session<'a>(state: &'a State, client: TcpStream, req_headers: ReqHeaders<'a>) -> Session<'a> {
        return Session { state , client, req_headers };
    }
}