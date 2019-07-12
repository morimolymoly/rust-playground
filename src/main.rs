#![feature(unboxed_closures)]
#![feature(fn_traits)]

mod closure;
mod ownership;
mod neko;
use neko::Animal;

fn main() {
    println!("Hello, world!");

    let mut s = String::from("uooo");
    s.push_str(", string");
    println!("push str mutable string {}", s);

    let mut x = 5;
    let y = x;
    x = 10;
    println!("x is {} y is {}", x, y);

    let s1 = String::from("test");
    let s2 = s1.clone();
    println!("cloning {} {}", s1, s2);

    ownership::take_ownership(s1);
    //println!("{} {}", s1, s2);

    let mut s3 = ownership::take_ownership_and_give(s2);
    println!("s3 {}", s3);

    ownership::not_take_ownership(&s3);
    ownership::not_take_ownership_but_mutable(&mut s3);
    println!("not_take_ownership_but_mutable {}", s3);

    /*
    let mr_s = &mut s3;
    let mr_s2 = &mut s3;
    println!("mr {} mr2 {}", mr_s, mr_s2);*/
    /*
    let r_s = &s3;
    r_s.push_str("aa");
    println!("r_s {}", r_s);*/
    {
        let mut s100 = String::from("test");
        let s101 = &s100;
        let s102 = &s100;
        //let s103 = &mut s100;
        //println!("borrow mixed {} {} {}", s101, s102, s103);
        println!("borrow not mixed {} {}", s101, s102);
    }

    {
        let mut s100 = String::from("test");
        let s101 = &s100;
        let s102 = &s100;
        println!("borrow not mixed {} {}", s101, s102);

        let s103 = &mut s100;
        println!("borrow {}", s103);
    }
    {
        let s = String::from("hello world!");
        let sr = &s[0..2];
        //s.clear();
    }

    let taro = neko::Neko{
        age: 10,
        name: String::from("taroo"),
    };
    taro.Naku();
    //taro.GetOld();

    let mut taro2 = neko::Neko{
        age: 10,
        name: String::from("mutable tarooo"),
    };
    taro2.Naku();
    taro2.GetOld();
    taro2.Naku();
    taro2.Die();

    {
        let test: u64 = 10;
        ownership::take_and_print(&test);
    }

    {
        let lambda = |arg: &String|
        {
            println!("{}", arg);
        };
        let mut a = String::from("test");
        lambda(&a);
        a.push_str("aaa");
        lambda(&a);
    }

    println!("closure call_once trait");
    let x = 1;
    for i in 0..4 {
        let c = closure::Closure{i: i};
        println!("{}", c(x));
    }

    println!("closure2 Fn + FnOnce + FnMut");
    let c2 = closure::Closure2{i: x};
    for i in 0..4 {
        println!("{}", c2(i));
    }
}
