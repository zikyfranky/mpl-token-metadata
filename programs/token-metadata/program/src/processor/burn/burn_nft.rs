use mpl_utils::assert_signer;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use spl_token_2022::state::Account;

use super::*;
use crate::{
    assertions::assert_owned_by,
    instruction::{Burn, Context},
    state::{Metadata, TokenMetadataAccount},
    utils::{unpack_initialized, SPL_TOKEN_ID},
};

use super::nonfungible::{burn_nonfungible, BurnNonFungibleArgs};

pub fn process_burn_nft<'a>(program_id: &Pubkey, accounts: &'a [AccountInfo<'a>]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let metadata_info = next_account_info(account_info_iter)?;
    let owner_info = next_account_info(account_info_iter)?;
    let mint_info = next_account_info(account_info_iter)?;
    let token_info = next_account_info(account_info_iter)?;
    let edition_info = next_account_info(account_info_iter)?;
    let spl_token_program_info = next_account_info(account_info_iter)?;

    let collection_metadata_info = account_info_iter.next();

    // Validate accounts

    // Assert signer
    assert_signer(owner_info)?;

    // Assert program ownership.
    assert_owned_by(metadata_info, program_id)?;
    assert_owned_by(edition_info, program_id)?;
    assert_owned_by(mint_info, &SPL_TOKEN_ID)?;
    assert_owned_by(token_info, &SPL_TOKEN_ID)?;

    // Check program IDs.
    if spl_token_program_info.key != &SPL_TOKEN_ID {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Deserialize accounts.
    let metadata = Metadata::from_account_info(metadata_info)?;
    let token = unpack_initialized::<Account>(&token_info.data.borrow())?;

    // Validate relationships between accounts.

    // Owner passed in matches the owner of the token account.
    if token.owner != *owner_info.key {
        return Err(MetadataError::InvalidOwner.into());
    }

    // Mint account passed in matches the mint of the token account.
    if &token.mint != mint_info.key {
        return Err(MetadataError::MintMismatch.into());
    }

    // Token account must have sufficient balance for burn.
    if token.amount != 1 {
        return Err(MetadataError::InsufficientTokenBalance.into());
    }

    // Metadata account must match the mint.
    if token.mint != metadata.mint {
        return Err(MetadataError::MintMismatch.into());
    }

    // Contruct our new Burn handler context so we can re-use the same code for both.
    let accounts = Burn {
        authority_info: owner_info,
        collection_metadata_info,
        metadata_info,
        edition_info: Some(edition_info),
        mint_info,
        token_info,
        master_edition_info: None,
        master_edition_mint_info: None,
        master_edition_token_info: None,
        edition_marker_info: None,
        token_record_info: None,
        // This handler doesn't get system program and sysvars instructions
        // but we need them to create the Burn struct. They are not used in the burn_nonfungible handler.
        system_program_info: spl_token_program_info,
        sysvar_instructions_info: spl_token_program_info,
        spl_token_program_info,
    };
    let context = Context { accounts };

    let args = BurnNonFungibleArgs {
        metadata,
        me_close_authority: false,
    };
    burn_nonfungible(&context, args)
}
