//Default weight annotations
#[pallet::weight(100_00)]

fn my_dispatchable()
{
    //--------------------
}


// Weights and database read/write operations
#[pallet::weight(T::DbWeight::get().read_writes(1,2)+ 20_000)]
fn my_dispatchable()
{
    //------------------
}

//custom weight


// WeighData<T> to determine the weight of the dispatch.
// ClassifyDispatch<T> to determine the class of the dispatch.
// Pays<T> to determine whether the sender of the dispatch pays fees.


struct LenWeight(u32);
impl<T> WeighData<T>  for LenWeight {
    fn weight_data(&self, target:T) -> Weight{
        let multiplier = self.0;
        let encoded_len = target.encode().len() as u32;

        multiplier * encoded_len
    }
}

impl<T> ClassifyDispatch<T> for LenWeight{
    fn classify_dispatch(&self, target: T) -> DispatchClass 
    {
        let encoded_len=target.encode().len() as u32;
        if encoded_len > 100{
            DispatchClass::Operational
        }else{
            DispatchClass::Normal
        }
    }
}


impl<T> PaysFee<T> {
    fn pays_fee(&self, target:T) -> Pays{
        let encoded_len=target.encode().len() as u32;
        if encoded_len > 10{
            Pays::Yes
        }else{
            Pays::No
        }
    }
}


//Custom inclusion fee
// The following example illustrates how to customize your inclusion fee. You must configure the appropriate associated types in the respective module.

// Assume this is the balance type
type Balance = u64;

// Assume we want all the weights to have a `100 + 2 * w` conversion to fees
struct CustomWeightToFee;
impl WeightToFee<Weight, Balance> for CustomWeightToFee {
    fn convert(w: Weight) -> Balance {
        let a = Balance::from(100);
        let b = Balance::from(2);
        let w = Balance::from(w);
        a + b * w
    }
}

parameter_types! {
    pub const ExtrinsicBaseWeight: Weight = 10_000_000;
}

impl frame_system::Config for Runtime {
    type ExtrinsicBaseWeight = ExtrinsicBaseWeight;
}

parameter_types! {
    pub const TransactionByteFee: Balance = 10;
}

impl transaction_payment::Config {
    type TransactionByteFee = TransactionByteFee;
    type WeightToFee = CustomWeightToFee;
    type FeeMultiplierUpdate = TargetedFeeAdjustment<TargetBlockFullness>;
}

struct TargetedFeeAdjustment<T>(sp_std::marker::PhantomData<T>);
impl<T: Get<Perquintill>> WeightToFee<Fixed128, Fixed128> for TargetedFeeAdjustment<T> {
    fn convert(multiplier: Fixed128) -> Fixed128 {
        // Don't change anything. Put any fee update info here.
        multiplier
    }
}


// Loosely coupled pallets

pub trait Currency<AccountId> {
    // -- snip --
    fn transfer(
        source: &AccountId,
        dest: &AccountId,
        value: Self::Balance,
        // don't worry about the last parameter for now
        existence_requirement: ExistenceRequirement,
    ) -> DispatchResult;
}

pub trait Config: frame_system::Config {
    type MyCurrency: Currency<Self::AccountId>;
}

impl<T: Config> Pallet<T> {
    pub fn my_function() {
        T::MyCurrency::transfer(&buyer, &seller, price, ExistenceRequirement::KeepAlive)?;
    }
}

