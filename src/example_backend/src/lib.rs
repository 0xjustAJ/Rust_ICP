// use candid::{CandidType, Decode, Deserialize, Encode};
// use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
// use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
// use std::{borrow::Cow, cell::RefCell};

#[macro_use]
extern crate serde;

use candid::{CandidType, Decode, Deserialize, Encode};

use ic_cdk::api::time;

use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};

use ic_stable_structures::{Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use ic_stable_structures::storable::Bound;

use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;

 const MAX_VALUE_SIZE: u64 = 0;

//  const BOUND: ic_stable_structures::storable::Bound = Exam;

#[derive(CandidType, Deserialize)]
struct Exam {
    out_of: u8, //what our exam score are out of

    course: String,
    curve: u8,
}

impl Storable for Exam {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
      
    }

    const BOUND: Bound = Bound::Bounded {

        max_size: 100,  // Replace with the actual max size
        is_fixed_size: false,


        // max_size: 100,
        // // A `()` should in theory be fixed in size, but this flag was initially
        // // set incorrectly and it cannot be fixed to maintain backward-compatibility.
        // is_fixed_size: false,
    };
}

//  trait Storable {
//     fn to_bytes(&self) -> Cow<[u8]>;
//     fn from_bytes(bytes: Cow<[u8]>) -> Self;
// }

//  trait BoundedStorable: Storable {
//     const MAX_SIZE: u32;
//     const IS_FIXED_SIZE: bool;
// }

// impl BoundedStorage for Storable {
//     const MAX_SIZE: u32 = MAX_VALUE_SIZE;
//     const IS_FIXED_SIZE: bool = false;
// }



// impl Bound {
//     /// Returns the maximum size of the type if bounded, panics if unbounded.
//      const fn MAX_SIZE(&self) -> u32 {
//         if let Bound::Bounded { max_size, .. } = self {
//             *max_size
//         } else {
//             panic!("Cannot get max size of unbounded type.");
//         }
//     }

//     /// Returns true if the type is fixed in size, false otherwise.
//     pub const fn IS_FIXED_SIZE(&self) -> bool {
//         if let Bound::Bounded { is_fixed_size, .. } = self {
//             *is_fixed_size
//         } else {
//             false
//         }
//     }
// }


thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static EXAM_MAP: RefCell<StableBTreeMap<u64, Exam, Memory>> = RefCell::new(StableBTreeMap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
    ));

    static PARTICIPATION_PERCENTAGE_MAP: RefCell<StableBTreeMap<u64, u64, Memory>> = RefCell::new(StableBTreeMap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))),
    ));
}

#[ic_cdk::query]
fn get_participation(key: u64) -> Option<u64> {
    PARTICIPATION_PERCENTAGE_MAP.with(|p| p.borrow().get(&key))
}

#[ic_cdk::query]
 fn get_exam(key: u64) -> Option<Exam> {
    EXAM_MAP.with(|p| p.borrow().get(&key))
 }

 #[ic_cdk::update]
 fn insert_exam(key: u64, value: Exam) -> Option<Exam> {
    EXAM_MAP.with(|p| p.borrow_mut().insert(key, value))
 }

 #[ic_cdk::update]
 fn insert_participation(key: u64, value: u64) -> Option<u64> {
    PARTICIPATION_PERCENTAGE_MAP.with(|p| p.borrow_mut().insert(key, value))
 }