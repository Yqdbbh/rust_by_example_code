// 控制流程
#![allow(unused_imports,unused,dead_code)]
use std::{result, vec};


fn if_else() {
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // let if 绑定
    let str = if number < 5 {
        "condition was true" // 不写分号，表示代码块返回最后的表达式值
    }else {
        "condition was false"
    };
    println!("{}", str);
}
#[allow(unused,dead_code)]
// loop 循环
fn fn_loop(){
    let mut counter = 0;
    loop{
        println!("counter: {}", counter);
        counter += 1;
        if counter == 3 {
            println!("break counter: {}", counter);
            break;
        }
    }

    // loop 嵌套
    let mut p =0;
    
    'outer : loop{  // 使用 'label 来标识标签
    println!("");
        p+=1;
        if(p>=10){
            break;
        }
        let mut q=0;
        'inner : loop{
            q+=1;
            if q>p{
                break; // 默认退出当前循环
            }

            let current = p*q;
            if current >30{
                continue 'outer; // 开始下一次循环
            }
            if current > 50{
                break 'outer; // 退出指定循环
            }
            
            print!("{:>2} x {:>2} = {:>2}\t",q,p,p*q);
        }
       
    }
    println!("");

    // loop 返回值

    let mut p =0;
    
    let result = 'outer : loop{  // 使用 'label 来标识标签
    println!("");
        p+=1;
        if(p>=10){
            break -1; // 这里是返回到 'outer, 是i32 类型
        }
        let mut q=0;
        'inner : loop{
            q+=1;
            if q>p{
                break ; // 默认退出当前循环 'inner 循环的返回值不需要，则为 () 类型
            }

            let current = p*q;
            // if current >30{
            //     continue 'outer; // 开始下一次循环
            // }
            if current > 50{
                break 'outer current; // 退出 'outer 循环，其返回值也应该是i32类型
            }
            
            print!("{:>2} x {:>2} = {:>2}\t",q,p,p*q);
        }
       
    };
    println!("return value {}", result);

    // let result = loop {
    //     counter += 1;
    //     if counter == 10 {
    //         break counter;
    //     }
    // };
    // println!("{}", result);
}

#[allow(unused,dead_code)]
fn fn_while(){
    // 条件满足时执行循环
    let mut n = 1;
    let mut result:Vec<String> = vec![];
    while n <101 {
        if n%15 == 0{
            result.push("fizzbuzz".to_string());
        }else if n%3 == 0{
            result.push("fizz".to_string());
        }else if n%5 == 0{
            result.push("buzz".to_string());
        }else{
            result.push(n.to_string());
        }
        n+=1;
    }
    println!("result: {:?}", result);
}

#[allow(unused,dead_code)]
/// for 循环
fn fn_for(){
    for n in 0..3{
        println!("{}", n);
    }

    // 0..3  --> [0,3)
    // 0..=3 --> [0,3]
    for n in 0..=3{
        println!("{}", n);
    }

    //  for 与迭代器
    let names = vec!["a","b","c"];
    for name in names.iter(){
        // iter() 借用元素，使用后元素不会改变
        println!("{}", name);
    }
    println!("{:?}",names); 

    let names = vec!["a","b","c"];
    for name in names.into_iter(){
        println!("{}", name);
    }
    // println!("{:?}",names); // into_iter() 后，names被移除了

    // iter_mut 可变的借用集合中的每个元素，集合本身需要是mut的，然后使用 *解引用
    let mut age = vec![10,20,30];
    for a in age.iter_mut(){
        *a+=1;
        println!("{}", a);
    }
    println!("{:?}",age);

}

#[allow(unused,dead_code)]
/// match 匹配
fn fn_match(){
    // match 匹配，类似switch, 不过需要覆盖所有的值
    let num = 3;
    match num {
        1 => println!("one"),
        2 => println!("two"),
        // 3 => println!("three"),
        _ => println!("other"),
    }

    // match 是表达式
    let num2 = match num{ 
        1 => 1,
        _ => 0
    };
    println!("num2: {}",num2);
}


/// match 结构
fn fn_match_decode(){
    // match 结构元组
    let triple = (1,2,3);
    println!("triple: {:?}", triple);

    match triple {
        //  将triple 结构，然后进行匹配，如果匹配成功，则执行对应的表达式
        (0,x,y)=>println!("zero x:{} y:{}",x,y),
        // 后三行都是成功的匹配，哪行在前，就执行哪行
        (1,..)=>println!("one, the rest doesn't matter"),
        (1,x,y)=>println!("one x:{} y:{}",x,y),
        (1,2,y)=>println!("one two y:{}",y),
        _=> println!("other"),
    }

    // match 结构枚举
    enum Color{
        Red,
        Green,
        Blue,
        Yellow
    }

    let color = Color::Yellow;
    match color {
        Color::Red => println!("red"),
        Color::Green => println!("green"),
        Color::Blue => println!("blue"),
        // 如果不加入这行，则可能产生不符合的匹配，将会编译失败， 如果match 已经完全覆盖了， _ 没必要写
        _ =>println!("other"), 
    }

    //
    // match 解构指针，解引用
    // 解引用使用 *
    // 解构使用 &、ref、 和 ref mut
    
    let reference  = &4;
    match reference {
        // 如果用 `&val` 这个模式去匹配 `reference`，就相当于做这样的比较：
        // `&i32`（译注：即 `reference` 的类型）
        // `&val`（译注：即用于匹配的模式）
        // ^ 我们看到，如果去掉匹配的 `&`，`i32` 应当赋给 `val`。
        // 译注：因此可用 `val` 表示被 `reference` 引用的值 4。
        4 => println!("111"),
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    //  todo 没理解这里的内容
    // let str1 = &"123";
    // match str1 {
    //     &"123" => println!("111"),
    //     val => println!("other"),
    // }

    // todo ref 模式
    let not_a_reference = 3; // i32 类型
    let ref is_a_reference=3;// &i32类型 使用 ref ，可以从字面量直接创建引用

    struct Point{
        x:i32,
        y:i32
    }
    // 解构struct
    
    fn match1(point:&Point){
        match point {
            Point { x, y: 0 } => println!("On the x axis at {}", x),
            Point { x: 0, y } => println!("On the y axis at {}", y),
            Point { x, y } => println!("On neither axis: ({}, {})", x, y),
        }
    }
    let point = Point { x: 0, y: 1 };
    let point1 = Point { x: 1, y: 0 };
    let point2 = Point { x: 1, y: 2 };
    let point3 = Point { x: 1, y: 1 };
    match1(&point);
    match1(&point1);
    match1(&point2);
    match1(&point3);
    // 卫语句 guard
    // 加上判断条件来过滤
    fn match2(point:&Point){
        match point {
            Point { x, y: 0 } => println!("On the x axis at {}", x),
            Point { x: 0, y } => println!("On the y axis at {}", y),
            Point { x, y } if x==y => println!("x==y: ({}, {})", x, y),
            Point { x, y } => println!("On neither axis: ({}, {})", x, y)
        }
    }
    match2(&point2);
    match2(&point3);

    // 匹配之后绑定
    // 实际上后面的这段代码的应用场景暂时还不太理解，
    // 那为什么不直接使用age呢？ 就算age 是一个复杂的类型，也可以拿到相关值吧
    let age = 15;
    match age {
        n @ 1..=18 => println!("{} is a child", n),
        n @ 19..=65 => println!("{} is an adult", n),
        n => println!("{} is a senior", n),
    }


}

fn fn_if_let(){
   // 将 `optional` 定为 `Option<i32>` 类型
    let optional = Some(7);

    match optional {
        Some(i) => {
            println!("This is a really long string and `{:?}`", i);
            // ^ 行首需要 2 层缩进。这里从 optional 中解构出 `i`。
            // 译注：正确的缩进是好的，但并不是 “不缩进就不能运行” 这个意思。
        },
        _ => {},
        // ^ 必须有，因为 `match` 需要覆盖全部情况。不觉得这行很多余吗？
    };

    let numer = Some(7);
    let letter:Option<i32> = None;
    let emoticon:Option<i32> = None;

    //  if let  构读作：若 `let` 将 `number` 解构成 `Some(i)`, 就执行后面的代码。
    if let Some(i) = letter {
        println!("letter {:?}!", i);
    } else if let Some(i) = emoticon {
        println!("emoticon {:?}!", i);
    } else if let Some(i) = numer {
        println!("number {:?}!", i);
    } else {
        println!("None");
    }

    enum Foo {
        Bar,
        Baz,
        Qux(u32)
    }

    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

      // 变量 a 匹配到了 Foo::Bar
    if let Foo::Bar = a {
        println!("a is foobar");
    }

    // if let 的好处是可用匹配枚举非参数话的变量，未实现 PartialEq 的枚举也能匹配
    // if Foo::Bar == a {
    //     println!("a is foobar");
    // }

    // 变量 b 没有匹配到 Foo::Bar，因此什么也不会打印。
    if let Foo::Bar = b {
        println!("b is foobar");
    }

    // 变量 c 匹配到了 Foo::Qux，它带有一个值，就和上面例子中的 Some() 类似。
    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }

    

}

/// while let 循环
fn while_let(){    
    // 将 `optional` 设为 `Option<i32>` 类型
    let mut optional = Some(0);

    // 重复运行这个测试。
    loop {
        match optional {
            // 如果 `optional` 解构成功，就执行下面语句块。
            Some(i) => {
                if i > 9 {
                    println!("Greater than 9, quit!");
                    optional = None;
                } else {
                    println!("`i` is `{:?}`. Try again.", i);
                    optional = Some(i + 1);
                }
                // ^ 需要三层缩进！
            },
            // 当解构失败时退出循环：
            _ => { break; }
            // ^ 为什么必须写这样的语句呢？肯定有更优雅的处理方式！
        }
    }


    // 将 `optional` 设为 `Option<i32>` 类型
    let mut optional = Some(0);

    // 这读作：当 `let` 将 `optional` 解构成 `Some(i)` 时，就
    // 执行语句块（`{}`）。否则就 `break`。
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
        // ^ 使用的缩进更少，并且不用显式地处理失败情况。
    }
    // ^ `if let` 有可选的 `else`/`else if` 分句，
    // 而 `while let` 没有。

}

fn main() {
    // if_else();
    // fn_loop();
    // fn_while();
    // fn_for();
    // fn_match();
    // fn_match_decode();
    // fn_if_let();
    while_let();
}
