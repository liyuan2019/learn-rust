// fn main() {
//     // let mut x = 5;
//     // println!("The value of x is: {x}");
//     // x = 6;
//     // println!("The value of x is: {x}");

//     // let x = 5;

//     // let x = x + 1; // shadowing

//     // {
//     //     let x = x * 2;
//     //     println!("The value of x in the inner scope is: {x}");
//     // }

//     // println!("The value of x is: {x}");

//     // let x: (i32, f64, u8) = (500, 6.4, 1);

//     // let five_hundred = x.0;

//     // let six_point_four = x.1;

//     // let one = x.2;

//     // let a = [1, 2, 3, 4, 5]; // 個数固定
//     // let a = [3; 5]; // let a = [3, 3, 3, 3, 3]; 

//     // let first = a[0];
//     // let second = a[1];
//     for number in (1..4).rev() {
//         println!("{number}!");
//     }
//     println!("LIFTOFF!!!");
// }

// fn main() {
//     let s = String::from("hello");  // s 进入作用域

//     takes_ownership(s);             // s 的值移动到函数里 ...
//                                     // ... 所以到这里不再有效

//     let x = 5;                      // x 进入作用域

//     makes_copy(x);                  // x 应该移动函数里，
//                                     // 但 i32 是 Copy 的，
//     println!("{}", x);              // 所以在后面可继续使用 x

// } // 这里，x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
//   // 没有特殊之处

// fn takes_ownership(some_string: String) { // some_string 进入作用域
//     println!("{some_string}");
// } // 这里，some_string 移出作用域并调用 `drop` 方法。
//   // 占用的内存被释放

// fn makes_copy(some_integer: i32) { // some_integer 进入作用域
//     println!("{some_integer}");
// } // 这里，some_integer 移出作用域。没有特殊之处

// fn main() {
//     let s1 = String::from("hello");

//     let len = calculate_length(&s1);

//     println!("The length of '{s1}' is {len}.");
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }



// fn main() {
//     let mut s = String::from("hello"); // 可变引用（mutable reference）

//     change(&mut s);
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }



fn main() {
    let my_string = String::from("hello world");

    // `first_word` 适用于 `String`（的 slice），部分或全部
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` 也适用于 `String` 的引用，
    // 这等价于整个 `String` 的 slice
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` 适用于字符串字面值，部分或全部
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // 因为字符串字面值已经 **是** 字符串 slice 了，
    // 这也是适用的，无需 slice 语法！
    let word = first_word(my_string_literal);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
