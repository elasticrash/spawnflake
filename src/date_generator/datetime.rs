use std::time::SystemTime;

use mysql::chrono::{DateTime, Datelike, NaiveDateTime, Utc};

use crate::{
    configuration::config_model::{GenericConfiguration, Patterns},
    random_number,
};

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

pub fn datetime_generator_exists(config: &GenericConfiguration, name: &str) -> bool {
    let name_generator = &config.types.datetime.iter().find(|x| x.name == name);

    name_generator.is_some()
}

pub fn generate_datetime_object(config: &GenericConfiguration, name: &str) -> String {
    let pattern = get_rules(config, name);

    generate_range_date_type(
        date_formats::DATETIME_FORMAT,
        &pattern.rules[0],
        &pattern.rules[1],
    )
}

pub fn generate_date_object(config: &GenericConfiguration, name: &str) -> String {
    let pattern = get_rules(config, name);

    generate_range_date_type(
        date_formats::DATE_FORMAT,
        &pattern.rules[0],
        &pattern.rules[1],
    )
}

pub fn generate_time_object(config: &GenericConfiguration, name: &str) -> String {
    let pattern = get_rules(config, name);

    generate_range_date_type(
        date_formats::TIME_FORMAT,
        &pattern.rules[0],
        &pattern.rules[1],
    )
}

pub fn get_rules(config: &GenericConfiguration, name: &str) -> Patterns<String> {
    config
        .clone()
        .types
        .datetime
        .into_iter()
        .find(|x| x.name == name)
        .unwrap()
}

pub fn generate_year() -> Option<String> {
    let current_date = mysql::chrono::Utc::now();
    Some(random_number!(i32)(1970, current_date.year()).to_string())
}

pub fn generate_random_date_type(date_format: &str) -> String {
    let naive = NaiveDateTime::from_timestamp(get_unix_timestamp() as i64, 0);
    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
    let newdate = datetime.format(date_format);
    newdate.to_string()
}

pub fn generate_range_date_type(date_format: &str, from: &str, to: &str) -> String {
    let from_unix = from.parse::<i64>();
    let to_unix = to.parse::<i64>();

    let random_timestamp = if let (Ok(f_unix), Ok(t_unix)) = (from_unix, to_unix) {
        random_number!(i64)(f_unix, t_unix)
    } else {
        get_unix_timestamp() as i64
    };

    let naive = NaiveDateTime::from_timestamp(random_timestamp, 0);
    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
    let newdate = datetime.format(date_format);
    newdate.to_string()
}

#[cfg(test)]
mod tests {
    use crate::date_generator::datetime::generate_year;

    #[test]
    fn generate_year_succesfully() {
        // bits are not supported yet
        assert!(generate_year().unwrap().parse::<i32>().unwrap_or(1) > 1969);
    }
}
