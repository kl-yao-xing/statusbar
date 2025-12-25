use std::time::Duration;

use tokio::time::sleep;

use crate::{eventbus::{Event, EventBus}, time::{TimeFormatter, get_current_time}};

pub struct TimeTask {
    event_bus: EventBus,
    formatter: TimeFormatter,
}

impl TimeTask {
    pub fn new(event_bus: EventBus) -> Self {
        let formatter = TimeFormatter::new();
        Self { event_bus, formatter }
    }

    pub async fn start(&mut self) {
        let current_time = get_current_time();
        let formatted_time = self.formatter.format_time(&current_time);
        let _ = self.event_bus.send_event(Event::TimeUpdated(formatted_time));

        loop {
            // 计算到下一分钟的时间间隔（类似TimeSource的逻辑）
            let current_time = get_current_time();
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
                if nanos > 0 && nanos <= u64::MAX as i128 {
                    nanos as u64
                } else {
                    60_000_000_000  // 60秒
                }
            );

            sleep(sleep_duration).await;

            let current_time = get_current_time();
            let formatted_time = self.formatter.format_time(&current_time);
            let _ = self.event_bus.send_event(Event::TimeUpdated(formatted_time));
        }
    }
}