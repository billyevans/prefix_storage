extern crate tst;
extern crate hyper;

use std::io::Read;
use std::io::Write;
use std::sync::Mutex;
use self::tst::tst::TST;
use std::env;

use hyper::status::StatusCode::{Ok, BadRequest, MethodNotAllowed, NotFound};
use hyper::server::{Handler, Server, Request, Response};
use hyper::uri::RequestUri::AbsolutePath;


struct RequestHandler {
    tst: Mutex<TST<String>>
}

// TODO: add wildcard_iter, longest_prefix usage
impl Handler for RequestHandler {
    fn handle(&self, mut req: Request, mut res: Response) {
        let mut body: String = "".to_string();
        req.read_to_string(&mut body);
        match req.uri {
            AbsolutePath(ref path) => match req.method {
                hyper::Get => {
                    let guard = self.tst.lock().unwrap();
                    let mut res = res.start().unwrap();
                    for x in guard.prefix_iter(path) {
                        write!(res, "{:?}", x);
                    }
                    res.end();
                },
                hyper::Post => {
                    self.tst.lock().unwrap().insert(path, body);
                },
                hyper::Delete => {
                    self.tst.lock().unwrap().remove(path);
                },
                _ => {
                    *res.status_mut() = MethodNotAllowed;
                }
            },
            _ => {
            }
        }
    }
}

fn main() {
    if env::args().count() < 1 {
        panic!("usage: {} <port>", &env::args().nth(0).unwrap());
    }
    let addr = format!("127.0.0.1:{}", &env::args().nth(1).unwrap());
    let paddr: &str = &addr;
    let server = Server::http(paddr).unwrap();
    let _guard = server.handle(RequestHandler {
        tst: Mutex::new(TST::new())
    });
    println!("Listening on http://{}", &addr);
}
