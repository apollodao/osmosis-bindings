use crate::query::OsmosisQueryWrapper;
use crate::route::OsmosisRoute;
use crate::{OsmosisQuery, PoolStateResponse};
use cosmwasm_std::{
    from_binary, to_binary, Querier, QuerierResult, QuerierWrapper, QueryRequest, StdResult,
};

pub struct OsmosisQuerier<'a> {
    querier: &'a QuerierWrapper<'a>,
}

impl Querier for OsmosisQuerier<'_> {
    fn raw_query(&self, bin_request: &[u8]) -> QuerierResult {
        self.querier.raw_query(bin_request)
    }
}

impl<'a> OsmosisQuerier<'a> {
    pub fn new(querier: &'a QuerierWrapper<'a>) -> Self {
        OsmosisQuerier { querier }
    }

    pub fn query_pool_state(&self, pool_id: u64) -> StdResult<PoolStateResponse> {
        let req = OsmosisQueryWrapper {
            route: OsmosisRoute::Gamm,
            query_data: OsmosisQuery::PoolState { id: pool_id },
        };

        from_binary::<PoolStateResponse>(
            &self
                .querier
                .raw_query(to_binary(&QueryRequest::Custom(&req))?.as_slice())
                .unwrap()
                .unwrap(),
        )
    }
}
