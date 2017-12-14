fn main() {
    let input = include_str!("../input/input");
    let mut firewall = Vec::new();
    for line in input.lines() {
        let mut split = line.split(": ");
        let pos: u32 = split.next().unwrap().parse().unwrap();
        let depth: u32 = split.next().unwrap().parse().unwrap();
        firewall.push((pos, depth));
    }
    let mut found = false;
    let mut step = 0;
    'try: while !found {

        if step % 500 == 0 {
            println!("Running... [{}]", step);
        }
        for entry in &firewall {
            let (pos, depth) = *entry;

            if (pos + step) % ((depth - 1) * 2) == 0 {
                step += 1;
                continue 'try;
            }

        }
        found = true;
        step += 1;
    }
    println!("result: {}", step - 1);
}
