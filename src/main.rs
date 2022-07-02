use std::io::BufRead;
fn main(){
    let it = std::io::stdin().lock().lines().next().unwrap().unwrap().parse::<i32>().unwrap();
    for _i in 0..it {
        let mut nt = std::io::stdin().lock().lines().next().unwrap().unwrap().parse::<i32>()
            .unwrap();
        let mut nstr = String::new();
        std::io::stdin().read_line(&mut nstr);
        let mut nvec = nstr.trim().split(" ").flat_map(str::parse::<i32>).collect::<Vec<_>>();
        let mut even = vec!{};
        let mut odd = vec!{};
        for (index,val) in nvec.iter().enumerate() {
            if val%2==0 {
                even.push(index+1)
            } else {
                odd.push(index+1)
            }
        }
        if odd.len()%2!=0 {
            odd.pop();even.pop();
        } else if even.len()>1 {
            even.pop();even.pop();
        } else {
            odd.pop();odd.pop();
        }
        odd.extend(even);
        for mut i_o in 0..odd.len() {
            if i_o%2!=0 {continue;}
            println!("{} {}",odd[i_o], odd[i_o+1]);
        }
    }
}