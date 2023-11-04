fn main22() {
    println!("cool");
}

// 包 Crate
// 包是一个独立的可编译单元, 编译后会生成一个可执行文件或者一个库
// 可以通过 use 将一个包引入当前项目作用域

// 项目 Package
// 工程/软件包, 包含独立的Cargo.toml文件
// 一个项目可以拥有多个包、一个库(library)

// cargo new *** 创建二进制 package
// cargo new *** --lib 创建库 package  //库只能用作第三方库被引用, 不能独立运行

// 一个项目的标准目录结构
// .
// ├── Cargo.toml
// ├── Cargo.lock
// ├── src
// │   ├── main.rs
// │   ├── lib.rs
// │   └── bin
// │       └── main1.rs
// │       └── main2.rs
// ├── tests
// │   └── some_integration_tests.rs
// ├── benches
// │   └── simple_bench.rs
// └── examples
//     └── simple_example.rs

// 唯一库包：src/lib.rs
// 默认二进制包：src/main.rs，编译后生成的可执行文件与 Package 同名
// 其余二进制包：src/bin/main1.rs 和 src/bin/main2.rs，它们会分别生成一个文件同名的二进制可执行文件
// 集成测试文件：tests 目录下
// 基准性能测试 benchmark 文件：benches 目录下
// 项目示例：examples 目录下