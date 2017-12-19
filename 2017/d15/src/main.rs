fn main() {
    // let gen_a_init = 65; // test
    // let gen_b_init = 8921; // test

    let gen_a_init = 277;
    let gen_b_init = 349;
    let mut gen_a = Generator::new_a(gen_a_init);
    let mut gen_b = Generator::new_b(gen_b_init);
    let res1 = judge(&mut gen_a, &mut gen_b, 40 * 1000 * 1000, false);
    println!("Solution 1: {}", res1);

    let mut gen_a = Generator::new_a(gen_a_init);
    let mut gen_b = Generator::new_b(gen_b_init);
    let res2 = judge(&mut gen_a, &mut gen_b, 5 * 1000 * 1000, true);

    println!("Solution 2: {}", res2);
}

fn judge(gen_a: &mut Generator, gen_b: &mut Generator, amount : usize, picky : bool) -> u32 {
    let div = 1 << 16;
    let mut total = 0;
    for i in 0..amount {
        let a = if picky {
            gen_a.picky_calculate()
        } else {
            gen_a.calculate()
        };
        let b = if picky {
            gen_b.picky_calculate()
        } else {
            gen_b.calculate()
        };
        if (a % div) == (b % div) {
            total += 1;
        }
    }
    total
}

struct Generator {
    status : u32, 
    factor : u32,
    disc : u32,
}

impl Generator {
    fn calculate(&mut self) -> u32 {
        let mut tmp : u64 = 0;
        tmp = (self.status as u64) * (self.factor as u64);
        tmp = tmp % 2147483647;
        self.status = tmp as u32;
        tmp as u32
    }

    fn picky_calculate(&mut self) -> u32 {
        let mut res = self.calculate();
        while res % self.disc != 0 {
            res = self.calculate();
        }
        res
    }

    fn new_a(starting : u32) -> Generator {
        Generator{status : starting, factor : 16807, disc : 4}
    }

    fn new_b(starting : u32) -> Generator {
        Generator{status : starting, factor : 48271, disc : 8}
    }
}

#[test]
fn test_generating() {
    let mut genA = Generator::new_A(65);
    let mut genB = Generator::new_B(8921);
    assert_eq!(genA.calculate(), 1092455);
    assert_eq!(genA.calculate(), 1181022009);
    assert_eq!(genA.calculate(), 245556042);
    assert_eq!(genA.calculate(), 1744312007);
    assert_eq!(genA.calculate(), 1352636452);

    assert_eq!(genB.calculate(), 430625591);
    assert_eq!(genB.calculate(), 1233683848);
    assert_eq!(genB.calculate(), 1431495498);
    assert_eq!(genB.calculate(), 137874439);
    assert_eq!(genB.calculate(), 285222916);
}
