use batch::process_batch;
use pinocchio::{
    account_info::AccountInfo, default_panic_handler, no_allocator, program_entrypoint,
    program_error::ProgramError, pubkey::Pubkey, ProgramResult,
};
use pinocchio_pubkey::pubkey;

use crate::processor::*;

program_entrypoint!(process_instruction);
// Do not allocate memory.
no_allocator!();
// Use the default panic handler.
default_panic_handler!();

/// Process an instruction.
///
/// The processor of the token program is divided into two parts to reduce the overhead
/// of having a large `match` statement. The first part of the processor handles the
/// most common instructions, while the second part handles the remaining instructions.
/// The rationale is to reduce the overhead of making multiple comparisons for popular
/// instructions.
///
/// Instructions on the first part of the processor:
///
/// - `0`: `InitializeMint`
/// - `3`:  `Transfer`
/// - `7`:  `MintTo`
/// - `9`:  `CloseAccount`
/// - `18`: `InitializeAccount3`
/// - `20`: `InitializeMint2`
#[inline(always)]
pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {

    let (discriminator, instruction_data) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match *discriminator {
        // 0 - InitializeMint
        0 => {
            #[cfg(feature = "logging")]
            pinocchio::msg!("Instruction: InitializeMint");

            process_initialize_mint(accounts, instruction_data, true)
        }

        // 3 - Transfer
        3 => {
            #[cfg(feature = "logging")]
            pinocchio::msg!("Instruction: Transfer");

            process_transfer(accounts, instruction_data)
        }
        // 7 - MintTo
        7 => {
            #[cfg(feature = "logging")]
            pinocchio::msg!("Instruction: MintTo");

            process_mint_to(accounts, instruction_data)
        }
        // 9 - CloseAccount
        9 => {
            #[cfg(feature = "logging")]
            pinocchio::msg!("Instruction: CloseAccount");

            process_close_account(accounts)
        }
        // 18 - InitializeAccount3
        18 => {
            #[cfg(feature = "logging")]
            pinocchio::msg!("Instruction: InitializeAccount3");

            process_initialize_account3(accounts, instruction_data)
        }
        // 20 - InitializeMint2
        20 => {
            #[cfg(feature = "logging")]
            pinocchio::msg!("Instruction: InitializeMint2");

            process_initialize_mint2(accounts, instruction_data)
        }
        // 255 - Batch
        255 => {
            #[cfg(feature = "logging")]
            pinocchio::msg!("Instruction: Batch");

            process_batch(accounts, instruction_data)
        }
        _ => process_remaining_instruction(accounts, instruction_data, *discriminator),
    }
}

/// Process the remaining instructions.
///
/// This function is called by the `process_instruction` function if the discriminator
/// does not match any of the common instructions. This function is used to reduce the
/// overhead of having a large `match` statement in the `process_instruction` function.
fn process_remaining_instruction(
    accounts: &[AccountInfo],
    instruction_data: &[u8],
    discriminator: u8,
) -> ProgramResult {
    match discriminator {
        // 1 - InitializeAccount
        1 => {
            #[cfg(feature = "logging")]
            pinocchio::msg!("Instruction: InitializeAccount");

            process_initialize_account(accounts)
        }
        // 2 - InitializeMultisig
        2 => {
            #[cfg(feature = "logging")]
            pinocchio::msg!("Instruction: InitializeMultisig");

            process_initialize_multisig(accounts, instruction_data)
        }
        // 4 - Approve
        4 => {
            #[cfg(feature = "logging")]
            pinocchio::msg!("Instruction: Approve");

            process_approve(accounts, instruction_data)
        }
        // 5 - Revoke
        5 => {
            #[cfg(feature = "logging")]
            pinocchio::msg!("Instruction: Revoke");

            process_revoke(accounts, instruction_data)
        }
        // 6 - SetAuthority
        6 => {
            #[cfg(feature = "logging")]
            pinocchio::msg!("Instruction: SetAuthority");

            process_set_authority(accounts, instruction_data)
        }
        // 8 - Burn
        8 => {
            #[cfg(feature = "logging")]
            pinocchio::msg!("Instruction: Burn");

            process_burn(accounts, instruction_data)
        }
        // 10 - FreezeAccount
        10 => {
            #[cfg(feature = "logging")]
            pinocchio::msg!("Instruction: FreezeAccount");

            process_freeze_account(accounts)
        }
        // 11 - ThawAccount
        11 => {
            #[cfg(feature = "logging")]
            pinocchio::msg!("Instruction: ThawAccount");

            process_thaw_account(accounts)
        }
        // 12 - TransferChecked
        12 => {
            #[cfg(feature = "logging")]
            pinocchio::msg!("Instruction: TransferChecked");

            process_transfer_checked(accounts, instruction_data)
        }
        // 13 - ApproveChecked
        13 => {
            #[cfg(feature = "logging")]
            pinocchio::msg!("Instruction: ApproveChecked");

            process_approve_checked(accounts, instruction_data)
        }
        // 14 - MintToChecked
        14 => {
            #[cfg(feature = "logging")]
            pinocchio::msg!("Instruction: MintToChecked");

            process_mint_to_checked(accounts, instruction_data)
        }
        // 15 - BurnChecked
        15 => {
            #[cfg(feature = "logging")]
            pinocchio::msg!("Instruction: BurnChecked");

            process_burn_checked(accounts, instruction_data)
        }
        // 16 - InitializeAccount2
        16 => {
            #[cfg(feature = "logging")]
            pinocchio::msg!("Instruction: InitializeAccount2");

            process_initialize_account2(accounts, instruction_data)
        }
        // 17 - SyncNative
        17 => {
            #[cfg(feature = "logging")]
            pinocchio::msg!("Instruction: SyncNative");

            process_sync_native(accounts)
        }
        // 19 - InitializeMultisig2
        19 => {
            #[cfg(feature = "logging")]
            pinocchio::msg!("Instruction: InitializeMultisig2");

            process_initialize_multisig2(accounts, instruction_data)
        }
        // 21 - GetAccountDataSize
        21 => {
            #[cfg(feature = "logging")]
            pinocchio::msg!("Instruction: GetAccountDataSize");

            process_get_account_data_size(accounts)
        }
        // 22 - InitializeImmutableOwner
        22 => {
            #[cfg(feature = "logging")]
            pinocchio::msg!("Instruction: InitializeImmutableOwner");

            process_initialize_immutable_owner(accounts)
        }
        // 23 - AmountToUiAmount
        23 => {
            #[cfg(feature = "logging")]
            pinocchio::msg!("Instruction: AmountToUiAmount");

            process_amount_to_ui_amount(accounts, instruction_data)
        }
        // 24 - UiAmountToAmount
        24 => {
            #[cfg(feature = "logging")]
            pinocchio::msg!("Instruction: UiAmountToAmount");

            process_ui_amount_to_amount(accounts, instruction_data)
        }
        _ => Err(ProgramError::InvalidInstructionData),
    }
}
