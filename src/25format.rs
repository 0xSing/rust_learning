fn main25() {
    
}

fn _fmt(){
    // {}
    println!("Hello");                 // => "Hello"
    println!("Hello, {}!", "world");   // => "Hello, world!"
    println!("The number is {}", 1);   // => "The number is 1"
    println!("{:?}", (3, 4));          // => "(3, 4)"
    println!("{value}", value=4);      // => "4"  //具名参数: 带名字的参数要放在不带名字的参数下面
    println!("{} {}", 1, 2);           // => "1 2"
    println!("{:04}", 42);             // => "0042" with leading zeros 补零
    // => Alice, this is Bob. Bob, this is Alice
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob"); //位置参数
    
    // 格式化参数
    // 1. 字符串填充    (左对齐)
    // 以下全部输出 "Hello x    !"
    // 为"x"后面填充空格，补齐宽度5
    println!("Hello {:5}!", "x");
    // 使用参数5来指定宽度
    println!("Hello {:1$}!", "x", 5);
    // 使用x作为占位符输出内容，同时使用5作为宽度
    println!("Hello {1:0$}!", 5, "x");
    // 使用有名称的参数作为宽度
    println!("Hello {:width$}!", "x", width = 5);
    // 使用参数5为参数x指定宽度，同时在结尾输出参数5 => Hello x    !5
    println!("Hello {:1$}!{}", "x", 5);

    // 2.数字填充: 符号和0 (右对齐)
    // 宽度是5 => Hello     5!
    println!("Hello {:5}!", 5);
    // 显式的输出正号 => Hello +5!
    println!("Hello {:+}!", 5);
    // 宽度5，使用0进行填充 => Hello 00005!
    println!("Hello {:05}!", 5);
    // 负号也要占用一位宽度 => Hello -0005!
    println!("Hello {:05}!", -5);

    // 3.对其
    // 左对齐 => Hello x    !
    println!("Hello {:<5}!", "x");
    // 右对齐 => Hello     x!
    println!("Hello {:>5}!", "x");
    // 居中对齐 => Hello   x  !
    println!("Hello {:^5}!", "x");
    // 对齐并使用指定符号填充 => Hello x&&&&!
    // 指定符号填充的前提条件是必须有对齐字符
    println!("Hello {:&<5}!", "x");

    // 4. 精度控制
    let v = 3.1415926;
    // 保留小数点后两位 => 3.14
    println!("{:.2}", v);
    // 带符号保留小数点后两位 => +3.14
    println!("{:+.2}", v);
    // 不带小数 => 3
    println!("{:.0}", v);
    // 通过参数来设定精度 => 3.1416，相当于{:.4}
    println!("{:.1$}", v, 4);

    let s = "hi我是Sunface孙飞";
    // 保留字符串前三个字符 => hi我
    println!("{:.3}", s);
    // {:.*}接收两个参数，第一个是精度，第二个是被格式化的值 => Hello abc!
    println!("Hello {:.*}!", 3, "abcdefg");

    // 5. 进制
    // 二进制 => 0b11011!
    println!("{:#b}!", 27);
    // 八进制 => 0o33!
    println!("{:#o}!", 27);
    // 十进制 => 27!
    println!("{}!", 27);
    // 小写十六进制 => 0x1b!
    println!("{:#x}!", 27);
    // 大写十六进制 => 0x1B!
    println!("{:#X}!", 27);
    // 不带前缀的十六进制 => 1b!
    println!("{:x}!", 27);
    // 使用0填充二进制，宽度为10 => 0b00011011!
    println!("{:#010b}!", 27);

    // 6. 指数
    println!("{:2e}", 1000000000); // => 1e9
    println!("{:2E}", 1000000000); // => 1E9

    // 7. 指针
    let v= vec![1, 2, 3];
    println!("{:p}", v.as_ptr()); // => 0x600002324050

    // 8. 转义
    // "{{" 转义为 '{'   "}}" 转义为 '}'   "\"" 转义为 '"'
    // => Hello "{World}" 
    println!(" Hello \"{{World}}\" ");

    // 格式化时捕获变量名
    let person = 32;
    println!("Hello, {person}!");





    // print! 将格式化文本输出到标准输出，不带换行符
    // println! 同上，但是在行的末尾添加换行符
    // format! 将格式化文本输出到 String 字符串

    // eprint!，eprintln! 标准输出错误

    // {} 适用于实现了 std::fmt::Display 特征的类型，用来以更优雅、更友好的方式格式化文本，例如展示给用户
    // {:?} 适用于实现了 std::fmt::Debug 特征的类型，用于调试场景

    // Debug特征可以使用派生实现 #[derive(Debug)] 比如结构体
    // Display特征: 无法派生实现
    // 1. 使用 {:?} 或 {:#?}
    // 2. 手动实现Display
    // 3. 使用newtype为外部类型实现Display

    // 方法二
    use std::fmt;
    impl fmt::Display for Person {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "大佬在上，请受我一拜，小弟姓名{}，年芳{}，家里无田又无车，生活苦哈哈",
                self.name, self.age
            )
        }
    }

    //方法三
    impl fmt::Display for Array {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "数组是：{:?}", self.0)
        }
    }

}   

struct Person {
    name: String,
    age: u8,
}

struct Array(Vec<i32>);