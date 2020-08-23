mod cwd;
mod prompt_char;
mod vcs;
mod venv;
mod config;

use colored::*;

fn main() {
    let param = std::env::args().nth(1).or(Some("".to_string())).unwrap();

    match param.as_ref() {
        "-z" => pista(true),
        "-h" => help(),
        _ => pista(false),
    };
}

fn help() {
    let msg = format!(" \
            rpure v{version} ~ {desc} \n \
            usage:\n \
            \t -z print zshel compatible prompt \n \
            \t -h this help", 
        version = env!("CARGO_PKG_VERSION"),
        desc = env!("CARGO_PKG_DESCRIPTION")
    );
    println!("{}", msg);
}

fn pista(zsh: bool) {
    // we force the color output, because PS1=$(rpure) will not print any color
    // have a point to not use colors ?¿ 
    // https://github.com/mackwic/colored/blob/master/src/control.rs#L105
    colored::control::set_override(true);

    let cfg = config::Config::init();
    let cwd = match cwd::cwd(&cfg) {
        Some(c) => c.color(String::from(&cfg.cwd_color)),
        None => "[directory does not exist]".color("red"),
    };
    let vcs_status = vcs::vcs_status(&cfg).unwrap_or(vcs::VcsStatus::blank());
    let venv = venv::get_name();
    let prompt_char = prompt_char::get_char(&cfg);
    let has_nl = if cfg.add_new_line { "\n" } else { "" };

    let status_color = match vcs_status.dirty_status {
        vcs::VcsDirty::IDmodified => cfg.git_index_modified_color,
        vcs::VcsDirty::WTmodified => cfg.git_wt_modified_color,
        vcs::VcsDirty::Green => cfg.git_clean_color,
    };

    let stash = match vcs_status.stash {
        true => cfg.git_stash_char.color(cfg.git_stash_color),
        false => "".color("white"),
    };

    let mut state = String::from("");
    if vcs_status.state != "" {
        state = format!("{}{}", state, "[".color(colored::Color::BrightBlack));
        state = format!("{}{}", state, vcs_status.state.color(cfg.git_action_color));
        state = format!("{}{}", state, "]".color(colored::Color::BrightBlack));
    }

    let mut arrow = String::from("");
    if vcs_status.ahead > 0 {
        arrow = format!("{}{}", arrow, &cfg.git_arrow_up_char.color(cfg.git_arrow_up_color));
    }
    if vcs_status.behind > 0 {
        arrow = format!("{}{}", arrow, &cfg.git_arrow_down_char.color(cfg.git_arrow_down_color));
    }

    let format = if zsh {
        format!(
            "%{{{cwd} {branch} {status} {state} {arrow} {stash}%}} %{{\n{venv}{pchar}%}} ",
            cwd = cwd,
            branch = vcs_status.name.color(String::from(&cfg.branch_color)),
            status = vcs_status.dirty.color(status_color),
            arrow = arrow,
            stash = stash,
            venv = venv.bright_black(),
            pchar = prompt_char,
            state = state
        )
    } else {
        format!(
            "{nl}{cwd} {branch} {status} {state} {arrow} {stash}\n{venv}{pchar} ",
            cwd = cwd,
            branch = vcs_status.name.color(cfg.branch_color),
            status = vcs_status.dirty.color(status_color),
            arrow = arrow,
            stash = stash,
            venv = venv,
            pchar = prompt_char,
            state = state,
            nl = has_nl
        )
    };

    print!("{}", format);
}


// https://github.com/Lucretiel/lazy_format
