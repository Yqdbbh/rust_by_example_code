/// 变量绑定

fn mut_binding() {
    let immutable_binding = 1;
    //immutable_binding = 2;  // 不加mut 不可变
    println!("immutable_binding: {}", immutable_binding);

    let mut mutable = 1;
    mutable=2; // 可变
    mutable=3;
    println!("mutable: {}", mutable); 

}

///
/// 变量作用域
fn scope() {
    //变量作用域限定在一个代码块("{}")中, 内部的声明的变量会遮蔽(隐藏)同名的外部变量

    let mut x =1;
    {
        x = 2;
        // 此时 x 取外面的值
        println!("x: {}", x);
        // 重新声明 x，
        let mut x: i32 = 3;
        println!("x: {}", x);
        x = 4;
        // x 被遮蔽，再次更改也不影响外面的值
        println!("x: {}", x);
        let y: i32 = 2;
        println!("y: {}", y);
    } // 内部的 x 也被销毁
    // println!("y: {}", y); // 已经失去作用域
    println!("x: {}", x);

}

/// 冻结
fn freeze(){
    // 私以为冻结为遮蔽的一种，是不可变变量遮蔽可变变量的情况
    let mut x = 1;
    {
        let x = 2; // 遮蔽可变变量
        // x = 3; // 赋值失败，不可变 immutable x
    }
    x = 3; // 可变 mutable x
}

fn main() {
    mut_binding();
    scope();
}
