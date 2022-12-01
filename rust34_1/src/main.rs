#![allow(warnings,unused)]
/*
指针是一个包含了内存地址的变量，该内存地址引用或者指向了另外的数据。

Box<T>是Rust中最常见的智能指针
Box<T>将一个值分配到堆上，然后在栈上保留一个智能指针指向堆上的数据。

堆栈的性能
小型数据，在栈上的分配性能和读取性能都要比堆上高
中型数据，栈上分配性能高，但是读取性能和堆上并无区别，因为无法利用寄存器或 CPU 高速缓存，最终还是要经过一次内存寻址
大型数据，只建议在堆上分配和使用

Box 的使用场景 Box 是简单的封装，除了将值存储在堆上外，并没有性能损耗
可以在以下场景中使用它：
1. 特意的将数据分配在堆上
2. 数据较大时，又不想在转移所有权时进行数据拷贝
3. 类型的大小在编译期无法确定，但是我们又需要固定大小的类型时
4. 特征对象，用于说明对象实现了一个特征，而不是某个特定的类型
*/
fn main() {
    let a = Box::new(3);
    println!("a = {}", a); // a = 3
    // let b = a + 1
    /*
    建一个智能指针指向了存储在堆上的 3，并且 a 持有了该指针
    1. println! 隐式地调用了 Deref 对智能指针 a 进行了解引用
    2. 在表达式中，我们无法自动隐式地执行 Deref 解引用 需要let b = *a + 1
    3. a 持有的智能指针将在作用域结束（main 函数结束）时，被释放掉，这是因为 Box<T> 实现了 Drop 特征
    */

    // 当栈上数据转移所有权时，实际上是把数据拷贝了一份
    // 最终新旧变量各自拥有不同的数据
    // 堆上底层数据并不会被拷贝，转移所有权仅仅是复制一份栈中的指针，
    // 再将新的指针赋予新的变量，然后让拥有旧指针的变量失效
    // 最终完成了所有权的转移
    let arr = [0; 1000];
    let arr1 = arr;
    println!("{:?}, {:?}", arr.len(), arr1.len());

    // 在堆上创建一个长度为1000的数组，然后使用一个智能指针指向它
    let arr = Box::new([0; 1000]);
    // 此仅仅拷贝了智能指针的结构体，底层数据并没有被拷贝
    // 所有权顺利转移给 arr1，arr 不再拥有所有权
    let arr1 = arr;

}

// 将动态大小类型变为 Sized 固定大小类型

// enum List {
//     Cons(i32, List),
//     Nil
// }
enum List {
    Cons(i32, Box<List>),
    Nil,
}

// 特征对象
// 想实现不同类型组成的数组只有两个办法：枚举和特征对象, 前者限制较多
trait Draw {
    fn draw(&self);
}
struct Button{ id: u32 }
impl Draw for Button {
    fn draw(&self) {
        println!("这是屏幕上第{}号按钮", self.id)
    }
}
struct Select { id: i32 }
impl Draw for Select {
    fn draw(&self) {
        println!("这个选择框贼难用{}", self.id)
    }
}
// 特征也是 DST 类型，而特征对象在做的就是将 DST 类型转换为固定大小类型。
fn trait_fn() {
    let elems: Vec<Box<dyn Draw>> = vec![Box::new(Button{id:1}), Box::new(Select{id:2})];
    for i in elems.iter(){
        i.draw();
    }
}

// Box::leak
// 消费掉 Box 并且强制目标值从内存中泄漏
// 可以把一个 String 类型，变成一个 'static 生命周期的 &str 类型
fn gen_static_str() -> &'static str{
    let mut s = String::new();
    s.push_str("hello, world");
    // 返回 'static &str 字符串切片
    Box::leak(s.into_boxed_str())
}
// 真正具有'static生命周期的往往都是编译期就创建的值，
// 例如 let v = "hello, world"，这里 v 是直接打包到二进制可执行文件中的，
// 再比如 const 常量。

// 使用场景：
// 需要一个在运行期初始化的值，但是可以全局有效，也就是和整个程序活得一样久