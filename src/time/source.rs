use std::time::Duration;

use time::OffsetDateTime;
use tokio::time::sleep;

use crate::eventbus::{Event, EventBus};

pub struct TimeSource {
    event_bus: EventBus,
    local_time: bool,
}

impl TimeSource {
    pub fn new(event_bus: EventBus) -> Self {
        Self { event_bus, local_time: false }
    }

    pub async fn start(&mut self) {
        let current_time = self.get_current_time();
        let _ = self.event_bus.send_event(Event::TimeTick(current_time));

        loop {
            let current_time = self.get_current_time();
            let next_time = match current_time.replace_second(0) {
                Ok(time) => match time.replace_nanosecond(0) {
                    Ok(time) => time + Duration::from_mins(1),
                    Err(_) => current_time + Duration::from_mins(1),
                },
                Err(_) => current_time + Duration::from_mins(1),
            };

            let duration_to_next_time = next_time - current_time;
            let nanos = duration_to_next_time.whole_nanoseconds();
            let sleep_duration = std::time::Duration::from_nanos(
                if nanos >0 && nanos <= u64::MAX as i128 {
                    nanos as u64
                }
                else {
                    60_000_000_000
                }
            );

            sleep(sleep_duration).await;

            let current_time = self.get_current_time();
            let _ = self.event_bus.send_event(Event::TimeTick(current_time));
        }
    }

    fn get_current_time(&self) -> OffsetDateTime {
        OffsetDateTime::now_local().unwrap_or(OffsetDateTime::now_utc())
    }
}