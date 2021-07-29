fn integer_set_sum(nums: &[u32]) -> Option<u32> {
    let mut sum = 0_u32;
    for n in nums {
        let x = sum.checked_add(*n);
        if x.is_none() {
            return None;
        }

        sum = x.unwrap()
    }

    Some(sum)
}

fn main() {
    let nums: [u32; 2] = [2147483648,2147483648]; // overflow
    println!("{:?} = {:?}", nums, integer_set_sum(&nums));

    let nums: [u32; 2] = [128,256];
    println!("{:?} = {:?}", nums, integer_set_sum(&nums))
}
