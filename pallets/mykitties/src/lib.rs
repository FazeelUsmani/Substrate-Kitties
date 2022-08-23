#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
  use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use frame_support::{
        sp_runtime::traits::Hash,
        traits::{ Randomness, Currency, tokens::ExistenceRequirement },
        transactional
    };
    use sp_io::hashing::blake2_128;

    #[cfg(feature = "std")]
    use serde::{Deserialize, Serialize};

    // ACTION #1: Write a Struct to hold Kitty information.
    type AccountOf<T> = <T as frame_system::Config>::AccountId;
    type BalanceOf<T> = 
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
    #[derive(Clone, Encode, Decode, PartialEq)]
    pub struct Kitty<T: Config> {
        pub dna: [u8; 16],
        pub price: Option<BalanceOf<T>>,
        pub gender: Gender,
        pub owner: AccountOf<T>,
    }
    
    // ACTION #2: Enum declaration for Gender.
    #[derive(Encode, Decode, Debug, Clone, PartialEq)]
    pub enum Gender {
        Male, 
        Female,
    }

    // ACTION #3: Implementation to handle Gender type in Kitty struct.
    impl Default for Gender {
        fn default() -> Self {
            Gender::Male
        }
    }


    #[pallet::pallet]
    #[pallet::generate_store(trait Store)]
    pub struct Pallet<T>(_);

    /// Configure the pallet by specifying the parameters and types it depends on.
    #[pallet::config]
    pub trait Config: pallet_balances::Config + frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        
        /// The Currency handler for the Kitties pallet.
        type Currency: Currency<Self::AccountId>;

        // ACTION #5: Specify the type for Randomness we want to specify for runtime.
        type KittyRandomness: Randomness<Self::H256, Self::BlockNumber>;
    }

    // Errors.
    #[pallet::error]
    pub enum Error<T> {
        // TODO Part III
    }

    // Events.
    #[pallet::event]
    // #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        // TODO Part III
    }

    #[pallet::storage]
    #[pallet::getter(fn all_kitties_count)]
    pub(super) type AllKittiesCount<T: Config> = StorageValue<_, u64, ValueQuery>;
    
    // ACTION #6: Add Nonce storage item.

    // ACTION #9: Remaining storage items.
    #[pallet::constant]
    type MaxKittyOwned: Get<u32>;

    // TODO Part IV: Our pallet's genesis configuration.

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        
        // TODO Part III: create_kitty
        
        // TODO Part III: set_price
        
        // TODO Part III: transfer

        // TODO Part III: buy_kitty
        
        // TODO Part III: breed_kitty
    }

    // ACTION #4: helper function for Kitty struct
    fn gen_gender()->Gender {
        let random = T::KittyRandomness::random(&b"gender"[..]).0;
        match random.as_ref()[0] % 2 {
            0 => Gender::Male,
            _ => Gender::Female,
        }
    }

    impl<T: Config> Pallet<T> {
        // TODO Part III: helper functions for dispatchable functions
        
        // ACTION #7: increment_nonce helper
        fn gen_dna() -> [u8; 16] {
            let payload = (
                T::KittyRandomness::random(&b"dna"[..]).0,
                <frame_system::Pallet<T>>::block_number(),
            );
            payload.using_encoded(blake2_128)
        }

        // ACTION #8: random_hash helper
        #[pallet::storage]
        #[pallet::getter(fn kitty)]
        pub(super) type Kitties<T: Config> = StorageMap<
            _, 
            Twox64Concat, 
            T::Hash, 
            Kitty<T>
            >;
        #[pallet::storage]
        #[pallet::getter(fn kitties_owned)]
        pub(super) type Kitties<T: Config> = StorageMap<
            _, 
            Twox64Concat, 
            T::AccountId, 
            BoundedVec<T::Hash, T::MaxKittyOwned>,
            ValueQuery
            >;


        // TODO: mint, transfer_from
        
    }
}