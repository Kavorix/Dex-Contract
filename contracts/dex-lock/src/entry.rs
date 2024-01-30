use crate::{
    error::Error,
    helper::{inputs_contain_owner_cell, position_dex_lock_in_inputs, DexArgs},
};
use ckb_std::{
    ckb_constants::Source,
    high_level::{load_cell_capacity, load_cell_lock},
};

pub fn main() -> Result<(), Error> {
    let args = DexArgs::from_script()?;
    // When the inputs contain a cell whose lock script is owner, it means that the owner is
    // cancelling the maker order.
    if inputs_contain_owner_cell(&args) {
        return Ok(());
    }

    // The buyer must pay the specified amount of assets(CKB, UDT, etc.) in the DEX lock script args
    // to the seller's lock script
    let dex_index = position_dex_lock_in_inputs()?;
    let output_lock = load_cell_lock(dex_index, Source::Output)?;
    if args.owner_lock != output_lock {
        return Err(Error::DexOwnerLockNotMatch);
    }
    let output_capacity = load_cell_capacity(dex_index, Source::Output)?;
    if args.total_value > output_capacity.into() {
        return Err(Error::DexTotalValueNotMatch);
    }

    Ok(())
}
