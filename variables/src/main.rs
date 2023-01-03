fn main() {
    println!("[Numbers]");
    numbers();
    println!("[Basic Math]");
    math_operations();
    println!("[Boolean]");
    boolean();
    println!("[Character]");
    character();
    println!("[Compound Types]");
    compound_types();
    println!("[Arrays]");
    arrays();
    println!("[Expressions]");
    println!("Return of Function is `{}`.",expressions());
    println!("[Conditions]");
    conditions();
    println!("[Loops]");
    loops();
}

fn numbers() {
    let  x: u8 = 5;
    println!("\tThe value of x is: {x}");
    let x = x + 6;
    println!("\tThe new value of x is: {x}");

    {
        let x = x + 2;
        println!("\tThe value of x in the inner scope is: {x}");
    }

     println!("\tThe value of x in the outer scope is {x}");
     
}

fn math_operations() {
    // Addition
    let mut sum: i8 = 5;
    sum += 10;

    // Subtraction
    let mut difference: f64 = 95.5;
    difference -= 4.3;

    // multiplication
    let product: i64 = 4 * 30;

    // Division
    let quotient: f32 = 56.7 / 32.2;
    let floored: i32 = 2 / 3; // 0

    // Remainder
    let remainder: i32 = 43 % 5;

    println!("\t5 + 10 = {sum}");
    println!("\t95.5 - 4.3 = {difference}");
    println!("\t4 x 30 = {product}");
    println!("\t56.7 / 32.2 = {quotient}");
    println!("\t2 / 3 = {floored}");
    println!("\t43 % 5= {remainder}");
    
}

fn boolean() {
    let t = true;
    let f: bool = false;
    println!("\tt is {t} & f is {f}");
}

fn character() {
    let c = 'z';
    let z: char = 'Z'; // Explicit type
    let heart_eyed_cat = '😻';

    println!("\tC is {c}.");
    println!("\tZ is {z}.");
    println!("\tHeart_Eyed_Cat is {heart_eyed_cat}.");
}

fn compound_types() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("x: {x}. y: {y}. z: {z}.");
    println!("tup.x: {}. tup.y: {}. tup.z: {}.", tup.0, tup.1, tup.2);
}

fn arrays() {
    let months: [&str; 12] = [ "January"
                                ,"February"
                                ,"March"
                                ,"April"
                                ,"May"
                                ,"June"
                                ,"July"
                                ,"August"
                                ,"September"
                                ,"October"
                                ,"November"
                                ,"December"];
    let mut index: usize = 0;
    while index < months.len() {
        let string = months[index];
        println!("\t{index}. {string}");
        index += 1;
    }
}

fn expressions() -> i64 {
    let expression: i64 = {
        let temp: i64 = 5;
        temp + 6
    };

    println!("\tExpression is `{}`.", expression);

    expression * 6400 * (62 * 35)

}

fn conditions() {
    let number: i64 = if 62 < 5 { 3 } else { 9 };
    println!("\tTernary returns {number}.");
}

fn loops() {
let mut number: u8 = 0;
   println!("\t(Loop)");
let result:u8 = loop {
        println!("\t\t...{number}");
        if number == 10 { break number; };
        number += 1;
   };
    println!("Result is {result}.");

    println!("\t(While)");
    number = 10;

   while number > 0  {
    println!("\t\t{number}..");
    number -= 1;
   };
   println!("\tLIFT OFF!!");
}