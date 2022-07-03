// looked up
use std::io::BufRead;

fn main() {
    let np = std::io::stdin().lock().lines().next().unwrap().unwrap().parse::<i32>().unwrap();
    for _i in 0..np {
        let mut s_i = String::new();
        std::io::stdin().read_line(&mut s_i);
        let mut li = s_i.trim().split(" ").flat_map(str::parse::<i32>).collect::<Vec<_>>();
        if li.contains(&0) == false {
            li.sort();
            let mut modd = li.iter().sum::<i32>() / 9;
            if li.iter().sum::<i32>() % 9 == 0 && li[0] >= modd && li[1] >= modd && li[2] >= modd {
                println!("YES");
            } else {
                println!("NO");
            }
        } else {
            println!("NO");
        }
    }
}