use std::io;

fn main() {
    println!("Enter a number");

    let mut n = String::new();
    let mut biggest: u64 = 1;

    io::stdin().read_line(&mut n).expect("Enter a valid number");

    let n: u64 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    println!("1");
    fibonacci(n, &mut biggest);

}

fn fibonacci(n: u64, biggest: &mut u64) -> u64 {
    let mut y: u64;
    
    if n <= 0 {
        0
    }
    else if n == 1 {
        1
    }
    else {
        y = fibonacci(n - 1, biggest) + fibonacci(n - 2, biggest);
        if biggest < &mut y || biggest == &mut 1 && n == 2 {
            *biggest = y;
            println!("{y}");   
        }
        y
    }
}



