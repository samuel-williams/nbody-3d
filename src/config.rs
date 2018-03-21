/* Parser for config file. */

use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

#[derive(Copy, Clone, Debug)]
pub struct Config {
    pub msaa_samples: u16,
    pub width: u32,
    pub height: u32,
}

impl Config {
    /* Default config. */
    pub fn new() -> Self {
        Config { 
            msaa_samples: 0, 
            width: 600,
            height: 800,
        }
    }

    /* Custom config from file. */
    pub fn from_file(file_name: &str) -> Self {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(file_name);
        let mut file = File::open(path).expect(&format!("{} not found in root", file_name));

        let mut file_contents = String::new();
        file.read_to_string(&mut file_contents)
            .expect(&format!("error reading {}", file_name));

        let mut cfg = Config::new();

        for line in file_contents.lines() {
            /* Skip comments and empty lines. */
            let first_char = line.chars().nth(0);
            if first_char == None || first_char == Some('#') {
                continue;
            }

            let mut tokens = line.split_whitespace();
            match tokens.next() {
                Some("samples") => {
                    if let Some(samples_str) = tokens.next() {
                        if let Ok(samples) = samples_str.parse() {
                            cfg.msaa_samples = samples;
                        } else {
                            Config::report_parse_error(line);
                        }
                    } else {
                        Config::report_parse_error(line);
                    }
                },
                Some("resolution") => {
                    if let Some(width_str) = tokens.next() {
                        if let Ok(width) = width_str.parse() {
                            cfg.width = width;
                        } else {
                            Config::report_parse_error(line);
                        }
                    } else {
                        Config::report_parse_error(line);
                    }

                    if let Some(height_str) = tokens.next() {
                        if let Ok(height) = height_str.parse() {
                            cfg.height = height;
                        } else {
                            Config::report_parse_error(line);
                        }
                    } else {
                        Config::report_parse_error(line);
                    }
                }
                _ => Config::report_parse_error(line),
            }
        }

        cfg
    }

    fn report_parse_error(line: &str) {
        println!("err (config): could not parse `{}`", line);
    }
}
