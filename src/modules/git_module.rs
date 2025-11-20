use colored::Colorize;
use git2::{BranchType, IndexAddOption, Repository, Signature};
use std::path::Path;

/// Display git status for current directory
pub fn git_status(path: &Path) {
    match Repository::open(path) {
        Ok(repo) => {
            println!("{}", "Git Status".cyan().bold());
            println!("{}", "=".repeat(80).cyan());

            // Get HEAD reference
            match repo.head() {
                Ok(head) => {
                    let branch_name = head.shorthand().unwrap_or("(detached)");
                    println!("{} {}", "Branch:".yellow().bold(), branch_name.green());
                }
                Err(_) => println!(
                    "{} {}",
                    "Branch:".yellow().bold(),
                    "(no commits yet)".yellow()
                ),
            }

            // Get status
            match repo.statuses(None) {
                Ok(statuses) => {
                    if statuses.is_empty() {
                        println!("\n{} Working tree clean", "✓".green().bold());
                    } else {
                        println!("\n{}", "Changes:".yellow().bold());
                        for entry in statuses.iter() {
                            let status = entry.status();
                            let path = entry.path().unwrap_or("unknown");

                            if status.is_wt_new() {
                                println!("  {} {}", "??".red(), path);
                            } else if status.is_wt_modified() {
                                println!("  {} {}", "M ".yellow(), path);
                            } else if status.is_wt_deleted() {
                                println!("  {} {}", "D ".red(), path);
                            } else if status.is_index_new() {
                                println!("  {} {}", "A ".green(), path);
                            } else if status.is_index_modified() {
                                println!("  {} {}", "M ".green(), path);
                            }
                        }
                    }
                }
                Err(e) => eprintln!("{} Failed to get status: {}", "Error:".red().bold(), e),
            }
        }
        Err(e) => eprintln!("{} Not a git repository: {}", "Error:".red().bold(), e),
    }
}

/// Clone a git repository
pub fn git_clone(url: &str, path: &Path) {
    println!("{} Cloning repository...", "→".cyan());
    println!("  URL:  {}", url.yellow());
    println!("  Path: {}", path.display().to_string().yellow());

    match Repository::clone(url, path) {
        Ok(_) => println!("\n{} Repository cloned successfully!", "✓".green().bold()),
        Err(e) => eprintln!(
            "{} Failed to clone repository: {}",
            "Error:".red().bold(),
            e
        ),
    }
}

/// Create a new branch
pub fn git_create_branch(path: &Path, branch_name: &str) {
    match Repository::open(path) {
        Ok(repo) => match repo.head() {
            Ok(head) => match head.peel_to_commit() {
                Ok(commit) => match repo.branch(branch_name, &commit, false) {
                    Ok(_) => println!(
                        "{} Created branch: {}",
                        "✓".green().bold(),
                        branch_name.green()
                    ),
                    Err(e) => eprintln!("{} Failed to create branch: {}", "Error:".red().bold(), e),
                },
                Err(e) => eprintln!("{} Failed to get commit: {}", "Error:".red().bold(), e),
            },
            Err(e) => eprintln!("{} Failed to get HEAD: {}", "Error:".red().bold(), e),
        },
        Err(e) => eprintln!("{} Not a git repository: {}", "Error:".red().bold(), e),
    }
}

/// Add all files to staging
pub fn git_add_all(path: &Path) {
    match Repository::open(path) {
        Ok(repo) => match repo.index() {
            Ok(mut index) => match index.add_all(["."].iter(), IndexAddOption::DEFAULT, None) {
                Ok(_) => match index.write() {
                    Ok(_) => println!("{} All changes staged", "✓".green().bold()),
                    Err(e) => eprintln!("{} Failed to write index: {}", "Error:".red().bold(), e),
                },
                Err(e) => eprintln!("{} Failed to add files: {}", "Error:".red().bold(), e),
            },
            Err(e) => eprintln!("{} Failed to get index: {}", "Error:".red().bold(), e),
        },
        Err(e) => eprintln!("{} Not a git repository: {}", "Error:".red().bold(), e),
    }
}

/// Commit changes
pub fn git_commit(path: &Path, message: &str) {
    match Repository::open(path) {
        Ok(repo) => {
            let signature = match Signature::now("CLI Helper", "cli@helper.local") {
                Ok(sig) => sig,
                Err(e) => {
                    eprintln!(
                        "{} Failed to create signature: {}",
                        "Error:".red().bold(),
                        e
                    );
                    return;
                }
            };

            match repo.index() {
                Ok(mut index) => match index.write_tree() {
                    Ok(tree_id) => match repo.find_tree(tree_id) {
                        Ok(tree) => {
                            let parent_commit =
                                repo.head().ok().and_then(|h| h.peel_to_commit().ok());
                            let parents = if let Some(ref p) = parent_commit {
                                vec![p]
                            } else {
                                vec![]
                            };

                            match repo.commit(
                                Some("HEAD"),
                                &signature,
                                &signature,
                                message,
                                &tree,
                                &parents,
                            ) {
                                Ok(_) => println!(
                                    "{} Committed: {}",
                                    "✓".green().bold(),
                                    message.yellow()
                                ),
                                Err(e) => {
                                    eprintln!("{} Failed to commit: {}", "Error:".red().bold(), e)
                                }
                            }
                        }
                        Err(e) => eprintln!("{} Failed to find tree: {}", "Error:".red().bold(), e),
                    },
                    Err(e) => eprintln!("{} Failed to write tree: {}", "Error:".red().bold(), e),
                },
                Err(e) => eprintln!("{} Failed to get index: {}", "Error:".red().bold(), e),
            }
        }
        Err(e) => eprintln!("{} Not a git repository: {}", "Error:".red().bold(), e),
    }
}

/// List branches
pub fn git_list_branches(path: &Path) {
    match Repository::open(path) {
        Ok(repo) => {
            println!("{}", "Branches:".cyan().bold());

            match repo.branches(Some(BranchType::Local)) {
                Ok(branches) => {
                    for branch in branches {
                        if let Ok((branch, _)) = branch {
                            let name = branch
                                .name()
                                .unwrap_or(Some("unknown"))
                                .unwrap_or("unknown");
                            if branch.is_head() {
                                println!("  {} {}", "*".green(), name.green().bold());
                            } else {
                                println!("    {}", name);
                            }
                        }
                    }
                }
                Err(e) => eprintln!("{} Failed to list branches: {}", "Error:".red().bold(), e),
            }
        }
        Err(e) => eprintln!("{} Not a git repository: {}", "Error:".red().bold(), e),
    }
}

/// Show recent commits
pub fn git_log(path: &Path, count: usize) {
    match Repository::open(path) {
        Ok(repo) => {
            println!("{}", "Recent Commits:".cyan().bold());
            println!("{}", "=".repeat(80).cyan());

            match repo.head() {
                Ok(head) => match head.peel_to_commit() {
                    Ok(mut commit) => {
                        for i in 0..count {
                            let id = commit.id().to_string();
                            let short_id = &id[..7];
                            let message = commit.message().unwrap_or("(no message)");
                            let author_name =
                                commit.author().name().unwrap_or("unknown").to_string();

                            println!("\n{} {}", "Commit:".yellow().bold(), short_id.green());
                            println!("Author: {}", author_name);
                            println!("Date:   {}", format_timestamp(commit.time().seconds()));
                            println!("\n    {}", message.trim());

                            if i < count - 1 {
                                match commit.parent(0) {
                                    Ok(parent) => commit = parent,
                                    Err(_) => break,
                                }
                            }
                        }
                    }
                    Err(e) => eprintln!("{} Failed to get commit: {}", "Error:".red().bold(), e),
                },
                Err(_) => println!("{} No commits yet", "Info:".yellow().bold()),
            }
        }
        Err(e) => eprintln!("{} Not a git repository: {}", "Error:".red().bold(), e),
    }
}

fn format_timestamp(seconds: i64) -> String {
    use std::time::{Duration, SystemTime, UNIX_EPOCH};
    let time = UNIX_EPOCH + Duration::from_secs(seconds as u64);
    match time.duration_since(UNIX_EPOCH) {
        Ok(duration) => {
            let secs = duration.as_secs();
            let days = secs / 86400;
            format!("{} days ago", days)
        }
        Err(_) => "unknown".to_string(),
    }
}
