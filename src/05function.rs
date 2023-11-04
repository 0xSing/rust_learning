fn main05() {
    let z = add_with_extra(2, 5);
    println!("{}",z);
}


// add_with_extra 函数名
// x y 入参
// i32 返回值的类型, 当没有显式返回值类型时,即返回值为(), 当返回值为!时,便是这是一个发散函数(永不返回)
// x+y 表达式返回值
fn  add_with_extra(x: i32, y: i32) -> i32 {
    let x = x + 1;
    let y = y + 5;
    x + y
}