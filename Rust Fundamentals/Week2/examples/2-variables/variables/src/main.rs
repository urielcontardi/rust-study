

fn main() {
    let message = "Name: Alfredo, Weight: ";

    // If you leave only 190, when calculating kilos the compiler will complain about an error: (no implementation for `{integer} / {float}`)
    let weight = 190.0;

    let kilos = weight / 2.2;
    println!("{}{}", message, weight);
    println!("{}{}", message, kilos);
    
}
