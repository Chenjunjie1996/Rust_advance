#![allow(warnings,unused)]

use std::collections::HashMap;
// 迭代器
// IntoIterator::into_iter 是使用完全限定的方式去调用 into_iter 方法，
// 跟 values.into_iter() 是等价的。
fn main() {
    let values = vec![1,2,3];
    {
        let result = match values.into_iter() {
            mut iter => loop {
                match iter.next() {
                    Some(x) => { println!("{}", x); },
                    None => break,
                }
            }
        };
        result
    }
    // into_iter 会夺走所有权
    let values = vec![1,2,3];
    for i in values.into_iter(){}
    // println!("{:?}", values);
    // iter 是借用
    let values = vec![1,2,3];
    for i in values.iter(){}
    // iter_mut 是可变借用
    let mut values = vec![1,2,3];
    for i in values.iter_mut() { *i += 1; }
    // into_ 之类的，都是拿走所有权，_mut 之类的都是可变借用，剩下的就是不可变借用。

    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();
    // sum 拿走了所有权
    let total: i32 = v1_iter.sum();

    // 消费者和适配器
    // 消费者是迭代器上的方法，它会消费掉迭代器中的元素，然后返回其类型的值
    // 这些消费者都有一个共同的特点：在它们的定义中，都依赖 next 方法来消费元素
    // 迭代器适配器会返回一个新的迭代器，这是实现链式方法调用的关键
    let v1 = vec![1,2,3];
    let v2: Vec<_> = v1.iter().map(|x| x+1).collect();
    assert_eq!(v2, vec![2,3,4]);

    // collect 方法 可以将一个迭代器中的元素收集到指定类型中
    let names = ["sunface", "sunfei"];
    let ages = [18, 18];
    let folks: HashMap<_, _> = names.into_iter().zip(ages.into_iter()).collect();
    println!("{:?}", folks);

    // 闭包作为适配器参数
    // map 方法中，我们使用闭包来作为迭代器适配器的参数
    // 可以就地实现迭代器中元素的处理，还在于可以捕获环境值：

    // 实现Iterator特征
    let mut counter = Counter::new();
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);

    // 默认实现
    let sum: u32 = Counter::new()
    .zip(Counter::new().skip(1))
    .map(|(a, b)| a * b)
    .filter(|x| x % 3 == 0)
    .sum();
    assert_eq!(18, sum); 

    let mut v = vec![1,2,3,4,5,6];
    for (mut i, v) in v.iter_mut().enumerate(){
        i += 1;
        println!("{}, {}", i, v);
    }

    let v = vec![1,2,3,4,5,6];
    let val = v.iter()
    .enumerate()
    .filter(|(idx,_)| idx%2==0)
    .map(|(idx, val)| val)
    .fold(0, |sum, acm| sum + acm);
    println!("{}", val);


}
struct Shoe{
    size: u32,
    style: String,
}
fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe>{
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

// 实现Iterator特征
struct Counter { count: u32 }
impl Counter {
    fn new() -> Self {
        Self { count: 0 }
    }
}
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}