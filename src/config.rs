use std::env;
use std::env::VarError;

#[derive(Debug)]
pub struct Config {
    pub shorten_cwd: bool,
    pub expand_home: bool,
    pub cwd_color: String,
    pub prompt_char: String,
    pub prompt_char_root: String,
    pub prompt_char_color: String,
    pub prompt_char_root_color: String,
    pub branch_color: String,
    pub commit_color: String,
    pub git_clean_color: String,
    pub git_wt_modified_color: String,
    pub git_index_modified_color: String,
    pub git_clean: String,
    pub git_wt_modified: String,
    pub git_index_modified: String,
    pub add_new_line: bool,
    pub git_arrow_up_char: String,
    pub git_arrow_down_char: String,
    pub git_arrow_up_color: String,
    pub git_arrow_down_color: String,
    pub git_stash_char: String,
    pub git_stash_color: String,
    pub git_action_color: String,
}

impl Config {
    pub fn init() -> Config {
        Config {
            shorten_cwd:              Config::unwrap_bool(env::var("RPURE_SHORT_DIR")),
            expand_home:             Config::unwrap_bool(env::var("RPURE_EXPAND_HOME")),
            cwd_color:                env::var("RPURE_COLOR_CURRENT_DIRECTORY").unwrap_or("cyan".into()),
            prompt_char:              env::var("RPURE_CHAR_PROMPT").unwrap_or("❯".into()),
            prompt_char_color:        env::var("RPURE_COLOR_PROMPT").unwrap_or("magenta".into()),
            prompt_char_root:         env::var("RPURE_CHAR_PROMPT_ROOT").unwrap_or("#".into()),
            prompt_char_root_color:   env::var("RPURE_COLOR_PROMPT_ROOT").unwrap_or("red".into()),
            branch_color:             env::var("RPURE_COLOR_BRANCH").unwrap_or("bright black".into()),
            commit_color:             env::var("RPURE_COLOR_COMMIT").unwrap_or("bright black".into()),
            git_clean:                env::var("RPURE_GIT_CHAR_CLEAN").unwrap_or("●".into()),
            git_clean_color:          env::var("RPURE_GIT_COLOR_CLEAN").unwrap_or("green".into()),
            git_wt_modified:          env::var("RPURE_GIT_CHAR_DIRTY").unwrap_or("●".into()),
            git_wt_modified_color:    env::var("RPURE_GIT_COLOR_CHAR_DIRTY").unwrap_or("red".into()),
            git_index_modified:       env::var("RPURE_GIT_CHAR_INDEX_MODIFIED").unwrap_or("±".into()),
            git_index_modified_color: env::var("RPURE_GIT_COLOR_INDEX_MODIFIED").unwrap_or("yellow".into()),
            git_arrow_up_char:        env::var("RPURE_GIT_CHAR_UNPUSHED").unwrap_or("↑".into()),
            git_arrow_down_char:      env::var("RPURE_GIT_CHAR_UNPULLED").unwrap_or("↓".into()),
            git_arrow_up_color:       env::var("RPURE_GIT_COLOR_UNPUSHED").unwrap_or("cyan".into()),
            git_arrow_down_color:     env::var("RPURE_GIT_COLOR_UNPULLED").unwrap_or("cyan".into()),
            git_stash_char:           env::var("RPURE_GIT_CHAR_STASH").unwrap_or("☷".into()),
            git_stash_color:          env::var("RPURE_GIT_COLOR_STASH").unwrap_or("yellow".into()),
            git_action_color:         env::var("RPURE_GIT_COLOR_ACTION").unwrap_or("red".into()),
            add_new_line:             Config::unwrap_bool(Ok(env::var("RPURE_PROMPT_ADD_NEWLINE").unwrap_or("1".into())))
        }
    }

    fn unwrap_bool(value: Result<String, VarError>) -> bool {
        return match value {
            Ok(value) => value == "1",
            Err(_) => false
        }
    }
}
