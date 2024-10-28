pub mod my_sub_mod;

/// 默认的项是私有的
fn private_function(){
    println!("called `my_mod::private_function()`");
}

/// 使用pub关键字来修改可见性
pub fn function(){
    println!("called `my_mod::public_function()`");
}

/// 内部成员访问私有对象
pub fn indirect_access(){
    println!("called `my_mod::indirect_access()`");
    private_function();
}

pub fn call_public_function_in_my_mod() {
    print!("called `my_mod::call_public_funcion_in_my_mod()`, that\n> ");
    my_sub_mod::public_function_in_my_mod();
    print!("> ");
    my_sub_mod::public_function_in_super_mod();
}

// `pub(crate)` 使得函数只在当前 crate 中可见
pub(crate) fn public_function_in_crate() {
    println!("called `my_mod::public_function_in_crate()");
}

  // 嵌套模块的可见性遵循相同的规则
mod private_nested {
    #[allow(dead_code)]
    pub fn function() {
        println!("called `my_mod::private_nested::function()`");
    }
}
