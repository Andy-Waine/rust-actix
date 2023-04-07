fn main() {
    let mut server = HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(blackjack_routes)
    });

    server = if let Some(l) = env::var("LISTEN_ADDRESS").ok() {
        server.listen(l).unwrap()
    } else {
        server.bind("127.0.0.1:8080").unwrap()
    };

    server.run().unwrap();
}