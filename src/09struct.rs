
fn main09() {
    _struct();
    _tuple_struct();
}

// 声明结构体
// 1. 在结构字段中使用引用类型,需要加上生命周期
// 2. 结构体没有实现display特征,不能直接使用{}打印, 需要使用#[derive(Debug)] + {:?}/{:#?}
// 3. 还可以使用 dbg! 宏进行debug信息;
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// 类构造函数
fn build_user(email: String, username:String) -> User {
    User {
        // email: email,
        email,
        // username: username,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn _struct() {
    // 初始化结构体
    // 1.所有字段都需要初始化
    // 2.字段数据不需要一致
    let mut user = User{
        email: String::from("xxxx@qq.com"),
        username: String::from("Sing"),
        active:true,
        sign_in_count: 1,
    };

    user.email = String::from("yyy@qq.com");

    let _user2 = build_user(String::from("zzz@qq.com"), String::from("Sing2"));

    let _user3 = User{
        email: String::from("aaa@qq.com"),
        // username: user.username,  //发生所用权移交
        // active: user.active,
        // sign_in_count:user.sign_in_count,
        ..user
        // 必须在最尾使用
    };
}

fn _tuple_struct() {
    let _black = Color(0,0,0);
    let e = AlwaysE;

    
}

// 元组结构体: 字段没有名字
struct Color(i32,i32,i32);

// 单元结构体: 
struct AlwaysE;
