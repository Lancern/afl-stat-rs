use std::path::Path;
use std::time::SystemTime;

#[derive(Clone, Debug)]
pub struct AFLStat {
    pub start_time: SystemTime,
    pub last_update: SystemTime,
    pub fuzzer_pid: u32,
    pub cycles_done: i32,
    pub execs_done: i64,
    pub execs_per_sec: i32,
    pub paths_total: i64,
    pub paths_favored: i64,
    pub paths_found: i64,
    pub paths_imported: i64,
    pub max_depth: i32,
    pub cur_path: i64,
    pub pending_favs: i64,
    pub pending_total: i64,
    pub variable_paths: i64,
    pub stability: f64,
    pub bitmap_cvg: f64,
    pub unique_crashes: i32,
    pub unique_hangs: i32,
    pub last_path: Option<SystemTime>,
    pub last_crash: Option<SystemTime>,
    pub last_hang: Option<SystemTime>,
    pub execs_since_crash: i64,
    pub exec_timeout: i32,
    pub slowest_exec_ms: i32,
    pub peak_rss_mb: Option<i32>,
    pub afl_banner: String,
    pub afl_version: String,
    pub target_mode: String,
    pub command_line: String,
}

impl AFLStat {
    pub fn load<T>(stat_file: T) where T: Into<&Path> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {

}
