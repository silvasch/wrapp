use std::process::ExitCode;

use thaterror::prelude::*;

#[derive(Debug)]
pub enum Error {
    InvalidCookingTime,
    Overcooked { excess_time: u32 },
    Undercooked { missing_time: u32 },
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidCookingTime => write!(f, "invalid cooking time"),
            Error::Overcooked { excess_time } => write!(
                f,
                "the meal was overcooked by {} minute{}",
                if *excess_time == 1 {
                    "a".to_string()
                } else {
                    excess_time.to_string()
                },
                if *excess_time == 1 { "" } else { "s" }
            ),
            Error::Undercooked { missing_time } => write!(
                f,
                "the meal was undercooked by {} minute{}",
                if *missing_time == 1 {
                    "a".to_string()
                } else {
                    missing_time.to_string()
                },
                if *missing_time == 1 { "" } else { "s" }
            ),
        }
    }
}

impl std::error::Error for Error {}

fn cook_meal(raw_cooking_time: String) -> Result<(), Error> {
    const PERFECT_COOKING_TIME: u32 = 10;

    let cooking_time = raw_cooking_time.parse::<u32>().map_err(|e| {
        Error::InvalidCookingTime
            .into_that_error()
            .with_source(Box::new(e))
    })?;

    match cooking_time.cmp(&PERFECT_COOKING_TIME) {
        std::cmp::Ordering::Greater => Err(Error::Overcooked {
            excess_time: cooking_time - PERFECT_COOKING_TIME,
        }
        .into()),
        std::cmp::Ordering::Less => Err(Error::Undercooked {
            missing_time: PERFECT_COOKING_TIME - cooking_time,
        }
        .into()),
        std::cmp::Ordering::Equal => {
            println!("meal was perfectly cooked!");
            Ok(())
        }
    }
}

fn main() -> ExitCode {
    let raw_cooking_time = match std::env::args().nth(1) {
        Some(raw_cooking_time) => raw_cooking_time,
        None => {
            eprintln!("usage: cook_meal <cooking time>");
            return ExitCode::FAILURE;
        }
    };

    if let Err(e) = cook_meal(raw_cooking_time) {
        eprintln!("error: {}", e);
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
