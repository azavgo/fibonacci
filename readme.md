## Library to calculate Fibonacci numbers

### How to use this library
1. Add to Cargo.toml: 
```Toml
    [dependencies]
    goldenratio = {git = "https://github.com/azavgo/fibonacci"}
```
2. Calculate a Fibonacci number or a sequence of Fibonacci numbers: 
```Rust
use fibonacci::Fibonacci;  

fn main() {
    let fibo = Fibonacci::new(10);
    println!("Fibonacci number at position {} is {}", &fibo.n(), &fibo.x());
    println!("First {} Fibonacci numbers {:?}", &fibo.n(), &fibo.sequence());
}
```
