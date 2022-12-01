#![allow(unused,warnings)]
use std::ops::{Deref,DerefMut,Drop};
use std::rc::Rc;
// 智能指针的名称来源，主要就在于它实现了 Deref 和 Drop 特征
// Deref解引用 Deref可以让智能指针像引用那样工作
// 这样可以写出同时支持智能指针和引用的代码，例如 *T
// Drop 允许你指定智能指针超出作用域后自动执行的代码，例如做一些数据清除等收尾工作

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}
impl Person {
    fn new(name: String, age: u8) -> Self { Self { name, age } }

    fn display(self: &mut Person, age:u8) {
        let Person{name, age} = &self;
        // 此时 &self 的类型是 &&mut Person
    }
}

struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let mut person = Person{ name:"s".to_string(), age:2 };
    person.display(3);
}

// 函数和方法中的隐式 Deref 转换
// Rust 提供了一个极其有用的隐式转换：Deref 转换。
// 若一个类型实现了 Deref 特征，那它的引用在传给函数或方法时，
// 会根据参数签名来决定是否进行隐式的 Deref 转换
fn display<'a>(s: &'a str) {
    println!("{}", s);
}
fn display_main() {
    let s = String::from("hello, world");
    display(&s);
}
// String 实现了 Deref 特征，可以在需要时自动被转换为 &str 类型
// &s 是一个 &String 类型，当它被传给 display 函数时，自动通过 Deref 转换成了 &str
// 必须使用 &s 的方式来触发 Deref(仅引用类型的实参才会触发自动解引用)

// 连续的隐式 Deref 转换
fn display_deref_continuous(){
    let s = MyBox::new(String::from("Rust"));
    display(&s);
    // disply(&((*s)[..]))
}

// 自动应用 Deref 的例子
fn deref_example() {
    let s = MyBox::new(String::from("hello, world"));
    let s1 = &s; // 两次Deref
    let s2 = s.to_string(); // 方法调用会自动解引用
}

// Deref 规则总结
// 一个类型为 T 的对象 foo，如果 T: Deref<Target=U>，那么，
// 相关 foo 的引用 &foo 在应用的时候会自动转换为 &U。

// 引用归一化
// Rust 会在解引用时自动把智能指针和 &&&&v 做引用归一化操作，
// 转换成 &v 形式，最终再对 &v 进行解引用：
// 1.把智能指针（比如在库中定义的，Box、Rc、Arc、Cow 等）从结构体脱壳
// 为内部的引用类型，也就是转成结构体内部的 &v
// 2.把多重&，例如 &&&&&&&v，归一成 &v， 标准库源码：
// impl<T: ?Sized> Deref for &T {
//     type Target = T;

//     fn deref(&self) -> &T {
//         *self
//     }
// }

fn foo(s: &str) {}
fn foo_test() {
    let owned = "Hello".to_string();
    // 由于 String 实现了 Deref<Target=str>
    foo(&owned);
}
fn foo_test1() {
    // String 实现了 Deref<Target=str>
    let owned = "Hello".to_string();
    // Rc 智能指针可以被自动脱壳为内部的 `owned` 引用： &String 
    // 然后 &String 再自动解引用为 &str
    let counted = Rc::new(owned);
    foo(&counted);
}

struct Foo;
impl Foo {
    fn foo(&self) {
        println!("Foo");
    }
}
fn foo_test2() {
    let f = &&Foo;
    f.foo();
    (&f).foo();
    (&&f).foo();
    (&&&&&&&&f).foo();
}

/* 三种 Deref 转换
之前，我们讲的都是不可变的 Deref 转换， Rust 还支持
将一个可变的引用转换成另一个可变的引用
以及将一个可变引用转换成不可变的引用，规则如下：
1. 当 T: Deref<Target=U>，可以将 &T 转换成 &U
2. 当 T: DerefMut<Target=U>，可以将 &mut T 转换成 &mut U
3. 当 T: Deref<Target=U>，可以将 &mut T 转换成 &U
*/

// DerefMut 例子
// 要实现 DerefMut 必须要先实现 Deref 特征：pub trait DerefMut: Deref
// T: DerefMut<Target=U> 解读：
//将 &mut T 类型通过 DerefMut 特征的方法转换为 &mut U 类型，
// 对应例中，就是将 &mut MyBox<String> 转换为 &mut String
struct MyBox1<T> {v: T}
impl<T> MyBox1<T> {
    fn new(x: T) -> MyBox1<T> {
        MyBox1 { v: x }
    }
}
impl<T> Deref for MyBox1<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.v
    }
}
impl<T> DerefMut for MyBox1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.v
    }
}
fn derefmut() {
    let mut s = MyBox1::new(String::from("hello, "));
    display_deref(&mut s)
}
fn display_deref(s: &mut String) {
    s.push_str("world");
    println!("{}", s);
}

/*
如果从 Rust 的所有权和借用规则的角度考虑，当你拥有一个可变的引用，
那该引用肯定是对应数据的唯一借用，那么此时将可变引用变成不可变引用
并不会破坏借用规则；但是如果你拥有一个不可变引用，那同时可能还存在
其它几个不可变的引用，如果此时将其中一个不可变引用转换成可变引用，
就变成了可变引用与不可变引用的共存，最终破坏了借用规则。
*/

/* 总结：
Deref 可以说是 Rust 中最常见的隐式类型转换，而且它可以连续的实现
如 Box<String> -> String -> &str 的隐式转换，
只要链条上的类型实现了 Deref 特征。



*/
