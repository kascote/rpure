use git2::{Repository, Status};
use std::env;
use std::path::Path;

#[derive(Debug)]
pub enum VcsDirty {
    WTmodified,
    IDmodified,
    Green,
}

#[derive(Debug)]
pub struct VcsStatus {
    pub name: String,
    pub dirty: String,
    pub dirty_status: VcsDirty,
    pub ahead: usize,
    pub behind: usize,
    pub stash: bool,
    pub state: String,
}

impl VcsStatus {
    pub fn blank() -> VcsStatus {
        VcsStatus {
            name: String::from(""),
            dirty: String::from(""),
            dirty_status: VcsDirty::Green,
            ahead: 0,
            behind: 0,
            stash: false,
            state: String::from(""),
        }
    }
}

pub fn vcs_status(cfg: &super::config::Config) -> Option<VcsStatus> {
    let current_dir = env::var("PWD").unwrap();
    let mut status = VcsStatus::blank() ;

    let mut repo = vcs_repo(&current_dir)?;

    //println!("{:?}", repo.is_empty());

    let state: String = format!("{:?}", repo.state());
    if state != "Clean" {
        status.state = state;
    }

    let (ahead, behind) = get_ahead_behind(&repo)?;
    status.ahead = ahead;
    status.behind = behind;

    let each_stash = |_idx: usize, _name: &str, _oid: &git2::Oid| -> bool { 
        status.stash = true;
        false 
    };
    repo.stash_foreach(each_stash).expect("error checking stashed files");

    status.name = get_repo_name(&repo);
    status.dirty = cfg.git_clean.to_string();

    let file_stats = repo.statuses(None).unwrap();
    for file in file_stats.iter() {
        match file.status() {
            // STATE: unstaged (working tree modified)
            Status::WT_NEW
                | Status::WT_MODIFIED
                | Status::WT_DELETED
                | Status::WT_TYPECHANGE
                | Status::WT_RENAMED => {
                    status.dirty = cfg.git_wt_modified.to_string();
                    status.dirty_status = VcsDirty::WTmodified;
                    break;
                }
            // STATE: staged (changes added to index)
            Status::INDEX_NEW
                | Status::INDEX_MODIFIED
                | Status::INDEX_DELETED
                | Status::INDEX_TYPECHANGE
                | Status::INDEX_RENAMED => {
                    status.dirty = cfg.git_index_modified.to_string();
                    status.dirty_status = VcsDirty::IDmodified;
                }
            // STATE: committed (changes have been saved in the repo)
            _ => {
                status.dirty_status = VcsDirty::Green
            }
        }
    }
    return Some(status);
}

fn vcs_repo(current_dir: &String) -> Option<Repository> {
    let mut repo: Option<Repository> = None; 
    let current_path = Path::new(&current_dir[..]);
    let mut idx = 0;
    for path in current_path.ancestors() {
        match Repository::open(path) {
            Ok(r) => {
                repo = Some(r);
                break;
            }
            Err(_) => {}
        }
        if idx == 10 {
            break;
        }
        idx += 1;
    }
    return repo;
}

fn get_ahead_behind(repo: &Repository) -> Option<(usize, usize)> {
    let head = match repo.head() {
        Ok(h) => h,
        Err(_) => return Some((0, 0)) // return None
    };
    if !head.is_branch() {
        return Some((0, 0)); // None;
    }

    let head_name = head.shorthand()?;
    // let tracking_branch_name = format!("{}@{{origin}}", head_name);
    let head_branch = (repo.find_branch(head_name, git2::BranchType::Local).ok())?;
    let upstream = match head_branch.upstream() {
        Ok(u) => u,
        Err(_) => {
            return Some((0, 0))
        }
    };
    let head_oid = head.target()?;
    let upstream_oid = (upstream.get().target())?;

    let status = match repo.graph_ahead_behind(head_oid, upstream_oid) {
        Ok(r) => {
            let mut ahead = 0;
            let mut behind = 0;
            if r.0 > 0 { // ahead
                ahead = r.0;
            } 
            if r.1 > 0 { // behind
                behind = r.1;
            } 
            (ahead, behind)
        },
        Err(_) => (0, 0)
    };

    return Some(status);
}

fn get_repo_name(repo: &Repository) -> String {
    let head = match repo.head() {
        Ok(r) => r,
        Err(_) => return String::from(""),
    };

    if head.is_branch() {
        return head.shorthand().unwrap_or("err").into();
    } else {
        let commit = head.peel_to_commit().unwrap();
        return format!("{:.6}", commit.id());
    }
}
