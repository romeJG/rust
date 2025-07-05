
use serde::Deserialize;
use prettytable::{Table, row};
use reqwest::blocking::get;
use std::collections::HashMap;
use colored::*;

#[derive(Debug, Deserialize)]
struct ApiResponse {
    #[serde(rename = "MRData")]
    mrdata: MrData,
}

#[derive(Debug, Deserialize)]
struct MrData {
    #[serde(rename = "StandingsTable")]
    standings_table: StandingsTable,
}

#[derive(Debug, Deserialize)]
struct StandingsTable {
    #[serde(rename = "StandingsLists")]
    standings_lists: Vec<StandingsList>,
}

#[derive(Debug, Deserialize)]
struct StandingsList {
    #[serde(rename = "DriverStandings")]
    driver_standings: Vec<DriverStanding>,
}

#[derive(Debug, Deserialize)]
struct DriverStanding {
    position: String,
    points: String,
    wins: String,
    #[serde(rename = "Driver")]
    driver: Driver,
    #[serde(rename = "Constructors")]
    constructors: Vec<Constructor>,
}

#[derive(Debug, Deserialize)]
struct Driver {
    code: String,
    #[serde(rename = "givenName")]
    given_name: String,
    #[serde(rename = "familyName")]
    family_name: String,
    nationality: String,
}

#[derive(Debug, Deserialize)]
struct Constructor {
    name: String,
    nationality: String,
}

fn main() {
    println!("🏎️  Fetching 2025 F1 Driver Standings...\n");

    let response = get("https://api.jolpi.ca/ergast/f1/2025/driverstandings/?format=json")
        .expect("❌ Failed to fetch data")
        .json::<ApiResponse>()
        .expect("❌ Failed to parse JSON");

    let standings_lists = &response.mrdata.standings_table.standings_lists;

    if standings_lists.is_empty() {
        println!("⚠️  No standings data found!");
        return;
    }

    let standings = &standings_lists[0].driver_standings;

    let mut constructor_totals: HashMap<String, f32> = HashMap::new();

    // Sum up constructor points
    for driver in standings {
        let constructor = &driver.constructors[0];
        let points: f32 = driver.points.parse().unwrap_or(0.0);

        *constructor_totals.entry(constructor.name.clone()).or_insert(0.0) += points;
    }

    let mut table = Table::new();
    table.add_row(row![
        "🏁 Pos",
        "🏎️ Code",
        "🧑 Name",
        "🏆 Points",
        "🏢 Constructor (Total)"
    ]);

    for driver in standings {
        let constructor = &driver.constructors[0];
        let total_points = constructor_totals.get(&constructor.name).unwrap_or(&0.0);

        // Choose color based on constructor
        let colored_constructor = match constructor.name.as_str() {
            "Red Bull" => format!("{} ({})", constructor.name.red(), total_points),
            "McLaren" => format!("{} ({})", constructor.name.bright_yellow(), total_points),
            "Ferrari" => format!("{} ({})", constructor.name.bright_red(), total_points),
            "Mercedes" => format!("{} ({})", constructor.name.cyan(), total_points),
            "Alpine F1 Team" => format!("{} ({})", constructor.name.blue(), total_points),
            "Aston Martin" => format!("{} ({})", constructor.name.green(), total_points),
            "Haas F1 Team" => format!("{} ({})", constructor.name.bright_white(), total_points),
            "Williams" => format!("{} ({})", constructor.name.bright_blue(), total_points),
            "RB F1 Team" => format!("{} ({})", constructor.name.magenta(), total_points),
            "Sauber" => format!("{} ({})", constructor.name.bright_cyan(), total_points),
            _ => format!("{} ({})", constructor.name, total_points), // Default no color
        };

        table.add_row(row![
            driver.position,
            driver.driver.code,
            format!("{} {}", driver.driver.given_name, driver.driver.family_name),
            driver.points,
            colored_constructor
        ]);
    }

    table.printstd();
}

