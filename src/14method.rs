fn main14() {
    let m = Message::Write(String::from("hello guy"));
    m.call();
}

// 结构体方法
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

// 定义方法
// 方法名允许跟结构体的字段名一致
// Rust 对对象调用方法有 自动引用和解引用功能 (隐式借用/移交所有权) 取决于方法内部处理
// 可以有多个impl 对不同功能的方法进行隔离
impl Circle {
    // 没有引用self的函数称为关联函数,不能称为方法
    // new为构造器的名称
    fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle { x, y, radius }
    }

    // self 拥有所有权概念
    // self 表示所有权转移到方法中, 这种形式用得较少: 通常用于将当前对象转为另一个对象, 且不再关注前对象, 防止误调用
    // &self 表示当前方法结构体的, 不可变借用
    // &mut self 表示可变借用
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

// 枚举方法
enum Message {
    Quit,
    Move {x:i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match &self {
            Message::Write(x) => println!("{}", x),
            _ => println!(),
        }
    }    
}
