
fn main04(){
    let c = 'z';
    let z = 'Z';
    let g = '国';
    let cat = '😻';
    println!("{}, {}, {}, {}", c, z, g, cat);

    println!("{}", std::mem::size_of_val(&c));

    let t = true;
    println!("bool类型占{}个字节", std::mem::size_of_val(&t));
}