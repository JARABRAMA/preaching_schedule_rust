use crate::preaching_day::PreachingDay;
use std::path::PathBuf;

fn get_path() -> PathBuf {
    let mut path: PathBuf = if cfg!(target_os = "windows") {
        dirs::data_local_dir().expect("Unable to get path")
    } else {
        dirs::home_dir().expect("Unable to get path")
    };
    path.push(".preaching_schedule");
    path.push("schedule.txt");
    path
}

fn parse_lines_to_preaching_days(lines: String) -> Result<Vec<PreachingDay>, String> {
    lines
        .lines()
        .map(|it| PreachingDay::from_string(it.trim()))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn getting_path_linux() {
        let path = get_path();
        let expected = "/home/brayan/.preaching_schedule/schedule.txt";
        assert_eq!(path.to_str().unwrap(), expected)
    }
}
