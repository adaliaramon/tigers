use tigers::DataFrame;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <path>", args[0]);
        std::process::exit(1);
    }
    let df: DataFrame = match DataFrame::from_csv(&args[1]) {
        Ok(df) => df,
        Err(e) => {
            eprintln!("Error reading CSV file \"{}\": {}", args[1], e);
            std::process::exit(1);
        }
    };
    println!("{}", df.head(5));
    println!("{}", df["Compound name"]);
    println!("{}", df[0]);
}
