use colored::*;
use chrono::*;

// TO DO: Implement easy custom configs
// TO DO: Implement file to set project level configs
// TO DO: Implement log file

struct Config {
    console_report_lvl: u8,
    file_report_lvl: u8,
    mute_list: Vec<u8>,
    write_to_file: bool,
    long_time_stamp: bool
}

impl Config {
    pub fn default() -> Config {
        Config { console_report_lvl: 5, file_report_lvl: 6, mute_list: Vec::new(), write_to_file: false, long_time_stamp: false }
    }

    pub fn verbose() -> Config {
        Config { console_report_lvl: 8, file_report_lvl: 8, mute_list: Vec::new(), write_to_file: false, long_time_stamp: true }
    }
}

pub struct Augur {
    config: Config
}

impl Augur {
    pub fn default() -> Augur {
        Augur { config: Config::default() }
    }

    pub fn verbose() -> Augur {
        Augur { config: Config::verbose() }
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

        println!("[{}] {}: {}", chrono::Local::now().format("%m-%d-%Y %T").to_string().truecolor(255, 0, 187), header, message);
    }
}
