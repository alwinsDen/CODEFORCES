use std::io::BufRead;
use std::collections::HashSet;
fn main() {
    let mut n=std::io::stdin().lock().lines().next().unwrap().unwrap().parse::<i32>().unwrap();
    for _ in 0..n{
        let mut k=String::new();
        let mut l=String::new();
        std::io::stdin().read_line(&mut k);
        std::io::stdin().read_line(&mut l);
        let mut k=k.trim().split(" ").flat_map(str::parse::<i32>).collect::<Vec<_>>();
        let mut l=l.trim().split(" ").flat_map(str::parse::<i32>).collect::<Vec<_>>();
        let mut poss = Vec::new();
        for jm in l.iter() {
            if *jm>k[1]+1-jm{
                poss.push(k[1]+1-jm);
                poss.push(*jm);
                continue;
            }
            poss.push(*jm);
            poss.push(k[1] + 1 - jm);
        }
        // println!("{:?}",poss);
        let mut cnt=0;
        let mut cnp = Vec::new();
        for (p1,p2) in poss.iter().enumerate(){
            if p1%2!=0 {continue;}
            if cnp.contains(p2) && cnp.contains(&poss[p1+1]) {
                continue;
            }
            if cnp.contains(p2){
                cnp.push(poss[p1+1]);
                continue;
            } else{
                cnp.push(*p2);
                continue;
            }
        }
        cnp.sort();
        for jk in 0..k[1] {
            if cnp.contains(&(jk +1)) {
                print!("A");
            } else {
                print!("B")
            }
        }
        println!("");
    }
}