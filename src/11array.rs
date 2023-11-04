use core::slice;
use std::array::from_fn;

fn main11() {
    
}

// 定长数组
fn _array() {
    // 数组初始化
    let a = [1,2,3,4,5];

    // 五个元素都是 3
    // 这种初始化方式, 依赖类型的copy特征, 引用类型无法使用
    let b = [3; 5];
    let c: [String; 8] = std::array::from_fn(|_i| String::from("rust is good!"));

    // 索引访问数组元素
    let _first = a[0];
    let _secound = b[1];

    // 数组切片
    let _slice = &a[1..3];
}

// 动态数组
fn _vector() {
    
}