use std::{collections::HashMap, path::Component};

use crate::eventbus::{Event, EventConsumer};

pub struct Display {
    consumer: EventConsumer,
    components: HashMap<String, String>,
}

impl Display {
    pub fn new(consumer: EventConsumer) -> Self {
        let components = HashMap::new();
        Self { consumer, components }
    }

    pub async fn start(&mut self) {
        loop {
            if let Some(event) = self.consumer.recv().await {
                match event {
                    Event::FormattedTime(formatted_time) => {
                        self.update_component("time", &formatted_time);
                        self.display();
                    }
                    _ => continue,
                }
            }
        }
    }

    fn update_component (&mut self, name: &str, value: &str) {
        self.components.insert(name.to_string(), value.to_string());
    }

    fn display(&self) {
        let output = self.format_output();
        println!("{}", output);
    }

    fn format_output(&self) -> String {
        let component_order = ["time"];
        let mut parts = Vec::new();
        for componment_name in &component_order {
            if let Some(value) = self.components.get(*componment_name) {
                if !value.is_empty() {
                    parts.push(format!("{{{}}}", value));
                } else {
                    parts.push("{}".to_string());
                } 
            } else {
                parts.push("{}".to_string());
            }
        }
        parts.join(" | ")
    }
}