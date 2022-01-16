use std::rc::Rc;
use actix_web::{ http ,HttpServer, App, middleware, web, HttpResponse };
use json::JsonValue;
use actix_cors::Cors;

mod cost_calculation;
mod graph;
mod astar;

fn test_check() {
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
    //astar::shortest_path(&gs, "A", "E", cost_calculation::CostCalculator::SimpleCalculation{});
}

fn compute_shortest_path(json_struct: JsonValue) -> JsonValue {
    let mut gs = graph::Graph::new();
    for val in json_struct["points"].members() {
        gs.add_vertex(val["name"].as_str().unwrap(), val["lon"].as_f64().unwrap(), val["lat"].as_f64().unwrap());
    }
    for val_edges in json_struct["edges"].members() {
        gs.add_edges(&Rc::clone(gs.get_vertex(val_edges["source"]["name"].as_str().unwrap()).unwrap()), &Rc::clone(gs.get_vertex(val_edges["target"]["name"].as_str().unwrap()).unwrap()));
    }
    let cost_calculator = Box::new(cost_calculation::HaversineCalculator);
    let (route, cost) = astar::shortest_path(&gs, json_struct["startPoint"]["name"].as_str().unwrap(), json_struct["endPoint"]["name"].as_str().unwrap(), cost_calculator);
    let mut return_object = JsonValue::new_object();
    return_object["route"] = JsonValue::new_array();
    for val in route {
        return_object["route"].push(val.as_str());
    }
    return_object["cost"] = JsonValue::from(cost);
    return_object
}


/// This handler uses json extractor
async fn get_shortest_distance(body: web::Bytes) -> HttpResponse {
    let body_json: JsonValue = match json::parse(std::str::from_utf8(&body).unwrap()) {
        Ok(val) => val,
        Err(e) => json::object! {"err" => e.to_string() },
    };
    let ret_json = compute_shortest_path(body_json.clone());
    HttpResponse::Ok()
        .content_type("application/json")
        .body(ret_json.dump()) // <- send response
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(
                Cors::default() // allowed_origin return access-control-allow-origin: * by default
                    .send_wildcard()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(4096)) // <- limit size of the payload (global configuration)
            .service(web::resource("/get-shortest-path").route(web::post().to(get_shortest_distance)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}