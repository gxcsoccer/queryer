use anyhow::Result;
use queryer::query;
// use polars::prelude::*;
// use std::io::Cursor;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let url = "https://raw.githubusercontent.com/owid/covid-19-data/master/public/data/latest/owid-covid-latest.csv";
    // let data = reqwest::get(url).await?.text().await?;
    // let csv_data = Cursor::new(data);
    // let df = CsvReadOptions::default()
    //     .with_infer_schema_length(Some(16))
    //     .into_reader_with_file_handle(csv_data)
    //     .finish()?;

    // let mask = df.column("new_deaths")?.gt(100);
    // let filtered = df.filter(&mask?)?;
    // println!(
    //     "{:?}",
    //     filtered.select([
    //         "location",
    //         "total_cases",
    //         "new_cases",
    //         "total_deaths",
    //         "new_deaths"
    //     ])
    // );

    let sql = format!(
        "SELECT location name, total_cases, new_cases, total_deaths, new_deaths \
      FROM {} where new_deaths >= 100 ORDER BY new_cases DESC, new_deaths DESC",
        url
    );
    let df = query(sql).await?;
    println!("{:?}", df);

    Ok(())
}
