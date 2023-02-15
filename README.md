## Rust Polars Dataframes

This is a simple Rust program that loads a CSV file into a Polars dataframe and performs basic data manipulation. The program uses the polars and csv crates to accomplish this.

### Requirements
To run this program, you need to have Rust and Cargo installed on your system. You can download Rust and Cargo from the official Rust website: https://www.rust-lang.org/tools/install.

### Usage
To use this program, simply clone the repository and navigate to the project directory in your terminal. Then, run the following command:
cargo run

This will run the program, which will load the CSV file specified in main.rs and output the first five rows of the dataframe and the shape of the dataframe.

### How it Works
The program consists of two functions. The load_csv_file function loads a CSV file into a Polars dataframe using the CsvReader class from the polars crate. The main function then calls load_csv_file with the path to the CSV file and prints the first five rows of the resulting dataframe and its shape.

### References
For more information on how to work with Polars dataframes in Rust, please refer to the official Polars documentation. 
