use tigers::DataFrame;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <path>", args[0]);
        std::process::exit(1);
    }
    let df: DataFrame = DataFrame::from_csv(&args[1]).unwrap();
    println!("{}", df.head(5));
    println!("{}", df["Compound name"]);
    println!("{}", df[0]);
}
