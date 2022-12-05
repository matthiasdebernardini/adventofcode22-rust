#[inpt::main]
fn main(i: String) {
    let mut v: Vec<i32> = vec![];
    let mut max = 0;
    for l in i.lines() {
        match l.parse::<i32>() {
            Ok(n) => v.push(n),
            _ => {
                let calories = v.iter().sum();
                if calories > max {
                    max = calories;
                }
                v.clear()
            }
        }
    }
    println!("{max}")
}
