use std::{fs::File, io::ErrorKind};

fn main20() {
    _panic()
}
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

// panic 不可恢复错误
// 程序出错的时候会被动调用触发
fn _panic() {
    // 主动调用
    panic!("你GG了");

    // 当发生panic的时候, 使用 RUST_BACKTRACE=1 cargo run, 可以栈展开/栈回溯, 查看调用链进行debug
    // 可以通过cargo.toml 文件进行配置release模式下 panic的终止流程: 栈展开和直接终止

    // 线程panic后会被终止, 所以不要在main线程上做太多的操作, 避免主线程终止

    // 处理Result的方法, unwarp、expect:如果返回Err, 不处理错误直接panic

    // 使用panic的情况:
    // 1.当你知道业务处理不可能会错误的时候, 可以使用panic;
    // 2.当出现可能导致全局有害状态时:
    //     1. 非预期错误
    //     2. 后续代码运行受到显著影响
    //     3. 内存安全问题

}

// 可恢复错误 Result
fn _err() {
    // 结合Result 和 match 对返回结果进行处理
    let f = File::open("hello.txt");

    let _f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    // 如果不需要错误细节处理, 可以直接使用unwrap 和 expect
    let _fc = File::open("hello.txt").unwrap(); // 成功: 取Ok<T>中的T, 失败: 直接panic

    let _fc = File::open("hello.txt").expect("带上自定义的错误信息"); // 可以带上自定义错误信息
}

// 错误传播(上抛)
// 通过Result对结果进行包装, 在函数返回值中进行错误传播
// 简化代码 ? 宏, 相当于match处理Result, 成功返回值, 失败return Err(传播错误), 同时?会帮Err进行类型提升
// 意味着可以使用一个大而全的Err类型进行返回值, 只需要实现了From特征就行了

// 除此之外, ? 宏还可以处理Option
// 注意 ? 失败时返回None, 成功时返回的是值, 需要变量进行承接

// try! 宏
// 对比
//  `?`
// let x = function_with_error()?; // 若返回 Err, 则立刻返回；若返回 Ok(255)，则将 x 的值设置为 255

// `try!()`
// let x = try!(function_with_error()); 
