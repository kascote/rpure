use colored::*;

pub fn get_char(cfg: &super::config::Config) -> colored::ColoredString {
    let euid = unsafe { libc::geteuid() };
    match euid {
        0 => return cfg.prompt_char_root.color(String::from(&cfg.prompt_char_root_color)),
        _ => return cfg.prompt_char.color(String::from(&cfg.prompt_char_color)),
    }
}
