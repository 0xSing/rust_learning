use core::fmt;
use std::fmt::{Display, write};

fn main17() {
    
}

// 关联类型: 在特征定义的代码块中声明自定义类型, 在特征的方法签名中可以使用;
// 同样需求可以使用 泛型 进行解决, 但当这个类型有特征约束的时候, 关联类型可读性会更强
pub trait Iterator {
    type Item; //关联类型

    fn next(&mut self) -> Option<Self::Item>;
}

// 默认泛型类型参数
// 以Add特征为例, 帮一个类型实现Add特征的时候, 如果不对Add泛型进行指定, 默认为Self(即为同类型相加)
// 好处为:
// 1. 减少实现的样板代码
// 2. 扩展类型但无需要大幅修改现有的代码

// 当一个类型和其实现的特征中有同名方法时:
// 1. 直接 . 调用会优先调用本类型中的方法;
// 2. 需要调用特征中的方法需要使用 特征::方法名 进行显式调用
//      显式调用如果参数有self, 需要进行入参;
//      如果参数中没有self, 则需要使用完全限定语法, <类型 as 特征>::方法名(receiver, args...), receiver用于接受self, 只有方法有, 函数没有

// 特征定义中的特征约束
trait SuperTrait: Display { //意为实现SuperTrait特征必须先实现Display特征
    
}

// newtype模式: 绕过孤儿规则, 给外部类型实现外部特征
struct Wrapper(Vec<String>); //元组结构体: 字段没有名字, 其中Vec为外部类型

impl fmt::Display for Wrapper { //Display为外部特征
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}
