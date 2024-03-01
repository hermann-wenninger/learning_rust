use warp::Filter;

#[tokio::main]
async fn main() {
    // Define a warp filter for the "/websocket" route
    let ws_route = warp::path("websocket")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            // Handle the WebSocket connection
            ws.on_upgrade(|websocket| handle_websocket(websocket))
        });

    // Combine the routes
    let routes = ws_route;

    // Start the warp server
    warp::serve(routes)
        .run(([127, 0, 0, 1], 8077))
        .await;
}

// Handle the WebSocket connection
async fn handle_websocket(ws: warp::ws::WebSocket) {
    // Convert the WebSocket into a warp::ws::Message stream
    let (tx, rx) = ws.split();

    // Process incoming messages
    rx.for_each(|msg| async {
        match msg {
            Ok(message) => {
                // Handle the received message (modify this part based on your needs)
                println!("Received message: {:?}", message);

                // Send a response back to the client
                if let Err(e) = tx.send(message).await {
                    eprintln!("Error sending message: {:?}", e);
                }
            }
            Err(e) => eprintln!("Error receiving message: {:?}", e),
        }
    })
    .await;

    // Handle the closure of the WebSocket connection
    println!("WebSocket connection closed");
}
