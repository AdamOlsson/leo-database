
pub mod req_headers {
    use std::str::from_utf8;
    use std::fmt;

    pub struct ReqHeaders<'a> {
        method: &'a str,
        uri: &'a str,
        protocol: &'a str,
        user_agent: &'a str,
        accept: &'a str,
        host: &'a str,
        content_type: &'a str,
        boundary: &'a str,
        expect: &'a str,
        content_length: usize,
    }

    impl<'a> fmt::Display for ReqHeaders<'a> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
                self.method,
                self.uri,
                self.protocol,
                self.user_agent,
                self.accept,
                self.host,
                self.content_type,
                self.boundary,
                self.expect,
                self.content_length)
        }
    }

    pub fn get_method<'a>(req_headers: &'a ReqHeaders) -> &'a str {
        return req_headers.method;
    }

    pub fn get_uri<'a>(req_headers: &'a ReqHeaders) -> &'a str {
        return req_headers.uri;
    }

    pub fn get_content_length<'a>(req_headers: &'a ReqHeaders) -> usize {
        return req_headers.content_length;
    }


    pub fn parse_req_headers<'a>(buffer: &mut [u8]) -> ReqHeaders{

        let req = match from_utf8(buffer) {
            Ok(value) => value,
            Err(_) => {
                println!("{:?}", buffer);
                panic!("Failed to parse.")
            },
        };


        let mut itr = req.split("\r\n");
        let status_line = itr.next().unwrap();
        
        // Status line 
        let mut status_line_itr = status_line.split(" ");
        let method: &str = status_line_itr.next().unwrap();
        let uri: &str = status_line_itr.next().unwrap();
        let protocol: &str = status_line_itr.next().unwrap();
        
        let mut host: &str = "";
        let mut user_agent: &str = "";
        let mut accept: &str = "";
        let mut content_length: usize = 0;
        let mut content_type: &str = "";
        let mut expect: &str = "";

        for v in itr {
            if v.eq("") {
                break;
            }
            match parse_header(v) {
                ("Host",value)           => host = value,
                ("User-Agent", value)     => user_agent = value,
                ("Accept", value)         => accept = value,
                ("Content-Length", value) => content_length = value.parse().unwrap(),
                ("Content-Type", value)   => content_type = value,
                ("Expect", value)         => expect = value,
                (head, _) => println!("{} could not be matched!", head),
            }
        }

        let vec: Vec<&str> = content_type.split("boundary=").collect();
        let boundary = vec.last().unwrap();

        let rqh :ReqHeaders = build_req_headers(method, uri, protocol, user_agent, accept, host, content_type, boundary, expect, content_length);

        return rqh;
    }

    fn build_req_headers<'a>(
        method: &'a str,
        uri: &'a str,
        protocol: &'a str,
        user_agent: &'a str,
        accept: &'a str,
        host: &'a str,
        content_type: &'a str,
        boundary: &'a str,
        expect: &'a str,
        content_length: usize) -> ReqHeaders<'a> {
            return ReqHeaders { method, uri, protocol, user_agent, accept, host, content_type, boundary, expect,  content_length };
    }

    fn parse_header(header: &str) -> (&str, &str) {
        let mut itr = header.split(": ");
        let head = itr.next().unwrap();
        let value = itr.next().unwrap();
        return (head, value)
    }
}