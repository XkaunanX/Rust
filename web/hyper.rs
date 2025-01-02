use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use std::convert::Infallible;

async fn hello_world(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("¡Hola desde Hyper!")))
}

#[tokio::main]
async fn main() {
    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(hello_world)) });
    let addr = ([127, 0, 0, 1], 8080).into();
    
    let server = Server::bind(&addr)
        .serve(make_svc);

    println!("Servidor ejecutándose en http://127.0.0.1:8080");
    
    if let Err(e) = server.await {
        eprintln!("Error al ejecutar el servidor: {}", e);
    }
}
