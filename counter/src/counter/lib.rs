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

    #[test]
    fn check_set_get_validity() {
        init();
        set(candid::Nat::from(100));
        assert_eq!(get(), candid::Nat::from(100));
    }

    #[test]
    fn check_initial_increment() {
        init();
        increment();
        assert_eq!(get(), candid::Nat::from(1));
    }

    #[test]
    fn check_setted_increment() {
        init();
        set(candid::Nat::from(1000));
        increment();
        assert_eq!(get(), candid::Nat::from(1001));
    }
}
