use polars::prelude::*;
use std::fs::File;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file = File::open("data/qc00156211.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        println!("Line: {}", line);
    }

    Ok(())
}

fn main() -> PolarsResult<()> {
    let file = File::open("data.txt")?;
    let reader = BufReader::new(file);

    let df = CsvReader::new(reader)
        .has_reader(true)
        .with_delimiter(b' ')
        .finish()?;
    
    println!("{}",df);
    Ok(())
}