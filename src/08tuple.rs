

fn _tuple() {
    // 元组初始化
    let tup: (i32, f64, i8) = (500, 6.4, 1);

    // 模式匹配结构元组
    let (_x, _y, _z) = tup;

    // 使用.访问元组
    let _a = tup.0;

    // 元组使用场景
    // 函数返回值通过元组返回多个数据
}