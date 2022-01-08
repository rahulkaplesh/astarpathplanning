//use std::rc::Rc;
use actix_web::{ HttpServer, App, middleware, web, HttpResponse };

mod graph;
mod astar;

/*fn main_not_working() {
    println!("Hello, world!");
    let mut gs = graph::Graph::new();
    gs.add_vertex("A", 0.0, 0.0);
    gs.add_vertex("B", 1.0, 0.0);
    gs.add_edges(&Rc::clone(gs.get_vertex("A").unwrap()), &Rc::clone(gs.get_vertex("B").unwrap()));
    gs.add_vertex("C", 0.0, 1.0);
    gs.add_vertex("D", 1.0, 1.0);
    gs.add_edges(&Rc::clone(gs.get_vertex("B").unwrap()), &Rc::clone(gs.get_vertex("C").unwrap()));
    gs.add_edges(&Rc::clone(gs.get_vertex("A").unwrap()), &Rc::clone(gs.get_vertex("D").unwrap()));
    gs.add_vertex("E", 1.0, 2.0);
    gs.add_edges(&Rc::clone(gs.get_vertex("D").unwrap()), &Rc::clone(gs.get_vertex("E").unwrap()));
    astar::shortest_path(&gs, "A", "E");
}*/

/// This handler uses json extractor
async fn get_shortest_distance() -> HttpResponse {
    println!("I got Request!!");
    HttpResponse::Ok().json({}) // <- send response
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(4096)) // <- limit size of the payload (global configuration)
            .service(web::resource("/get-shortest-path").route(web::post().to(get_shortest_distance)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}