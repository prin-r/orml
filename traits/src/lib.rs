#![cfg_attr(not(feature = "std"), no_std)]
use codec::{Decode, Encode};
use sp_runtime::{DispatchResult, RuntimeDebug};
use sp_std::cmp::{Eq, PartialEq};

pub use auction::{Auction, AuctionHandler, AuctionInfo, OnNewBidResult};
pub use currency::{
	BalanceStatus, BasicCurrency, BasicCurrencyExtended, BasicLockableCurrency, BasicReservableCurrency,
	LockIdentifier, MultiCurrency, MultiCurrencyExtended, MultiLockableCurrency, MultiReservableCurrency, OnReceived,
};
pub use price::{DefaultPriceProvider, PriceProvider};

use sp_std::prelude::Vec;

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
pub trait MultiDataProvider<ProviderId, Key, Value> {
	/// Provide a new value for given key and ProviderId from an operator
	fn get(source: ProviderId, key: &Key) -> Option<Value>;
}

/// TODO: Modify this macro to support get_no_op and get_all_values
#[macro_export]
macro_rules! create_median_value_data_provider {
	(
		$TypeName:ident, $( $Provider:ty ),*
	) => {
		pub struct $TypeName;
		impl DataProvider<CurrencyId, Price> for $TypeName {
			fn get(key: &CurrencyId) -> Option<Price> {
				let mut values: Vec<Option<Price>> = Vec::new();
				$(
					values.push(<$Provider as DataProvider<CurrencyId, Price>>::get(&key));
				)*

				values.retain(|&x| x != None);
				values.sort_by(|a, b| a.cmp(&b));
				if values.len() == 0 {
					return None;
				}
				let mid = values.len() / 2;
				if values.len() % 2 == 0 {
					match (values[mid - 1], values[mid]) {
						(Some(x), Some(y)) => Some((x + y) / FixedU128::saturating_from_integer(2)),
						_ => None,
					}
				} else {
					values[mid]
				}
			}
		}
	};
}
