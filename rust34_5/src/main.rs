#![allow(warnings,unused)]
use std::cell::{Cell,RefCell};
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
/* Cell RefCell
可以在拥有不可变引用的同时修改目标数据，对于正常的代码实现来说，是不可能的
（要么一个可变借用，要么多个不可变借用）。
内部可变性的实现是因为 Rust 使用了 unsafe 来做到这一点，对于使用者来说
这些都是透明的，因为这些不安全代码都被封装到了安全的 API 中

Cell 和 RefCell 在功能上没有区别，区别在于 
Cell<T> 适用于 T 实现 Copy 的情况：
*/
fn cell_test() {
    // "asdf" 是 &str 类型，它实现了 Copy 特征
    // c.get 用来取值，c.set 用来设置新值
    let c = Cell::new("asdf");
    // let c = Cell::new(String::from("asdf"));
    let one = c.get();
    c.set("qwer");
    let two = c.get();
    println!("{}, {}", one, two);
}
/*Cell使用并不多，因为我们要解决的往往是可变，不可变引用共存导致的问题
RUST规则 VS 智能指针带来的额外规则：
一个数据只有一个所有者 vs Rc/Arc让一个数据可以拥有多个所有者
要么多个不可变借用，要么一个可变借用 vs RefCell实现编译期可变不可变引用共存
违背规则导致编译错误 vs 违背规则导致运行时panic
*/
fn refcell_test() {
    let s = RefCell::new(String::from("hello"));
    let s1 = s.borrow();
    let s2 = s.borrow_mut();
}
/* RefCell总结
1.与 Cell 用于可 Copy 的值不同，RefCell 用于引用
2.RefCell 只是将借用规则从编译期推迟到程序运行期，并不能帮你绕过这个规则
3.RefCell 适用于编译期误报或者一个引用被在多处代码使用、修改以至于难于管理借用关系时
4.使用 RefCell 时，违背借用规则会导致运行期的 panic

Cell 只适用于 Copy 类型，用于提供值，而 RefCell 用于提供引用
Cell 不会 panic，而 RefCell 会
*/
fn cell_compare() {
    let x = Cell::new(1);
    let y = &x;
    let z = &x;
    x.set(2);
    y.set(3);
    z.set(4);
    println!("{}", x.get());

    // let mut x = 1;
    // let y = &mut x;
    // let z = &mut x;
    // x = 2;
    // *y = 3;
    // *z = 4;
}

/* 内部可变性
何为内部可变性？简单来说，对一个不可变的值进行可变借用
let x = 5;
let y = &mut x;
*/
fn demo() {
    /*可以对一个可变值进行不可变借用
    当值不可变时，可能会有多个不可变的引用指向它，此时若将修改其中一个为可变的，
    会造成可变引用与不可变引用共存的情况；而当值可变时，
    最多只会有一个可变引用指向它，将其修改为不可变，
    那么最终依然是只有一个不可变的引用指向它。
    */
    let mut x = 5;
    let y = &x;
    println!("{}", x);
}
trait Messenger{
    fn send(&mut self, msg: String);
}
struct MsgQueue{ msg_cache: Vec<String> }
impl Messenger for MsgQueue {
    fn send(&mut self, msg: String) {
        self.msg_cache.push(msg)
    }
}
// 特征是定义在外部库中：
pub trait Messenger1{
    fn send(&self, msg: String);
}
struct MsgQueue1{ msg_cache: RefCell<Vec<String>> }
impl Messenger1 for MsgQueue1 {
    fn send(&self, msg: String) {
        self.msg_cache.borrow_mut().push(msg)
    }
}
fn msg_test(){
    let mq = MsgQueue1{
        msg_cache: RefCell::new(Vec::new())
    };
    mq.send("asd".to_string());
}
/*内部可变性的核心用法：通过包裹一层 RefCell，
成功的让 &self 中的 msg_cache 成为一个可变值，然后实现对其的修改。
*/

/*Rc + RefCell 组合使用
在 Rust 中，一个常见的组合就是 Rc 和 RefCell 在一起使用，
前者可以实现一个数据拥有多个所有者，后者可以实现数据的可变性：
*/
fn rc_ref_conbine() {
    let s = Rc::new(RefCell::new("我很善变，还拥有多个主人".to_string()));
    let s1 = Rc::clone(&s);
    let s2 = Rc::clone(&s);
    s2.borrow_mut().push_str(", on yeah");
    println!("{:?}\n{:?}\n{:?}",s,s1,s2);
}
fn main() {
    cell_test();
    // refcell_test();
    cell_compare();
    msg_test();
    rc_ref_conbine();
    cell_ref_test();
}

#[derive(Debug)]
struct CellRef<'a>{
    a: &'a str,
    b: &'a mut i32,
    c: Vec<String>,
    d: HashMap<i32, i32>,
}

impl<'a> CellRef<'a> {
    fn new(a: &'a str, b: &'a mut i32, c: Vec<String>, d: HashMap<i32, i32>) -> Self { Self { a, b, c, d } }
}

fn cell_ref_test() {
    let mut hm = HashMap::new();
    hm.insert(3, 4);
    let vc = vec!["he".to_string(), "llow".to_string()];
    let (a, mut b,c,d) = ("a".to_string(), 3, vc , hm);
    let cr = CellRef::new(&a, &mut b, c, d);
    println!("{:?}", cr);

    let s = Rc::new(RefCell::new(cr));
    let s1 = Rc::clone(&s);
    s.borrow_mut().c.append(&mut vec!["rust".to_string()]);
    println!("{:?}", s);
    println!("{:?}", s1);
}
/*
由于 Rust 的 mutable 特性，一个结构体中的字段，
要么全都是 immutable，要么全部是 mutable，
不支持针对部分字段进行设置。比如，在一个 struct 中，
可能只有个别的字段需要修改，而其他字段并不需要修改，
为了一个字段而将整个 struct 变为 &mut 也是不合理的。

所以，实现内部可变性的 Cell 和 RefCell 正是为了解决诸如这类问题存在的
通过它们可以实现 struct 部分字段可变，
而不用将整个 struct 设置为 mutable。
*/