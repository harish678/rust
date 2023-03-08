use std::io;

fn main() {
    temperature();
    fibonacci();
}

fn fib(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    fib(n - 1) + fib(n - 2)
}

fn fibonacci() {
    println!("Input n:");
    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    let n: i32 = match n.trim().parse::<i32>() {
        Ok(num) => num,
        Err(_) => {
            println!("Not a number");
            0
        }
    };

    let output = fib(n);
    println!("Nth fibonacci: {output}")
}

fn temperature() {
    println!("Input the temperature:");
    let mut value = String::new();

    io::stdin()
        .read_line(&mut value)
        .expect("Failed to read line");

    let value: i32 = match value.trim().parse::<i32>() {
        Ok(num) => num,
        Err(_) => {
            println!("Not a number");
            0
        }
    };

    let mut unit = String::new();

    io::stdin()
        .read_line(&mut unit)
        .expect("Failed to read line");

    let unit: char = match unit.trim().parse() {
        Ok(ch) => ch,
        Err(_) => {
            println!("Not a character");
            'F'
        }
    };

    if unit == 'F' {
        let output = (value as f32 - 32.0) * 0.5556;
        println!("{value} F to C: {output}");
    } else if unit == 'C' {
        let output = (value as f32 * 1.8) + 32.0;
        println!("{value} C to F: {output}");
    }
}
