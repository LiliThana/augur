use colored::*;
use chrono::*;

// TO DO: Implement log file

type Result<T> = std::result::Result<T, errors::Error>;

pub struct Augur {
    config: config::Config,

}

impl Augur {
    pub fn new() -> Result<Augur> {
        let cfg = config::Config::new();
        match cfg {
            Ok(t) => return Result::Ok(Augur {config: t}),
            Err(e) => return Result::Err(e)
        };
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

fn augur_message(message: &str) {
    let aug = format!("{}{}{}", "[".cyan(), "Augur".magenta(), "]".cyan());
    eprintln!("{} {}", aug, message);
}

pub mod config {
    use serde::{Serialize, Deserialize};
    use std::path::PathBuf;
    use std::{env, fs};
    
    #[derive(Debug)]
    #[derive(Serialize, Deserialize)]
    pub struct Config {
        pub console_report_lvl: u8,
        pub log_report_lvl: u8,
        pub long_time_stamp: bool,
        pub log_to_file: bool,
        pub use_custom_log_path: bool,
        pub custom_log_path: PathBuf
    }

    impl Config {
        pub fn new() -> crate::Result<Config> {
            Config::load_config_file()
        }

        fn load_config_file() -> crate::Result<Config> {
            crate::augur_message("Loading configuration file...");
            let mut path = env::current_dir()?;

            path.push("config/augurcfg.toml");
            if !path.exists() {
                crate::augur_message("Unable to find configuration file. A new default file will be created.");
                crate::augur_message("Creating the new conguration file...");
                &path.pop();
                fs::create_dir_all(&path)?;
                &path.push("augurcfg.toml");
                fs::write(&path, "# Configuration File for Augur
# lowest notification level that Augur will print to the console
console_report_lvl = 4
# lowest notification level that Augur will print to the log file
log_report_lvl = 6
# false will use 'HH:MM:SS' format, true will use 'mm-dd-yyy HH:MM:SS' format
long_time_stamp = false
# whether to print log to a file
log_to_file = true
# whether to use a custom path for the log file
use_custom_log_path = false
# the custom path for the log file
custom_log_path = \"\"")?;
                crate::augur_message("Successfully created a new deafult configuration file!");
            }

            let txt = fs::read_to_string(path)?;
            Result::Ok(toml::from_str(txt.as_str()).unwrap())
        }
    }
}



pub mod errors {
    use std::fmt;
    
    #[derive(Debug)]
    pub struct Error {
        details: String
    }

    impl Error {
        pub fn new(msg: &str) -> Error {
            Error{details: msg.to_string()}
        }
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.details)
        }
    }

    impl std::error::Error for Error {
        fn description(&self) -> &str {
            &self.details
        }
    }

    impl std::convert::From<std::io::Error> for Error {
        fn from(e: std::io::Error) -> Error {
            Error::new(&e.to_string())
        }
    }

}
