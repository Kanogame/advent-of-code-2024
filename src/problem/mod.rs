use crate::generic_problem;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;

pub static MODULE_LIST: [fn() -> generic_problem::Day; 8] = [
    day1::init,
    day2::init,
    day3::init,
    day4::init,
    day5::init,
    day6::init,
    day7::init,
    day8::init,
];
