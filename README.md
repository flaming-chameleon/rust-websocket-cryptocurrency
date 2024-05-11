
[![Watch the video](https://img.youtube.com/vi/dfc-NNsf0Mc/maxresdefault.jpg)](https://youtu.be/dfc-NNsf0Mc)


# Rust WebSocket Crypto Ticker

This project uses Rust with Tokio-Tungstenite to connect to the Binance WebSocket API for streaming real-time cryptocurrency data. It specifically tracks Bitcoin (BTC) price updates in US Dollars (USD) every minute. The data includes the symbol and closing price for each minute, which is logged to the console.

## Features

- **Real-Time BTC Price Updates**: Connects to the Binance WebSocket and logs the closing price of BTCUSD at the end of each minute.
- **Error Handling**: Robust error handling for WebSocket connections and data processing.
- **Logging**: Uses the `log` and `env_logger` crates for logging important events and errors.

## Prerequisites

Before you can run this project, you need to have Rust installed. If you haven't installed Rust, you can download and install it from [the official Rust website](https://www.rust-lang.org/tools/install).

## Dependencies

This project depends on several external crates, which should be included in your `Cargo.toml`:

- `tokio-tungstenite` - For asynchronous WebSocket communication.
- `serde_json` - For parsing JSON data from the WebSocket.
- `chrono` - For handling date and time.
- `log` and `env_logger` - For logging purposes.
- `futures-util` - For stream processing utilities.

## Setup and Running

1. **Clone the Repository**:
   ```bash
   git clone https://github.com/flaming-chameleon/rust-websocket-cryptocurrency.git
   cd rust-websocket-cryptocurrency
   ```

2. **Build the Project**:
   ```bash
   cargo build
   ```

3. **Run the Server**:
   ```bash
   cargo run
   ```

The application will start and connect to the WebSocket. Price updates will be logged to the console every minute.

## Example Log Output

```
23:45:01 - BTCUSDT: $61000.11
23:45:02 - BTCUSDT: $61005.23
```

## Testing

You can test the application simply by running it. It will display logs in real-time, indicating successful connections and data reception.

## Contributions

Contributions are welcome. If you have improvements or encounter issues, feel free to open a pull request or submit an issue on GitHub.
