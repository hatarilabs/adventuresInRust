use polars::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use chrono::NaiveDateTime;

fn main() -> PolarsResult<()> {

    let file = File::open("data/qc00156211.txt")?;
    let reader = BufReader::new(file);

    let mut date_vec: Vec<i64> = Vec::new();
    let mut ppt_vec: Vec<f64> = Vec::new();
    let mut tmax_vec: Vec<f64> = Vec::new();
    let mut tmin_vec: Vec<f64> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        
        let fields: Vec<String> = line
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        let (year, month, day) = (fields[0].clone(), fields[1].clone(), fields[2].clone());

        // let date=NaiveDate::from_ymd_opt(year, month, day);
        let date_str = format!("{}-{}-{} 00:00:00",year,month,day);
        let date_value = NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%d %H:%M:%S")
            .unwrap()
            .and_utc()
            .timestamp_millis();

        // other values    
        let ppt_value: f64 = fields[3].clone().parse().expect("Failed to parse ppt");
        let tmax_value: f64 = fields[4].clone().parse().expect("Failed to parse tmax");
        let tmin_value: f64 = fields[5].clone().parse().expect("Failed to parse tmin");
        
        // push values to the vecs
        date_vec.push(date_value);
        ppt_vec.push(ppt_value);
        tmax_vec.push(tmax_value);
        tmin_vec.push(tmin_value);
    };

    // Create series
    let date_series = Series::new("date",date_vec).
        cast(&DataType::Datetime(TimeUnit::Milliseconds, None))?;
    let ppt_series = Series::new("ppt", ppt_vec);
    let tmax_series = Series::new("tmax",tmax_vec);
    let tmin_series = Series::new("tmin",tmin_vec);

    let tmax_fix = tmax_series
        .apply(|v| if v == -99.9 { None } else { Some(v) }, GetOutput::Same)
        .cast(&DateType::Float64)?;
    let tmin_fix = tmin_series
        .apply(|v| if v == -99.9 { None } else { Some(v) }, GetOutput::Same)
        .cast(&DateType::Float64)?;
    
    // // Build the DataFrame from series
    let df = DataFrame::new(vec![date_series,ppt_series,tmax_fix,tmin_fix])?;

    println!("{:?}", df.head(Some(5)));
    Ok(())
}
