use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

use std::borrow::BorrowMut;
use std::collections::BTreeMap;

use crate::utils::{WrapIterator, WrappedIterator};

/// Alias for price type.
pub type Price = Decimal;

/// Alias for stock/contract amount type.
pub type Quantity = Decimal;

/// Represents raw trade.
#[derive(Clone)]
pub struct Trade {
    pub price: Price,
    pub quantity: Quantity,
    /// The trade ID officially marked by the exchange.
    pub trade_id: String,
    /// Received timestamp of the trade
    pub timestamp: DateTime<Utc>,
    /// Is buyer maker of this trade?
    pub is_buyer_maker: Option<bool>,
}

/// Type alias for orderbook iterators.
type OrderbookIterator<'s> = WrappedIterator<'s, (&'s Price, &'s Quantity)>;

/// Trait for orderbook.
/// The reason why I made a trait for this is because
/// there can be several different structs of an Orderbook.
///
/// Following list contains variations on level;
/// - L1: Best ask/bid
/// - L2: Aggregated asks/bids
/// - L3: Non-aggregated asks/bids (Currently not supported)
///
/// And also an orderbook can be unsized or have fixed size.
pub trait Orderbook {
    /// Return an iterator that yields asks from the best to the worst.
    fn iter_ask<'s>(&'s self) -> OrderbookIterator<'s>;
    /// Return an iterator that yields bids from the best to the worst.
    fn iter_bid<'s>(&'s self) -> OrderbookIterator<'s>;

    /// Apply delta on current orderbook.
    /// This method does not guarantee that the modified state is valid.
    fn apply_delta(&mut self, price: Price, quantity: Quantity, is_ask: bool) -> ();

    /// Return first ask price, first ask quantity, and iterator of remaining ask levels.
    /// If there is no ask, return `Price::MIN` as price and `Quantity::ZERO` as quantity.
    /// You can safely drop iterator if there is no needs.
    fn best_ask<'s>(&'s self) -> (&'s Price, &'s Quantity, OrderbookIterator<'s>) {
        let mut iter_ask = self.iter_ask();
        let (price, quantity) = iter_ask.next().unwrap_or((&Price::MIN, &Quantity::ZERO));
        (price, quantity, iter_ask)
    }

    /// Return first bid price, first bid quantity, and iterator of remaining bid levels.
    /// If there is no bid, return `Price::MAX` as price and `Quantity::ZERO` as quantity.
    /// You can safely drop iterator if there is no needs.
    fn best_bid<'s>(&'s self) -> (&'s Price, &'s Quantity, OrderbookIterator<'s>) {
        let mut iter_bid = self.iter_bid();
        let (price, quantity) = iter_bid.next().unwrap_or((&Price::MAX, &Quantity::ZERO));
        (price, quantity, iter_bid)
    }

    /// Validate if current state of this orderbook is valid.
    fn validate(&self) -> bool {
        let (mut prev_ask_p, prev_ask_q, mut iter_ask) = self.best_ask();
        let (mut prev_bid_p, prev_bid_q, mut iter_bid) = self.best_bid();
        if (prev_ask_p != &Price::MIN && prev_bid_p != &Price::MIN && prev_ask_p < prev_bid_p)
            || (prev_ask_p != &Price::MIN && prev_ask_q.is_zero())
            || (prev_bid_p != &Price::MAX && prev_bid_q.is_zero())
        {
            // 1. Both ask and bid are not empty, but price is reversed
            // 2. Ask is not empty but ask quantity is zero
            // 3. Bid is not empty but bid quantity is zero
            return false;
        }

        while let Some((worse_ask_p, worse_ask_q)) = iter_ask.next() {
            // Ask prices should be in increasing order
            if prev_ask_p > worse_ask_p || worse_ask_q.is_zero() {
                return false;
            }
            prev_ask_p = worse_ask_p;
        }
        while let Some((worse_bid_p, worse_bid_q)) = iter_bid.next() {
            // Bid prices should be in decreasing order
            if prev_bid_p < worse_bid_p || worse_bid_q.is_zero() {
                return false;
            }
            prev_bid_p = worse_bid_p;
        }
        true
    }
}

/// Represents unsized orderbook.
#[derive(Clone, Default)]
pub struct UnsizedOrderbook {
    asks: BTreeMap<Price, Quantity>,
    bids: BTreeMap<Price, Quantity>,
}

impl Orderbook for UnsizedOrderbook {
    fn iter_ask<'s>(&'s self) -> WrappedIterator<'s, (&'s Price, &'s Quantity)> {
        self.asks.iter().wrap_iter()
    }

    fn iter_bid<'s>(&'s self) -> WrappedIterator<'s, (&'s Price, &'s Quantity)> {
        self.bids.iter().wrap_iter()
    }

    fn apply_delta(&mut self, price: Price, quantity: Quantity, is_ask: bool) -> () {
        let mapping = if is_ask {
            self.asks.borrow_mut()
        } else {
            self.bids.borrow_mut()
        };
        let entry = mapping.entry(price).or_default();
        *entry += quantity;
    }
}
