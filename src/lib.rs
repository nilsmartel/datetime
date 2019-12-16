///
/// DateTime can store the year, month, day of month, hour and minute of a date
///
/// until the year 4096
#[derive(Copy, Clone, Debug)]
pub struct DateTime(u32);

impl DateTime {
    pub fn new(year: u32, month: u32, day: u32, hour: u32, minute: u32) -> {
        
    }

    pub fn year(self, year: u32) -> DateTime {
        self.0 &= !(0o777<< 8);
        self.0 |= (year & 0o777)<< 8;
        self
    }
}
