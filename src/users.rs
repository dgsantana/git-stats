use std::collections::{HashMap, HashSet};

use anyhow::{Context, Result};
use chrono::prelude::*;
use git2::{Commit, Repository};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct UserInfo {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub stats: UserStats,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UserStats {
    pub total_commits: u32,
    pub average_commits_per_day: f32,
    pub average_commits_per_month: f32,
    pub average_commits_per_year: f32,
    pub total_lines_added: usize,
    pub total_lines_removed: usize,
    pub average_lines_added_per_day: f32,
    pub average_lines_removed_per_day: f32,
    pub average_lines_added_per_month: f32,
    pub average_lines_removed_per_month: f32,
    pub average_lines_added_per_year: f32,
    pub average_lines_removed_per_year: f32,
    pub line_changes_per_year: HashMap<u32, usize>,
    pub line_changes_per_month: HashMap<u32, usize>,
    pub line_changes_per_day: HashMap<u32, usize>,
}

impl Eq for UserInfo {}

impl std::hash::Hash for UserInfo {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.name.hash(state);
        self.email.hash(state);
    }
}

impl UserInfo {
    pub fn new(name: String, email: String) -> Self {
        UserInfo {
            id: Uuid::new_v4(),
            name,
            email,
            stats: UserStats::default(),
        }
    }

    pub fn has_not_stats(&self) -> bool {
        self.stats.total_commits == 0
            && self.stats.total_lines_added == 0
            && self.stats.total_lines_removed == 0
    }
}

pub fn get_users(repo: &Repository) -> Result<Vec<UserInfo>> {
    let mut users = HashSet::new();
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

    for oid in revwalk {
        let oid = oid?;
        let commit = repo.find_commit(oid)?;
        let author = commit.author();
        let user_info = UserInfo::new(
            author.name().unwrap_or("Unknown").to_string(),
            author.email().unwrap_or("Unknown").to_string(),
        );
        users.insert(user_info);
    }

    Ok(users.into_iter().collect())
}

pub fn update_user_stats(
    users: &mut [UserInfo],
    repo: &Repository,
    _branch: &str,
) -> anyhow::Result<()> {
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

    #[derive(Default)]
    struct InternalUserStats {
        commit_by_date: HashMap<NaiveDate, u64>,
        changes_by_date: HashMap<NaiveDate, (usize, usize)>,
    }

    let mut user_stats: HashMap<Uuid, InternalUserStats> = HashMap::new();

    let progress = indicatif::ProgressBar::new_spinner();
    progress.set_message("Processing commits...");
    progress.set_style(
        indicatif::ProgressStyle::with_template("{spinner} {msg} [{elapsed_precise}]")
            .unwrap()
            .tick_chars("⠋⠙⠸⠼⠧⠇⠏"),
    );

    for oid in revwalk {
        progress.tick();
        let oid = oid?;
        let commit = repo.find_commit(oid)?;
        let author = commit.author();

        let user_email = author.email().unwrap_or_default();
        let Some(user) = users.iter().find(|u| u.email == user_email) else {
            continue;
        };

        let commit_date = utc_from_commit(&commit)?.date_naive();
        let (added, removed) = get_lines_changed(repo, &commit)?;

        user_stats
            .entry(user.id)
            .and_modify(|u| {
                u.commit_by_date
                    .entry(commit_date)
                    .and_modify(|x| *x += 1)
                    .or_insert(1);
                u.changes_by_date
                    .entry(commit_date)
                    .and_modify(|(a, b)| {
                        *a += added;
                        *b += removed;
                    })
                    .or_insert((added, removed));
            })
            .or_default();
    }
    progress.finish_and_clear();

    for user in users.iter_mut() {
        let Some(stats) = user_stats.get(&user.id) else {
            continue;
        };

        user.stats.total_commits = stats.commit_by_date.values().sum::<u64>() as u32;

        let total_days = stats.commit_by_date.len() as f32;
        user.stats.average_commits_per_day = if total_days > 0.0 {
            user.stats.total_commits as f32 / total_days
        } else {
            0.0
        };
        let total_years = stats
            .commit_by_date
            .keys()
            .map(|d| d.year())
            .collect::<HashSet<_>>()
            .len() as f32;
        user.stats.average_commits_per_year = if total_years > 0.0 {
            user.stats.total_commits as f32 / total_years
        } else {
            0.0
        };

        let total_months = stats
            .commit_by_date
            .keys()
            .map(|d| d.month())
            .collect::<HashSet<_>>()
            .len() as f32;
        user.stats.average_commits_per_month = if total_months > 0.0 {
            user.stats.total_commits as f32 / total_months
        } else {
            0.0
        };

        for (date, (added, removed)) in &stats.changes_by_date {
            let year = date.year();
            let month = date.month();
            let day = date.day();

            user.stats
                .line_changes_per_year
                .entry(year as u32)
                .and_modify(|v| *v += added + removed)
                .or_insert(0);
            user.stats
                .line_changes_per_month
                .entry(month)
                .and_modify(|v| *v += added + removed)
                .or_insert(0);
            user.stats
                .line_changes_per_day
                .entry(day)
                .and_modify(|v| *v += added + removed)
                .or_insert(0);
        }

        user.stats.total_lines_added = stats
            .changes_by_date
            .values()
            .map(|(added, _)| *added)
            .sum();
        user.stats.total_lines_removed = stats
            .changes_by_date
            .values()
            .map(|(_, removed)| *removed)
            .sum();

        user.stats.average_lines_added_per_day = if total_days > 0.0 {
            user.stats.total_lines_added as f32 / total_days
        } else {
            0.0
        };
        user.stats.average_lines_removed_per_day = if total_days > 0.0 {
            user.stats.total_lines_removed as f32 / total_days
        } else {
            0.0
        };

        user.stats.average_lines_added_per_month = if total_months > 0.0 {
            user.stats.total_lines_added as f32 / total_months
        } else {
            0.0
        };
        user.stats.average_lines_removed_per_month = if total_months > 0.0 {
            user.stats.total_lines_removed as f32 / total_months
        } else {
            0.0
        };
        user.stats.average_lines_added_per_year = if total_years > 0.0 {
            user.stats.total_lines_added as f32 / total_years
        } else {
            0.0
        };
        user.stats.average_lines_removed_per_year = if total_years > 0.0 {
            user.stats.total_lines_removed as f32 / total_years
        } else {
            0.0
        };
    }

    Ok(())
}

fn utc_from_commit(commit: &Commit) -> anyhow::Result<DateTime<Utc>> {
    let base_time =
        DateTime::from_timestamp(commit.time().seconds(), 0).context("failed to convert")?;
    let timezone_offset = commit.time().offset_minutes();
    let timezone = FixedOffset::east_opt(timezone_offset * 60).context("invalid offset")?;
    Ok(DateTime::<Local>::from_naive_utc_and_offset(base_time.naive_utc(), timezone).to_utc())
}

fn get_lines_changed(repo: &Repository, commit: &Commit) -> anyhow::Result<(usize, usize)> {
    let commit_tree = commit.tree()?;

    let mut insertions = 0;
    let mut deletions = 0;

    if commit.parent_count() > 0 {
        for i in 0..commit.parent_count() {
            let parent = commit.parent(i)?;
            let parent_tree = parent.tree()?;

            let diff = repo.diff_tree_to_tree(Some(&parent_tree), Some(&commit_tree), None)?;

            let stats = diff.stats()?;
            insertions += stats.insertions();
            deletions += stats.deletions();
        }
    } else {
        let diff = repo.diff_tree_to_tree(None, Some(&commit_tree), None)?;

        let stats = diff.stats()?;
        insertions = stats.insertions();
        deletions = stats.deletions();
    }

    Ok((insertions, deletions))
}
