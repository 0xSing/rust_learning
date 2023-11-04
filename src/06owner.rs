fn main06() {
    owner_move();
    copyable();
    mut_copyable();
}

//所有权示例
fn owner_move(){
    let s = String::from("hello");  // s 进入作用域

    takes_ownership(s);             // s 的值移动到函数里 ...
                                    // ... 所以到这里不再有效

    let x = 5;                      // x 进入作用域
    makes_copy(x);                  // x 应该移动函数里，
                                    // 但 i32 是 Copy 的，所以在后面可继续使用 x

} // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
  // 所以不会有特殊操作

fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
}

//引用和借用
fn copyable() {
    let x = 5;
    let y = &x; //引用 : 默认为不可变引用

    assert_eq!(5, x);
    assert_eq!(5, *y); //解引用
}

fn mut_copyable() {
    let mut s = String::from("hello");

    change(&mut s); // 可变引用
    // 统一作用域,可变引用只能同时存在一个(避免数据竞争)
    // 变量的生命未结束之前,可变引用和不可变引用不能同时存在
    // Rust 不会发送悬垂引用(引用仍存在,但引用指向的值已经被释放)
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}