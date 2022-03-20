use ic_cdk::export::candid;
use ic_cdk_macros::*;

static mut COUNTER: Option<candid::Nat> = None;

#[init]
fn init() {
    unsafe {
        COUNTER = Some(candid::Nat::from(0));
    }
}

#[update]
fn increment() -> () {
    unsafe {
        COUNTER.as_mut().unwrap().0 += 1u64;
    }
}

#[query]
fn get() -> candid::Nat {
    unsafe { COUNTER.as_mut().unwrap().clone() }
}

#[update]
fn set(input: candid::Nat) -> () {
    unsafe {
        COUNTER.as_mut().unwrap().0 = input.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_initial_get() {
        init();
        assert_eq!(get(), candid::Nat::from(0));
    }
}
