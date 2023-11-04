fn main_var(){
    let a = 10;

    let b: i32 = 20;

    let h = 30i32;
    // println!("{}", _);
    let c = 11;

    let d = 30_i32;

    let e = add(add(a, b), add(c, d));

    println!("( a + b ) + ( c + d ) = {}", e);

    // 变量解构
    let (a, mut b): (bool,bool) = (true, false);
    // a = true,不可变; b = false，可变
    println!("a = {:?}, b = {:?}", a, b);

    b = true;
    assert_eq!(a, b);


    // 解构赋值
    let (a, b, c, d, e);

    (a, b) = (1, 2);
    // _ 代表匹配一个值，但是我们不关心具体的值是什么，因此没有使用一个变量名而是使用了 _
    [c, .., d, _] = [1, 2, 3, 4, 5];
    Struct { e, .. } = Struct { e: 5 , _f: 2};

    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);


    // const
    const MAX_SUPPLY :i32 = 10_000;
    println!("{}", MAX_SUPPLY);


    // var shadowing
    let x = 90;
    let x = x + 10;
    println!("{}", x);


    // var guess
    let guess: i32 = "cool".parse().expect("Not a Number");
    println!("{}", guess);

}

fn add(i: i32, j: i32) -> i32 {
    i + j
}

struct Struct {
    e: i32,
    _f: i32
}