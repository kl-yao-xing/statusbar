use std::{thread::sleep, time::Duration};

use time::{OffsetDateTime, format_description};

struct StatusBar {
    time_format: time::format_description::OwnedFormatItem,
    time: String,
}

impl StatusBar {
    fn init() -> Result<Self, Box<dyn std::error::Error>> {
        let time_format = format_description::parse_owned::<2>("[year]-[month]-[day] [Weekday] [hour]:[minute]:[second]")?;

        let mut statusbar = Self {
            time_format,
            time: String::new(),
        };
        
        statusbar.update_time();
        Ok(statusbar)
    }
    fn display(&self) -> String{
        format!(
        "{}",self.time
        )
    }
    fn update_time(&mut self){
        if let Ok(localtime) = OffsetDateTime::now_local(){
            if let Ok(localtime_formatted) = localtime.format(&self.time_format){
                self.time = localtime_formatted;
                return;
            }
        }
        let utctime = OffsetDateTime::now_utc();
        self.time = utctime.format(&self.time_format).unwrap_or_else(|_| "退化到UTC时间且格式化错误！".to_string());
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut statusbar = StatusBar::init()?;
    println!("{}", statusbar.display());
    loop {
        sleep(Duration::from_secs(1));
        statusbar.update_time();
        println!("{}", statusbar.display());
    }
}