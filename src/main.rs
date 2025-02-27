// use std::io;

// fn main() {
//    let mut input = String::new();
//    io::stdin().read_line(&mut input).expect("expected to read line");

//    let int_input: i64 = input.trim().parse().unwrap();

//    println!("{}",int_input + 2);

// }

// fn main() {
//    another_function(5);
//   }
//   fn another_function(x: i32) {
//    println!("The value of x is: {x}");
//   }

// fn main() {
//    print_labeled_measurement(11, 'd');
//   }
//   fn print_labeled_measurement(value: i32, unit_label: char) {
//    println!("The measurement is: {value}{unit_label}");
//   }

// fn main() {
//    let x = pluse_one(5);
//    println!("the value of x is : {x}");
// }

// fn pluse_one(x: i32) -> i32 {
//    x + 1
// } 

// fn main() {
//    let cond = (2 as f32) <= 2.2;

//    let cond2 = false || !cond;

//    println!("{}", cond2);
// }

// fn main() {
//    let food = "bread";
   
//    if food == "cookie"{
//       println!("I like cookies too!");
//    }else if food == "fruit" {
//        println!("that sounds healthy!");
//    }else if food == "bread" {
//       println!("that sounds boring!");
//    }else {
//        println!("oh, thats too bad!");
//    }
// }

// fn main() {
//    let number = {
//       let x = 3;
//       x + 1
//    };
//    println!("{}", number);   
// }

// fn main() {
//    let result = add_numbers(3, 4);
//    println!("{}",result);  
// }

// fn add_numbers(x: i32, y: i32) -> i32{
//    let result = x + y;
//    if result > 10 {
//       return result - 10;
//    }
//    result
// }


// fn main() {
//    // Hardcoded values for the diameter (in centimeters) and price (in euros) of the pizza
//    let diameter: f64 = 30.0; // example: 30 cm
//    let price: f64 = 12.99;   // example: 12.99 euros

//    // Calculate the area of the pizza
//    let area = calculate_pizza_area(diameter);

//    // Calculate the value (area per euro)
//    let value = calculate_value_per_euro(area, price);

//    println!("Pizza Diameter: {:.2} cm", diameter);
//    println!("Pizza Price: {:.2} euros", price);
//    println!("Pizza Area: {:.2} cm^2", area);
//    println!("Value (Area per Euro): {:.2} cm^2/euro", value);
// }

// /// Calculates the area of a pizza given its diameter (in cm).
// fn calculate_pizza_area(diameter: f64) -> f64 {
//    let radius = diameter / 2.0;
//    std::f64::consts::PI * radius * radius
// }

// /// Calculates the size-per-euro value of a pizza given its area and price.
// fn calculate_value_per_euro(area: f64, price: f64) -> f64 {
//    if price > 0.0 {
//        area / price
//    } else {
//        0.0 // Handle division by zero gracefully
//    }
// }

// fn main() {
//    println!("while loop example:");
//    // first_loop();
//    // while_loop();
//    // third();
//    fourth();
// }

// fn first_loop() {
//    let mut x = 0;
//    loop {
//          x+= 1;
//          println!("x is: {}", x);

//          if x == 10 {
//             println!("we did it!");
//             break;
//          }
//    }
// }

// while loop example:

// fn while_loop() {
//    let mut x = 0;
//    while x < 10 {
//       x += 1;
//       println!("x is: {}", x);
//    }
// }

// fn third() {
//    let a = [10, 20, 30, 40, 50, 60];
//    let mut index = 0;

//    while index < 5 {
//       println!("the value is: {}", a[index]);
//       index += 1;
//    }
// }

// fn fourth() {
//    for x in 1..11 {
//       println!("x is: {}", x);
      
//    }
// }

// fn main() {
//     println!("owenership practice");
//    first();
// }

// fn first() {
//    let s1 = String::from("hello");
//    let s2 = s1.clone();

//    println!("{}....{}, world!",s1,s2);
// }

// fn main() {
//    println!("reference practice");
   
//    let mut x = 10;
//    {
//       let y = &mut x;
//       *y += 1;
//       println!("y is: {}", y);
//    } 

//    println!("x is: {}", x);

// }

// fn main() {
//    let a = [1, 2, 3, 4, 5];

//    println!("Please enter an array index.");

//    let mut index = String::new();

//    io::stdin()
//       .read_line(&mut index)
//       .expect("Failed to read line.");

//    let index: usize = index
//       .trim()
//       .parse()
//       .expect("Please input a number.");

//    let element = a[index];
//    println!("The element at index {} is: {}", index, element);
// }

// use rand::{thread_rng, seq::SliceRandom};
// #[derive(Debug)]
// struct Deck {
//     cards: Vec<String>
// }

// impl Deck {
//       fn new() -> Self {
//             // list of 'suits'- 'hearts', 'diamonds', 'clubs', 'spades'
//             // list of 'values' - '2', '3', '4', '5', '6', '7', '8', '9', '10', 'J', 'Q', 'K', 'A'
//             let suits = ["hearts", "diamonds", "clubs", "spades"];
//             let values = ["2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A"];

//             let mut cards = vec![];

//             //Double nested for loop 
//             for suit in suits{
//                   for value in values {
//                         let card = format!("{} of {}", value, suit);
//                         cards.push(card);
//                   }
//             }

//             Deck { cards }
//       }

//       fn shuffle(&mut self) {
//           let mut rng = thread_rng();
//           self.cards.shuffle(&mut rng);
//       }

//       fn deal(&mut self, num_cards: usize) -> Vec<String> {
//           self.cards.split_off(self.cards.len() - num_cards)
//       }
// }

// fn main() {
//       let mut deck = Deck::new();

//       // deck.shuffle();
//       let cards = deck.deal(3);

//       println!("here is your hand: {:#?}", cards);
//       println!("here is your deck: {:#?}", deck);
// }


// // Define a constant for the age
// const AGE: i32 = 35;

// fn student_age_check(age: i32) {
//     if age >= 18 && age <= 30 {
//         println!("welcome");
//     } else if age < 18 {
//         println!("sorry, no kids");
//     } else if age >= 31 {
//         println!("sorry, go home to grandma");
//     }
// }

// fn main() {
//     // Call the function with the constant age
//     student_age_check(AGE);
// }


// fn main() {
//     let mut count = 0;
//     let mut a = 0;
//     let mut b = 1;

//     println!("The first 15 Fibonacci numbers are:");

//     loop {
//         if count >= 15 {
//             break;
//         }

//         // Print the current Fibonacci number
//         println!("{}", a);

//         // Compute the next Fibonacci number
//         let next = a + b;
//         a = b;
//         b = next;

//         // Increment the counter
//         count += 1;
//     }
// }


// fn main() {
//     let mut number = 1337;
//     while number != 0 {
//     println!("{number}!");
//     number -= 1;
//     }
//     println!("Oh my go...ssip we just hit the zero!!!");
//    }


// fn main() {
//     // let count = 0;
//     let mut a = 0;
//     let mut b = 1;

//     println!("The first 15 Fibonacci numbers are:");

//     for _ in 0..14 {
//         // Print the current Fibonacci number
//         println!("{}", a);

//         // Compute the next Fibonacci number
//         let next = a + b;
//         a = b;
//         b = next;
//     }
// }


// fn main() {
//     let a = [10, 20, 30, 40, 50];
//     for element in a {
//     println!("the value is: {element}");
//     }
//    }


// fn main() {
//     let mut count = 0;
//     let mut a = 0;
//     let mut b = 1;

//     println!("The first 15 Fibonacci numbers are:");

//     while count < 15 {
//         // Print the current Fibonacci number
//         println!("{}", a);

//         // Compute the next Fibonacci number
//         let next = a + b;
//         a = b;
//         b = next;

//         // Increment the counter
//         count += 1;
//     }
// }


// fn main() {
//     let mut count = 0;
//     let mut a = 0;
//     let mut b = 1;

//     println!("The first 15 Fibonacci numbers are:");

//     loop {
//         // Print the current Fibonacci number
//         println!("{}", a);

//         // Compute the next Fibonacci number
//         let next = a + b;
//         a = b;
//         b = next;

//         // Increment the counter
//         count += 1;

//         // Check the condition at the end, breaking if count is 15 or more
//         if count >= 15 {
//             break;
//         }
//     }
// }


// // Function to compute the BMI from weight (in kilograms) and height (in meters)
// fn calculate_bmi(weight: f32, height: f32) -> f32 {
//     weight / (height * height)
// }

// // Function to interpret the BMI and print the explanation
// fn judge_bmi(bmi: f32) {
//     println!("BMI: {}", bmi);
//     if bmi < 18.5 {
//         println!("Underweight");
//     } else if bmi < 25.0 {
//         println!("Normal weight");
//     } else if bmi < 30.0 {
//         println!("Overweight");
//     } else {
//         println!("Obesity");
//     }
// }

// // Main function that chains the above functions without intermediate steps
// fn main() {
//     let weight = 70.0; // Example weight in kilograms
//     let height = 1.75; // Example height in meters

//     // Chain the function calls
//     judge_bmi(calculate_bmi(weight, height));
// }


// // Function to compute the BMI from weight (in kilograms) and height (in meters)
// fn calculate_bmi(weight: f32, height: f32) -> f32 {
//     weight / (height * height)
// }

// // Function to interpret the BMI and print the explanation
// fn judge_bmi(bmi: f32) {
//     println!("BMI: {}", bmi);
//     if bmi < 18.5 {
//         println!("Underweight");
//     } else if bmi < 25.0 {
//         println!("Normal weight");
//     } else if bmi < 30.0 {
//         println!("Overweight");
//     } else {
//         println!("Obesity");
//     }
// }

// // New function to validate BMI against a given category
// fn validate_bmi(category: &str, bmi: f32) -> bool {
//     match category {
//         "underweight" => bmi < 18.5,
//         "normal weight" => bmi >= 18.5 && bmi < 25.0,
//         "overweight" => bmi >= 25.0 && bmi < 30.0,
//         "obesity" => bmi >= 30.0,
//         _ => false, // If an unknown category is passed
//     }
// }

// // Main function that chains the above functions without intermediate steps
// fn main() {
//     let weight = 70.0; // Example weight in kilograms
//     let height = 1.75; // Example height in meters

//     // Calculate BMI and judge it
//     let bmi = calculate_bmi(weight, height);
//     judge_bmi(bmi);

//     // Example validations
//     println!("Validation for Normal weight and BMI 27: {}", validate_bmi("normal weight", 27.0));
//     println!("Validation for Normal weight and BMI 41: {}", validate_bmi("normal weight", 41.0));
// }


// fn main() {
//     struct Customer {
//     id: u16,
//     name: String,
//     city: String,
//     }
//     let customer1 = Customer {
//     id: 1337,
//     name: String::from("Peter Progger"),
//     city: String::from("Codehausen"),
//     };
//     println!("{}", customer1.id,);
//     println!("{}", customer1.name,);
    
//    }


// // Define a struct to represent a car
// struct Car {
//     make: String,
//     model: String,
//     year_of_manufacturing: u32,
// }

// fn main() {
//     // Instantiate the first car
//     let car1 = Car {
//         make: "Toyota".to_string(),
//         model: "Corolla".to_string(),
//         year_of_manufacturing: 2010,
//     };

//     // Instantiate the second car
//     let mut car2 = Car {
//         make: "Ford".to_string(),
//         model: "Mustang".to_string(),
//         year_of_manufacturing: 2020,
//     };

//     // Print details of the cars before modification
//     println!("Car 1: {} {} {}", car1.make, car1.model, car1.year_of_manufacturing);
//     println!("Car 2: {} {} {}", car2.make, car2.model, car2.year_of_manufacturing); 

//     // Increment the year of manufacturing for the second car
//     car2.year_of_manufacturing += 10;

//     // Print details of the second car after modification
//     println!("Updated Car 2: {} {} {}", car2.make, car2.model, car2.year_of_manufacturing);
// }


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
