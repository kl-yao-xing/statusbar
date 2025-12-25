use time::OffsetDateTime;
pub fn get_current_time() -> OffsetDateTime {
    OffsetDateTime::now_local().unwrap_or(OffsetDateTime::now_utc())
}
