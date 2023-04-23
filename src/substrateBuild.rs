/// This function is safe to execute without an additional transactional storage layer.
#[without_transactional]
fn set_value(x: u32) -> DispatchResult {
    Self::check_value(x)?;
    MyStorage::set(x);
    Ok(())
}


// single storage value
#[pallet::storage]

type SomePrivateValue<T> = StorageValue<_, u32, ValueQuery>;

#[pallet::storage]
#[pallet::getter(fn some_primitive_value)]
pub(super) type SomePrimitiveValue<T> = StorageValue<_, u32, ValueQuery>;


#[pallet::storage]
pub(super) type SomeComplexValue<T: Config> = StorageValue<_, T::AccountId, ValueQuery>;

