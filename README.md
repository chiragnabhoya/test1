# Rust Car Struct Example

This is a simple Rust program that demonstrates the use of structs in Rust by modeling a car.

## Description

The program defines a `Car` struct with three fields:
- `make`: A `String` representing the car's manufacturer.
- `model`: A `String` representing the car's model.
- `year`: A `u32` representing the year of manufacture.

Two instances of the `Car` struct are created:
1. `car1`: A Toyota Corolla (2015 model)
2. `car2`: A Ford Mustang (2020 model)

The program prints the details of `car2`, then increments its manufacturing year by 10 years and prints the updated details.

## Code

```rust
struct Car {
    make: String,
    model: String,
    year: u32,
}

fn main() {
    // Instantiate two cars with sensible values
    let car1 = Car {
        make: String::from("Toyota"),
        model: String::from("Corolla"),
        year: 2015,
    };
    
    let mut car2 = Car {
        make: String::from("Ford"),
        model: String::from("Mustang"),
        year: 2020,
    };
    
    // Print original details of car2
    println!("Before update: {} {} ({})", car2.make, car2.model, car2.year);
    
    // Increment the year of manufacturing by 10 years for car2
    car2.year += 10;
    
    // Print updated details of car2
    println!("After update: {} {} ({})", car2.make, car2.model, car2.year);
}
```

## How to Run

Ensure you have Rust installed. If not, install it from [rust-lang.org](https://www.rust-lang.org/).

1. Save the Rust code to a file, e.g., `main.rs`.
2. Open a terminal and navigate to the directory containing `main.rs`.
3. Compile and run the program using:
   ```sh
   rustc main.rs
   ./main
   ```

## Expected Output

```
Before update: Ford Mustang (2020)
After update: Ford Mustang (2030)
```

## License
This project is open-source and free to use.
