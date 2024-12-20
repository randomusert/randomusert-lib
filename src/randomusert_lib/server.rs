use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::net::SocketAddr;

pub mod server;

        /// Starts an HTTP server on the specified address.
        pub async fn start_server(addr: SocketAddr, handler: fn(Request<Body>) -> Response<Body>) -> Result<(), hyper::Error> {
        // Create a service function
            let make_svc = make_service_fn(move |_| {
            let handler = handler.clone();
            async move {
                Ok::<_, hyper::Error>(service_fn(move |req: Request<Body>| {
                    let response = handler(req);
                    async { Ok::<_, hyper::Error>(response) }
                }))
            }
            
    });
    let server = Server::bind(&addr).serve(make_svc);
            println!("Server running on http://{}", addr);
        
            server.await
        }
