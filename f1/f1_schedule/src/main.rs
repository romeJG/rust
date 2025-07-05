
use serde::Deserialize;
use prettytable::{Table, row};
use chrono::{DateTime, NaiveDate, NaiveTime, NaiveDateTime, Utc};
use chrono_tz::Asia::Manila;
use reqwest::blocking::get;
use colored::*;
use chrono_tz::Tz;

#[derive(Debug, Deserialize)]
struct ApiResponse {
    #[serde(rename = "MRData")]
    mrdata: MrData,
}

#[derive(Debug, Deserialize)]
struct MrData {
    #[serde(rename = "RaceTable")]
    race_table: RaceTable,
}

#[derive(Debug, Deserialize)]
struct RaceTable {
    #[serde(rename = "Races")]
    races: Vec<Race>,
}

#[derive(Debug, Deserialize)]
struct Race {
    #[serde(rename = "raceName")]
    race_name: String,
    date: String,
    time: Option<String>,
    #[serde(rename = "Circuit")]
    circuit: Circuit,
    #[serde(rename = "FirstPractice")]
    first_practice: Option<Session>,
    #[serde(rename = "SecondPractice")]
    second_practice: Option<Session>,
    #[serde(rename = "ThirdPractice")]
    third_practice: Option<Session>,
    #[serde(rename = "Qualifying")]
    qualifying: Option<Session>,
    #[serde(rename = "SprintQualifying")]
    sprint_qualifying: Option<Session>,
    #[serde(rename = "Sprint")]
    sprint: Option<Session>,
}

#[derive(Debug, Deserialize)]
struct Circuit {
    #[serde(rename = "circuitName")]
    circuit_name: String,
    #[serde(rename = "Location")]
    location: Location,
}

#[derive(Debug, Deserialize)]
struct Location {
    locality: String,
    country: String,
}

#[derive(Debug, Deserialize)]
struct Session {
    date: String,
    time: String,
}

fn convert_to_local(date: &str, time: &str) -> DateTime<Tz> {
    let naive_date = NaiveDate::parse_from_str(date, "%Y-%m-%d").expect("❌ Failed to parse date");
    let naive_time = NaiveTime::parse_from_str(time, "%H:%M:%SZ").expect("❌ Failed to parse time");

    let naive_dt = NaiveDateTime::new(naive_date, naive_time);
    let utc_dt = DateTime::<Utc>::from_naive_utc_and_offset(naive_dt, Utc);
    utc_dt.with_timezone(&Manila)
}

fn format_date_time(dt: DateTime<Tz>) -> (String, String) {
    (
        dt.format("%b %d %a").to_string(),
        dt.format("%H:%M").to_string(),
    )
}

fn main() {
    println!("🏎️  Fetching F1 2025 schedule...\n");

    let response = get("https://api.jolpi.ca/ergast/f1/2025/?format=json")
        .expect("❌ Failed to fetch data")
        .json::<ApiResponse>()
        .expect("❌ Failed to parse JSON");

    let now = Utc::now().with_timezone(&Manila);
    let today = now.date_naive();

    let upcoming_races: Vec<_> = response.mrdata.race_table.races
        .into_iter()
        .filter(|r| NaiveDate::parse_from_str(&r.date, "%Y-%m-%d")
            .map(|d| d >= today)
            .unwrap_or(false))
        .take(2)
        .collect();

    if upcoming_races.is_empty() {
        println!("⚠️  No upcoming races found!");
        return;
    }

    for race in upcoming_races {
        println!(
            "🏁 {} - {}, {} ({})",
            race.race_name,
            race.circuit.location.locality,
            race.circuit.location.country,
            race.circuit.circuit_name
        );

        let mut table = Table::new();
        table.add_row(row!["📅 Session", "🗓️ Date", "⏰ Time"]);

        if let Some(fp) = race.first_practice {
            add_session_row("Practice 1", &fp.date, &fp.time, &mut table, now);
        }
        if let Some(fp) = race.second_practice {
            add_session_row("Practice 2", &fp.date, &fp.time, &mut table, now);
        }
        if let Some(fp) = race.third_practice {
            add_session_row("Practice 3", &fp.date, &fp.time, &mut table, now);
        }
        if let Some(sq) = race.sprint_qualifying {
            add_session_row("Sprint Qualifying", &sq.date, &sq.time, &mut table, now);
        }
        if let Some(sprint) = race.sprint {
            add_session_row("Sprint", &sprint.date, &sprint.time, &mut table, now);
        }
        if let Some(qual) = race.qualifying {
            add_session_row("Qualifying", &qual.date, &qual.time, &mut table, now);
        }
        if let Some(race_time) = race.time {
            add_session_row("Grand Prix", &race.date, &race_time, &mut table, now);
        } else {
            let date_str = NaiveDate::parse_from_str(&race.date, "%Y-%m-%d")
                .map(|d| d.format("%b %d %a").to_string())
                .unwrap_or_else(|_| race.date.clone());
            table.add_row(row!["Grand Prix", date_str, ""]);
        }

        table.printstd();
        println!("\n------------------------------------------\n");
    }
}

fn add_session_row(session: &str, date: &str, time: &str, table: &mut Table, now: DateTime<Tz>) {
    let dt = convert_to_local(date, time);
    let (date_str, time_str) = format_date_time(dt);

    if dt < now {
        table.add_row(row![
            session.dimmed().to_string(),
            date_str.dimmed().to_string(),
            time_str.dimmed().to_string()
        ]);
    } else {
        table.add_row(row![session, date_str, time_str]);
    }
}


