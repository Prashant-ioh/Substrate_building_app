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
