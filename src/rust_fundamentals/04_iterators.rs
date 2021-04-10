pub struct Stepper {
    curr: i32,
    step: i32,
    max: i32,
}

impl Iterator for Stepper {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr >= self.max {
            return None
        }
        let res = self.curr;
        self.curr += self.step;
        Some(res)
    }
}

fn main() {
    // * Loop with match *
    let mut st = Stepper {
        curr: 2,
        step: 3,
        max: 15,
    };

    loop {
        match st.next() {
            Some(value) => println!("Value: {}", value),
            None => break,
        }
    }

    // * While Loop *
    let mut st2 = Stepper {
        curr: 2,
        step: 4,
        max: 15,
    };

    while let Some(n) = st2.next() {
        println!("2 {}", n);
    };

    // * For Loop *
    let st3 = Stepper {
        curr: 2,
        step: 4,
        max: 15,
    };

    for i in st3 {
        println!("3 iter - {}", i);
    }
}

fn loops() {
        let mut n = 0;
    loop {
        n += 1;
        if (n > 10) { break; };

        println!("Loop, {}", n);
    }

    let mut p = 0;
    while p < 10 {
        p += 1;
        println!("While {}", p)
    }

    for i in 0..=10 {
        println!("jj - {}", i)
    }
}
