#![allow(warnings,unused)]
// Newtype
// 自定义类型可以让我们给出更有意义和可读性的类型名，例如与其使用 u32 作为距离的单位类型，我们可以使用 Meters，它的可读性要好得多
// 对于某些场景，只有 newtype 可以很好地解决
// 隐藏内部类型的细节

use std::{fmt::Display, ops::Add};

// 为外部类型实现外部特征
// 避免孤儿规则
struct Wrapper(Vec<String>);
impl Display for Wrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

// 更好的可读性及类型异化
#[derive(Clone, Copy)]
struct Meters(u32);
impl Display for Meters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "距离{}米", self.0)
    }
}
impl Add for Meters {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
impl Meters {
    fn calculate_distance(self, rhs: Meters) -> Self {
        self + rhs
    }
}
fn calculate_distance(d1: Meters, d2: Meters) -> Meters {
    d1 + d2
}

fn main() {
    // 为外部类型实现外部特征
    let w = Wrapper(
        vec![String::from("hello"), String::from("world")]
    );
    println!("{}", w);

    // 更好的可读性及类型异化
    let (m1, m2) = (Meters(10), Meters(20));
    let d = calculate_distance(m1, m2);
    let dd = m1.calculate_distance(m2);
    println!("{}, {}", d, dd);

    // 类型别名
    type Meters = u32;
    let x: u32 = 5;
    let y: Meters = 5;
    println!("{}", x+y);

    // let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));
    // f是一个 Box<dyn T> 类型的特征对象，实现了 Fn() 和 Send 特征,生命周期为 'static。
    type Thunk = Box<dyn Fn() + Send + 'static>;
    let f: Thunk = Box::new(|| println!("hi"));

    // type Result<T> = std::result::Result<T, std::io::Error>;
    // 其它库只需要使用 std::io::Result<T>

}
