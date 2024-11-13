/// 泛型
/// 
pub mod generic_val;
pub mod generic_trait;
pub mod generic_bound;
pub mod generic_assoc_items;
fn main() {
    // let x = generic_val::Val{val:4};
    // let y = generic_val::GenVal{gen_val:"hello"};
    // println!("x:{},y:{}",x.value(),y.value());

    // generic_trait::test();
    // generic_bound::test();
    generic_assoc_items::test();
}
