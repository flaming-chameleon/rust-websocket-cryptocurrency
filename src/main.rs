// Importing
use tokio_tungstenite::{ connect_async, tungstenite::protocol::Message };
use serde_json::Value;
use chrono::Local;
use log::{ info, error, LevelFilter };
use futures_util::StreamExt;

#[tokio::main]
async fn main() {
    // Set up logging to stdout
    env_logger::builder().filter_level(LevelFilter::Info).init();

    // WebSocket URL for BTCUSDT kline data
    let url = "wss://stream.binance.com:9443/ws/btcusdt@kline_1m";
    info!("Connecting to {}", url);

    // Connect to the WebSocket server
    match connect_async(url).await {
        Ok((mut ws_stream, _)) => {
            info!("Connected to the WebSocket server");

            // Start receiving messages from the WebSocket stream
            while let Some(message) = ws_stream.next().await {
                match message {
                    Ok(Message::Text(text)) => handle_message(&text),
                    Ok(_) => (), // Ignore non-Text messages
                    Err(e) => error!("Error during the WebSocket communication: {}", e),
                }
            }
        }
        Err(e) => error!("Failed to connect: {}", e),
    }
}

// Function to handle incoming WebSocket messages
fn handle_message(text: &str) {
    match serde_json::from_str::<Value>(&text) {
        Ok(data) => {
            match (data["s"].as_str(), data["k"]["c"].as_str()) {
                // Check if both pair_symbol and close_price are Some, then log the data
                (Some(pair_symbol), Some(close_price)) => {
                    info!("{} - {}: ${}", Local::now().format("%H:%M:%S"), pair_symbol, close_price);
                }
                _ => error!("One or both required fields are missing in the data"),
            }
        }
        Err(_) => error!("Failed to parse the message: {}", text),
    }
}
