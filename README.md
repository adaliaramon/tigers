# Tigers

A DataFrame library for Rust, inspired by Pandas.

## Usage

Add the following to your `Cargo.toml`:

```toml
[dependencies]
tigers = "0.1.1"
```

## Examples

```rust
use tigers::DataFrame;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <path>", args[0]);
        std::process::exit(1);
    }
    let df: DataFrame = DataFrame::from_csv(&args[1]).unwrap();
    println!("{}", df.head(5));
}
```

## License
Tigers is licensed under the [MIT license](LICENSE).
