static mut CELL: u64 = 1;

#[ic_cdk_macros::update]
fn mul(n: u64) -> u64 {
    unsafe {
        CELL *= n * 3;
        CELL
    }
}

#[ic_cdk_macros::query]
fn read() -> u64 {
    unsafe { CELL }
}
