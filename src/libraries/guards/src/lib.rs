use candid::Principal;

pub fn caller_is_authenticated() -> Result<(), String> {
    let caller = ic_cdk::caller();

    if caller == Principal::anonymous() {
        return Err("Caller is not authenticated".to_string());
    }

    Ok(())
}
