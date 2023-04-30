// Unit test
cargo test

cargo help test

// Test pallet log in a mock runtime
frame_support::construct_runtime!(
    pub enum Test where
     Block = Block,
     NodeBlock = Block,
     UncheckedExtrinsic = UncheckedExtrinsic,
    {
     System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
     TemplateModule: pallet_template::{Pallet, Call, Storage, Event<T>},
    }
   );
   
   impl frame_system::Config for Test {
    // -- snip --
    type AccountId = u64;
   }

   //If Test implements pallet_balances::Config, the assignment might use u64 for the Balance type

   impl pallet_balances::Config for Test {
    // -- snip --
    type Balance = u64;
   }


   //Test storage in a mock runtime

   pub struct ExtBuilder;

impl ExtBuilder {
 pub fn build(self) -> sp_io::TestExternalities {
  let mut t = system::GenesisConfig::default().build_storage::<TestRuntime>().unwrap();
  let mut ext = sp_io::TestExternalities::new(t);
  ext.execute_with(|| System::set_block_number(1));
  ext
 }
}

#[test]
fn fake_test_example() {
 ExtBuilder::default().build_and_execute(|| {
  // ...test logics...
 });
}

// Test events in a mock runtime

fn fake_test_example() {
    ExtBuilder::default().build_and_execute(|| {
     System::set_block_number(1);
     // ... test logic that emits FakeEvent1 and then FakeEvent2 ...
     System::assert_has_event(Event::FakeEvent1{}.into())
     System::assert_last_event(Event::FakeEvent2 { data: 7 }.into())
     assert_eq!(System::events().len(), 2);
    });
   }

   //Advanced event testing
   fn only_example_events() -> Vec<super::Event<Runtime>> {
    System::events()
     .into_iter()
     .map(|r| r.event)
     .filter_map(|e| if let RuntimeEvent::TemplateModule(inner) = e { Some(inner) } else { None })
     .collect::<Vec<_>>();
   }

   parameter_types! {
    static ExamplePalletEvents: u32 = 0;
   }
   
   fn example_events_since_last_call() -> Vec<super::Event<Runtime>> {
    let events = only_example_events();
    let already_seen = ExamplePalletEvents::get();
    ExamplePalletEvents::set(events.len() as u32);
    events.into_iter().skip(already_seen as usize).collect()
   }

   fn fake_test_example() {
    ExtBuilder::default().build_and_execute(|| {
     System::set_block_number(1);
     // ... test logic that emits FakeEvent1 ...
     assert_eq!(
      example_events_since_last_call(),
      vec![Event::FakeEvent1{}]
     );
     // ... test logic that emits FakeEvent2 ...
     assert_eq!(
      example_events_since_last_call(),
      vec![Event::FakeEvent2{}]
     );
    });
   }

   // Genesis config

   impl ExtBuilder {
    pub fn build(self) -> sp_io::TestExternalities {
     let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
     pallet_balances::GenesisConfig::<Test> {
      balances: vec![
       (1, 10),
       (2, 20),
       (3, 30),
       (4, 40),
       (5, 50),
       (6, 60)
      ],
     }
      .assimilate_storage(&mut t)
      .unwrap();
   
     let mut ext = sp_io::TestExternalities::new(t);
     ext.execute_with(|| System::set_block_number(1));
     ext
    }
   }

   pub struct GenesisConfig<T: Config<I>, I: 'static = ()> {
    pub balances: Vec<(T::AccountId, T::Balance)>,
   }

   // Block production

   fn run_to_block(n: u64) {
    while System::block_number() < n {
     if System::block_number() > 0 {
      ExamplePallet::on_finalize(System::block_number());
      System::on_finalize(System::block_number());
     }
     System::reset_events();
     System::set_block_number(System::block_number() + 1);
     System::on_initialize(System::block_number());
     ExamplePallet::on_initialize(System::block_number());
    }
   }

   #[test]
fn my_runtime_test() {
 with_externalities(&mut new_test_ext(), || {
  assert_ok!(ExamplePallet::start_auction());
  run_to_block(10);
  assert_ok!(ExamplePallet::end_auction());
 });
}