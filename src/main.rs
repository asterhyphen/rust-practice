//local modules
mod conditionals;
mod division;
mod loops;
mod options;
mod variables;

fn main() {
    //run is fn name in all modules almost
    let (weight, age) = variables::run();

    conditionals::run(weight);
    loops::run(age);
    options::run();

    let a = 10;
    let b = 20;

    match division::divide(a, b) {
        Some(v) => println!("Result of {}/{} = {}", a, b, v),
        None => println!("Error: Division by zero"),
    }
}
