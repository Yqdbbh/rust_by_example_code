#![allow(unused,dead_code)]

struct S;
struct GenericVal<T>(T); //泛型定义

impl GenericVal<i32>{}

impl GenericVal<S>{}

impl <T> GenericVal<T>{}

pub struct Val{
    pub val: i32
}   

pub struct GenVal<T> {
    pub gen_val:T
}

impl Val  {
    pub  fn value(&self)->&i32{&self.val}
}

 impl<T> GenVal<T> {
    pub    fn value(&self)->&T{&self.gen_val}
}