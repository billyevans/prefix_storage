extern crate tst;
extern crate hyper;

use std::io::Read;
use std::io::Write;
use std::sync::Mutex;
use self::tst::TSTMap;
use std::env;

use hyper::status::StatusCode::{BadRequest, MethodNotAllowed, NotFound};
use hyper::server::{Handler, Server, Request, Response};
use hyper::uri::RequestUri::AbsolutePath;


struct RequestHandler {
    tst: Mutex<TSTMap<String>>,
}

impl RequestHandler {
    fn process_query(&self, query: &str, key: &str, guard: &TSTMap<String>, mut res: Response) {
        match query {
            "prefix" => {
                let mut res = res.start().unwrap();
                for x in guard.prefix_iter(key) {
                    write!(res, "{:?}", x);
                }
                res.end();
            }
            "wildcard" => {
                let mut res = res.start().unwrap();
                for x in guard.wildcard_iter(key) {
                    write!(res, "{:?}", x);
                }
                res.end();
            }
            "longest_prefix" => {
                let mut res = res.start().unwrap();
                let pref = guard.longest_prefix(key);
                write!(res, "{:?}", pref);
                res.end();
            }
            _ => {
                *res.status_mut() = BadRequest;
            }
        }
    }
}

impl Handler for RequestHandler {
    fn handle(&self, mut req: Request, mut res: Response) {
        let mut body: String = "".to_string();
        req.read_to_string(&mut body);
        match req.uri {
            AbsolutePath(ref path) => {
                match req.method {
                    hyper::Get => {
                        let is_query = path.find("?");
                        let guard = self.tst.lock().unwrap();
                        match is_query {
                            None => {
                                let fnd = guard.get(&path[1..]);
                                match fnd {
                                    None => {
                                        *res.status_mut() = NotFound;
                                    }
                                    Some(val) => {
                                        let mut res = res.start().unwrap();
                                        write!(res, "{}", val);
                                        res.end();
                                    }
                                }
                            }
                            Some(n) => {
                                if path[n + 1..].starts_with("query=") {
                                    self.process_query(&path[n + 7..], &path[1..n], &*guard, res);
                                } else {
                                    *res.status_mut() = BadRequest;
                                }
                            }
                        }
                    }
                    hyper::Post => {
                        self.tst.lock().unwrap().insert(&path[1..], body);
                    }
                    hyper::Delete => {
                        self.tst.lock().unwrap().remove(&path[1..]);
                    }
                    _ => {
                        *res.status_mut() = MethodNotAllowed;
                    }
                }
            }
            _ => {}
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
    let _guard = server.handle(RequestHandler { tst: Mutex::new(TSTMap::new()) });
    println!("Listening on http://{}", &addr);
}
