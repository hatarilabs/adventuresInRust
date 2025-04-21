// use polars::prelude::*;
use anyhow::Result;
use std::fs::File;
use std::any::type_name;
use std::io::{BufRead, BufReader};
use chrono::{NaiveDate, Datelike};

fn print_type_of<T>(_: &T) {
    println!("{}", type_name::<T>());
}

fn main() -> Result<()> {

    let file = File::open("data/qc00156211.txt")?;
    let reader = BufReader::new(file);

    let mut df = DataFrame::new(vec![
        Series::new(datetime, Vec::<i64>::new()),
        Series::new("tmin", Vec::<f64>::new()),
        Series::new("tmax", Vec::<f64>::new()),
        Series::new("ppt", Vec::<f64>::new()),
    ])

    let mut col1 = Vec::new();
    let mut col2 = Vec::new();
    let mut col3 = Vec::new();
    let mut col4 = Vec::new();
    let mut col5 = Vec::new();
    let mut col6 = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let fields: Vec<&str> = line.split_whitespace().collect();

        if fields.len() >= 6 {
            col1.push(fields[0].parse::<i32>()?);
            col2.push(fields[1].parse::<i32>()?);
            col3.push(fields[2].parse::<i32>()?);
            col4.push(fields[3].parse::<f64>()?);
            col5.push(fields[4].parse::<f64>()?);
            col6.push(fields[5].parse::<f64>()?);
        }

        let a = fields[0].to_string();
        let b = fields[1].to_string();
        let c = a + b.as_str();

        let date = NaiveDate::from_ymd_opt(
            fields[0].parse::<i32>()?,
            fields[1].parse::<u32>()?,
            fields[2].parse::<u32>()?)
                .expect("Invalid date");

        // Convert to Unix timestamp
        let timestamp = date.and_hms_opt(0,0,0).timestamp_millis();

        let datetime_col = Datetime::new(timestamp);

        

        // let timestamps: NativeDateTime::parse_from_str(fields[0] + &fields[1] + &fields[2])
        //             .unwarp()
        //             .timestamp_millis();

        println!("{}", timestamp);
        // println!("{}", type_name::<date>);
        print_type_of(&date);
    };

    // let df = df![
    //     "Year" => col1,
    //     "Month" => col2,
    //     "Day" => col3,
    //     "Ppt" => col4,
    //     "Tmin" => col5,
    //     "Tmax" => col6,
    // ]?;

    // println!("{:?}",df);

    // let df = CsvReader::from_path("data/qc00156211.txt")?
    //     .has_reader(true)
    //     .with_delimiter(b' ')
    //     .finish()?;
    
    // println!("{}",df);

    Ok(())
}