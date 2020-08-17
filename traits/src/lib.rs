#![cfg_attr(not(feature = "std"), no_std)]
use crate::Instance;
use codec::{Decode, Encode};
use sp_runtime::{DispatchResult, RuntimeDebug};
use sp_std::{
	cmp::{Eq, PartialEq},
	prelude::Vec,
	prelude::*,
};

pub use auction::{Auction, AuctionHandler, AuctionInfo, OnNewBidResult};
pub use currency::{
	BalanceStatus, BasicCurrency, BasicCurrencyExtended, BasicLockableCurrency, BasicReservableCurrency,
	LockIdentifier, MultiCurrency, MultiCurrencyExtended, MultiLockableCurrency, MultiReservableCurrency, OnReceived,
};
pub use price::{DefaultPriceProvider, PriceProvider};

pub mod arithmetic;
pub mod auction;
pub mod currency;
pub mod price;

/// New data handler
#[impl_trait_for_tuples::impl_for_tuples(30)]
pub trait OnNewData<AccountId, Key, Value> {
	/// New data is available
	fn on_new_data(who: &AccountId, key: &Key, value: &Value);
}

/// A simple trait to provide data
pub trait DataProvider<Key, Value> {
	/// Get data by key
	fn get(key: &Key) -> Option<Value>;
}

/// Data provider with ability to insert data
pub trait DataProviderExtended<Key, Value, AccountId>: DataProvider<Key, Value> {
	/// Provide a new value for a given key from an operator
	fn feed_value(who: AccountId, key: Key, value: Value) -> DispatchResult;
}

/// Combine data provided by operators
pub trait CombineData<Key, TimestampedValue> {
	/// Combine data provided by operators
	fn combine_data(
		key: &Key,
		values: Vec<TimestampedValue>,
		prev_value: Option<TimestampedValue>,
	) -> Option<TimestampedValue>;
}

/// Indicate if should change a value
#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug)]
pub enum Change<Value> {
	/// No change.
	NoChange,
	/// Changed to new value.
	NewValue(Value),
}

/// A simple trait to provide data from a given ProviderId
pub trait MultiDataProvider<ProviderId, Key, Value>: frame_system::Trait {
	/// Provide a new value for given key and ProviderId from an operator
	fn get(source: ProviderId, key: &Key) -> Option<Value>;
}

pub struct AggregatedDataProvider<T, I>;

impl<T: Trait<I>, I: Instance> DataProvider<T::OracleKey, T::OracleValue> for AggregatedDataProvider<T, I> {
	fn get(key: &T::OracleKey) -> Option<T::OracleValue> {
		Self::get(key).map(|timestamped_value| timestamped_value.value)
	}
}

// #[macro_export]
// macro_rules! create_median_value_data_provider {
// 	( TypeName:ident, $(Providers:ty),+ ) => {
// 		pub trait Trait: system::Trait {
// 			type Source1: DataProvider<CurrencyId, Price>;
// 			type Source2: DataProvider<CurrencyId, Price>;
// 			type Source3: DataProvider<CurrencyId, Price>;
// 		}

// 		impl $TypeName {
// 			fn get(source: DataProviderId, key: &CurrencyId) -> Option<Price> {
// 				match source {
// 					DataProviderId::AcalaDataProvier => <AcalaOracle as DataProvider<CurrencyId, Price>>::get(&key),
// 					DataProviderId::BandDataProvider => <BandOracle as DataProvider<CurrencyId, Price>>::get(&key),
// 					DataProviderId::AggergatedDataProvider => None,
// 				}
// 			}

// 			fn get(_key: &<T as Trait<I>>::OracleKey) -> Option<<T as Trait<I>>::OracleValue> {
// 				let expires_in = ExpiresIn::get();
// 				let now = T::Time::now();

// 				values.retain(|x| x.timestamp + expires_in > now);

// 				let count = values.len() as u32;
// 				let minimum_count = MinimumCount::get();
// 				if count < minimum_count {
// 					return prev_value;
// 				}

// 				values.sort_by(|a, b| a.value.cmp(&b.value));

// 				let median_index = count / 2;
// 				Some(values[median_index as usize].clone())
// 			}
// 		}
// 	};
// }
