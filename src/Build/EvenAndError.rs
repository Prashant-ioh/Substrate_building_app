
//A pallet can emit events when it wants to send notification 
//about changes or conditions in the runtime to external entities like users, chain explorers, or dApps.

#[pallet::event]
#[pallet::generate_deposit(pub(super) fn deposit_event)]
pub enum Event<T: Config> {
	/// Set a value.
	ValueSet { value: u32, who: T::AccountId },
}


//RuntimeEvent type is needed to aggregate them for the runtime
#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}



    //Exposing events to your runtime

    //To expose events to the runtime:

    //1. Open the /runtime/src/lib.rsfile in a text editor
   //2. Implement the RuntimeEvent type in the configuration trait for your pallet:
    impl template::Config for Runtime {
        type RuntimeEvent = RuntimeEvent;
   }

   //3. Add the RuntimeEvent type to the construct_runtime! macro:
   construct_runtime!(
    pub enum Runtime where
     Block = Block,
      NodeBlock = opaque::Block,
      UncheckedExtrinsic = UncheckedExtrinsic
    {
   // --snip--
      TemplateModule: template::{Pallet, Call, Storage, Event<T>},
      //--add-this------------------------------------->
        }
);



// Depositing an event

// 1. Use the `generate_deposit` attribute when declaring the Events enum.
#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)] // <------ here ----
	#[pallet::metadata(...)]
	pub enum Event<T: Config> {
		// --snip--
	}

// 2. Use `deposit_event` inside the dispatchable function
#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(1_000)]
		pub(super) fn set_value(
			origin: OriginFor<T>,
			value: u64,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			// --snip--
			Self::deposit_event(RawEvent::ValueSet(value, sender));
		}
	}

    //Errors
    //Each FRAME pallet can define a custom DispatchError by using the #[pallet::error] macro
    #[pallet::error]
    pub enum Error<T> {
		/// Error names should be descriptive.
		InvalidParameter,
		/// Errors should have helpful documentation associated with them.
		OutOfSpace,
	}

    frame_support::ensure!(param < T::MaxVal::get(), Error::<T>::InvalidParameter);