// The runtime origin is used by dispatchable functions to check where a call has come from

//Raw origins
//Substrate defines three raw origins which can be used in your runtime pallets:
pub enum RawOrigin<AccountId> {
	Root,
	Signed(AccountId),
	None,
}


//Origin call
// Root
proposal.dispatch(system::RawOrigin::Root.into())

// Signed
proposal.dispatch(system::RawOrigin::Signed(who).into())

// None
proposal.dispatch(system::RawOrigin::None.into())


//Custom origins

//These can be used as authorization checks inside functions from specific modules in your runtime, 
// or to define custom access-control logic around the sources of runtime requests