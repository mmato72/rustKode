const SECONDS_IN_A_MINUTE: u32 = 60;
const MINUTES_IN_AN_HOUR: u32 = 60;
const HOURS_IN_A_DAY: u32 = 24;
const SECONDS_IN_A_DAY: u32 = SECONDS_IN_A_MINUTE * MINUTES_IN_AN_HOUR * HOURS_IN_A_DAY;


fn main() {
    println!("{}", SECONDS_IN_A_DAY);
}
