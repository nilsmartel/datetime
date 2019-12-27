///
/// DateTime can store the year, month, day of month, hour and minute of a date
///
/// until the year 4096
#[derive(Copy, Clone, Debug)]
pub struct DateTime(u32);

const OFFSET_YEAR: u32 = 8;
const MASK_YEAR: u32 = 0o777;

const OFFSET_MINUTE: u32 = 32 - 6;
const MASK_MINUTE: u32 = 0o77;

const OFFSET_HOUR: u32 = 32 - 6 - 4;
const MASK_HOUR: u32 = 0xF;

const OFFSET_DAY: u32 = 32 - 6 - 4 - 5;
const MASK_DAY: u32 = 0x1F;

const OFFSET_MONTH: u32 = 32 - 6 - 4 - 5 - 4;
const MASK_MONTH: u32 = 0xF;

impl DateTime {
    pub fn new(year: u32, month: u32, day: u32, hour: u32, minute: u32) -> {

    }

    pub fn year(self, year: u32) -> DateTime {
        self.0 &= !(MASK_YEAR<< MASK_YEAR);
        self.0 |= (year & MASK_YEAR)<< MASK_YEAR;
        self
    }

    pub fn month(self, month: u32) -> DateTime {
        self.0 &= !(MASK_MONTH<< MASK_MONTH);
        self.0 |= (month & MASK_MONTH)<< MASK_MONTH;
        self
    }
    pub fn day(self, day: u32) -> DateTime {
        self.0 &= !(MASK_DAY<< MASK_DAY);
        self.0 |= (day & MASK_DAY)<< MASK_DAY;
        self
    }
}
