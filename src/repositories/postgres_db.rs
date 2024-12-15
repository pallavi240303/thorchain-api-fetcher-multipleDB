use async_trait::async_trait;
use native_tls::TlsConnector;
use postgres_native_tls::MakeTlsConnector;
use std::{error::Error, time::Instant};
use tokio_postgres::Client;

use crate::{
    models::{DepthInterval, EarningInterval, RunePoolInterval, SwapsInterval,Pool},
    services::db_traits::Database,
};

pub struct PostgresDb {
    client: Client,
}

impl PostgresDb {
    pub async fn new(conn: &str) -> Result<Self, Box<dyn Error>> {
        let connector = TlsConnector::builder().build().unwrap();
        let connector = MakeTlsConnector::new(connector);
        let (client, connection) = tokio_postgres::connect(&conn, connector).await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Database Connection error: {}", e);
            }
        });

        Ok(PostgresDb { client })
    }
}

#[async_trait]
impl Database for PostgresDb {
    async fn store_depth_intervals(
        &self,
        interval: DepthInterval,
    ) -> Result<std::time::Duration, Box<dyn Error>> {
        println!("inside store depth function");
        let start_time = Instant::now();
        self.client.execute("INSERT INTO depthinterval (asset_depth, asset_price, asset_price_usd, end_time, liquidity_units, luvi, members_count, rune_depth, start_time, synth_supply, synth_units, units) 
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12) ON CONFLICT (end_time) DO NOTHING;
        ", 
            &[
                &interval.asset_depth ,
                &interval.asset_price ,
                &interval.asset_price_usd ,
                &interval.end_time ,
                &interval.liquidity_units ,
                &interval.luvi ,
                &interval.members_count ,
                &interval.rune_depth,
                &interval.start_time,
                &interval.synth_supply,
                &interval.synth_units,
                &interval.units,
            ],
            ).await?;
        let duration = start_time.elapsed(); 
        Ok(duration)
    }

    async fn store_swaps_intervals(
        &self,
        swap: SwapsInterval,
    ) -> Result<std::time::Duration, Box<dyn Error>> {
        let start_time = Instant::now();
        self.client.execute("INSERT INTO swapsinterval (average_slip, end_time, from_trade_average_slip, from_trade_count, from_trade_fees, from_trade_volume, from_trade_volume_usd, rune_price_usd, start_time, synth_mint_average_slip, synth_mint_count, synth_mint_fees, synth_mint_volume, synth_mint_volume_usd, synth_redeem_average_slip, synth_redeem_count, synth_redeem_fees, synth_redeem_volume, synth_redeem_volume_usd, to_asset_average_slip, to_asset_count, to_asset_fees, to_asset_volume, to_asset_volume_usd, to_rune_average_slip, to_rune_count, to_rune_fees, to_rune_volume, to_rune_volume_usd, total_count, total_fees, total_volume, total_volume_usd) 
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30, $31, $32, $33) ON CONFLICT (end_time) DO NOTHING
        ;", 
            &[
                &swap.average_slip,
                &swap.end_time,
                &swap.from_trade_average_slip,
                &swap.from_trade_count,
                &swap.from_trade_fees,
                &swap.from_trade_volume,
                &swap.from_trade_volume_usd,
                &swap.rune_price_usd,
                &swap.start_time,
                &swap.synth_mint_average_slip,
                &swap.synth_mint_count,
                &swap.synth_mint_fees,
                &swap.synth_mint_volume,
                &swap.synth_mint_volume_usd,
                &swap.synth_redeem_average_slip,
                &swap.synth_redeem_count,
                &swap.synth_redeem_fees,
                &swap.synth_redeem_volume,
                &swap.synth_redeem_volume_usd,
                &swap.to_asset_average_slip,
                &swap.to_asset_count,
                &swap.to_asset_fees,
                &swap.to_asset_volume,
                &swap.to_asset_volume_usd,
                &swap.to_rune_average_slip,
                &swap.to_rune_count,
                &swap.to_rune_fees,
                &swap.to_rune_volume,
                &swap.to_rune_volume_usd,
                &swap.total_count,
                &swap.total_fees,
                &swap.total_volume,
                &swap.total_volume_usd,
            ],
        ).await?;
        let duration = start_time.elapsed(); 
        Ok(duration)
    }

    async fn store_earnings_intervals(
        &self,
        interval: EarningInterval,
    ) -> Result<std::time::Duration, Box<dyn Error>> {
        let pools_json = serde_json::to_value(&interval.pools).map_err(|e| {
            eprintln!("Error serializing pools: {}", e);
            e
        })?;
        let start_time = Instant::now();
        self.client.execute("INSERT INTO earninginterval (avg_node_count, block_rewards, bonding_earnings, earnings, end_time, liquidity_earnings, liquidity_fees, rune_price_usd, start_time ,pools) 
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9 , $10) ON CONFLICT (end_time) DO NOTHING
            ;",
                &[
                    &interval.avg_node_count,
                    &interval.block_rewards,
                    &interval.bonding_earnings,
                    &interval.earnings,
                    &interval.end_time,
                    &interval.liquidity_earnings,
                    &interval.liquidity_fees,
                    &interval.rune_price_usd,
                    &interval.start_time,
                    &pools_json
                ],
            ).await?;
        let duration = start_time.elapsed(); 
        Ok(duration)
    }

    async fn store_runepool_intervals(
        &self,
        runepool: RunePoolInterval,
    ) -> Result<std::time::Duration, Box<dyn Error>> {
        let start_time = Instant::now();
        self.client
            .execute(
                "INSERT INTO runepoolinterval (count, end_time, start_time, units) 
            VALUES ($1, $2, $3, $4) ON CONFLICT (end_time) DO NOTHING;",
            &[
                &runepool.count,
                &runepool.end_time,
                &runepool.start_time,
                &runepool.units,
                ],
            )
            .await?;
        let duration = start_time.elapsed(); 
        Ok(duration)
    }
    async fn read_depth_intervals(
        &self,
    ) -> Result<(Vec<DepthInterval>, std::time::Duration), Box<dyn Error>> {
        let start_time = Instant::now();
        let rows = self
        .client
        .query("SELECT * FROM depthinterval", &[])
        .await?;
        let duration = start_time.elapsed();
        let mut intervals = Vec::new();
        for row in rows {
            let interval = DepthInterval {
                asset_depth: row.get("asset_depth"),
                asset_price: row.get("asset_price"),
                asset_price_usd: row.get("asset_price_usd"),
                end_time: row.get("end_time"),
                liquidity_units: row.get("liquidity_units"),
                luvi: row.get("luvi"),
                members_count: row.get("members_count"),
                rune_depth: row.get("rune_depth"),
                start_time: row.get("start_time"),
                synth_supply: row.get("synth_supply"),
                synth_units: row.get("synth_units"),
                units: row.get("units"),
            };
            intervals.push(interval);
        }
        Ok((intervals, duration))
    }

    async fn read_swaps_intervals(
        &self,
    ) -> Result<(Vec<SwapsInterval>, std::time::Duration), Box<dyn Error>> {
        let start_time = Instant::now();
        let rows = self
        .client
        .query("SELECT * FROM swapsinterval", &[])
        .await?;
        let duration = start_time.elapsed();
        let swaps_intervals: Vec<SwapsInterval> = rows
            .into_iter()
            .map(|row| SwapsInterval {
                average_slip: row.get("average_slip"),
                end_time: row.get("end_time"),
                from_trade_average_slip: row.get("from_trade_average_slip"),
                from_trade_count: row.get("from_trade_count"),
                from_trade_fees: row.get("from_trade_fees"),
                from_trade_volume: row.get("from_trade_volume"),
                from_trade_volume_usd: row.get("from_trade_volume_usd"),
                rune_price_usd: row.get("rune_price_usd"),
                start_time: row.get("start_time"),
                synth_mint_average_slip: row.get("synth_mint_average_slip"),
                synth_mint_count: row.get("synth_mint_count"),
                synth_mint_fees: row.get("synth_mint_fees"),
                synth_mint_volume: row.get("synth_mint_volume"),
                synth_mint_volume_usd: row.get("synth_mint_volume_usd"),
                synth_redeem_average_slip: row.get("synth_redeem_average_slip"),
                synth_redeem_count: row.get("synth_redeem_count"),
                synth_redeem_fees: row.get("synth_redeem_fees"),
                synth_redeem_volume: row.get("synth_redeem_volume"),
                synth_redeem_volume_usd: row.get("synth_redeem_volume_usd"),
                to_asset_average_slip: row.get("to_asset_average_slip"),
                to_asset_count: row.get("to_asset_count"),
                to_asset_fees: row.get("to_asset_fees"),
                to_asset_volume: row.get("to_asset_volume"),
                to_asset_volume_usd: row.get("to_asset_volume_usd"),
                to_rune_average_slip: row.get("to_rune_average_slip"),
                to_rune_count: row.get("to_rune_count"),
                to_rune_fees: row.get("to_rune_fees"),
                to_rune_volume: row.get("to_rune_volume"),
                to_rune_volume_usd: row.get("to_rune_volume_usd"),
                total_count: row.get("total_count"),
                total_fees: row.get("total_fees"),
                total_volume: row.get("total_volume"),
                total_volume_usd: row.get("total_volume_usd"),
            })
            .collect();
        Ok((swaps_intervals, duration))
    }

    async fn read_earnings_intervals(&self) -> Result<(Vec<EarningInterval>, std::time::Duration), Box<dyn Error>> {
        let start_time = Instant::now();
        let rows = self.client.query("SELECT * FROM earninginterval", &[]).await?;
        let duration = start_time.elapsed();
        let earnings_intervals: Vec<EarningInterval> = rows.into_iter().map(|single_row| {  
            let pools_json: Option<String> = single_row.get("pools");
            let pools: Vec<Pool> = if let Some(json_data) = pools_json {
                serde_json::from_str(&json_data).unwrap_or_else(|_| Vec::new())  
            } else {
                Vec::new()
            };
    
            EarningInterval {
                avg_node_count: single_row.get("avg_node_count"),
                block_rewards: single_row.get("block_rewards"),
                bonding_earnings: single_row.get("bonding_earnings"),
                earnings: single_row.get("earnings"),
                end_time: single_row.get("end_time"),
                liquidity_earnings: single_row.get("liquidity_earnings"),
                liquidity_fees: single_row.get("liquidity_fees"),
                rune_price_usd: single_row.get("rune_price_usd"),
                start_time: single_row.get("start_time"),
                pools,  
            }
        }).collect();
        Ok((earnings_intervals, duration))
    }

    async fn read_runepool_intervals(
        &self,
    ) -> Result<(Vec<RunePoolInterval>, std::time::Duration), Box<dyn Error>> {
        let start_time = Instant::now();
        let rows = self
            .client
            .query("SELECT * FROM runepoolinterval", &[])
            .await?;
        let duration = start_time.elapsed();
        let runepool_intervals: Vec<RunePoolInterval> = rows
            .into_iter()
            .map(|row| RunePoolInterval {
                count: row.get("count"),
                end_time: row.get("end_time"),
                start_time: row.get("start_time"),
                units: row.get("units"),
            })
            .collect();
        Ok((runepool_intervals, duration))
    }
}
