use std::io::BufRead;
fn main() {
    let mut si = std::io::stdin().lock().lines().next().unwrap().unwrap().parse::<i32>().unwrap();
    for _ in 0..si {
        let mut li = std::io::stdin().lock().lines().next().unwrap().unwrap().parse::<i32>()
            .unwrap();
        if li%2==0 {
            println!("{} {} {}",li/2,0,0);
        } else {
            println!("-1");
        }
    }
}