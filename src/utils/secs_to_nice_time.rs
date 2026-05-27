use std::time::Duration;

pub fn secs_to_nice_time(secs: Duration) -> String {
    let num_secs_as_num = secs.as_secs_f64();

    let num_hours: u64 = num_secs_as_num as u64 / (60 * 60);
    let hours_leftovers: u64 = (num_secs_as_num as u64) % (60 * 60);

    let num_minutes: u64 = hours_leftovers / 60;
    let num_secs = hours_leftovers % 60;

    return format!("{}:{}:{}", num_hours, num_minutes, num_secs);
}
