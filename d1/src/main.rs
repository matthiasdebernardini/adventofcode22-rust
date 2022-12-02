#[inpt::main]
fn main(i: String) {
    let mut elves: Vec<Vec<i32>> = vec![];
    let mut v: Vec<i32> = vec![];
    for l in i.lines() {
        match l.parse::<i32>() {
            Ok(n) => v.push(n),
            _ => {
                elves.push(v.clone());
                v.clear()
            }
        }
    }
    let mut max = 0;
    let mut best_elf = 0;
    for (i, e) in elves.iter().enumerate() {
        let calories:i32 = e.iter().sum();
        if calories > max {
            max = calories;
            best_elf = i;
        }
    }
    
    println!("{best_elf:?} {max}")
}
