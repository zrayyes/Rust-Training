extern crate futures;
extern crate hyper;

use futures::future;
use hyper::rt::Future;
use hyper::service::service_fn_ok;
use hyper::{Body, Method, Request, Response, Server, StatusCode};

type BoxFut = Box<Future<Item = Response<Body>, Error = hyper::Error> + Send>;

fn main() {
    let addr = ([127, 0, 0, 1], 3000).into();

    let new_svc = || service_fn_ok(hello_world);

    let server = Server::bind(&addr)
        .serve(new_svc)
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Server running on http://{}/", addr);
    hyper::rt::run(server);
}

fn hello_world(_req: Request<Body>) -> Response<Body> {
    Response::new(Body::from("Hello, World!"))
}

fn echo(req: Request<Body>) -> BoxFut {
    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            *response.body_mut() = Body::from("Try POSTing data to /echo");
        }
        (&Method::POST, "/echo") => {
            *response.body_mut() = req.into_body();
        }
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    };

    Box::new(future::ok(response))
}
