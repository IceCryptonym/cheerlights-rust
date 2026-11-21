# Cheerlights Rust [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)


## Example

```rust
let cheerlights_http = cheerlights::http::Client::default();
let history = cheerlights_http.fetch_history().await.unwrap();

for update in history.updates {
    println!(
        "Color changed to {:?} at {:?}",
        update.color, update.created_at
    );
}

let mut cheerlights_mqtt = cheerlights::mqtt::Client::open();
cheerlights_mqtt.subscribe().await.unwrap();

while let Ok(color) = cheerlights_mqtt.poll().await {
    println!("Color changed to {:?} ({:?})", color, color.to_rgb());
}
```
