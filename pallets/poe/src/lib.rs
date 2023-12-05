#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// Type representing the weight of this pallet
        type WeightInfo: WeightInfo;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    // The pallet's runtime storage items.
    // https://docs.substrate.io/main-docs/build/runtime-storage/
    #[pallet::storage]
    #[pallet::getter(fn something)]
    // Learn more about declaring storage items:
    // https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
    pub type Something<T> = StorageValue<_, u32>;

    //  单键存储映射
    #[pallet::storage]
    #[pallet::getter(fn claims)]
    pub type Claims<T: Config> =
        StorageMap<_, Blake2_128Concat, T::Hash, (T::AccountId, BlockNumberFor<T>)>;
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    /// 单键存储映射
    #[pallet::storage]
    #[pallet::getter(fn some_map)]
    pub(super) type SomeMap<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, u32, ValueQuery>;

    /// 双键存储映射
    #[pallet::storage]
    #[pallet::getter(fn some_dmap)]
    pub(super) type SomeDoubleMap<T: Config> =
        StorageDoubleMap<_, Blake2_128Concat, u32, Blake2_128Concat, T::AccountId, u32, ValueQuery>;

    /// 多键存储映射
    #[pallet::storage]
    #[pallet::getter(fn some_nmap)]
    pub(super) type SomeNMap<T: Config> = StorageNMap<
        _,
        (
            NMapKey<Blake2_128Concat, u32>,
            NMapKey<Blake2_128Concat, T::AccountId>,
            NMapKey<Twox64Concat, u32>,
        ),
        u32,
        ValueQuery,
    >;

    // Pallets use events to inform users when important changes are made.
    // https://docs.substrate.io/main-docs/build/events-errors/
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event documentation should end with an array that provides descriptive names for event
        /// parameters. [something, who]
        SomethingStored { something: u32, who: T::AccountId },

        /// 申明存证后触发该事件
        ClaimCreated { who: T::AccountId, claim: T::Hash },
        /// 撤销存证后触发该事件
        ClaimRevoked { who: T::AccountId, claim: T::Hash },
        /// 发送存证后触发该事件
        ClaimTransfered {
            sender: T::AccountId,
            receiver: T::AccountId,
            claim: T::Hash,
        },
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// Error names should be descriptive.
        NoneValue,
        /// Errors should have helpful documentation associated with them.
        StorageOverflow,

        /// 存证已经被声明
        AlreadyClaimed,
        /// 存证不存在
        NoSuchClaim,
        /// 存证被另一个用户使用中
        NotClaimOwner,
    }

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(2)]
        #[pallet::weight(0)]
        // #[pallet::weight(T::WeightInfo::create_claim())]
        pub fn create_claim(origin: OriginFor<T>, claim: T::Hash) -> DispatchResult {
            // Check that the extrinsic was signed and get the signer.
            // This function will return an error if the extrinsic is not signed.
            // https://docs.substrate.io/main-docs/build/origins/
            let who = ensure_signed(origin)?;

            // Verify that the specified claim has not already been stored.
            ensure!(
                !Claims::<T>::contains_key(&claim),
                Error::<T>::AlreadyClaimed
            );

            // Get the block number from the FRAME System pallet.
            let current_block = <frame_system::Pallet<T>>::block_number();

            // Store the claim with the sender and block number.
            Claims::<T>::insert(&claim, (&who, current_block));

            // Emit an event.
            Self::deposit_event(Event::ClaimCreated {
                who: who,
                claim: claim,
            });
            // Return a successful DispatchResultWithPostInfo
            Ok(())
        }

        #[pallet::call_index(3)]
        #[pallet::weight(0)]
        // #[pallet::weight(T::WeightInfo::revoke_claim())]
        pub fn revoke_claim(origin: OriginFor<T>, claim: T::Hash) -> DispatchResult {
            // Check that the extrinsic was signed and get the signer.
            // This function will return an error if the extrinsic is not signed.
            // https://docs.substrate.io/main-docs/build/origins/
            let who = ensure_signed(origin)?;

            // Get owner of the claim, if none return an error.
            let (owner, _) = Claims::<T>::get(&claim).ok_or(Error::<T>::NoSuchClaim)?;

            // Verify that sender of the current call is the claim owner.
            ensure!(who == owner, Error::<T>::NotClaimOwner);

            // Remove claim from storage.
            Claims::<T>::remove(&claim);

            // Emit an event.
            Self::deposit_event(Event::ClaimRevoked {
                who: who,
                claim: claim,
            });
            // Return a successful DispatchResultWithPostInfo
            Ok(())
        }

        #[pallet::call_index(4)]
        #[pallet::weight(0)]
        // #[pallet::weight(T::WeightInfo::transfer_claim())]
        pub fn transfer_claim(
            origin: OriginFor<T>,
            to: T::AccountId,
            claim: T::Hash,
        ) -> DispatchResult {
            // Check that the extrinsic was signed and get the signer.
            // This function will return an error if the extrinsic is not signed.
            // https://docs.substrate.io/main-docs/build/origins/
            let who = ensure_signed(origin)?;

            // Get owner of the claim, if none return an error.
            let (owner, _) = Claims::<T>::get(&claim).ok_or(Error::<T>::NoSuchClaim)?;

            // Verify that sender of the current call is the claim owner.
            ensure!(who == owner, Error::<T>::NotClaimOwner);

            // Remove claim from storage.
            Claims::<T>::remove(&claim);

            let current_block = <frame_system::Pallet<T>>::block_number();
            // Store the claim with the sender and block number.
            Claims::<T>::insert(&claim, (to.clone(), current_block));

            // Emit an event.
            Self::deposit_event(Event::ClaimTransfered {
                sender: who,
                receiver: to,
                claim: claim,
            });
            // Return a successful DispatchResultWithPostInfo
            Ok(())
        }

        /// An example dispatchable that takes a singles value as a parameter, writes the value to
        /// storage and emits an event. This function must be dispatched by a signed extrinsic.
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::do_something())]
        pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
            // Check that the extrinsic was signed and get the signer.
            // This function will return an error if the extrinsic is not signed.
            // https://docs.substrate.io/main-docs/build/origins/
            let who = ensure_signed(origin)?;

            // Update storage.
            <Something<T>>::put(something);

            // Emit an event.
            Self::deposit_event(Event::SomethingStored { something, who });
            // Return a successful DispatchResultWithPostInfo
            Ok(())
        }

        /// An example dispatchable that may throw a custom error.
        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::cause_error())]
        pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
            let _who = ensure_signed(origin)?;

            // Read a value from storage.
            match <Something<T>>::get() {
                // Return an error if the value has not been set.
                None => return Err(Error::<T>::NoneValue.into()),
                Some(old) => {
                    // Increment the value read from storage; will error in the event of overflow.
                    let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
                    // Update the value in storage with the incremented result.
                    <Something<T>>::put(new);
                    Ok(())
                }
            }
        }
    }
}
