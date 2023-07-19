pub mod garden; // 告诉编译器，src目录下的garden.rs文件应该被包括
use crate::{garden::vegetables::Asparagus, lib::eat_at_restaurant}; // 使用Asparagus
pub mod lib;

use std::{io::{self, Read, Write}, net::{Ipv4Addr, Ipv6Addr}};
use rand::Rng;
use std::cmp::Ordering;
use std::fs;
use std::env;

fn test_rust() {
    println!("Hello, world!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // 循环
    loop {
        println!("Please input your guess:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");
        // guess = "10".to_owned();
        // println!("You guessed: {guess}");
        // 使用隐藏之前的值来进行类型转换，同名不需要定义新的变量
        // trim 会消除输入后的回车(\n 或者 \r\n)
        // let guess: i32 = guess.trim().parse().expect("Please input a number!");
        // 处理无效输入
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number!");
                continue;
            },
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                // 有break循环后面的代码才有效
                break;
            },
        }
    }
    println!("Secret number is: {secret_number}");
}

fn test_variable()
{
    println!("=== test_variable ===");
    // 常量要全部大写加下划线，编译会自己运算，这么写有助理解
    const THREE_HOURS_IN_SECONDS : u32 = 60 * 60 * 3;
    let x = 5;
    // 隐藏前一个x，使用同名定义一个新变量
    let x = x + 1;
    {
        // 作用域内新的变量x，作用域结束后不再对作用域外的x进行隐藏
        let x = x * 2;
        println!("inner x: {x}");
    }
    println!("out x: {x}");
    // 隐藏可以重新改变类型，但是mut不行
    let x = "";
    // let mut y = 1;
    // y = ""; // 编译错误
}

fn test_data_type()
{
    println!("=== test_data_type ===");
    // i32 n:32，-2(n-1)次方 到 2(n-1)次方 -1范围
    // u32 n:32，0 到 2(n)次方-1范围
    let i32_max = i32::MAX;
    println!("i32_max: {i32_max}");
    // isize uisize按照系统架构来定，64就是i64，32就是i32
    let arch_max = isize::MAX;
    let i64_max = i64::MAX;
    let i32_max = i32::MAX;
    println!("arch_max: {arch_max}, i64_max: {i64_max}, i32_max: {i32_max}");
    let t1 = 57u8;
    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let bytes = b'A';
    println!("t1: {t1}, decimal: {decimal}, octal: {octal}, binary: {binary}, bytes: {bytes}");
    let float_x = 2.0; // 默认f64 双精度浮点数
    let float_x2: f32 = 3.0; // 指定f32 单精度浮点数
    let bool_x = true;
    let bool_x2: bool = false;
    let char_x = '1';
    let char_x2: char = '0';
    let tup: (i32, f64, u8) = (500, 6.4, 1); // 指定类型
    let tup2 = (500, 6.4, 1); // 自动推断类型
    let (tup_x, tup_y, tup_z) = tup2; // 进行绑定
    println!("tup_x: {tup_x}, tup_y: {tup_y}, tup_z: {tup_z}");
    let tup_x = tup2.0; // 绑定第一个
    let tup_y = tup2.1; // 绑定第二个
    let tup_z = tup2.2; // 绑定第三个
    println!("tup_x: {tup_x}, tup_y: {tup_y}, tup_z: {tup_z}");
    let array = [1, 2, 3, 4, 5];
    let array2: [i32; 5] = [1, 2, 3, 4, 5];// 指定类型和数组大小
    let array3 = [3; 5]; // 指定元素个数为5，每个的值都是3
    let arr_index_2 = array[2];
    println!("arr_index_2: {arr_index_2}");
}

fn test_func(x: i32, y: char) 
{
    println!("=== test_func ===");
    println!("x: {x}, y: {y}");
}

fn test_exp()
{
    println!("=== test_exp ===");
    // let x = (let y = 6); // 不支持此写法，let y = 6没有返回值
    let x= {
        let y = 3;
        y + 1 // 最后一句就是返回值，不加分号结束
    };
    println!("x value: {x}");
}

fn test_return(x: i32) -> i32 {
    x + 2
}

fn test_control()
{
    println!("=== test_control ===");
    let number = 3;
    if number < 5 {
        println!("Number less than 5");
    } else if number == 5 {
        println!("Number equals 5");
    } else {
        println!("Number large than 5");
    }
    let number2 = if number > 5 { 1 } else { 0 };
    println!("number2: {number2}");

    let mut loop_counter = 0;
    let loop_ret = loop {
        loop_counter += 1;
        if loop_counter >= 10 {
            break loop_counter * 2; // 跳出循环并返回值
        }
    };
    println!("loop_ret: {loop_ret}");
    loop_counter = 0;
    'counting_up: loop { // 'counting_up作为循环的标签
        println!("loop_counter: {loop_counter}");
        let mut remaining = 10;
        loop {
            println!("remaining: {remaining}");
            if remaining == 9 {
                break; // 这里只是结束最内层的循环
            }
            if loop_counter == 2 {
                break 'counting_up; // 这里通过标签结束外层的循环
            }
            remaining -= 1;
        }
        loop_counter += 1;
    }
    println!("end loop_counter: {loop_counter}");
    loop_counter = 3;
    while loop_counter != 0 {
        println!("while counter: {loop_counter}");
        loop_counter -= 1;
    }
    let array = [1, 2, 4, 6];
    let mut index = 0;
    while index < array.len() {
        println!("while index: {}, value: {}", index, array[index]);
        index += 1;
    }
    for element in array {
        println!("for value: {element}");
    }
    for element in (1..4).rev() { // 3 2 1，不包含4
        println!("for value2: {element}");
    }
}

fn test_copy() {
    println!("=== test_copy ===");
    let x = 5;
    let y = x; // x在栈上被快速拷贝，没有需要被移动到y，所以依然可使用
    println!("x: {x}, y: {y}");
    let s1 = String::from("Hi");
    let s2 = s1;
    // println!("s1: {s1}"); // s1已经被移动到s2，不再有效，所以不能被使用
    let s3 = s2.clone(); // s2克隆到s3，s2仍然有效
    println!("s2: {s2}, s3: {s3}");

    // copy类型
    // 所有整数类型、bool、所有浮点数类型，字符类型，元组（必须要包含的类型都是copy类型才可以，如果有类似String就不行）
}

// s借用了传参的变量，并不拥有变量，所有这里不能修改借用后的s变量的值
fn get_ref_string_length(s: &String) -> usize {
    // s.push_str("123"); // 修改值不被允许
    s.len()
}

// 可变的引用
fn change_mutable_ref_string(s: &mut String) {
    s.push_str("qwertyuiop");
}
// fn dangle() -> &String { // dangle 返回一个字符串的引用
//     let s = String::from("hello"); // s 是一个新字符串
//     &s // 返回字符串 s 的引用
// } // 这里 s 离开作用域并被丢弃。其内存被释放。
//   // 危险！

// 这里的解决方法是直接返回 String，所有权被移动出去，所以没有值被释放
// fn no_dangle() -> String {
//     let s = String::from("hello");
//     s
// }

fn test_reference() {
    println!("=== test_reference ===");
    let s1 = String::from("我是谁");
    let len1 = get_ref_string_length(&s1);
    println!("s1: {s1}, len1: {len1}");
    let mut s2 = String::from("Hi, ");
    change_mutable_ref_string(&mut s2);
    println!("s2: {s2}, len2: {}", s2.len());

    let mut s3 = String::from("hello");
    // let r1 = &mut s3;
    // let r2 = &mut s3; // 不能同时将同一个可变变量同时引用给两个变量借用
    // println!("{}, {}", r1, r2);

    // let mut s = String::from("hello");
    // let r1 = &s; // 没问题
    // let r2 = &s; // 没问题 多个不可变引用是可以的
    // let r3 = &mut s; // 大问题 不能拥有不可变引用的同时拥有可变引用
    // println!("{}, {}, and {}", r1, r2, r3);
}

fn test_slice_1(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// 2比1通用，可以传String或者str引用
fn test_slice_2(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn test_slice() {
    println!("=== test_slice ===");
    let s = String::from("Hel lo");
    let len = s.len();
    let slice1 = &s[0..2];
    let slice2 = &s[..2]; // 从0开始可以不写0
    let slice3 = &s[3..len];
    let slice4 = &s[3..]; // 取到末尾可以不写最后的值
    let slice5 = &s[0..len]; // 取所有
    let slice6 = &s[..]; //取所有可以前后都不写值
    println!("slice1: {slice1}, slice2: {slice2}, slice3: {slice3}, slice4: {slice4}, slice5: {slice5}, slice6: {slice6}");
    let slice7 = test_slice_1(&s);
    println!("slice7: {slice7}");
    // s.clear(); // Rust 不允许 clear 中的可变引用和 word 中的不可变引用同时存在
    let slice8 = test_slice_2(&s[..]);
    let slice9 = test_slice_2(&s[2..]);
    println!("slice8: {slice8}, slice9: {slice9}");
    let arr1 = [1, 2, 3, 4, 5];
    let arr2 = &arr1[0..3];
    println!("arr1: {:?}, arr2: {:?}", arr1, arr2);
}
#[derive(Debug)]
struct TestRectangle{
    width: u32,
    height: u32
}

impl TestRectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &TestRectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    // 关联函数，可以理解为跟C++一样的静态函数并且返回一个实例
    fn square(size: u32) -> Self {
        Self { width: (size), height: (size) }
    }
}

fn test_struct() {
    println!("=== test_struct ===");
    #[derive(Debug)]
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }
    let user = User {
        active: true,
        username: String::from("test"),
        email: String::from("123@mail.com"),
        sign_in_count: 1,
    };
    println!("username: {}, email: {}", user.username, user.email);
    // 可变实例
    let mut user2 = User {
        active: true,
        username: String::from("test"),
        email: String::from("123@mail.com"),
        sign_in_count: 1,
    };
    user2.email = String::from("456@google.com");
    println!("username2: {}, email2: {}", user2.username, user2.email);
    fn build_user(email: String, name: String) -> User {
        User { 
            active: (true),
            username: (name), 
            email: (email), 
            sign_in_count: (1) }
    }
    fn build_user2(email: String, name: String) -> User {
        User { 
            active: (true),
            username: (name), 
            email, // 参数名与字段名完全一直可以直接使用，不需要再赋值
            sign_in_count: (1) }
    }
    let mut user3 = User {
        active: true,
        username: user2.username, // 可以使用其他实例的值来赋值
        email: String::from("123@mail.com"),
        sign_in_count: 1,
    };
    let mut user4 = User {
        active: true,
        sign_in_count: 2,
        ..user3 // 使用user3在赋值剩下的字段, 必须放在最后一句
    };
    // println!("user2: {:?}", user2); // user2已经不能再使用，部分字段已经被转移
    println!("user2 email: {}", user2.email); // user2的未转移的字段仍然可使用
    // println!("user3: {:?}", user3); // user3已经不能再使用，部分字段已经被转移
    println!("user4: {:?}", user4);
    // :#?可以按照类定义的格式打印出来对应的字段和值
    println!("user4 2: {:#?}", user4);

    // 元组结构体，没有字段名只有字段类型
    #[derive(Debug)]
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    // 虽然类型字段类型都相同，但以下两个实例不是相等的
    let black = Color(0, 0, 0);
    let point = Point(0, 0, 0);
    // // 可以跟元组一样通过解构获取值 // 文档写可以，实测不行
    // let (red, green, blue) = point;
    // println!("red: {red}, green: {green}, blue: {blue}");
    // 可以通过索引获取值
    println!("x: {}, y: {}, z: {}", point.0, point.1, point.2);

    // 类单元结构体， 没有任何字段
    struct AlwaysEqual;
    let subject = AlwaysEqual;
    // dbg!返回所有权，所以不影响实际写法，并且可以顺便打印宏定义调用里面的值
    let test_dbg = Color(100, dbg!(10 * 8), 255);
    // dbg!需要传的是引用
    dbg!(&test_dbg);
    let test_rectangle = TestRectangle{width: 1, height: 1};
    let test_rectangle2 = TestRectangle{width: 2, height: 10};
    let test_rectangle3 = TestRectangle::square(20);
    println!("rectangle are: {}, can hold: {}", test_rectangle2.area(), test_rectangle2.can_hold(&test_rectangle));
}

fn test_enum() {
    println!("=== test_enum ===");
    enum IpAddrKind {
        V4,
        V6
    }
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1")
    };
    // 用枚举直接替代结构体
    enum IpAddr2 {
        V4(String),
        V6(String)
    }
    let home = IpAddr2::V4(String::from("127.0.0.1"));
    let home2 = IpAddr2::V6(String::from("::1"));
    #[derive(Debug)]
    enum IpAddr3 {
        V4(u8, u8, u8, u8),
        V6(String)
    }
    let home = IpAddr3::V4(127, 0, 0, 1);

    //使用系统标准库 Ipv4Addr Ipv6Addr
    #[derive(Debug)]
    enum IpAddr4 {
        V4(Ipv4Addr),
        V6(Ipv6Addr)
    }
    let home = IpAddr4::V4(Ipv4Addr::new(1, 6, 8, 9));
    let home2 = IpAddr4::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0xfff, 0x11));
    // 可以给枚举定义实现方法
    impl IpAddr4 {
        fn print_address(&self) {
            println!("address: {:?}", self);
        }
    }
    home.print_address();
    home2.print_address();

    // option 枚举
    enum Options<T> {
        None,
        Some(T)
    }
    let some_number = Some(5);
    let some_char = Some('1');
    // 必须要指定Option<type>类型
    let none_number:Option<i32> = None;
    let number: i32 = 6;
    // 不能操作相加，两种是不同的类型
    // let sum = some_number + number;
    let sum = some_number.unwrap() + number;
    // 匹配并返回option
    let opt1 = match some_number {
        Some(i) => Some(i + 1),
        None => None,
    };
    println!("opt1: {:?}", opt1);

    #[derive(Debug, Clone, Copy)]
    enum UsState {
        Alabama,
        Alaska
    }
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }
    let coin = Coin::Quarter(UsState::Alaska);
    let coin_value = match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Quarter state: {:?}", state);
            25
        },
    };
    println!("coin value: {}", coin_value);
    // if let 是只处理匹配一个模式，可以不像match那样穷尽，另外这里括号中的state需要支持copy才可以
    if let Coin::Quarter(state) = coin {
        println!("Quarter state2: {:?}", state);
    } else {
        println!("Coin unmatch quarter.");
    }

    let dice_roll = 0;
    match dice_roll {
        3 => println!("Value is 3"),
        _ => println!("Unknow value") // 使用_对应没有匹配的值
    }
}

fn test_vector() {
    println!("=== test_vector ===");
    // 创建包含初始值的vector
    let v2 = vec![1, 2, 3];
    // 创建一个空的vector, 没有放数据所以需要特别指定类型
    let mut v: Vec<i32> = Vec::new();
    // 增加元素
    v.push(1);
    v.push(256);
    v.push(1000);
    // 获取元素，如果超过索引，这个获取方式会发生panic
    let first: &i32 = &v[0];
    // 可以超过元素获取，会返回None
    let third: Option<&i32> = v.get(2);
    let invalid: Option<&i32> = v.get(20);
    // 不允许借用后在使用前增加数据，这会使得借用的引用指向被释放的内存
    // v.push(2);
    println!("Get value1: {first}, value3: {:?}, invalid value: {:?}", third, invalid);
    // 遍历元素
    for i in &v {
        println!("value: {i}");
    }
    // 遍历引用使得可以改变值
    for i in &mut v {
        // 先用*i 获取i的值，再用*i去赋值
        *i += 1;
    }
    for i in &v {
        println!("value2: {i}");
    }
    // 枚举存储多种类型
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(3.14),
        SpreadsheetCell::Text(String::from("Hello")),
    ];

    //字符串跟vec类似，vec<u8>的封装
    let mut str = String::new();
    let data = "Hello word";
    // 以下三种都可以赋值String
    let s = data.to_string();
    let s = "fjdlsajl".to_string();
    let s = String::from("fdsaf");
    // 附加字符串
    str.push_str("123");
    str.push('a');
    str.push_str(data);

    let s1 = String::from("Hello");
    let s2 = String::from(" world!");
    let s3 = s1 + &s2;
    // println!("s1: {s1}"); // s1被借用不能再使用
    println!("s3: {s3}");
    // 拼接多个字符串最高效的方法
    let s4 = format!("{str}-{s2}-{s3}");
    println!("s4: {s4}");
    // let sIndex = s4[0]; // 字符串不支持索引
    let hello = "Здравствуйте";
    let s = &hello[0..4]; // hello的前两位字符，每位字符占两个字节
    println!("s: {s}"); //
    // 运行时触发错误，因为utf-8存储的hello，第一个字符占了2个字节，这里只索引第一个rust会认为无效，运行时报错
    // let ss = &hello[0..1];
    // println!("ss: {ss}");
    // 这里调用chars会返回各自的字符
    for c in hello.chars() {
        println!("hello char: {c}");
    }
    // 打印各个字节的值，数量是utf-8字符数量的2倍
    for b in hello.bytes() {
        println!("hello byte: {b}");
    }
}

use std::collections::HashMap;
fn test_hashmap() {
    println!("=== test_hashmap ===");
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("fsfas"), 23);
    scores.insert(String::from("3453cdf"), 2);
    println!("scores: {:?}", scores);
    // 覆盖一个值
    scores.insert(String::from("fsfas"), 203);
    println!("scores2: {:?}", scores);
    // 不存在就插入，否则不做操作
    scores.entry(String::from("123")).or_insert(10);
    scores.entry(String::from("fsfas")).or_insert(11);
    println!("scores3: {:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    // 根据空格拆分字符串
    for word in text.split_whitespace() {
        // 存在就插入0，否则不做操作，or_insert方法返回这个键的值的一个可变引用
        let count = map.entry(word).or_insert(0);
        // 根据旧值进行更新新值，这里我们将这个可变引用储存在 count 变量中，所以为了赋值必须首先使用星号（*）解引用 count
        *count += 1;
    }
    println!("{:?}", map);

}

fn test_error() {
    println!("=== test_error ===");
    // 测试panic!
    // let v = vec![1, 2, 3];
    // v[99];
    let dir = match env::current_dir() {
        Ok(dir) => dir,
        Err(err) => panic!("Get current dir failed, err: {:?}", err)
    };
    println!("Current dir: {:?}", dir);
    let file = fs::File::open(dir.join("Cargo.toml"));
    let mut result = match file {
        Ok(file) => file,
        Err(err) => match err.kind() {
            io::ErrorKind::NotFound => match fs::File::create("test.txt") {
                Ok(mut fc) => {
                    // 如果result不是OK，则触发panic!
                    fc.write_all("Hello test.txt".as_bytes()).unwrap();
                    fc
                },
                Err(err) => panic!("Problem create test.txt file, err: {:?}", err)
            },
            other_error => {
                panic!("Problem read file, err: {:?}", other_error);
            }
        }
    };
    let mut str = String::new();
    // expect如果不是ok，则会触发panic!同时打印msg的提示
    result.read_to_string(&mut str).expect("Read file to string content failed");
    println!("Read file content: {str}");
    test_throw_err();
}

fn test_throw_err() -> Result<String, io::Error> {
    let mut file = match fs::File::open("1.txt") {
        Ok(f) => f,
        Err(err) => return Err(err), // 直接return返回错误
    };
    let mut content = String::new();
    // match file.read_to_string(&mut content) {
    //     Ok(_) => Ok(content), // 返回内容
    //     Err(e) => Err(e), // 返回错误
    // }
    // ?运算符跟match的实现类似，简化
    // 如果 Result 的值是 Ok，这个表达式将会返回 Ok 中的值而程序将继续执行。如果值是 Err，Err 中的值将作为整个函数的返回值
    // 跟match不同的是，如果发生了错误，?运算符会使整个函数提前返回并将任何 Err 值返回给调用代码
    file.read_to_string(&mut content)?;
    Ok(content)
}

// 使用?运算符简化test_throw_err的实现
fn test_throw_err2() -> Result<String, io::Error> {
    let mut content = String::new();
    fs::File::open("1.txt")?.read_to_string(&mut content)?;
    Ok(content)
}

// 使用fs简化test_throw_err2实现
fn test_throw_err3() -> Result<String, io::Error> {
    fs::read_to_string("1.txt")
}
fn last_char_of_first_line(text: &str) -> Option<char> {
    // lines拆分字符串的每一行为迭代器，next获取第一行，如果获取到，调用chars分解为字符，调用last获取最后一个字符
    text.lines().next()?.chars().last()
}

// 因为T可以是很多类型，函数中判断大于号的写法rust不知道该怎么去判断类型支不支持，
// 所以限定T为继承std::cmp::PartialOrd则可以判断，因为调用的i32和char都是继承了PartialOrd的实现
fn find_largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    return largest;
}

// 可以指定两种不同的类型
struct GenericPoint<T, U> {
    x: T,
    y: U,
}

// 必须在 impl 后面声明 T，这样就可以在 GenericPoint<T> 上实现的方法中使用 T 了
impl<T, U> GenericPoint<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &U {
        &self.y
    }
}

// 也可以限定具体的类型
impl GenericPoint<f32, f32> {
    fn get_distance(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<X1, Y1> GenericPoint<X1, Y1> {
    // 限定不同的类型
    fn mixup<X2, Y2>(self, other: GenericPoint<X2, Y2>) -> GenericPoint<X1, Y2> {
        GenericPoint { x: self.x, y: other.y }
    }
}

// enum泛型
enum GenericOption<T> {
    Some(T),
    None,
}

// 测试泛型
fn test_generic() {
    println!("=== test_generic ===");
    let number_list = vec![34, 50, 25, 100, 65];
    let result = find_largest(&number_list);
    println!("The largest number is {}", result);
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = find_largest(&char_list);
    println!("The largest char is {}", result);
    let both_integer = GenericPoint { x: 5, y: 10 };
    println!("both_integer x: {}, y: {}", both_integer.x(), both_integer.y());
    let both_float = GenericPoint { x: 1.0, y: 4.0 };
    println!("both_float x: {}, y: {}", both_float.x(), both_float.y());
    // 只有两个都是float才可以调get_distance函数
    println!("both_float distance from zero: {}", both_float.get_distance());
    let integer_and_float = GenericPoint { x: 5, y: 4.0 };
    let mixup = both_integer.mixup(integer_and_float);
    println!("mixup x: {}, y: {}", mixup.x(), mixup.y());
    let opt1 = GenericOption::Some(5.0);
    let opt2 = GenericOption::Some(5);

}

pub trait Summary {
    fn summarize(&self) -> String;
    fn default_impl(&self) -> String {
        String::from("Default impl of Summary")
    }
}

pub trait Displays {
    fn impls(&self) -> String {
        String::from("Display impl")
    } 
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn default_impl(&self) -> String {
        format!("{}: {}, self impl", self.username, self.content)
    }
}

// trait 做为参数
fn summary_notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn summary_notify2<T: Summary>(item: &T) {
    println!("Breaking news2! {}", item.summarize());
}

fn summary_notify3<T: Summary>(item: &T, item2: &T) {
    println!("Breaking news3! {}, {}", item.summarize(), item2.default_impl());
}

pub struct SDImpl {
    pub name: String,
}
impl Summary for SDImpl {
    fn summarize(&self) -> String {
        format!("SDImpl summary")
    }
}
impl Displays for SDImpl {
    
}

// 用+号指定两个trait bound
fn summay_and_display<T: Summary + Displays> (item: &T) {
    println!("summay_and_display: summary: {}, display: {}", item.summarize(), item.impls());
}
// 如果bound过多，可以使用where在后面声明范围
fn summay_and_display2<T> (item: &T)
where T: Summary + Displays
{
    println!("summay_and_display2: summary: {}, display: {}", item.summarize(), item.impls());
}

// 返回trait
fn return_summary_trait() -> impl Summary {
    Tweet {
        username: String::from("Summary return"),
        content: String::from("Return summary content."),
        reply: false,
        retweet: false,
    }
}

use std::fmt::Display;
struct PairTrait<T> {
    x: T,
    y: T,
}

impl <T> PairTrait<T> {
    fn news(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// 实现有条件的方法（带显示和排序的trait）
impl <T: Display + PartialOrd> PairTrait<T> {
    fn com_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn test_trait() {
    println!("=== test_trait ===");
    let tweet = Tweet {
        username: String::from("Hi"),
        content: String::from("Tweet content."),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}, default: {}", tweet.summarize(), tweet.default_impl());
    summary_notify(&tweet);
    summary_notify2(&tweet);
    summary_notify3(&tweet, &tweet);
    let sdi = SDImpl {
        name: String::from("sdimpl yyy"),
    };
    summay_and_display(&sdi);
    summay_and_display2(&sdi);
    let pair = PairTrait::news(1.4, 8.0);
    pair.com_display();
}

// 返回的引用字符声明周期跟x或者y相同
fn longest_lifetime<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 指定返回的字符声明周期跟x一样长，此时不能返回y
fn longest_lifetime2<'a>(x: &'a str, y: & str) -> &'a str {
    x
}

// StructLifetime的声明周期不能比part引用的字符的生命周期存在的更久
struct StructLifetime<'a> {
    part: &'a str,
}

// 声明周期省略
// Rust 团队发现在特定情况下 Rust 程序员们总是重复地编写一模一样的生命周期注解。这些场景是可预测的并且遵循几个明确的模式。
// 接着 Rust 团队就把这些模式编码进了 Rust 编译器中，如此借用检查器在这些情况下就能推断出生命周期而不再强制程序员显式的增加注解。
fn first_word111(s: &str) -> &str {
    &s
}
fn first_word222<'a>(s: &'a str) -> &'a str {
    &s
}

impl<'a> StructLifetime<'a> {
    fn level(&self) -> i32 {
        3
    }
    // 有两个输入生命周期，所以 Rust 应用第一条生命周期省略规则并给予 &self 和 announcement 他们各自的生命周期。
    // 接着，因为其中一个参数是 &self，返回值类型被赋予了 &self 的生命周期，这样所有的生命周期都被计算出来了。
    fn announce(&self, announcement: &str) -> &str {
        self.part
    }
}

// 生命周期也是泛型，所以生命周期参数 'a 和泛型类型参数 T 都位于函数名后的同一尖括号列表中
fn longest_with_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T
) -> &'a str
where T: Display
{
    println!("longest_with_announcement: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn test_lifetime() {
    println!("=== test_lifetime ===");
    // &i32        // 引用
    // &'a i32     // 带有显式生命周期的引用
    // &'a mut i32 // 带有显式生命周期的可变引用
    let x = String::from("123456");
    let y = String::from("abcde");
    let longest = longest_lifetime(&x, &y);
    println!("longest: {longest}");
    let struct_lifetime = StructLifetime {part: &y};
    println!("struct_lifetime: {}", struct_lifetime.part);
    // 静态声明周期，其生命周期能够存活于整个程序期间
    let s_str: &'static str = "Hello static lifetime.";
    let longest = longest_with_announcement(&x, &y, s_str);
    println!("longest_with_announcement: {longest}");
}

// 这些代码在正常编译运行的时候不会跑到
// 默认所有测试都是多线程并行运行， 需要每次运行一个执行cargo test -- --test-threads=1
// 测试可以调用私有函数
#[cfg(test)]
mod auto_test {
    // 全局导入当前模块之外的所有内容
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    // should_panic 预测本条测试应该是产生panic!才会测试通过
    // 否则会提示test did not panic as expected
    #[test]
    #[should_panic]
    fn another() {
        println!("=======Msg: Test with panic");
        panic!("Make this test fail");
    }

    #[test]
    fn lager_can_hold_smaller() {
        let larger = TestRectangle {
            width: 10,
            height: 10,
        };
        let smaller = TestRectangle {
            width: 1,
            height: 1,
        };
        assert!(larger.can_hold(&smaller));
    }

    fn greet(name: &str) -> String {
        format!("Hello {}!", name)
    } 

    #[test]
    fn test_greet() {
        let result = greet("pan");
        // 自定义失败的提示信息
        assert!(result.contains("pan"), "Greet did not contain name, value was `{}`", result)
    }

    // 返回result不能使用should_panic
    // cargo test -- --show-output输出测试成功部分的printlin!
    #[test]
    fn test_with_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            println!("=======Msg: test_with_result will success");
            Ok(())
        } else {
            println!("=======Msg: test_with_result will failed");
            Err(String::from("Test result with err"))
        }
    }

    // ignore，此测试将不会运行
    #[test]
    #[ignore]
    fn test_ignore() {
        println!("=======Msg: test_ignore will success");
    }

}
use rust_learn::IOConfig;
fn test_io() {
    println!("=== test_io ===");
    let args: Vec<String> = env::args().collect();
    println!("args size: {}", args.len());
    // 如果返回error，则进入闭包中执行
    let config = IOConfig::build(&args).unwrap_or_else(|err| {
        // println!会输出到控制台或者重定向输出的文件中
        println!("Problem parsing arguments: {err}");
        // eprintln!只会输出到控制台，不会被重定向到文件中
        eprintln!("eprint==== Problem parsing arguments: {err}");
        std::process::exit(1);
    });
    println!("Serching for {}", config.query);
    println!("File path for {}", config.file_path);
    // 使用 if let 来检查 run 是否返回一个 Err 值，不同于 unwrap_or_else，并在出错时调用 process::exit(1)
    if let Err(e) = rust_learn::io_run(config) {
        println!("io run error: {e}");
        eprintln!("eprint==== io run error: {e}");
        std::process::exit(1);
    }
}

fn main() {
    // test_rust();
    test_variable();
    test_data_type();
    test_func(1, 'a');
    test_exp();
    let test_ret = test_return(3);
    println!("test_return: {test_ret}");
    test_control();
    test_copy();
    test_reference();
    test_slice();
    test_struct();
    test_enum();
    let plant = Asparagus{name : String::from("No.1")};
    println!("Test crate plant: {:#?}", plant);
    // 调用lib.rs里面的实现
    eat_at_restaurant();
    test_vector();
    test_hashmap();
    test_error();
    test_generic();
    test_trait();
    test_lifetime();
    test_io();
}