fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");

    {
        let x = x + 200;
        println!("scope- The value of x is: {x}");
    }

    x = 100;
    println!("The value of x is: {x}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (a, b, c) = tup;
    println!("Tuple: {:?}", tup);
    println!("Tuple: ({}, {}, {})", a, b, c);

    // Array
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array: {:?}", arr);
}
