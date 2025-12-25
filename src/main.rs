use crate::{display::Display, time::TimeTask};

mod time;
mod display;
mod eventbus;

#[tokio::main]
async fn main() {
    let (event_bus, consumer) = eventbus::EventBus::new();

    let mut time_task = TimeTask::new(event_bus.clone());

    let mut display = Display::new(consumer);

    let time_task_handle = tokio::spawn(async move {
        time_task.start().await;
    });

    display.start().await;

    let _ = time_task_handle.await;
}