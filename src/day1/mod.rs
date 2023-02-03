pub fn solve() {
    let contents = include_str!("./input.txt");
    //Create a new u32 vector

    let mut maxval: u32 = 0;
    let mut currval: u32 = 0;

    for line in contents.lines() {
        if line.is_empty() {
            if currval > maxval {
                maxval = currval;
            }
            currval = 0;
            continue;
        }

        let res = line.parse::<u32>();
        if res.is_err() {
            println!(
                "Error failed to parse line: {} err: {}",
                line,
                res.unwrap_err()
            );
        } else {
            currval += res.unwrap();
        }
    }

    println!("Maxval is : {}", maxval);
}
