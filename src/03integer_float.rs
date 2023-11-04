use num::complex::Complex;

fn main03(){

    // integer
    let a:u8 = 255;
    let b = a.wrapping_add(20); //只管溢出计算
    println!("{}", b);

    let c = a.checked_add(20); //如果发生溢出返回None
    assert_eq!(c, None);
    // println!("{}", c);

    let d = a.overflowing_add(20); //返回计算结果 和 bool是否发生过溢出
    // assert_eq!(true, d.1);
    println!("{}", d.0);
    println!("{}", d.1);

    let e = a.saturating_add(20); //只能达到最大或者最小值
    println!("{}", e);

    // float
    let x = 2.0;
    let y: f64 = x + 0.1;
    println!("{}", y);

    // float test
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    println!("{} + {} = {:x}", abc.0, abc.1, (abc.0+abc.1).to_bits());
    // 浮点数运算陷阱，会有微小差别，所以尽量避免使用浮点数去进行比较

    // NaN Not a Number error
    // 获取负数的平方根
    let x = (-42.0_f32).sqrt();
    if x.is_nan() { //is_nan用于判断一个变量是否为NaN
        println!("未定义的数学行为");
    }

    // range
    for i in 1..=5{ //1..5 表示1到4 1..=5  表示1到5
        println!("{}", i);
    }

    //
    let a = Complex {re: 2.1, im: -1.2};
    let b = Complex::new(11.1, 22.2);
    let result = a + b;

    println!("{}", result)

}