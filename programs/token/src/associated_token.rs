use {
    mollusk_svm::Mollusk,
    solana_sdk::{account::AccountSharedData, pubkey::Pubkey},
};

pub const ID: Pubkey = solana_sdk::pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");

pub const ELF: &[u8] = include_bytes!("elf/associated_token.so");

pub fn add_program(mollusk: &mut Mollusk) {
    // Loader v2
    mollusk.add_program_with_elf_and_loader(
        &ID,
        ELF,
        &mollusk_svm::program::loader_keys::LOADER_V2,
    );
}

pub fn account() -> AccountSharedData {
    // Loader v2
    mollusk_svm::program::create_program_account_loader_v2(ELF)
}

/// Get the key and account for the SPL Associated Token program.
pub fn keyed_account() -> (Pubkey, AccountSharedData) {
    (ID, account())
}
