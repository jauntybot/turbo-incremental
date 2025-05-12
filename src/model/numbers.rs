use super::*;

pub struct Numbers {}
impl Numbers {
    pub fn format(num: u64) -> String {
        let mut num = num as f64;
        let mut suffix = "";

        if num >= 10_000.0 {
            if num >= 1_000_000_000.0 {
                num /= 1_000_000_000.0;
                suffix = "B";
            } else if num >= 1_000_000.0 {
                num /= 1_000_000.0;
                suffix = "M";
            } else if num >= 10_000.0 {
                num /= 1_000.0;
                suffix = "K";
            }
            // Format to the tenths place
            format!("{:.1}{}", num, suffix)
        } else {
            // For numbers under 10,000, return without decimals
            format!("{:.0}", num)
        }
    }
}