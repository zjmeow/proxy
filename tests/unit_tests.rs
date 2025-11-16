use mollusk_svm::Mollusk;
use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;
extern crate alloc;

use solana_sdk::rent::Rent;
use solana_sdk::sysvar::Sysvar;

pub const PROGRAM: Pubkey = pubkey!("SysvarRent111111111111111111111111111111111");

pub const RENT: Pubkey = pubkey!("SysvarRent111111111111111111111111111111111");

pub const PAYER: Pubkey = pubkey!("41LzznNicELmc5iCR9Jxke62a3v1VhzpBYodQF5AQwHX");

pub fn mollusk() -> Mollusk {
    let mollusk = Mollusk::new(&PROGRAM, "target/deploy/proxy");
    mollusk
}

pub fn get_rent_data() -> Vec<u8> {
    let rent = Rent::default();
    unsafe {
        core::slice::from_raw_parts(&rent as *const Rent as *const u8, Rent::size_of()).to_vec()
    }
}
#[test]
fn test_get_rent_data() {
    println!("{:?}", get_rent_data());
}