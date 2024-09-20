fn main() {
    let add = |a: i32, b: i32| -> i32 { return a + b };
    let result = add(2, 3);
    println!("The result is {}", result);

    let mut x = 10;
    let mut print_x = |num: i32| -> i32 {
        println!("x: {}", x + num);
        x = x + num;
        x
    };

    let y = print_x(10);
    println!("y: {}", y * 12);

    let v = vec![1, 2, 3];
    let consume_v = || {
        println!("v: {:?}", v);
        // std::mem::drop(v);
    };

    consume_v();
    println!("v: {:?}", v);

    let v = vec![10, 20, 30];

    let mut iter = v.iter();
    println!("iter: {:?}", iter);

    // dbg!(&iter.next());

    // Using the next method
    assert_eq!(iter.next(), Some(&v[0]));
    println!("iter: {:?}", iter);
    assert_eq!(iter.next(), Some(&20));
    println!("iter: {:?}", iter);
    assert_eq!(iter.next(), Some(&v[2]));
    println!("iter: {:?}", iter);
    assert_eq!(iter.next(), None);
    println!("iter: {:?}", iter);

    let v = vec![1, 2, 3, 4, 5];
    let even_numbers: Vec<i32> = v.iter().filter(|&&x| x % 2 == 0).map(|x| x * 2).collect();
    println!("even_numbers: {:?}", even_numbers);
}
