
fn main07() {
    let a = 3;
    let b = a;
    println!("{} {}", a, b);

    
    // 对String 进行slice切片的时候注意, 每个字符占的字节, 如果切在中间将会崩溃, 如中文,一个字占三个字节
    // 因为不同的语言,单字符占字节并不确定, 所以Rust不支持字符串索引(毫无意义)
}

fn _transfer() {
    // &str转换String
    let s = String::from("value");
    let _s2 = "value".to_string();

    // String转换&str
    let _str = &s; //deref隐式强制转换
}

fn _str_add() {
    let mut s = String::from("hello ");

    s.push_str("rust");

    s.push('!');
}

fn _str_insert() {
    let mut s = String::from("Hello rust!");

    s.insert(5,',');

    s.insert_str(6, "I like");
}

fn _str_replace() {
    let s = String::from("I like rust!");

    let _s2 = s.replace("rust", "RUST");

    let _s3 = s.replacen("rust", "RUST", 1);

    let mut s4 = String::from("I like rust!");
    s4.replace_range(7..8, "R");
}

fn _str_delete() {
    // pop 删除最后一位
    // remove 删除并返回指定位置的字符
    // truncate 删除从指定位置开始到结尾的全部字符
    // clear 清空所有字符
}

fn _str_connect() {
    // + or += 连接字符串
    // format!() 方法连接
}