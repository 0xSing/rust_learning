fn main10() {
    // let heart = PokerSuit::Hearts;
    // let diamond = PokerSuit::Diamonds;

    // print_suit(heart);
    // print_suit(diamond);

    // let c = PokerCard {
    //     suit: PokerSuit::Clubs,
    //     value:1,
    // };

    // let c2 = PokerCard {
    //     suit: PokerSuit::Spades,
    //     value: 13,
    // };
}

// fn print_suit(card: PokerSuit) {
//     println!("{:?}", card);
// }

// #[derive(Debug)]
// enum PokerSuit {
//     Clubs,
//     Spades,
//     Diamonds,
//     Hearts,
// }

// struct PokerCard {
//     suit: PokerSuit,
//     value: u8
// }

// 更简洁实现 扑克牌
enum PokerCard {
    Clubs(u8),
    Spades(u8),
    Diamonds(u8),
    Hearts(u8),
}

//Rust 引入了Option<T> 处理空值
// 只用遇到Option<T>时, 才需要去考虑是否没有值
// 可以通过match去处理, some 或者 None情况