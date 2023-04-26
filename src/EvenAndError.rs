
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