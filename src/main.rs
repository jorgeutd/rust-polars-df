
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


fn count_passengers_by_sex(df: DataFrame) -> DataFrame {
    df.groupby(&vec!["Sex"])
        .expect("Failed to group by 'Sex' column")
        .select(&["PassengerId"])
        .count()
        .unwrap()
}





fn main() {
    let file_path = "data/test.csv";
    let df = load_csv_file(file_path).unwrap();
    let name_col = df.column("Name").unwrap();
    let name_col_head = name_col.head(Some(5));
    let count_df = count_passengers_by_sex(df.clone());
    println!("{:?}", df.shape());
    println!("{:?}",  df.get_column_names());
    println!("{}", df.head(Some(5)));
    println!("{}", name_col_head);

    println!("{}", count_df);
    
}

/* references 
https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html 
*/