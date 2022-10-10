fn main() {
    let s = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim_end().to_owned()
    };

    let n = {
        let mut ws = s.split_whitespace();
        let n: i32 = ws.next().unwrap().parse().unwrap();
        n
    };

    let mut m1: Vec<i32> = Vec::new();
    let mut m2: Vec<i32> = Vec::new();

    for _ in 0..n {
        let s = {
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
            s.trim_end().to_owned()
        };

        let mut ws = s.split_whitespace();
        let n: i32 = ws.next().unwrap().parse().unwrap();
        let m: i32 = ws.next().unwrap().parse().unwrap();

        m1.push(n);
        m2.push(m);
    }

    println!("{:?}", m1);
    println!("{:?}", m2);
}
