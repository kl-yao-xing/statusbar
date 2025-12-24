use std::fmt::format;

use time::{error, format_description::{self, OwnedFormatItem}};

use crate::eventbus::{Event, EventBus, EventConsumer};

pub struct TimeFormatter {
    format : Option<OwnedFormatItem>,
    event_bus: EventBus,
    consumer: EventConsumer,
}

impl TimeFormatter {
    pub fn new(event_bus: EventBus, consumer: EventConsumer) -> Self {
        let format = format_description::parse_owned::<2>("[year]-[month]-[day]-[weekday] [hour]:[minute]")
            .map(|desc| desc.to_owned())
            .ok();
        Self { event_bus, consumer, format}
    }

    pub async fn start(&mut self) {
        if self.format.is_none() {
            let _ = self.event_bus.send_event(Event::FormattedTime("TimeFormatErr:0".to_string()));
            return ;
        }
        while let Some(event) = self.consumer.recv().await {
            match event {
                Event::TimeTick(time) => {
                    if let Some(ref format_desc) = self.format {
                        match time.format(format_desc) {
                            Ok(formatted_time) => {
                                let _ = self.event_bus.send_event(Event::FormattedTime(formatted_time));
                            },
                            Err(e) => {
                                let error_msg = format!("TimeFormatErr:{}",e);
                                let _ = self.event_bus.send_event(Event::FormattedTime(error_msg));
                            },
                        }
                    }
                },
                _ => continue,
            }
        }
    }
}