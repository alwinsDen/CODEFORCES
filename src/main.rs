use std::io::BufRead;
use std::process::exit;

fn binary_search(arr: &Vec<i32>, x: i32) -> i32 {
    let mut high = (arr.len() - 1) as i32;
    let mut low = 0;
    while high >= low {
        let mut mid = low + (high - low) / 2;
        if arr[mid as usize] == x {
            return mid;
        }
        if arr[mid as usize] > x {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }
    -1
}

fn main() {
    let mut m = std::io::stdin().lock().lines().next().unwrap().unwrap().parse::<i32>().unwrap();
    let mut l1 = String::new();
    std::io::stdin().read_line(&mut l1);
    let mut l1 = l1.trim().split(" ").flat_map(str::parse::<i32>).collect::<Vec<_>>();
    l1.sort();
    if m == 2 {
        println!("1 1");
        exit(0);
    }
    let mut cl1 = l1.clone();
    let mut val_list: Vec<i32> = Vec::new();
    for l in 0..l1[l1.len() - 1] + 1 {
        if l != 0 && l1[l1.len() - 1] % l == 0 {
            val_list.push(binary_search(&cl1, l));
        }
    }
    // println!("{}",cl1.len());
    let mut offset = 0;
    for (m, j) in val_list.iter().enumerate() {
        cl1.remove(*j as usize - offset as usize);
        offset += 1;
    }
    println!("{:?} {:?}", l1[l1.len() - 1], cl1[cl1.len()-1]);
}