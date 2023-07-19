mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("hosting: add_to_waitlist");
        }
        pub fn seat_at_table() {
            // 直接调用同模块下的函数
            add_to_waitlist();
            println!("hosting: seat_at_table");
            // 使用super进入hosting的父模块，可以调用父模块及子模块内的函数
            super::handling();
            super::serving::take_order();
        }
    }
    fn handling() {
        println!("front_of_house: handling");
    }
    pub mod serving {
        pub fn take_order() {
            println!("serving: take_order");
        }
        fn server_order() {
            println!("serving: server_order");
        }
        fn take_payment() {
            println!("serving: take_payment");
        }
    }
    #[derive(Debug)]
    pub struct Breakfast {
        pub toast: String, // toast对外是公有字段
        seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast { 
                toast: String::from(toast),
                seasonal_fruit: String::from("apple"), 
            }
        }
    }
    // 使用pub在枚举上，里面所有的成员都成为公有
    pub enum Appetizer {
        Soup,
        Salad,
    }
}


// 使用use引入调用
use front_of_house::serving;
pub use front_of_house::Breakfast;
mod customer {
    use super::front_of_house::serving; // 使用super定位到父模块同地址
    pub fn eat() {
        // use只对同一个作用域有效，必须要在本模块内指明引用，不然会找不到
        serving::take_order();
    }
}
pub fn eat_at_restaurant() {
    // 以src/lib.rs为根的绝对路径
    // crate::front_of_house::hosting::add_to_waitlist();
    // 相对路径
    front_of_house::hosting::seat_at_table();
    let mut breakfast = front_of_house::Breakfast::summer("Rye");
    println!("Pre breakfast: {:?}", breakfast); 
    breakfast.toast = String::from("Wheat");
    println!("Now breakfast: {:?}", breakfast);
    let apptizer = front_of_house::Appetizer::Salad;
    // 使用user后，可以将进入的直接使用
    serving::take_order();
}

// use std::fmt::Result;
// use std::io::Result as IoResult; // 使用as对引入的类型指定一个别名

// use std::{cmp::Ordering, io}; // 使用嵌套同事引入作用域
use std::io::{self, Write}; // 使用self替代std::io的引入，另一个是引入std::io::Write

use std::collections::*; // 通过glob运算符将所有定义引入作用域，通常不这么做



use std::error::Error;
use std::fs;

pub struct IOConfig {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl IOConfig {
    pub fn build(args: &[String]) -> Result<IOConfig, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = std::env::var("IGNORE_CASE").is_ok();
        Ok(IOConfig {query, file_path, ignore_case})
    }
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}
// trait 对象 Box<dyn Error>是std::error::Error作用域中的
// Box<dyn Error> 意味着函数会返回实现了 Error trait 的类型，不过无需指定具体将会返回的值的类型。
// 这提供了在不同的错误场景可能有不同类型的错误返回值的灵活性。这也就是 dyn，它是 “动态的”（“dynamic”）的缩写
pub fn io_run(config: IOConfig) -> Result<(), Box<dyn Error>>{
    // 最后面的问号是如果遇到错误会直接返回错误到上层，不继续往下执行
    let contents = fs::read_to_string(config.file_path)?;
    println!("With text: \n {contents}");
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    for line in results {
        println!("Io search find line: {line}");
    }
    Ok(())
}

#[cfg(test)]
mod test_io {
    use super::*;

    #[test]
    fn test_search() {
        let query = "duct";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.";
        let result = search(query, contents);
        println!("io search result: {:?}", result);
    }    
    
    #[test]
    fn test_search_case_insensitive() {
        let query = "duct";
        let contents = "\
        Rust:
        safe, fast, productive.
        Duct tap.
        Pick three.";
        let result = search_case_insensitive(query, contents);
        println!("io search result: {:?}", result);
    }
}