use std::collections::HashMap;

fn main19() {
    _hash_map();
}

// HashMap KV
fn _hash_map() {
    let mut hm = HashMap::new(); //HashMap::with_capacity(capacity) 初始化容量
    hm.insert("红宝石", 1); //插入数据

    // 将vec 转为 hashmap
    let team_list = vec![
        ("a", 100), 
        ("b", 10),
        ("c", 50),
    ];

    let _map: HashMap<_, _> = team_list.into_iter().collect();  // 将vec 转为 迭代器 再进行收集为 hashmap

    // 所有权规则, 如类型Copy特征, 拷贝进hashmap, 如类型没有Copy特征, 将所有权移交到hashmap
    // 如果在insert 选择引用进hashmap, 要确保引用的生命周期跟hashmap一样长命
    
    // get 查询
    let _get = hm.get("红宝石");
    // 1. 返回的是一个option类型, 有可能为None
    // 2. 返回的值为引用, 因为防止所有权移交出来了
    // 如果想获取里面的值
    let _copied = _get.copied().unwrap_or(0);

    // 遍历
    for (k,v) in &_map {
        println!("{}:{}", k, v);
    }

    // 更新hm中的值
    // 覆盖已有值
    hm.insert("红宝石", 2); // 覆盖后返回旧值
    // 查询是否存在, 不存在插入且返回插入值, 存在不插入且返回已有值
    hm.insert("黄宝石", 50);

    // 判断k是否存在, 并修改v
    let entry = hm.entry("黄宝石");
    let _insert = entry.or_insert(0); //返回的是&met i32, 可以直接进行解引用, 更新值

    // 可以通过修改hash算法来达到hashmap, 安全和性能上的取舍
    // let mut hashmapcool: HashMap<_,_,BuildHasherDefault<第三方库hash算法> = Default::default();
    
}