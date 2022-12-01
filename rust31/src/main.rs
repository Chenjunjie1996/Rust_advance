#![allow(warnings,unused)]

//生命周期的主要作用是避免悬垂引用

use std::fmt::Display;

fn main() {
    // 函数
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("longest {}", result);

    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest1(string1.as_str(), string2.as_str());
    }
    println!("longest {}", result);

    // 结构体
    let i;
    {
        let novel = String::from("call me ishmael. some years age...");
        let first_sentene = novel.split('.').next().expect("could not find a '.'");
        i = ImportantExcerpt{part: first_sentene};
    }
    // println!("{:?}",i);
    // 结构体比它引用的字符串活得更久，引用字符串在内部语句块末尾 }
    // 被释放后，println! 依然在外面使用了该结构体，因此会导致无效的引用

    let s: &'static str = "我没啥优点，就是活得久，嘿嘿";
}

// 在存在多个引用时，编译器有时会无法自动推导生命周期，
// 此时就需要我们手动去标注，通过为参数标注合适的生命周期来
// 帮助编译器进行借用检查的分析
// 标记的生命周期只是为了取悦编译器，让编译器不要难为我们
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// y 完全没有被使用，因此 y 的生命周期与 x 和返回值的生命周期
// 没有任何关系，意味着我们也不必再为 y 标注生命周期，
// 只需要标注 x 参数和返回值即可。
fn longest1<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// 悬垂引用：result函数结束后被释放，result的引用依然在
// fn longest<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()
// }

// 生命周期语法用来将函数的多个引用参数和返回值的作用域关联到一起
// 一旦关联到一起后，Rust 就拥有充分的信息来确保我们的操作是内存安全的。

// 结构体中的生命周期
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// 生命周期消除
// 每一个引用类型都有一个生命周期，编译器为了简化用户的使用，运用了生命周期消除大法。
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..]
}
//对于first_word函数，它的返回值是一个引用类型，该引用只有两种情况：
// 从参数获取，从函数体内部新创建的变量获取
// 如果是后者，就会出现悬垂引用，最终被编译器拒绝

// 三条消除规则
// 1.每一个引用参数都会获得独自的生命周期
// fn foo<'a, 'b>(x: &'a i32, y: &'b i32)
// 2.若只有一个输入生命周期(函数参数中只有一个引用类型)该生命周期会被赋给所有的输出生命周期
// fn foo<'a>(x: &'a i32) -> &'a i32
// 3 若存在多个输入生命周期，且其中一个是 &self 或 &mut self，则 &self 的生命周期被赋给所有的输出生命周期

// 方法中的生命周期
// 泛型语法
struct Point<T> {
    x: T,
    y: T,
}
impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
    fn x(&self) -> &T {
        &self.x
    }
}
struct ImportantExcerpt1<'a> {
    part: &'a str,
}
impl<'a> ImportantExcerpt1<'a> {
    fn level(&self) -> i32 {
        3
    }
}
// 第三规则应用
// 首先，编译器应用第一规则，给予每个输入参数一个生命周期:
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part1<'b>(&'a self, announcement: &'b str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
// 接着，编译器应用第三规则，将 &self 的生命周期赋给返回值 &str
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part2<'b>(&'a self, announcement: &'b str) -> &'a str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
// 结果
impl <'a> ImportantExcerpt1<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
// 返回的生命周期改为'b
impl<'a: 'b, 'b> ImportantExcerpt<'a> {
    fn announce_and_return_part(&'a self, announcement: &'b str) -> &'b str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
// 'a: 'b，是生命周期约束语法，跟泛型约束非常相似
// 用于说明 'a 必须比 'b 活得久


// 静态生命周期 'static
// 拥有该生命周期的引用可以和整个程序活得一样久。

fn longest_with_an_announcement<'a, T> (
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {x} else {y}
}

