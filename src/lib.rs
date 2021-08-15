use colored::*;
use chrono::*;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use std::env;
use std::fs;

// TO DO: Implement file to set project level configs
// TO DO: Implement log file
// TO DO: Error Handling

#[derive(Serialize, Deserialize)]
struct Config {
    console_report_lvl: u8,
    log_report_lvl: u8,
    long_time_stamp: bool,
    log_to_file: bool,
    use_custom_log_path: bool,
    custom_log_path: PathBuf
}

impl Config {
    pub fn new() -> Config {
        let cfg = Config::load_config_file();
        println!("{}", cfg.console_report_lvl);
        cfg
    }

    pub fn load_config_file() -> Config {
        let mut p = env::current_dir().unwrap_or_else(|e| {
            panic!("{}: Error getting the current directory.", e);
        });

        p.push("config/augurcfg.toml");
        if !p.exists() {
            fs::copy("config/.augurcfg_default.toml", "config/augurcfg.toml");
        }
        let txt = fs::read_to_string(p).unwrap();
        toml::from_str(txt.as_str()).unwrap()
    }
}

pub struct Augur {
    config: Config,

}

impl Augur {
    pub fn new() -> Augur {
        Augur { config: Config::new() }
    }

    pub fn log(&self, message: &str, lvl: u8) {
        if lvl > self.config.console_report_lvl {
            return
        }

        let header = match lvl {
            0 => "EMERGENCY".black().bold().on_red().blink(),
            1 => "ALERT".truecolor(255, 0, 0).bold().blink(),
            2 => "CRITICAL".truecolor(255, 0, 0).bold(),
            3 => "ERROR".truecolor(255, 63, 0),
            4 => "WARNING".truecolor(255, 127, 0),
            5 => "NOTICE".truecolor(255, 255, 0),
            6 => "INFO".truecolor(0, 255, 0),
            7 => "DEBUG".truecolor(0, 255, 255),
            8 => "TRACE".truecolor(0, 127, 255),
            _ => "NF".white()
        };

        eprintln!("[{}] {}: {}", Local::now().format("%m-%d-%Y %T").to_string().truecolor(255, 0, 187), header, message);
    }
}
