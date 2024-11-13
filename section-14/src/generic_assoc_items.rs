#![allow(unused,dead_code)]
///
/// 关联类型
/// 

struct Container(i32,i32);

trait Contains{
    type A;
    type B;
    fn contains(&self,_:&Self::A,_:&Self::B)->bool;
    fn first(&self)->&Self::A;
    fn second(&self)->&Self::B;
}


impl Contains for Container{
    type A = i32;
    type B = i32;
    fn contains(&self,num1:&Self::A,num2:&Self::B)->bool{
        (&self.0==num1)&&(&self.1==num2)
    }
    fn first(&self)->&Self::A{
        &self.0
    }
    fn second(&self)->&Self::B{
        &self.1
    }    
}


pub fn test(){
    let c = Container(10,20);
    assert!(c.contains(&10,&20));
    assert!(!c.contains(&10,&21));
    assert_eq!(c.first(),&10);
    assert_eq!(c.second(),&20);
}