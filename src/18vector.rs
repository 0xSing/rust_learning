fn main18() {
    
}

// Vector 动态数组
fn _vector() {
    // new 初始化
    let mut v: Vec<i32> = Vec::new();  // Vec::with_capacity(10) 根据容量创建数组, 避免内存分配和拷贝影响性能
    v.push(1); 


    // vec![] 宏初始化
    let mut v2 = vec![1,2,3];
    v2.push(4);

    // 读取数组元素
    // 1. 索引访问
    let _item = &v2[2]; //不安全但简洁高效, 越界会直接panic, 确保不会越界的时候使用

    // 2.通过get访问
    let _item2 = v2.get(2); //返回option 防止索引越界

    // 遍历
    for _i in &v {
        // do something
    }
    // 如果在遍历过程中想要修改
    for _i in &mut v{
        //do something
    }

    // 排序
    let mut vec = vec![1, 5, 10, 2, 15];    
    //非稳定排序(对相等元素不保证是否会发生重新排序), 速度比稳定排序快, 稳定排序会额外分配原数组一半的空间
    // 浮点类型不能使用排序, 浮点类型包含NaN值, 没有完全实现Ord特性, 只能在不包含NaN的情况下,使用 sort_unstable_by(|a, b| a.partial_cmp(b).unwrap()); 进行排序
    vec.sort_unstable(); 

    // 根据结构体中的某字段进行排序
    // 1. sort_unstable_by(|a, b| a.字段.cmp(&b.字段));
    // 2. 在保证结构体中的字段类型都实现了Ord特性后, 帮结构体实现Ord特性, 然后直接使用sort_unstable排序;
}

// 储存不同类型的元素
// 1. 使用枚举: 使用枚举类将不同类型封装一次
enum Wrapper {
    X(i32),
    Y(f64),
}
fn _eunm_list() {
    let v = vec![
        Wrapper::X(3),
        Wrapper::Y(1.1),
    ];
}

// 2. 使用特征对象: 使用一个特征对象对两个类型进行封装
trait Wrapper2 {
    fn wrap(&self);
}

impl Wrapper2 for i32 {
    fn wrap(&self) {
        // do something
    }
}

impl Wrapper2 for f64 {
    fn wrap(&self) {
        // do someting
    }
}

fn _trait_list () {
    let v: Vec<Box<dyn Wrapper2>> = vec![
        Box::new(3),
        Box::new(1.1),
    ];
}