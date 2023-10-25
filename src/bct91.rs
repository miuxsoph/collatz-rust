use std::{thread::sleep, time::Duration};

fn mc91(n: i32) -> i32 {
    if n > 100 {
        return n - 10;
    } else {
        return mc91(mc91(n + 11));
    }
}

fn main() {
    let mut q = "100100100100100".to_string();
    let mut r = "101110101011011101001110101110101110100000".to_string();
    let w = 3;

    while q[w..].len() > 0 {
        if r.chars().next() == Some('0') {
            q = q.chars().skip(1).collect();
        } else {
            r = r.chars().skip(1).chain(r.chars().take(1)).collect();
            if q.chars().next() == Some('1') {
                q.push(r.chars().next().unwrap());
            }
        }
        r = r.chars().skip(1).chain(r.chars().take(1)).collect();
        println!("{}", q);
        sleep(Duration::from_secs_f32(0.03));
        q = mc91(q.parse().unwrap()).to_string();
    }
}
