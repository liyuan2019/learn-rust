// use std::env;
// use std::fs;
// use std::process;
// use std::error::Error;

// fn main() {
//     let args: Vec<String> = env::args().collect();

//     // let config = Config::new(&args);
//     let config = Config::build(&args).unwrap_or_else(|err| {
//         println!("Problem parsing arguments: {err}");
//         process::exit(1)
//     });

//     println!("Searching for {}", config.query);
//     println!("In file {}", config.file_path);

//     // run(config);
//     if let Err(e) = run(config) {
//         println!("Application error: {e}");
//         process::exit(1);
//     }

//     // --snip--
// }

// // fn run(config: Config) {
// //     let contents = fs::read_to_string(config.file_path)
// //         .expect("Should have been able to read the file");

// //     println!("With text:\n{contents}");
// // }
// fn run(config: Config) -> Result<(), Box<dyn Error>> {
//     let contents = fs::read_to_string(config.file_path)?;

//     println!("With text:\n{contents}");

//     Ok(())
// }

// // --snip--

// struct Config {
//     query: String,
//     file_path: String,
// }

// impl Config {
//     // fn new(args: &[String]) -> Config {
//     //     if args.len() < 3 {
//     //         panic!("not enough arguments");
//     //     }

//     //     let query = args[1].clone();
//     //     let file_path = args[2].clone();

//     //     Config { query, file_path }
//     // }

//     fn build(args: &[String]) -> Result<Config, &'static str> {
//         if args.len() < 3 {
//             return Err("not enough arguments");
//         }

//         let query = args[1].clone();
//         let file_path = args[2].clone();

//         Ok(Config { query, file_path })
//     }
// }

use std::env;
use std::process;

use minigrep::Config;

// fn main() {
//     // --snip--
//     let args: Vec<String> = env::args().collect();

//     let config = Config::build(&args).unwrap_or_else(|err| {
//         println!("Problem parsing arguments: {err}");
//         process::exit(1);
//     });

//     println!("Searching for {}", config.query);
//     println!("In file {}", config.file_path);

//     if let Err(e) = minigrep::run(config) {
//         // --snip--
//         println!("Application error: {e}");
//         process::exit(1);
//     }
// }

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        // 使用 eprintln! 将错误信息写入标准错误而不是标准输出
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
