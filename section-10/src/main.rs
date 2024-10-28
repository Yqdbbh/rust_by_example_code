#![allow(dead_code,unused)]
/// 模块系统
/// 

mod my_mod;
fn function(){
    println!("called `function()`");
}

/// 解构体的可见性
mod struct_mod{

    // 默认所有的字段都是私有的
    struct Point{
        x: i32,
        y: i32
    }

    // 公有结构体，带有公有字段
    pub struct OpenBox<T>{
        pub contents: T
    }

    // 公有结构体，私有字段
    pub struct ClosedBox<T>{
        contents: T
    }

    // 为公有结构体定义一个公有的构造方法
    impl <T> ClosedBox<T> {
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox {
                contents
            }
        }
    }

}

mod deeply{
    pub mod nested {
        pub fn function(){
            println!("called `deeply::nested::function()`")
        }
    }
}

// use 可以对路径进行重命名
use deeply::nested::function as deeply_function;

fn main() {

    // 可见性
    function();
    my_mod::function();
    my_mod::indirect_access();
    my_mod::call_public_function_in_my_mod();
    my_mod::public_function_in_crate();
    my_mod::my_sub_mod::function();
    // my_mod::private_function();

    // 结构体的可见性
    // let open_box = struct_mod::OpenBox{
    //     contents: "public information"
    // };
    // // let close_box = struct_mod::ClosedBox{
    // //     contents: "classified information" // private
    // // };
    // let close_box = struct_mod::ClosedBox::new( "classified information");
    // println!("the inner content is: {}", close_box.contents);

    // use 遮蔽
    // deeply::nested::function();
    // deeply_function();
    // println!("Enter block");
    // {
    //     // use 重命名路径可以遮蔽外部同名函数，只在作用域内遮蔽
    //     use deeply::nested::function;
    //     function();
    //     println!("leaving block");
    // }
    // function();

}
