
//A pallet can emit events when it wants to send notification 
//about changes or conditions in the runtime to external entities like users, chain explorers, or dApps.

#[pallet::event]
#[pallet::generate_deposit(pub(super) fn deposit_event)]
pub enum Event<T: Config> {
	/// Set a value.
	ValueSet { value: u32, who: T::AccountId },
}