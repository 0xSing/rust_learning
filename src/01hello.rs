fn main_01() {
    say_hello()
}


fn say_hello() {
    let chinese = "哟哟哟";
    let english = "yoyoyo";
    let regions = [chinese, english];
    for r in regions.iter(){
        println!("{}", &r)
    }
}