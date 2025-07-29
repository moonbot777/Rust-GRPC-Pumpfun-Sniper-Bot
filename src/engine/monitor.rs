use anchor_client::solana_sdk::{hash::Hash, pubkey::Pubkey, signature::Signature};
use borsh::from_slice;
use maplit::hashmap;
use spl_token::solana_program::native_token::{lamports_to_sol, LAMPORTS_PER_SOL};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::{collections::HashSet, time::Duration};
use tokio::process::Command;

use super::swap::{SwapDirection, SwapInType};
use crate::common::config::{JUPITER_PROGRAM, OKX_DEX_PROGRAM, PROGRAM_DATA_PREFIX};
use crate::common::{
    config::{AppState, LiquidityPool, Status, SwapConfig, PUMP_LOG_INSTRUCTION},
    logger::Logger,
};
use crate::core::tx;
use crate::dex::pump_fun::{
    Pump, INITIAL_VIRTUAL_SOL_RESERVES, INITIAL_VIRTUAL_TOKEN_RESERVES,
    PUMP_FUN_CREATE_IX_DISCRIMINATOR, PUMP_PROGRAM,
};
use anyhow::{anyhow, Result};
use chrono::Utc;
use colored::Colorize;
use futures_util::stream::StreamExt;
use futures_util::SinkExt;
use std::str::FromStr;
use tokio::{
    task,
    time::{self, Instant},
};
use yellowstone_grpc_client::{ClientTlsConfig, GeyserGrpcClient};
use yellowstone_grpc_proto::geyser::{
    subscribe_update::UpdateOneof, CommitmentLevel, SubscribeRequest,
    SubscribeRequestFilterTransactions, SubscribeUpdateTransaction,
};

#[derive(Clone, Debug)]
pub struct BondingCurveInfo {
    pub bonding_curve: Pubkey,
    pub new_virtual_sol_reserve: u64,
    pub new_virtual_token_reserve: u64,
}
