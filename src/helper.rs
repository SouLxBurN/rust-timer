/// Outside Docs
pub fn convert_seconds(seconds: u32) -> (u32, u32, u32) {
    let hours = seconds / 3600;
    let mins = (seconds % 3600) / 60;
    let secs = seconds % 60;

    (hours, mins, secs)
}
