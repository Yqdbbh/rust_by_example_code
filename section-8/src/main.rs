// 控制流程

use std::result;

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

fn fn_for(){
    // todo
}

fn main() {
    // if_else();
    // fn_loop();
    fn_while();
}
