#![allow(unused,warnings)]
/*
Drop 释放资源
在 Rust 中，我们之所以可以一拳打跑 GC 的同时一脚踢翻手动资源回收，
主要就归功于 Drop 特征，同时它也是智能指针的必备特征之一。
*/

struct Foo;
fn main() {
    let foo = Foo;
    drop(foo)
    // drop函数在std::prelude里。
    // 在绝大多数情况下，我们都无需手动去 drop 以回收内存资源，因为 Rust 会自动帮我们完成这些工作
    
}
