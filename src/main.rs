use std::cell::RefCell;

use aago::trace::{List, Pathtrace, Trace};
use actix_web::{post, web, App, HttpServer};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Response {
    result: String,
}

#[post("/trace")]
async fn trace(data: web::Data<RefCell<List>>, trace: web::Json<Trace>) -> web::Json<Response> {
    let mut mutable_list = &mut data.borrow_mut().pathtraces;
    let tracing = trace.0;
    add_to_data(&mut mutable_list, Box::new(tracing));

    web::Json(Response {
        result: "success".to_string(),
    })
}

fn add_to_data(data: &mut Vec<Pathtrace>, trace_data: Box<Trace>) {
    let mut pathtrace = Pathtrace::new();
    pathtrace.push(*trace_data);

    data.push(pathtrace);
    println!("{data:?}");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(RefCell::new(List { pathtraces: vec![] })))
            .service(trace)
    })
    .bind(("127.0.0.1", 3030))?
    .run()
    .await
}
