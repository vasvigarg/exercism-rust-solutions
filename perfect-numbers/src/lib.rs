#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None; 
    }

    let mut sum = 1; 
    let sqrt = (num as f64).sqrt() as u64;

    for i in 2..=sqrt {
        if num % i == 0 {
            sum += i;
            let paired = num / i;
            if paired != i {
                sum += paired;
            }
        }
    }

    match sum.cmp(&num) {
        std::cmp::Ordering::Less => Some(Classification::Deficient),
        std::cmp::Ordering::Equal => Some(Classification::Perfect),
        std::cmp::Ordering::Greater => Some(Classification::Abundant),
    }
}
