use std::time::SystemTime;

use mysql::chrono::{DateTime, NaiveDateTime, Utc};

use crate::random_number;

pub mod date_formats {
    pub const DATE_FORMAT: &str = "%Y-%m-%d";
    pub const TIME_FORMAT: &str = "%H:%M:%S";
    pub const DATETIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";
}

fn get_unix_timestamp() -> u64 {
    let now = SystemTime::now();
    random_number!(u64)(
        0,
        now.duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    )
}

pub fn generate_date_type(date_format: &str) -> String {
    let naive = NaiveDateTime::from_timestamp(get_unix_timestamp() as i64, 0);
    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
    let newdate = datetime.format(date_format);
    newdate.to_string()
}
