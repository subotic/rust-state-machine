mod balances;
mod system;

// This is our main Runtime.
// It accumulates all of the different pallets we want to use.
#[derive(Debug)]
pub struct Runtime {
    /* TODO: create a field `system` which is of type `system::Pallet`. */
    /* TODO: create a field `balances` which is of type `balances::Pallet`. */
    system: system::Pallet,
    balances: balances::Pallet,
}

impl Runtime {
    // Create a new instance of the main Runtime, by creating a new instance of each pallet.
    fn new() -> Self {
        /* TODO: Create a new `Runtime` by creating new instances of `system` and `balances`. */
        Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
        }
    }
}

fn main() {
    /* TODO: Create a mutable variable `runtime`, which is a new instance of `Runtime`. */
    let mut runtime = Runtime::new();

    /* TODO: Set the balance of `alice` to 100, allowing us to execute other transactions. */
    runtime.balances.set_balance(&"alice".to_string(), 100);

    // start emulating a block
    /* TODO: Increment the block number in system. */
    runtime.system.inc_block_number();

    /* TODO: Assert the block number is what we expect. */
    assert!(runtime.system.block_number() == 1);

    // first transaction
    /* TODO: Increment the nonce of `alice`. */
    runtime.system.inc_nonce(&"alice".to_string());
    /* TODO: Execute a transfer from `alice` to `bob` for 30 tokens.
        - The transfer _could_ return an error. We should use `map_err` to print
          the error if there is one.
        - We should capture the result of the transfer in an unused variable like `_res`.
    */
    let _res = runtime
        .balances
        .transfer("alice".to_string(), "bob".to_string(), 30)
        .map_err(|e| eprintln!("{}", e));

    // second transaction
    /* TODO: Increment the nonce of `alice` again. */
    runtime.system.inc_nonce(&"alice".to_string());

    /* TODO: Execute another balance transfer, this time from `alice` to `charlie` for 20. */
    let _res = runtime
        .balances
        .transfer("alice".to_string(), "charlie".to_string(), 20)
        .map_err(|e| eprintln!("{}", e));

    println!("{:#?}", runtime);
}
