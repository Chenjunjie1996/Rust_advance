#![allow(warnings,unused)]
// Sized 和不定长类型 DST

fn main() {
    // 切片也是一个典型的 DST 类型

}
trait MyThing{}
fn foobar_1(thing: &dyn MyThing) {}
fn foobar_2(thing: Box<dyn MyThing>) {}
// fn foobar_3(thing: MyThing){} ERROR
// Rust 中常见的 DST 类型有: str、[T]、dyn Trait，
// 它们都无法单独被使用，必须要通过引用或者 Box 来间接使用。
// 总结：只能间接使用的 DST

// sized 特征
fn generic<T>(t: T){}
// 编译器自动帮我们加上了 Sized 特征约束
fn generic_auto<T: Sized>(t: T) {}
// 在泛型函数中使用动态数据类型
fn generic_dst<T: ?Sized>(t: &T) {}

// Box 可以将一个动态大小的特征变成一个具有固定大小的特征
// str它是一个动态类型，同时还是 String 和 &str 的底层数据类型
fn box_str(){
    let s1: Box<str> = "Hello there".into();
}