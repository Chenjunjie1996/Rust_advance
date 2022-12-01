#![allow(unused,warnings)]
fn main() {
    // p1
    let i = 3;                                             
    {                                                    
        let borrow1 = &i;
        println!("borrow1: {}", borrow1);
    }
    {
        let borrow2 = &i; 
        println!("borrow2: {}", borrow2);               
    }

    // p2
    {
        let r;
        {
            let x= 5;
            // r = &x; 悬垂引用
            r = x;
        }
        println!("r: {}", r);
    }

    // p5
    let (four, nine) = (4, 9);
    print_refs(&four, &nine);
    // 这里，four 和 nine 的生命周期必须要比函数 print_refs 长
    failed_borrow();
    // `failed_borrow` 没有传入任何引用去限制生命周期 `'a`，因此，此时的 `'a` 生命周期是没有任何限制的，它默认是 `'static`

    // p6
    let x = 18;
    let y = 15;
    let single = Borrowed(&x);
    let double = NamedBorrowed { x: &x, y: &y };
    let reference = Either::Ref(&x);
    let number    = Either::Num(y);

    // p7
    {
        let var_a = 35;
        let example: Example;
        let var_b = NoCopyType{};
        example = Example{a: &var_a, b: &var_b};
        println!("(Success!) {:?}", example);
    }

    // p8 
    {
        let no_copy = NoCopyType {};
        let example = Example { a: &1, b: &no_copy };
        fix_me(&example);
        println!("Success!")
    }

    // 方法
    let mut owner = Owner(18);
    owner.add_one();
    owner.print();

    // 再引用

    let mut p = Point { x: 0, y: 0 };
    let r = &mut p;
    // reborrow! 此时对`r`的再借用不会导致跟上面的借用冲突
    let rr: &Point = &*r;
    // 再借用`rr`最后一次使用发生在这里，在它的生命周期中，我们并没有使用原来的借用`r`，因此不会报错
    println!("{:?}", rr);
    // 再借用结束后，才去使用原来的借用`r`
    r.move_to(10, 10);
    println!("{:?}", r);

}

// p3 
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// p4
fn invalid_output() -> String { 
    String::from("foo") 
}
fn invalid_output1() -> &'static str { 
    "foo"
}
fn invalid_output2<'a>(s: &'a String) -> &'a String {
    s
}
// p5
// `print_refs` 有两个引用参数，它们的生命周期 `'a` 和 `'b` 至少得跟函数活得一样久
fn print_refs(x: &i32, y: &i32) {
    println!("x is {} and y is {}", x, y);
}
fn failed_borrow<'a>() {
    let _x = 12;

    // ERROR: `_x` 活得不够久does not live long enough
    // let y: &'a i32 = &_x;
    let y: &i32 = &_x;

    // 在函数内使用 `'a` 将会报错，原因是 `&_x` 的生命周期显然比 `'a` 要小
    // 你不能将一个小的生命周期强转成大的
}

// p6 struct
#[derive(Debug)]
struct Borrowed<'a>(&'a i32);
#[derive(Debug)]
struct NamedBorrowed<'a> {
    x: &'a i32,
    y: &'a i32,
}
#[derive(Debug)]
enum Either<'a> {
    Num(i32),
    Ref(&'a i32)
}

// p7 struct
#[derive(Debug)]
struct NoCopyType{}
#[derive(Debug)]
struct Example<'a> {
    a: &'a u32,
    b: &'a NoCopyType,
}

// p8 
fn fix_me<'a>(foo: &'a Example<'a>) -> &'a NoCopyType
{ foo.b }

// 方法的生命周期
struct Owner(i32);
impl Owner {
    fn add_one(&mut self) {self.0 += 1;}
    fn print(&self){println!("print: {}", self.0);}
}

// p9
struct ImportantExcerpt2<'a>{
    part: &'a str
}
impl<'a> ImportantExcerpt2<'a> {
    fn level(&'a self) -> i32 {3}
}

// 生命周期消除规则
fn nput(x: &i32){ println!("annotated_input: {}", x); }
fn pass(x: &i32) -> &i32 {x}
fn longest2<'a>(x: &'a str, y: &'a str ) -> &'a str {x}

struct Owner1(i32);
impl Owner1 {
    fn add_one(&mut self) { self.0 += 1; }
    fn print(&self) {println!("{}", self.0);}
}

struct Person<'a> {
    age: u8,
    name: &'a str,
}
enum Either1<'a> {
    Num(i32),
    Ref(&'a i32),
}

// 再引用
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
impl Point {
    fn move_to(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
}


// Advanced p1
struct DoubleRef<'a, T>{
    r: &'a T,
    s: &'a T
}

// p2
struct ImportantExcerpt3<'a>{ part: &'a str }
impl<'a: 'b, 'b> ImportantExcerpt3<'a> {
    fn announce_and_return_part(&'a self, announcemet: &'b str) -> &'b str{
        println!("{}", announcemet);
        self.part
    }
}

// p3
fn f<'a, 'b>(x: &'a i32, mut y: &'b i32) where 'a: 'b{
    y = x;
    let r: &'b &'a i32 = &&0;
}

// p5
fn practice5(){
    let mut data = 5;
    let ref1 = &mut data;
    let ref2 = &mut *ref1;
    *ref2 += 2;
    *ref1 += 1;
    println!("{}", data);
}

// p6
struct Manager<'a>{ text: &'a str }
struct List<'a>{ manager: Manager<'a> }
impl<'a> List<'a> {
    pub fn get_interface<'b>(&'b mut self) -> Interface<'b,'a>
    where 'a: 'b {
        Interface { manager: &mut self.manager }
    }
}
struct Interface<'b, 'a: 'b> { manager: &'b Manager<'a> }
impl<'b, 'a: 'b> Interface<'b, 'a> {
    pub fn noop(self) {println!("interface consumed");}
}
fn practice6(){
    let mut list = List{ manager:Manager { text: "hello" } };
    list.get_interface().noop();
}
fn use_list(list: &List) { println!("{}", list.manager.text); }

//&'static 和 T: 'static 后者的使用形式会更加复杂一些。
// 如果你需要添加 &'static 来让代码工作，那很可能是设计上出问题了
// 如果你希望满足和取悦编译器，那就使用 T: 'static，很多时候它都能解决问题

fn static_compare() {let s: &'static str = "hello world";}
// 'static 也可以用于特征约束中:
fn generic<T>(x: T) where T: 'static {}
// 作为引用生命周期，&'static 说明该引用指向的数据跟程序活得一样久
