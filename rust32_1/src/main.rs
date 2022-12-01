#![allow(warnings,unused)]
// 闭包
use std::thread;
use std::time::Duration;

fn main() {
    // 闭包是一种匿名函数，它可以赋值给变量也可以作为参数传递给其它函数，
    // 不同于函数的是，它允许捕获调用者作用域中的值
    let x = 1;
    let sum = |y| x + y;
    assert_eq!(3, sum(2));
    // sum：可以赋值给变量，允许捕获调用者作用域中的值。

        // 强度
    let intensity = 10;
    // 随机值用来决定某个选择
    let random_number = 7;
    // 开始健身
    workout(intensity, random_number);

    // 同一个功能的函数和闭包实现形式
    // fn add_one_v1 (x: u32) -> u32 { x + 1 }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x: u32| {x+1};
    let add_one_v4 = |x:u32| x + 1;

    // 捕获作用域中的值
    let x = 4;
    let equal_to_x = |z| z == x;
    // 函数不能访问x
    // fn equal_to_x(z: i32) -> bool {z == x};
    let y = 4;
    assert!(equal_to_x(y));

    // FnOnce
    let x = vec![1, 2, 3];
    fn_once1(|z|{z == x.len()});
    
    let result =  factory(3);
    let result1 = result(3);
    println!("{}", result1);
}
fn muuuuu(intensity: u32) -> u32 {
    println!("muuuu.....");
    thread::sleep(Duration::from_secs(2));
    intensity
}
fn workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            "今天活力满满，先做 {} 个俯卧撑!",
            muuuuu(intensity)
        );
        println!(
            "旁边有妹子在看，俯卧撑太low，再来 {} 组卧推!",
            muuuuu(intensity)
        );
    } else if random_number == 3 {
        println!("昨天练过度了，今天还是休息下吧！");
    } else {
        println!(
            "昨天练过度了，今天干干有氧，跑步 {} 分钟!",
            muuuuu(intensity)
        );
    }
}

// 把函数赋值给一个变量
fn workout1(intensity: u32, random_number: u32) {
    let action = muuuuu;
    if intensity < 25 {
        println!(
            "今天活力满满, 先做 {} 个俯卧撑!",
            action(intensity)
        );
        println!(
            "旁边有妹子在看，俯卧撑太low, 再来 {} 组卧推!",
            action(intensity)
        );
    } else if random_number == 3 {
        println!("昨天练过度了，今天还是休息下吧！");
    } else {
        println!(
            "昨天练过度了，今天干干有氧, 跑步 {} 分钟!",
            action(intensity)
        );
    }
}
// 闭包实现
// 只要修改闭包 action 的实现即可
fn workout2(intensity: u32, random_number: u32) {
    let action = || {
        println!("muuuu.....");
        thread::sleep(Duration::from_secs(22));
        intensity
    };
    if intensity < 25 {
        println!(
            "今天活力满满，先做 {} 个俯卧撑!",
            action()
        );
        println!(
            "旁边有妹子在看，俯卧撑太low，再来 {} 组卧推!",
            action()
        );
    } else if random_number == 3 {
        println!("昨天练过度了，今天还是休息下吧！");
    } else {
        println!(
            "昨天练过度了，今天干干有氧，跑步 {} 分钟!",
            action()
        );
    }

}

// 多个参数就 |param1, param2,...|
// |param1, param2,...| {
//     语句1;
//     语句2;
//     返回表达式
// }
// 1. 闭包中最后一行表达式返回的值，就是闭包执行后的返回值，
//    因此 action() 调用返回了 intensity 的值 10
// 2. let action = ||... 只是把闭包赋值给变量 action，
// 并不是把闭包执行后的结果赋值给 action，
// 因此这里 action 就相当于闭包函数，可以跟函数一样进行调用：action()


// 结构体中的闭包
// T: Fn(u32) -> u32 意味着 query 的类型是 T，
// 该类型必须实现了相应的闭包特征 Fn(u32) -> u32
struct Cacher<T>
where T: Fn(u32) -> u32,
{
    query: T,
    value: Option<u32>,
}
impl<T> Cacher<T>
where T: Fn(u32) -> u32,
{
    fn new(query: T) -> Cacher<T> {
        Self {
            query,
            value: None,
        }
    }
    // 先查询缓存值 `self.value`，若不存在，则调用 `query` 加载
    fn value(&mut self, arg:u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.query)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}

// 闭包捕获变量有三种途径，恰好对应函数参数的三种传入方式：
// 转移所有权、可变借用、不可变借用，因此相应的 Fn 特征也有三种：
fn fn_once<F>(func: F)
where 
    F: FnOnce(usize) -> bool,
{
    println!("{}", func(3));
    // println!("{}", func(4));
}
// func的类型F实现了Copy特征，调用时使用的是它的拷贝，没有所有权的转移。
fn fn_once1<F>(func: F)
where
    F: FnOnce(usize) -> bool + Copy,
{
    println!("{}", func(3));
    println!("{}", func(4)); 
}

// 如果你想强制闭包取得捕获变量的所有权，可以在参数列表前添加 move 
// 通常用于闭包的生命周期大于捕获变量的生命周期时，
// 例如将闭包返回或移入其他线程。
fn fn_move() {
    let v = vec![1,2,3];
    let handle = thread::spawn(move ||{
        println!("{:?}", v);
    });
    handle.join().unwrap();
}

// FnMut 它以可变借用的方式捕获了环境中的值，因此可以修改该值
fn fn_mut() {
    let mut s = String::new();
    let mut update_string = |str| s.push_str(str);
    update_string("hello");
    println!("{:?}",s);
}

fn fn_mut1() {
    let mut s = String::new();
    let update_string = |str| s.push_str(str);
    exec(update_string);
    println!("{:?}", s);
}
// update_string 实现了FnMut的特征
fn exec<'a, F: FnMut(&'a str)>(mut f: F) {
    f("hello")
}

// Fn 特征，它以不可变借用的方式捕获环境中的值
fn fn_fn(){
    let s = "hello, ".to_string();
    let update_string = |str| println!("{},{}",s, str);
    exec1(update_string);
    println!("{:?}", s);
}
fn exec1<F:Fn(String) -> ()>(f: F) {
    f("world".to_string())
}

// 三种 Fn 的关系
// 实际上，一个闭包并不仅仅实现某一种 Fn 特征，规则如下：
// 所有的闭包都自动实现了 FnOnce 特征，因此任何一个闭包都至少可以被调用一次
// 没有移出所捕获变量的所有权的闭包自动实现了 FnMut 特征
// 不需要对捕获变量进行改变的闭包自动实现了 Fn 特征
fn fn_relationship(){
    let s = String::new();
}
fn exec_fnonce<F: FnOnce()>(f: F) {
    f()
}
fn exec_fnmut<F: FnMut()>(mut f: F) {
    f()
}
fn exec_fn<F: Fn()>(f: F) {
    f()
}

// 闭包作为函数的返回值
fn factory(x: i32) -> Box<dyn Fn(i32) -> i32> {
    let num = 5;
    if x > 1 {
        Box::new(move |x| x + num)
    }else {
        Box::new(move |x| x - num)
    }
}