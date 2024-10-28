pub fn function(){
    println!("called `my_mod::my_sub_mod::public_function()`");
}

fn private_function(){
    println!("called `my_mod::my_sub_mod::private_function()`");
}

pub fn public_in_parent_mod(){
    println!("called `my_mod::my_sub_mod::public_in_parent_mod()`");
    function();
    private_function();
}

// 使用 `pub(in path)` 语法定义的函数只在给定的路径中可见。
// `path` 必须是父模块（parent module）或祖先模块（ancestor module）
pub(in crate::my_mod) fn public_function_in_my_mod() {
    print!("called `my_mod::nested::public_function_in_my_mod()`, that\n > ");
    public_function_in_nested()
}

// 使用 `pub(self)` 语法定义的函数则只在当前模块中可见。
pub(self) fn public_function_in_nested() {
    println!("called `my_mod::nested::public_function_in_nested");
}

// 使用 `pub(super)` 语法定义的函数只在父模块中可见。
pub(super) fn public_function_in_super_mod() {
    println!("called my_mod::nested::public_function_in_super_mod");
}