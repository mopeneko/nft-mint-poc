#![cfg_attr(all(not(test), not(feature = "export-abi")), no_main)]

#[cfg(feature = "export-abi")]
fn main() {
    contract::main();
}

#[cfg(test)]
fn main() {}
