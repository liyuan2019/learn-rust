// fn main() {
//     let mut v = Vec::new();
//     v.push(5);
//     v.push(6);
//     v.push(7);
//     v.push(8);

//     let v = vec![1, 2, 3, 4, 5];
//     let does_not_exist = &v[100];
//     let does_not_exist = v.get(100);

//     let mut v = vec![100, 32, 57];
//     for i in &mut v {
//         *i += 50;
//     }

//     let row = vec![
//         SpreadsheetCell::Int(3),
//         SpreadsheetCell::Text(String::from("blue")),
//         SpreadsheetCell::Float(10.12),
//     ];
// }

// enum SpreadsheetCell {
//     Int(i32),
//     Float(f64),
//     Text(String),
// }

// fn main() {
//     let mut s1 = String::from("foo");
//     let s2 = "bar";
//     s1.push_str(s2);
//     println!("s2 is {s2}");

//     let s1 = String::from("tic");
//     let s2 = String::from("tac");
//     let s3 = String::from("toe");

//     let s = format!("{s1}-{s2}-{s3}");

//     let hello = "Здравствуйте";
//     let s = &hello[0..4];

//     for c in "Зд".chars() {
//         println!("{c}");
//     }

//     for b in "Зд".bytes() {
//         println!("{b}");
//     }
// }

use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // get 方法返回 Option<&V>，如果某个键在哈希 map 中没有对应的值，get 会返回 None。程序中通过调用 copied 方法来获取一个 Option<i32> 而不是 Option<&i32>，接着调用 unwrap_or 在 scores 中没有该键所对应的项时将其设置为零。
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // entry 函数的返回值是一个枚举 Entry 它代表了可能存在也可能不存在的值。
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}");

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}
