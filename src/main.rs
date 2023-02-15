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
    let file_path = "data/test.csv";
    let df = load_csv_file(file_path).unwrap();
    let comments_col = df.column("Name").unwrap();
    let comments_col_head = comments_col.head(Some(5));
    println!("{}", df.head(Some(5)));
    println!("{}", comments_col_head);
    println!("{:?}", df.shape());
}

/* references 
https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html 
*/