fn f(i: i32) -> i32 {
    return i + 1;
}

fn g(i: f64) -> f64 {
    i * 2.0
}

fn hello_world() {
    println!("Hello, world!");
    let mut x: i32 = 5;
    println!("x:{x}");
    x = 6;
    println!("x:{x}");
    x = f(x);
    println!("x:{x}");

    let mut y: f64 = 1.5;
    y = g(y);
    println!("y:{y}");

    let z = 1;
    println!("z:{z}");
    {
        let z = z + 1;
        println!("z:{z}");
    }
    println!("z:{z}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let t1 = tup.0;
    let t2 = tup.1;
    let t3 = tup.2;
    println!("t1:{t1}");
    println!("t2:{t2}");
    println!("t3:{t3}");

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let b = [3; 5];
    let a2 = a[2];
    let b2 = b[2];
    println!("a2:{a2}");
    println!("b2:{b2}");

    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("result:{result}");

    let c = [10, 20, 30, 40, 50];

    for element in c {
        println!("the value is: {element}");
    }

    for number in (1..10).rev() {
        println!("{number}");
    }
}

fn say_hello(s: &str) {
    println!("{}", s);
}

fn first_word(s: &String) -> &str {
    &s[..1]
}

fn main() {
    let s = String::from("hello,world!");
    say_hello(&s);
    say_hello(&s[..]);
    say_hello(s.as_str());

    let s: String = String::from("hello, world!");
    let word = first_word(&s);
    println!("the first word is: {}", word);
}
