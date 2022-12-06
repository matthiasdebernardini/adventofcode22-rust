// input looks like
// 1
// 2
// 3

// 2
// 3
// 4

// 3
// 4
// 5
#[inpt::main]
fn main(i: String) {
    let mut v: Vec<i32> = vec![];
    let mut vv: Vec<Vec<i32>> = vec![];
    for l in i.lines() {
        match l.parse::<i32>() {
            Ok(n) => v.push(n),
            _ => {
                vv.push(v.clone());
                v.clear()
            }
        }
    }
    vv.iter_mut().for_each(|a: &mut Vec<i32>| {
        let b = a.iter().sum::<i32>();
        std::mem::replace(a, vec![b]);
    });
    vv.sort();
    vv.reverse();
    let top_three = vv.iter().take(3).flatten();
    let c = top_three.fold(0, |acc, x| acc + x);
    // let first = top_three.next().unwrap();
    // let second = top_three.next().unwrap();
    // let third= top_three.next().unwrap();
    println!("{c}")
}
