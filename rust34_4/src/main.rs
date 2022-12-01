#![allow(unused,warnings)]
use std::rc::Rc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
/*
Rc 与 Arc
通过引用计数的方式，允许一个数据资源在同一时刻拥有多个所有者。
前者适用于单线程，后者适用于多线程
*/
fn main() {
    let s = String::from("hello, world");
    let a = Box::new(s);
    // let b = Box::new(s);

    // 使用 Rc 就可以轻易解决：
    let a = Rc::new(String::from("hello, world"));
    let b = Rc::clone(&a);
    assert_eq!(2, Rc::strong_count(&a));
    assert_eq!(Rc::strong_count(&a), Rc::strong_count(&b));
    /*
    这里的clone仅仅复制了智能指针并增加了引用计数，并没有克隆底层数据
    因此 a 和 b 是共享了底层的字符串 s，这种复制效率是非常高的。
    */

    let a = Rc::new(String::from("test ref counting"));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Rc::clone(&a);
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Rc::clone(&a);
        println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    /*
    1.由于变量 c 在语句块内部声明，当离开语句块时因为超出作用域而被释放
    所以引用计数会减少 1，事实上这个得益于 Rc<T> 实现了 Drop 特征
    2.a、b、c 三个智能指针引用计数都是同样的，并且共享底层的数据，因此打印计数时用哪个都行
    3.当 a、b 超出作用域后，引用计数会变成 0，最终智能指针和它指向的底层字符串都会被清理释放
    */
    rc_test();
    // rc_thread_test()
    arc_thread_test();
}

/*
事实上，Rc<T> 是指向底层数据的不可变的引用，因此你无法通过它来修改数据
这也符合 Rust 的借用规则：要么存在多个不可变借用，要么只能存在一个可变借用。

实际开发中我们往往需要对数据进行修改，这时单独使用 Rc<T> 无法满足需求
需要配合其它数据类型来一起使用，例如内部可变性的 RefCell<T> 类型以及
互斥锁 Mutex<T>，在多线程编程中，Arc 跟 Mutex 锁的组合使用非常常见
它们既可以让我们在不同的线程中共享数据，又允许在各个线程中对其进行修改。
*/
struct Owner{ name: String }
struct Gadget {
    id: i32,
    owner: Rc<Owner>
}

fn rc_test() {
    let gadget_owner = Rc::new(Owner{
        name: "Gadget man".to_string(),
    });
    let gadget1 = Gadget{ id: 1, owner: Rc::clone(&gadget_owner) };
    let gadget2 = Gadget{ id: 2, owner: Rc::clone(&gadget_owner) };
    // 释放掉第一个Rc<Owner>
    drop(gadget_owner);
    /*
    尽管在上面我们释放了 gadget_owner，但是依然可以使用 owner信息
    drop 之前，存在三个指向 Gadget Man 的智能指针引用，上面仅仅
    drop 掉其中一个智能指针引用，而不是 drop 掉 owner 数据
    外面还有两个引用指向底层的 owner 数据，引用计数尚未清零
    因此 owner 数据依然可以被使用
    */
    // println!("{}", gadget_owner.name);
    println!("{}, {}", gadget1.id, gadget1.owner.name);
    println!("{}, {}", gadget2.id, gadget2.owner.name);
    /*
    最后，`gadget1` 和 `gadget2` 也被释放，最终引用计数归零，
    随后底层数据也被清理释放    
    */
}
/* Rc总结
1.Rc/Arc 是不可变引用，你无法修改它指向的值，只能进行读取，如果要修改，
需要配合后面章节的内部可变性 RefCell 或互斥锁 Mutex
2.一旦最后一个拥有者消失，则资源会自动被回收，这个生命周期是在编译期就确定下来的
3.Rc只能用于同一线程内部，想要用于线程之间的对象共享，你需要使用 Arc
4.Rc<T> 是一个智能指针，实现了 Deref 特征，因此你无需先解开 Rc 指针，
再使用里面的 T，而是可以直接使用 T，例如上例中的 gadget1.owner.name
*/
fn rc_thread_test() {
    // 首先通过 thread::spawn 创建一个线程，
    // 然后使用 move 关键字把克隆出的 s 的所有权转移到线程中。
    let s = Rc::new(String::from("multi-thread test"));
    for _ in 0..10{
        let s = Rc::clone(&s);
        // let handle = thread::spawn(move || {
        //     println!("{}", s)
        // });
    }
    /*
    Rc<T> 不能在线程间安全的传递，实际上是因为它没有实现 Send 特征
    而该特征是恰恰是多线程间传递数据的关键
    */
}
fn arc_thread_test() {
    /* 原子化的 Rc<T> 智能指针
    它能保证我们的数据能够安全的在线程间共享
    线程安全伴随着性能损耗，大部分时候我们开发的程序都在一个线程内。
    */
    let s = Arc::new(String::from("multi-thread test"));
    for _ in 0..10 {
        let s = Arc::clone(&s);
        let handle = thread::spawn(move || {
            println!("{}", s)
        });
    }
    thread::sleep(Duration::from_secs(1));
    println!("{}", Arc::strong_count(&s));
}
/*
Rc 和 Arc 的区别在于，后者是原子化实现的引用计数，因此是线程安全的，
可以用于多线程中共享数据。

这两者都是只读的，如果想要实现内部数据可修改，必须配合内部可变性 
RefCell 或者互斥锁 Mutex 来一起使用。

*/