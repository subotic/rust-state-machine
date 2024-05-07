mod balances;
mod system;

// This is our main Runtime.
// It accumulates all of the different pallets we want to use.
#[derive(Debug)]
pub struct Runtime {
    system: system::Pallet<Self>,
    balances: balances::Pallet<Self>,
}

impl balances::Config for Runtime {
    type Balance = u128;
}

impl system::Config for Runtime {
    type AccountID = String;
    type BlockNumber = u32;
    type Nonce = u32;
}

impl Runtime {
    // Create a new instance of the main Runtime, by creating a new instance of each pallet.
    fn new() -> Self {
        Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
        }
    }
}

fn main() {
    let mut runtime = Runtime::new();
    let alice = "alice".to_string();
    let bob = "bob".to_string();
    let charlie = "charlie".to_string();

    runtime.balances.set_balance(&alice, 100);

    runtime.system.inc_block_number();
    assert!(runtime.system.block_number() == 1);

    runtime.system.inc_nonce(&alice);
    let _res = runtime
        .balances
        .transfer(alice.clone(), bob, 30)
        .map_err(|e| eprintln!("{}", e));

    runtime.system.inc_nonce(&alice);
    let _res = runtime
        .balances
        .transfer(alice, charlie, 20)
        .map_err(|e| eprintln!("{}", e));

    println!("{:#?}", runtime);
}
