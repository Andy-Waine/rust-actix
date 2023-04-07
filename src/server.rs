use actix_web::{App, HttpServer};

pub fn server() -> std::io::Result<()> {
    let mut server = HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(routes::game_routes)
    });

    server = if let Some(l) = env::var("LISTEN_ADDRESS").ok() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:8080")?
    };

    server.run()
}