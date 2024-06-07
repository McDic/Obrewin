use serde::{Deserialize, Serialize};

use crate::market::{Price, Quantity};
use crate::misc::DateTimeUTC;

/// Type alias for an order ID.
pub type OrderID = String;

/// Type alias for symbol.
pub type Symbol = String;

/// Represents an order content.
#[derive(Clone, Serialize)]
pub enum OrderContent {
    /// Represents a new direct order.
    /// Negative `quantity` means sell order.
    NewDirect { price: Price, quantity: Quantity },

    /// Represents a market order.
    /// Negative `quantity` means sell order.
    NewMarket { quantity: Quantity },

    /// Represents a cancel order.
    Cancel { original_client_order_id: OrderID },
}

/// Represents a TIF.
#[derive(Clone, Serialize)]
pub enum TimeInForce {
    /// Good Til Cancel (with provided `expiration_time`)
    GTC { expiration_time: DateTimeUTC },
    /// Immediate or cancel
    IoC,
    /// Fill or Kill = IoC + AoN (All or None)
    FoK,
}

/// Represents an order request.
#[derive(Clone, Serialize)]
pub struct OrderRequest {
    /// Main content of this order request.
    pub content: OrderContent,
    /// Target symbol.
    pub symbol: Symbol,
    /// Order request ID from client's side.
    pub client_order_id: OrderID,
}

/// Represents an order response status.
#[derive(Clone, Deserialize)]
pub enum OrderResponseStatus {
    Ok,
    Filled {
        executed_price: Price,
        executed_quantity: Quantity,
    },
    Rejected {
        code: Option<i32>,
        message: Option<String>,
    },
}

/// Represents an order response.
#[derive(Clone, Deserialize)]
pub struct OrderResponse {
    pub status: OrderResponseStatus,
    /// Same as `OrderRequest::client_order_id`.
    pub client_order_id: OrderID,
}

impl OrderResponse {
    /// Get notional value of this execution.
    pub fn notional_value(&self) -> Quantity {
        match self.status {
            OrderResponseStatus::Filled {
                executed_price,
                executed_quantity,
            } => executed_price * executed_quantity,
            _ => Quantity::ZERO,
        }
    }
}
