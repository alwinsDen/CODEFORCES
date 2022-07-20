use std::io::BufRead;

fn main() {
 let mut n = std::io::stdin().lock().lines().next().unwrap().unwrap().parse::<i32>().unwrap();
 for _ in 0..n {
  let mut k = String::new();
  std::io::stdin().read_line(&mut k);
  let mut k = k.trim();
  let mut tcount = -1;
  let mut scount = Vec::new();
  let mut fln = String::new();
  let radix: u32 = 10;
  for (jm, jl) in k.chars().rev().enumerate() {
   if jm == k.len() - 1 { continue; }
   let mut sum = jl.to_digit(radix).unwrap() + (k.as_bytes()[k.len() - jm - 2] as char).to_digit
   (radix)
    .unwrap();
   if sum > 9 {
    tcount = jm as i32+1;
    break;
   } else {
    scount.push(k.len() as i32 - 1 - jm as i32);
   }
  }
  //loop for dealing with values greater that 10
  if tcount > -1 {
   let mut lec = 2;
   for (jm, jl) in k.chars().enumerate() {
    if lec==1 {lec-=1;continue;}
    if jm as i32 == k.len() as i32 - 1 -tcount {
     let sum = jl.to_digit(radix).unwrap() + (k.as_bytes()[jm+1] as char)
      .to_digit(radix).unwrap();
     print!("{}", sum);
     lec-=1;
    } else {
     print!("{}", jl);
    }
   }
  }

  //loop for dealing with  values less of equal to 9
  else if scount.len()>0 {
   let mut tec=2;
   for (jm,jl) in k.chars().enumerate(){
    if jm==1 {continue;}
    if jm==0{
     let sum = jl.to_digit(radix).unwrap()+(k.as_bytes()[jm+1] as char).to_digit(radix).unwrap();
     print!("{}",sum);
    } else {
     print!("{}",jl);
    }
   }
  }
  println!("");
 }
}