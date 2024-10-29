use steel::*;
use super::PdaRentPayerAccount;

/// This empty struct represents the payer vault account
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct RentVault {}

/// This empty struct represents the account
/// that the vault will pay for
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct NewAccount {}

account!(PdaRentPayerAccount, RentVault);
account!(PdaRentPayerAccount, NewAccount);