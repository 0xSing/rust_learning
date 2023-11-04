fn main14() {
    _at();
}

enum Direction {
    East,
    West,
    North,
    South,
}


enum Color {
    Black(u8, u8, u8),
    Red,
}

fn _match(){
    let dire = Direction::South;
    
    // match 表达式
    // 1.必须穷举所有可能性, _表示保底方案
    // 2.每一个分支,都是表达式, 且表达式返回值类型一致
    // 3.x|y 或 条件满足其中之一就行
    let _newDire = match dire{
        Direction::East => println!(),
        Direction::North | Direction::West => {

        },
        _ => println!(),
    };

    // 模式绑定: 匹配后获取模式中绑定的数据
    let black = Color::Black(0, 0, 0);

    match black {
        // 在模式匹配后,用if在此匹配,称为匹配守卫
        Color::Black(a, b, c) if a == 0 => {
            println!("{}{}{}", a,b,c);
        },
        _ => println!(),
    }

    // 穷尽匹配: 除了使用_ 进行处理穷尽的模式外, 还可以使用一个变量处理, 如other


}

// 如果只有一个模式需要被匹配处理, 那就使用if let
fn _if_let() {
    let v = Some(3u8);
    if let Some(3) = v {
        print!("three");
    }
}

// match! 匹配宏,用于一个表达式跟模式进行匹配,返回bool


// @ 运算符允许一个字段绑定一个变量
enum Message {
    Hello { id: i32 },
}

fn _at(){
    let msg = Message::Hello { id: 6 };

    match msg {
        // 使用@将id绑定到字段中
        Message::Hello { id: id_variable @ 3..=7 } => {
            println!("Found an id in range: {}", id_variable)
        },
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        },
        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        },
    }
}
