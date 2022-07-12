// 整数结合求和
fn sum_t(temp_t: &[u32]) -> Option<u32> {
    let mut mm = 0u32;

    for x in temp_t.iter() {
        match mm.checked_add(*x) {
            None => return None,
            Some(t) => mm += x,
        }
    }
    Some(mm)
}

fn main() {
    let ttt = vec![4u32; 10];
    let temp_m = sum_t(&ttt);
    match temp_m {
        Some(t) => println!("{:?} sum is  {}", ttt, t),
        None => println!("None"),
    }

    let fff = vec![4294967295u32; 2];
    let temp_fff = sum_t(&fff);
    match temp_fff {
        Some(t) => println!("{}", t),
        None => println!("{:?} u32 sum overflow is None", fff),
    }
}
