use std::io::BufRead;
use std::collections::HashSet;
fn main() {
    pub fn binary_search(arr: &Vec<i32>, x: i32) -> isize {
        let mut high = arr.len() as i32 - 1;
        let mut low = 0;
        while high >= low {
            let mut mid = low + (high - low) / 2;
            if arr[mid as usize] == x {
                return mid as isize;
            }
            if arr[mid as usize] > x {
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }
        return -1;
    }
    let mut k = std::io::stdin().lock().lines().next().unwrap().unwrap().parse::<i32>().unwrap();
    for _ in 0..k {
        let mut l1 = String::new();
        let mut l2 = String::new();
        std::io::stdin().read_line(&mut l1);
        std::io::stdin().read_line(&mut l2);
        let mut l1 = l1.trim().split(" ").flat_map(str::parse::<i32>).collect::<Vec<_>>();
        let mut l2 = l2.trim().split(" ").flat_map(str::parse::<i32>).collect::<Vec<_>>();
        let mut l3 = Vec::new();
        'csm: for i in 0..l1[0] {
            if i == l1[0 as usize] - 1 {
                break 'csm;
            }
            if l2[i as usize] < l2[i as usize + 1] {
                l3.extend([l2[i as usize], l2[i as usize + 1]]);
            }
        }
        l2.sort();
        let mut sorted_count = 0;
        let mut lst = 1000001;
        let mut hsl = HashSet::new();
        'rsm: for (n, k) in l3.iter().enumerate() {
            if n % 2 == 1 { continue; }
            let mut rt = binary_search(&l2, *k);
            if rt == -1 { continue; }
            if &l2[rt as usize] == k && l2[rt as usize + 1] == l3[n + 1] {
                hsl.insert(l2[rt as usize]);
                hsl.insert(l2[rt as usize + 1]);
                if lst==l2[rt as usize] {
                    lst=l2[rt as usize + 1];
                    continue 'rsm;
                }
                sorted_count += 1;
                lst=l2[rt as usize + 1];
            }
        }
        let srp = l1[0] - hsl.len() as i32;
        let nsrp = l1[1] - sorted_count;
        if srp <= nsrp {
           println!("YES");
        } else {
            println!("NO");
        }
    }
}