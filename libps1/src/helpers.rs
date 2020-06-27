use git2::{Repository, Status};
use libc;
use std::{env, path::Path};
use tico::tico;

pub fn prompt_char(root: char, regular: char) -> char {
    const ROOT_UID: u32 = 0;
    let uid = unsafe { libc::geteuid() };

    if uid == ROOT_UID {
        root
    } else {
        regular
    }
}

pub fn cwd(shorten_home: bool, shorten_dirs: bool) -> Option<String> {
    let path_env = env::current_dir().ok()?;
    let mut path = format!("{}", path_env.display());

    if shorten_home {
        let home_dir = env::var("HOME").ok()?;
        let home_dir_ext = format!("{}/", home_dir);

        if (path == home_dir) || path.starts_with(&home_dir_ext) {
            path = path.replacen(&home_dir, "~", 1);
        }
    }

    if shorten_dirs {
        path = tico(&path);
    }

    Some(path)
}

pub enum GitStatus {
    Clean,
    /// Has some unstaged changed.
    Unstaged,
    /// All changes staged.
    Staged,
}

pub fn git_status() -> Option<(String, GitStatus)> {
    let current_dir = env::var("PWD").ok()?;

    let repo = {
        let mut repo: Option<Repository> = None;
        let current_path = Path::new(&current_dir[..]);
        for path in current_path.ancestors() {
            if let Ok(r) = Repository::open(path) {
                repo = Some(r);
                break;
            }
        }
        repo?
    };

    let reference = repo.head().ok()?;

    let branch = if reference.is_branch() {
        format!("{}", reference.shorthand()?)
    } else {
        let commit = reference.peel_to_commit().ok()?;
        let id = commit.id();

        format!("{:.6}", id)
    };

    let mut repo_status = GitStatus::Clean;

    for file in repo.statuses(None).ok()?.iter() {
        match file.status() {
            // STATE: unstaged (working tree modified)
            Status::WT_NEW
            | Status::WT_MODIFIED
            | Status::WT_DELETED
            | Status::WT_TYPECHANGE
            | Status::WT_RENAMED => {
                repo_status = GitStatus::Unstaged;
                break;
            }
            // STATE: staged (changes added to index)
            Status::INDEX_NEW
            | Status::INDEX_MODIFIED
            | Status::INDEX_DELETED
            | Status::INDEX_TYPECHANGE
            | Status::INDEX_RENAMED => {
                repo_status = GitStatus::Staged;
            }
            // STATE: committed (changes have been saved in the repo)
            _ => {}
        }
    }

    Some((branch, repo_status))
}

pub fn get_ahead_behind(r: &Repository) -> Option<(usize, usize)> {
    let head = r.head().ok()?;
    if !head.is_branch() {
        return None;
    }

    let head_name = head.shorthand()?;
    let head_branch = r.find_branch(head_name, git2::BranchType::Local).ok()?;
    let upstream = head_branch.upstream().ok()?;
    let head_oid = head.target()?;
    let upstream_oid = upstream.get().target()?;

    r.graph_ahead_behind(head_oid, upstream_oid).ok()
}
