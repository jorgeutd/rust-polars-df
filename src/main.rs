use polars::prelude::{CsvReader, DataFrame, PolarsError};
use std::result::Result;
use polars::prelude::SerReader;

fn load_csv_file(file_path: &str) -> Result<DataFrame, PolarsError> {
    let df = CsvReader::from_path(file_path)?
        .infer_schema(None)
        .has_header(true)
        .finish()?;
    Ok(df)
}

fn main() {
    let file_path = "C:/Users/jorge.grisman/Documents/Not Scored_NPS_Survey_Comment_Analysis2.csv";
    let df = load_csv_file(file_path).unwrap();
    println!("{}", df.head(Some(5)));
    println!("{:?}", df.shape());
}

/* references 
https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html 
*/