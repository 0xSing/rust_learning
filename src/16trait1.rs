use std::fmt::{Display, Debug};

fn main16() {
    
}

// trait 特征/接口
// 1.实现特征的 孤儿规则, 实现体 / 特征, 必须至少有一个在当前作用域中(防止破坏他人代码/他人破坏你的代码)
// 定义特征
pub trait Summary {
    // 特征的方法
    fn semmarize(&self) -> String;

    // 默认实现
    // 特征中的默认实现, 可以让类型无需实现, 也可以给类型进行重载
    // 默认实现允许调用本特征中的其他方法
    fn test(&self) {
        println!("cool{}", self.semmarize())
    }
}

pub struct Post<T> {
    pub title: String,
    pub author: String,
    pub content: T,
}

// 为结构体实现特征
impl<T> Summary for Post<T> {
    fn semmarize(&self) -> String {
        format!("文章{}, 作者{}", self.title, self.author)
    }
}

// 使用特征作为函数参数
// 实现了特征的元素作为参数
// 语法糖表达法
pub fn notify(item: &impl Summary) {
    println!("{}", item.semmarize())
}

//完整书写
pub fn notify2<T: Summary>(item: &T) {
    println!("{}", item.semmarize())
}

// 多重约束: 同时实现多个特征的元素
pub fn notify3(item: &(impl Summary + Display)) {
    println!("{}", item.semmarize())
}

// where约束: 约束很多的时候使用
pub fn notify4<T,U>(t: &T, u:&U) -> ()
    where T: Summary + Display,
          U: Clone + Debug  {
    println!()
}

// 使用特征约束特定条件调用方法
impl<T: Display + Summary> Post<T> {
    // 只有T拥有以上特征的Post, 才能拥有这里面的方法
}

// 有条件的实现: 所有拥有T特征的类型,都实现了ToString
// impl<T: Display> ToString for T {
    
// }

// 约束返回类型
// 返回拥有Summay特征的 Post类型对象
// ps: 实现体中只能有一个具体的类型, 如果需要返回多种具体类型, 需要使用dyn关键字, 声明特征对象
fn _returns_summar() -> impl Summary {
    Post{
        title: String::from(""),
        author: String::from(""),
        content: String::from(""),
    }
}

// derive 派生特征: 被 derive 标记的对象会自动实现对应的默认特征代码，继承相应的功能
// #[derive(Debug)] 派生实现debug特征

// 笔记:
// 1. T 泛型可以解决同一种类型的泛化业务
// 2. 特征约束 可以解决实现了某种特征的同一种类型泛化业务
// 3. 特征对象 可以解决实现了某种特征, 不同类型的泛化业务

// 特征对象的限制
// 只有对象安全的特征才能拥有特征对象
// 1.方法的返回类型不能是Self
// 2.方法没有任何泛型参数