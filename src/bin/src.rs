
use rand::{thread_rng, Rng};

fn main(){

    let f: f64 = thread_rng().gen::<f64>();
    let fa: f64 = thread_rng().gen::<f64>();
    let faa: f64 = thread_rng().gen::<f64>();
    let faaaaa: f64 = thread_rng().gen::<f64>();

    println!("{:?}",f);
    println!("{:?}",fa);
    println!("{:?}",faaaaa);
    println!("{:?}",faa);
    println!("{:?}",thread_rng().gen_range(0..=1));

}