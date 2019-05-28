use futures::future;
use hyper::rt::{Future, Stream};
use hyper::{Body, Method, Request, Response, StatusCode};

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

    // Convert to uppercase before sending back to client.
    (&Method::POST, "/echo/uppercase") => {
      let mapping = req.into_body().map(|chunk| {
        chunk
          .iter()
          .map(|byte| byte.to_ascii_uppercase())
          .collect::<Vec<u8>>()
      });

      *response.body_mut() = Body::wrap_stream(mapping);
    }

    (&Method::POST, "/echo/reversed") => {
      let reversed = req.into_body().concat2().map(move |chunk| {
        let body = chunk.iter().rev().cloned().collect::<Vec<u8>>();
        *response.body_mut() = Body::from(body);
        response
      });

      return Box::new(reversed);
    }

    _ => {
      *response.status_mut() = StatusCode::NOT_FOUND;
    }
  };

  Box::new(future::ok(response))
}
