use ic_cdk::export::candid;

#[ic_cdk_macros::import(canister = "multiply")]
struct CounterCanister;

#[ic_cdk_macros::update]
async fn read() -> candid::Nat {
    CounterCanister::read.await.0
}
