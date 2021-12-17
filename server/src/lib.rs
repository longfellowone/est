use actix_cors::Cors;
use actix_web::dev::Server;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use anyhow::Result;
use std::net::TcpListener;

mod postgres;

pub fn run(listener: TcpListener) -> Result<Server> {
    // let data = web::Data::new();

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            // .app_data(data.clone())
            .configure(routes)
    })
    .listen(listener)?
    .run();

    Ok(server)
}

fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1")
            .route("", web::get().to(health_check))
        // .service(
        //     web::scope("/syn")
        //         .route("", web::get().to(syn::health_check))
        //         .route("/punchin", web::post().to(syn::punchin))
        //         .route("/punchout", web::post().to(syn::punchout))
        //         .service(
        //             web::scope("/employees")
        //                 .route("", web::get().to(employees::list))
        //                 .service(
        //                     web::resource("/{employee}")
        //                         .route(web::get().to(employees::get))
        //                         .route(web::post().to(employees::update)),
        //                 ),
        //         ),
        // ),
    );
}

pub async fn health_check(_req: HttpRequest) -> impl Responder {
    println!("health_check hit");

    HttpResponse::Ok()
}
