use rust_learn;

// 总共有三种测试：单元测试、集成测试、文档测试
// test目录下的所有rust都是集成测试，如果单元测试失败了，集成测试不会执行
// 通过cargo test --test test可以只运行集成测试test目录下的test.rs中的测试函数

// 声明引用common模块
mod common;

#[test]
fn it_adds_two() {
    // 调用common模块里的setup函数（可以在多个测试中调用）
    common::setup();
    assert_eq!(2, 2);
}



