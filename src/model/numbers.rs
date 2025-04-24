use super::*;

pub struct Numbers {}
impl Numbers {
    pub fn format(num: u32) -> String {
        let mut num = num as f64;
        let mut suffix = "";
        if num >= 1_000_000_000.0 {
            num /= 1_000_000_000.0;
            suffix = " B";
        } else if num >= 1_000_000.0 {
            num /= 1_000_000.0;
            suffix = " M";
        } else if num >= 1_000.0 {
            num /= 1_000.0;
            suffix = "k";
        } else {
            // For numbers under 1,000, return without decimals
            return format!("{:.0}", num);
        }
        format!("{:.2}{}", num, suffix)
    }
}