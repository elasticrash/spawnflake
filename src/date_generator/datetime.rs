use std::time::SystemTime;

use mysql::chrono::{DateTime, NaiveDateTime, Utc};

use crate::random_number;

fn get_unix_timestamp() -> u64 {
    let now = SystemTime::now();
    random_number!(u64)(
        0,
        now.duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    )
}

pub fn generate_datetime() -> String {
    let naive = NaiveDateTime::from_timestamp(get_unix_timestamp() as i64, 0);
    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
    let newdate = datetime.format("%Y-%m-%d %H:%M:%S");
    newdate.to_string()
}

pub fn generate_time() -> String {
    let naive = NaiveDateTime::from_timestamp(get_unix_timestamp() as i64, 0);
    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
    let newdate = datetime.format("%H:%M:%S");
    newdate.to_string()
}

pub fn generate_date() -> String {
    let naive = NaiveDateTime::from_timestamp(get_unix_timestamp() as i64, 0);
    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
    let newdate = datetime.format("%Y-%m-%d");
    newdate.to_string()
}
