use std::time::{SystemTime};

use mysql::chrono::{DateTime, NaiveDateTime, Utc};

use crate::random_number;

pub fn generate_datetime() -> String {
    let now = SystemTime::now();
    let unix_timestamp = random_number!(u64)(
        0,
        now.duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    );
    let naive = NaiveDateTime::from_timestamp(unix_timestamp as i64, 0);
    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
    let newdate = datetime.format("%Y-%m-%d %H:%M:%S");
    newdate.to_string()
}
