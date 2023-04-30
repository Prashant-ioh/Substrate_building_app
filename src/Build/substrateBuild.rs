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

//Single key storage map
#[pallet::storage]
#[pallet::getter(fn some_map)]
pub(super) type SomeMap<T: Config> = StorageMap<
    _, 
    Blake2_128Concat, T::AccountId, 
    u32, 
    ValueQuery
>;

// Double key storage map
#[pallet::storage]
pub(super) type SomeDoubleMap<T: Config> = StorageDoubleMap<
    _, 
    Blake2_128Concat, u32, 
    Blake2_128Concat, T::AccountId, 
    u32, 
    ValueQuery
>;

// Multi-key storage map
#[pallet::storage]
#[pallet::getter(fn some_nmap)]
pub(super) type SomeNMap<T: Config> = StorageNMap<
    _,
    (
        NMapKey<Blake2_128Concat, u32>,
        NMapKey<Blake2_128Concat, T::AccountId>,
        NMapKey<Twox64Concat, u32>,
    ),
    u32,
    ValueQuery,
>;
