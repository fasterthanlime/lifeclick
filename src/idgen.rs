
pub fn idgen() -> i64 {
    static mut SEED: i64 = 0;
    unsafe {
        SEED += 1;
        SEED
    }
}