use obrewin_data_structures::market::Trade;
use obrewin_data_structures::order::{OrderRequest, OrderResponse};

use crate::utils::BAM;

/// Represents an adapter.
#[async_trait::async_trait]
pub trait Adapter {
    /// Send an order request, then return a received order response.
    async fn send_order_request(&self, request: OrderRequest) -> OrderResponse;

    /// Call given `handler` on each received trades.
    async fn receive_trades<'se, 'h>(
        &'se self,
        handler: BAM<'h, Trade, anyhow::Result<()>>,
    ) -> anyhow::Result<()>
    where
        'h: 'se;

    async fn receive_orderbook<'se, 'h>(
        &'se self,
        handler: BAM<'h, Trade, anyhow::Result<()>>,
    ) -> anyhow::Result<()>
    where
        'h: 'se;
}
