fn main15() {
    
}

// Generics 泛型
// 使用 T 作为泛型进行业务处理的时候,需要根据业务给T添加限制
fn _add<T : std::ops::Add<Output = T>>(a:T, b:T) -> T {
    a + b
}

// 给结构体使用泛型
// 1.需要在结构体名称处进行声明
// 2.T只能表示一种类型, 如果需要不同类型的泛型, 需要额外再进行声明一个新的名字, 如Point<Y,U>
struct Point<T>{
    x: T,
    y: T,
}

// 枚举中使用泛型, 比如
// enum Option<T> {
//     Some<T>,
//     None,
// }
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// 方法中使用泛型
// 方法中使用额外的泛型(非结构体中的泛型)
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    // 额外声明泛型
    fn _mixup<U,W>(self, other: Point<T>) {
        //
    }
}

// 方法为特定泛型实现方法
impl Point<i32> {
    fn y(&self) -> &i32 {
        &self.y
    }
}

// const 泛型
// 针对值的泛型
// N 为代表数组长度的泛型
fn _display_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("{:?}", arr);
}