use std::collections::HashMap;

use crate::eventbus::{Event, EventConsumer};

pub struct Display {
    consumer: EventConsumer,
    component: HashMap<String, String>,
    component_order: Vec<String>,
}

impl Display {
    pub fn new(consumer: EventConsumer) -> Self {
        let component_order = vec![
            "time".to_string(),
        ];
        Self {
            consumer,
            component: HashMap::new(),
            component_order,
        }
    }

    pub async fn start(&mut self) {
        while let Some(event) = self.consumer.receiver.recv().await {
            match event {
                Event::TimeUpdated(time_str) => {
                    self.update_component("time".to_string(), time_str);
                    self.render();
                }
            }
        }
    }

    pub fn update_component(&mut self, key: String, value: String) {
        self.component.insert(key, value);
    }

    fn render(&self) {
        let mut parts: Vec<String> = Vec::new();

        for component_key in &self.component_order {
            if let Some(value) = self.component.get(component_key) {
                parts.push(value.clone());
            }
        }

        let output = parts.join (" | ");
        println!("{}", output);
    }
}