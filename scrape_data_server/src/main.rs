use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use actix_web::middleware::Logger;
use actix_web_actors::ws;
use tungstenite::protocol::Message;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    Logger::env_logger::init(); // Initialize the logger

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/websocket/").route(web::get().to(websocket_handler)))
    })
    .bind("127.0.0.1:8077")?
    .run()
    .await
}

// Index handler
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, Actix!")
}

// WebSocket handler
async fn websocket_handler(req: actix_web::HttpRequest, stream: web::Payload) -> Result<HttpResponse, actix_web::Error> {
    let ws = ws::start(WebsocketSession {}, &req, stream);
    ws.map(|_| HttpResponse::Ok())
}

// WebSocket session handler
struct WebsocketSession;

impl ws::Actor for WebsocketSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // WebSocket session started
        println!("WebSocket session started: {:?}", ctx.address());
    }
}

impl ws::Handler for WebsocketSession {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        match msg {
            ws::Message::Ping(ping) => ctx.pong(&ping),
            ws::Message::Pong(_) => (),
            ws::Message::Text(text) => {
                // Handle text messages (modify this part based on your needs)
                println!("Received text message: {}", text);

                // Echo the message back to the client
                ctx.text(text);
            }
            ws::Message::Binary(bin) => {
                // Handle binary messages (modify this part based on your needs)
                println!("Received binary message: {:?}", bin);

                // Echo the message back to the client
                ctx.binary(bin);
            }
            ws::Message::Close(_) => {
                // Handle WebSocket connection closure
                ctx.stop();
            }
            _ => (),
        }
    }
}
