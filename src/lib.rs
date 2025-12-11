use chrono::{Duration, Local, NaiveDate};
use dirs::data_dir;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    pub date: NaiveDate,
    pub minutes: i64,
    pub topic: String,
}

fn data_file_path() -> std::io::Result<PathBuf> {
    let base = data_dir().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::Other, "Could not find data directory")
    })?;

    let dir = base.join("clistudy");
    if !dir.exists() {
        create_dir_all(&dir)?;
    }

    Ok(dir.join("sessions.json"))
}

fn load_sessions() -> std::io::Result<Vec<Session>> {
    let path = data_file_path()?;

    if !path.exists() {
        return Ok(Vec::new());
    }

    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    if contents.trim().is_empty() {
        return Ok(Vec::new());
    }

    let sessions: Vec<Session> = serde_json::from_str(&contents).map_err(|e| {
        std::io::Error::new(std::io::ErrorKind::Other, format!("JSON error: {e}"))
    })?;

    Ok(sessions)
}

fn save_sessions(sessions: &[Session]) -> std::io::Result<()> {
    let path = data_file_path()?;
    let mut file = File::create(path)?;
    let contents = serde_json::to_string_pretty(sessions).map_err(|e| {
        std::io::Error::new(std::io::ErrorKind::Other, format!("JSON error: {e}"))
    })?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}

pub fn add_session(minutes: i64, topic: String) -> std::io::Result<()> {
    let mut sessions = load_sessions()?;
    let today = Local::now().date_naive();

    sessions.push(Session {
        date: today,
        minutes,
        topic,
    });

    save_sessions(&sessions)
}

pub fn summary_today() -> std::io::Result<HashMap<String, i64>> {
    let sessions = load_sessions()?;
    let today = Local::now().date_naive();
    Ok(summary_for_date(&sessions, today))
}

pub fn summary_week() -> std::io::Result<HashMap<String, i64>> {
    let sessions = load_sessions()?;
    let today = Local::now().date_naive();
    let week_ago = today - Duration::days(6); // last 7 days

    Ok(summary_between(&sessions, week_ago, today))
}

fn summary_for_date(sessions: &[Session], date: NaiveDate) -> HashMap<String, i64> {
    let mut totals = HashMap::new();

    for s in sessions.iter().filter(|s| s.date == date) {
        *totals.entry(s.topic.clone()).or_insert(0) += s.minutes;
    }

    totals
}

fn summary_between(
    sessions: &[Session],
    from: NaiveDate,
    to: NaiveDate,
) -> HashMap<String, i64> {
    let mut totals = HashMap::new();

    for s in sessions
        .iter()
        .filter(|s| s.date >= from && s.date <= to)
    {
        *totals.entry(s.topic.clone()).or_insert(0) += s.minutes;
    }

    totals
}