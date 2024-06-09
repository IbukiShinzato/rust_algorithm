use proconio::input;

//バブルソート
pub fn run() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    for i in 0..n {
        for j in 0..n {
            if a[j] > a[i] {
                let tmp = a[j];
                a[j] = a[i];
                a[i] = tmp;
            }
        }
    }

    trace(&a, n);
}

fn trace(a: &[usize], n: usize) {
    let mut res = vec![];
    for i in 0..n {
        res.push(format!("{}", a[i]));
    }
    println!("{}", res.join(" "));
}
