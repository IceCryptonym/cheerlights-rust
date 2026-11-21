use std::str::FromStr;

use rumqttc::{Event, EventLoop, MqttOptions, Packet, QoS};
use thiserror::Error;

use crate::types::Color;

const ID: &str = "cheerlights-api";
const MQTT_URL: &str = "mqtt.cheerlights.com";
const MQTT_PORT: u16 = 1883;
const MQTT_COLOR_TOPIC: &str = "color";

pub struct Client {
    mqtt: rumqttc::AsyncClient,
    event_loop: EventLoop,
}

#[derive(Debug, Error)]
pub enum PollError {
    #[error("Invalid UTF-8")]
    InvalidUtf8,
    #[error("Unknown color")]
    UnknownColor,
    #[error("Connection failed")]
    ConnectionFail,
}

#[derive(Debug, Error)]
#[error("Failed to un/subscribe to topic")]
pub struct SubscribeError;

impl Client {
    pub fn open() -> Self {
        let options = MqttOptions::new(ID, MQTT_URL, MQTT_PORT);
        let (mqtt, event_loop) = rumqttc::AsyncClient::new(options, 10);

        Self { mqtt, event_loop }
    }

    pub async fn subscribe(&self) -> Result<(), SubscribeError> {
        self.mqtt
            .subscribe(MQTT_COLOR_TOPIC, QoS::AtMostOnce)
            .await
            .map_err(|_| SubscribeError)
    }

    pub async fn unsubscribe(&self) -> Result<(), SubscribeError> {
        self.mqtt
            .unsubscribe(MQTT_COLOR_TOPIC)
            .await
            .map_err(|_| SubscribeError)
    }

    pub async fn poll(&mut self) -> Result<Color, PollError> {
        loop {
            let event = self
                .event_loop
                .poll()
                .await
                .map_err(|_| PollError::ConnectionFail)?;

            if let Event::Incoming(Packet::Publish(publish)) = event {
                let value = str::from_utf8(&publish.payload).map_err(|_| PollError::InvalidUtf8)?;
                let color = Color::from_str(value).map_err(|_| PollError::UnknownColor)?;

                return Ok(color);
            };
        }
    }
}
