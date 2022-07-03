// never solved
use std::io::{BufRead};
use std::collections::VecDeque;
fn main() {
    let ins = std::io::stdin().lock().lines().next().unwrap().unwrap().parse::<i32>().unwrap();
    let mut si = String::new();
    std::io::stdin().read_line(&mut si);
    let mut sil = si.trim().split(" ").flat_map(str::parse::<i32>).collect::<VecDeque<_>>();
    let mut dollar = 0;
    sil.push_front(0);
    for index in 0..sil.len() {
        if index!=sil.len()-1 && sil[index] - sil[index + 1] < 0  {
            if index as i32 != sil.len() as i32 -2 {
                dollar += (sil[index] - sil[index + 1]).abs();
                println!("dollar {}",dollar);
            } else {
                dollar += (sil[index]-sil[index+1]).abs();
                println!("dollars {}",dollar);
            }
        }
    }
    println!("{}",dollar);
}