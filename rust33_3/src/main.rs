#![allow(unused, warnings)]
#[derive(Debug)]
enum EnumToInt {
    A(i32),
    B(i32),
}

fn main() {
    // 枚举转整数
    let a = EnumToInt::A(4);
    match a {
        EnumToInt::A(i) => println!("{}", i),
        EnumToInt::B(_) => todo!(),
    }


    let x = MyEnum::C;
    let y = x as i32;
    let z: MyEnum = unsafe { std::mem::transmute(y) };

    // match the enum that came from an int
    match z {
        MyEnum::A => { println!("Found A"); }
        MyEnum::B => { println!("Found B"); }
        MyEnum::C => { println!("Found C"); }
    }
}

// 整数转换为枚举
#[repr(i32)]
enum MyEnum {
    A = 1, B, C
}
