//! State channel implementation


// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

use srml_support::{StorageValue, dispatch::Result, decl_module, decl_storage, decl_event};
use system::ensure_signed;


decl_storage! {
    trait Store for Module<T: Trait> as StateChannel {

    }
}


decl_event! (



);



decl_module! {
    // Simple declaration of the `Module` type. Lets the macro know what its working on.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
    
    }
}