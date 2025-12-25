use time::{OffsetDateTime, format_description::{self, OwnedFormatItem}};

pub struct TimeFormatter {
    format : Option<OwnedFormatItem>,
}

impl TimeFormatter {
    pub fn new() -> Self {
        let format = format_description::parse_owned::<2>("[year]-[month]-[day]-[weekday] [hour]:[minute]")
            .map(|desc| desc.to_owned())
            .ok();
        Self {format}
    }
    
    pub fn format_time(&self, time: &OffsetDateTime) -> String {
        if let Some(ref format_desc) = self.format {
            match time.format(format_desc) {
                Ok(formatted_time) => formatted_time,
                Err(e) => format!("TimeFomatterErr:{}", e),
            }
        } else {
            "TimeFormatterErr:0".to_string()
        }
    }
}