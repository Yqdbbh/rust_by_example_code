#![allow(dead_code,unused)]
use std::{fmt::{Debug, Display}, task::ready};
///
/// 泛型约束
/// 


fn printer<T:Display>(t:T){
    println!("{}",t);
}

struct S<T:Display>(T);
// const s = S(vec![1]);
// const  s:S<i32> = S(1);

trait HasArea{
    fn area(&self)->f64;
}

impl HasArea for Rectangle{
    fn area(&self)->f64 {
        self.width * self.height
    }
}

#[derive(Debug)]
struct Rectangle{
    width:f64,
    height:f64
}

struct Triangle{
    width:f64,
    height:f64
}

fn print_debug<T:Debug>(t:T){
    println!("{:?}",t);
}

fn area<T:HasArea>(t:&T)->f64{
    t.area()
}

/// 空约束, 可以作为特殊的标记
/// 

trait Red {}
trait Blue {
    
}

struct Cardinal;
struct BlueJay;
struct TurKey;

fn red<T:Red>(_:T)->&'static str{"red"}

fn blue<T:Blue>(_:T)->&'static str{"blue"}

// 多重约束, 需要同时满足才可以调用
fn purple<T:Red+Blue>(_:T)->&'static str{"purple"} 

impl Red for Cardinal {
    
}

impl Red for TurKey {
    
}

impl  Blue for BlueJay {
    
}

impl  Blue for TurKey {
    
}

pub fn test(){
    let rect = Rectangle{
        width:30.0,
        height:20.0
    };
    let _triangle = Triangle{
        width:30.0,
        height:20.0
    };

    print_debug(&rect);
    println!("area:{}",area(&rect));
    // print_debug(&_triangle);

    let cardinal = Cardinal;
    let blue_jay = BlueJay;
    let turkey = TurKey;
    red(cardinal);
    // blue(cardinal);
    // red(blue_jay);
    blue(blue_jay);

    purple(turkey);
}