use std::env;
use tico::tico;
use std::path::PathBuf;

pub fn cwd(cfg: &super::config::Config) -> Option<String> {
    let path_env = match env::var("PWD") {
        Ok(p) => PathBuf::from(p),
        Err(_) => {
            match env::current_dir() {
                Ok(p) => p,
                Err(_) => {
                    PathBuf::from("err current_dir")
                }
            }
        }
    };

    let mut path: String = path_env.to_string_lossy().into_owned();
    let home = env::var("HOME").unwrap();
    
    if !cfg.expand_home {
        let home_dir_ext = format!("{}{}", home, "/");
        if (&path == &home) || (path.starts_with(&home_dir_ext)) {
            path = path.replacen(&home[..], "~", 1);
        }
    }

    if cfg.shorten_cwd { 
        return Some(tico(&path, Option::None));
    } else {
        return Some(path);
    }
}
