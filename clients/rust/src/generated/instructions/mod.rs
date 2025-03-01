//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

pub(crate) mod r#approve_collection_authority;
pub(crate) mod r#approve_use_authority;
pub(crate) mod r#bubblegum_set_collection_size;
pub(crate) mod r#burn;
pub(crate) mod r#burn_edition_nft;
pub(crate) mod r#burn_nft;
pub(crate) mod r#burn_v1;
pub(crate) mod r#close_escrow_account;
pub(crate) mod r#collect;
pub(crate) mod r#convert_master_edition_v1_to_v2;
pub(crate) mod r#create;
pub(crate) mod r#create_escrow_account;
pub(crate) mod r#create_master_edition_v3;
pub(crate) mod r#create_metadata_account_v3;
pub(crate) mod r#create_v1;
pub(crate) mod r#delegate;
pub(crate) mod r#delegate_authority_item_v1;
pub(crate) mod r#delegate_collection_item_v1;
pub(crate) mod r#delegate_collection_v1;
pub(crate) mod r#delegate_data_item_v1;
pub(crate) mod r#delegate_data_v1;
pub(crate) mod r#delegate_locked_transfer_v1;
pub(crate) mod r#delegate_programmable_config_item_v1;
pub(crate) mod r#delegate_programmable_config_v1;
pub(crate) mod r#delegate_sale_v1;
pub(crate) mod r#delegate_staking_v1;
pub(crate) mod r#delegate_standard_v1;
pub(crate) mod r#delegate_transfer_v1;
pub(crate) mod r#delegate_utility_v1;
pub(crate) mod r#deprecated_mint_new_edition_from_master_edition_via_printing_token;
pub(crate) mod r#freeze_delegated_account;
pub(crate) mod r#lock;
pub(crate) mod r#lock_v1;
pub(crate) mod r#migrate;
pub(crate) mod r#mint;
pub(crate) mod r#mint_new_edition_from_master_edition_via_token;
pub(crate) mod r#mint_new_edition_from_master_edition_via_vault_proxy;
pub(crate) mod r#mint_v1;
pub(crate) mod r#print;
pub(crate) mod r#print_v1;
pub(crate) mod r#puff_metadata;
pub(crate) mod r#remove_creator_verification;
pub(crate) mod r#revoke;
pub(crate) mod r#revoke_authority_item_v1;
pub(crate) mod r#revoke_collection_authority;
pub(crate) mod r#revoke_collection_item_v1;
pub(crate) mod r#revoke_collection_v1;
pub(crate) mod r#revoke_data_item_v1;
pub(crate) mod r#revoke_data_v1;
pub(crate) mod r#revoke_locked_transfer_v1;
pub(crate) mod r#revoke_migration_v1;
pub(crate) mod r#revoke_programmable_config_item_v1;
pub(crate) mod r#revoke_programmable_config_v1;
pub(crate) mod r#revoke_sale_v1;
pub(crate) mod r#revoke_staking_v1;
pub(crate) mod r#revoke_standard_v1;
pub(crate) mod r#revoke_transfer_v1;
pub(crate) mod r#revoke_use_authority;
pub(crate) mod r#revoke_utility_v1;
pub(crate) mod r#set_and_verify_collection;
pub(crate) mod r#set_and_verify_sized_collection_item;
pub(crate) mod r#set_collection_size;
pub(crate) mod r#set_token_standard;
pub(crate) mod r#sign_metadata;
pub(crate) mod r#thaw_delegated_account;
pub(crate) mod r#transfer;
pub(crate) mod r#transfer_out_of_escrow;
pub(crate) mod r#transfer_v1;
pub(crate) mod r#unlock;
pub(crate) mod r#unlock_v1;
pub(crate) mod r#unverify;
pub(crate) mod r#unverify_collection;
pub(crate) mod r#unverify_collection_v1;
pub(crate) mod r#unverify_creator_v1;
pub(crate) mod r#unverify_sized_collection_item;
pub(crate) mod r#update;
pub(crate) mod r#update_as_authority_item_delegate_v2;
pub(crate) mod r#update_as_collection_delegate_v2;
pub(crate) mod r#update_as_collection_item_delegate_v2;
pub(crate) mod r#update_as_data_delegate_v2;
pub(crate) mod r#update_as_data_item_delegate_v2;
pub(crate) mod r#update_as_programmable_config_delegate_v2;
pub(crate) mod r#update_as_programmable_config_item_delegate_v2;
pub(crate) mod r#update_as_update_authority_v2;
pub(crate) mod r#update_metadata_account_v2;
pub(crate) mod r#update_primary_sale_happened_via_token;
pub(crate) mod r#update_v1;
pub(crate) mod r#use;
pub(crate) mod r#use_v1;
pub(crate) mod r#utilize;
pub(crate) mod r#verify;
pub(crate) mod r#verify_collection;
pub(crate) mod r#verify_collection_v1;
pub(crate) mod r#verify_creator_v1;
pub(crate) mod r#verify_sized_collection_item;

pub use self::r#approve_collection_authority::*;
pub use self::r#approve_use_authority::*;
pub use self::r#bubblegum_set_collection_size::*;
pub use self::r#burn::*;
pub use self::r#burn_edition_nft::*;
pub use self::r#burn_nft::*;
pub use self::r#burn_v1::*;
pub use self::r#close_escrow_account::*;
pub use self::r#collect::*;
pub use self::r#convert_master_edition_v1_to_v2::*;
pub use self::r#create::*;
pub use self::r#create_escrow_account::*;
pub use self::r#create_master_edition_v3::*;
pub use self::r#create_metadata_account_v3::*;
pub use self::r#create_v1::*;
pub use self::r#delegate::*;
pub use self::r#delegate_authority_item_v1::*;
pub use self::r#delegate_collection_item_v1::*;
pub use self::r#delegate_collection_v1::*;
pub use self::r#delegate_data_item_v1::*;
pub use self::r#delegate_data_v1::*;
pub use self::r#delegate_locked_transfer_v1::*;
pub use self::r#delegate_programmable_config_item_v1::*;
pub use self::r#delegate_programmable_config_v1::*;
pub use self::r#delegate_sale_v1::*;
pub use self::r#delegate_staking_v1::*;
pub use self::r#delegate_standard_v1::*;
pub use self::r#delegate_transfer_v1::*;
pub use self::r#delegate_utility_v1::*;
pub use self::r#deprecated_mint_new_edition_from_master_edition_via_printing_token::*;
pub use self::r#freeze_delegated_account::*;
pub use self::r#lock::*;
pub use self::r#lock_v1::*;
pub use self::r#migrate::*;
pub use self::r#mint::*;
pub use self::r#mint_new_edition_from_master_edition_via_token::*;
pub use self::r#mint_new_edition_from_master_edition_via_vault_proxy::*;
pub use self::r#mint_v1::*;
pub use self::r#print::*;
pub use self::r#print_v1::*;
pub use self::r#puff_metadata::*;
pub use self::r#remove_creator_verification::*;
pub use self::r#revoke::*;
pub use self::r#revoke_authority_item_v1::*;
pub use self::r#revoke_collection_authority::*;
pub use self::r#revoke_collection_item_v1::*;
pub use self::r#revoke_collection_v1::*;
pub use self::r#revoke_data_item_v1::*;
pub use self::r#revoke_data_v1::*;
pub use self::r#revoke_locked_transfer_v1::*;
pub use self::r#revoke_migration_v1::*;
pub use self::r#revoke_programmable_config_item_v1::*;
pub use self::r#revoke_programmable_config_v1::*;
pub use self::r#revoke_sale_v1::*;
pub use self::r#revoke_staking_v1::*;
pub use self::r#revoke_standard_v1::*;
pub use self::r#revoke_transfer_v1::*;
pub use self::r#revoke_use_authority::*;
pub use self::r#revoke_utility_v1::*;
pub use self::r#set_and_verify_collection::*;
pub use self::r#set_and_verify_sized_collection_item::*;
pub use self::r#set_collection_size::*;
pub use self::r#set_token_standard::*;
pub use self::r#sign_metadata::*;
pub use self::r#thaw_delegated_account::*;
pub use self::r#transfer::*;
pub use self::r#transfer_out_of_escrow::*;
pub use self::r#transfer_v1::*;
pub use self::r#unlock::*;
pub use self::r#unlock_v1::*;
pub use self::r#unverify::*;
pub use self::r#unverify_collection::*;
pub use self::r#unverify_collection_v1::*;
pub use self::r#unverify_creator_v1::*;
pub use self::r#unverify_sized_collection_item::*;
pub use self::r#update::*;
pub use self::r#update_as_authority_item_delegate_v2::*;
pub use self::r#update_as_collection_delegate_v2::*;
pub use self::r#update_as_collection_item_delegate_v2::*;
pub use self::r#update_as_data_delegate_v2::*;
pub use self::r#update_as_data_item_delegate_v2::*;
pub use self::r#update_as_programmable_config_delegate_v2::*;
pub use self::r#update_as_programmable_config_item_delegate_v2::*;
pub use self::r#update_as_update_authority_v2::*;
pub use self::r#update_metadata_account_v2::*;
pub use self::r#update_primary_sale_happened_via_token::*;
pub use self::r#update_v1::*;
pub use self::r#use::*;
pub use self::r#use_v1::*;
pub use self::r#utilize::*;
pub use self::r#verify::*;
pub use self::r#verify_collection::*;
pub use self::r#verify_collection_v1::*;
pub use self::r#verify_creator_v1::*;
pub use self::r#verify_sized_collection_item::*;
