fn main12() {
    
}

fn _if() {
    let condition = true;
    
    // 使用if作表达式赋值时, 保证if else 里面的表达式类型一致
    let _number  = if condition {
        5
    } else {
        6
    };
}

fn _else_if() {
    let n = 6;
    
    if n % 4 == 0 {
        //
    } else if n % 5 == 0 {
        //
    } else {
        //
    }
}

fn _for() {
    // for 元素 in 集合
    // 注意这个集合如果是复杂类型(不具有copy特征的): 一般使用 &引用, 除非后面将不会再使用这个集合(所有权移交)
    for _i in 1..=5 {
        //
    }

    // 在循环中获取元素索引, 使用iter()将数组变成迭代器
    let a = [1; 5];
    for (_i, _v) in a.iter().enumerate() {
        
    }
}

// continue跳过当次循环  break跳出整个循环, 可以带返回值(类似return)

fn _while() {
    let mut n = 0;
    while n <= 5 {
        //
        n += 1;
    }
}

// loop无条件循环
fn _loop() {
    let condition = true;

    // loop是表达式, 可以返回值
    loop {
        if condition {
            break;
        }
    }
}