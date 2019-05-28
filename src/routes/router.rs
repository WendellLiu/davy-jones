use futures::future;
use hyper::{Body, Method, Request, Response, StatusCode};
use hyper::rt::{Future};

// Just a simple type alias
type BoxFut = Box<Future<Item = Response<Body>, Error = hyper::Error> + Send>;

pub fn echo(req: Request<Body>) -> BoxFut {
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
