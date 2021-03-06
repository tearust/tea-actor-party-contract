#[macro_use]
extern crate log;
use bincode::Result as SerdeResult;
use interface::{Balance, Tsid, TOKEN_ID_TEA};
use prost::Message;
use sample::SampleTxn;
use serde::{Deserialize, Serialize};
use tea_actor_utility::{action::reply_intercom, actor_statemachine};
use token_state::token_context::TokenContext;
use vmh_codec::message::structs_proto::tokenstate::*;
use wascc_actor::prelude::codec::messaging::BrokerMessage;
use wascc_actor::prelude::*;
use wascc_actor::HandlerResult;

const VERSION: &str = env!("CARGO_PKG_VERSION");

actor_handlers! {
	codec::messaging::OP_DELIVER_MESSAGE => handle_message,
	tea_codec::OP_ACTOR_EXEC_TXN => handle_txn_exec,
	codec::core::OP_HEALTH_REQUEST => health
}
fn handle_message(msg: BrokerMessage) -> HandlerResult<Vec<u8>> {
	debug!("simple-actor received deliver message, {:?}", &msg);

	match handle_message_inner(msg) {
		Ok(res) => Ok(res),
		Err(e) => {
			error!("simple-actor handle test task error {}", e);
			Err(e)
		}
	}
}
fn handle_message_inner(msg: BrokerMessage) -> HandlerResult<Vec<u8>> {
	let channel_parts: Vec<&str> = msg.subject.split('.').collect();
	match &channel_parts[..] {
		["tea", "system", "init"] => handle_system_init()?,
		["actor", "version"] => version(&msg)?,
		_ => (),
	};
	Ok(vec![])
}

fn version(msg: &BrokerMessage) -> HandlerResult<()> {
	reply_intercom(
		&msg.reply_to,
		tea_codec::serialize(tea_codec::ActorVersionMessage {
			version: VERSION.to_string(),
		})?,
	)?;
	Ok(())
}

fn handle_system_init() -> anyhow::Result<()> {
	info!("simple actor system init...");
	Ok(())
}

fn helper_get_state_tsid() -> HandlerResult<Tsid> {
	let tsid_bytes: Vec<u8> = actor_statemachine::query_state_tsid()?;
	let tsid: Tsid = bincode::deserialize(&tsid_bytes)?;
	Ok(tsid)
}
fn handle_txn_exec(msg: BrokerMessage) -> HandlerResult<()> {
	info!("enter handle_txn_exec");
	let (tsid, txn_bytes): (Tsid, Vec<u8>) = bincode::deserialize(&msg.body)?;
	info!("before SampleTxn der");
	let sample_txn: SampleTxn = bincode::deserialize(&txn_bytes)?;
	info!("decode the sample_txn {:?}", &sample_txn);
	let base: Tsid = helper_get_state_tsid()?;
	info!("base tsid is {:?}", &base);
	let context_bytes = match sample_txn {
		SampleTxn::Topup { acct, amt } => {
			let ctx = TokenContext::new(tsid, base, TOKEN_ID_TEA);
			let ctx_bytes = bincode::serialize(&ctx)?;
			let to: u32 = acct;
			let amt: Vec<u8> = bincode::serialize(&amt)?;
			actor_statemachine::topup(TopupRequest {
				ctx: ctx_bytes,
				to,
				amt,
			})?
		}
		SampleTxn::TransferTea { from, to, amt } => {
			info!("TransferTea from to amt: {:?},{:?},{:?}", &from, &to, &amt);
			let ctx = TokenContext::new(tsid, base, TOKEN_ID_TEA);
			let ctx_bytes = bincode::serialize(&ctx)?;
			let amt: Vec<u8> = bincode::serialize(&amt)?;
			actor_statemachine::mov(MoveRequest {
				ctx: ctx_bytes,
				from,
				to,
				amt,
			})?
		}
		SampleTxn::PostMessage { from, ttl } => {
			info!("PostMessage from ttl: {:?},{:?}", &from, &ttl);
			let cost_please_add_app_logic_here = (ttl / 1024u32) as Balance;
			let to_please_add_app_logic_here_should_be_bonding_curve = 0u32;
			let ctx = TokenContext::new(tsid, base, TOKEN_ID_TEA);
			let ctx_bytes = bincode::serialize(&ctx)?;
			let amt: Vec<u8> = bincode::serialize(&cost_please_add_app_logic_here)?;
			actor_statemachine::mov(MoveRequest {
				ctx: ctx_bytes,
				from,
				to: to_please_add_app_logic_here_should_be_bonding_curve,
				amt,
			})?
		}
		SampleTxn::PrivateMessage { from, to, ttl } => {
			info!("PrivateMessage from ttl: {:?},{:?},{:?}", &from, &to, &ttl);
			let cost_please_add_app_logic_here = (ttl / 1024u32) as Balance;
			let to_please_add_app_logic_here_should_be_bonding_curve = 0u32;
			let ctx = TokenContext::new(tsid, base, TOKEN_ID_TEA);
			let ctx_bytes = bincode::serialize(&ctx)?;
			let amt: Vec<u8> = bincode::serialize(&cost_please_add_app_logic_here)?;
			actor_statemachine::mov(MoveRequest {
				ctx: ctx_bytes,
				from,
				to: to_please_add_app_logic_here_should_be_bonding_curve,
				amt,
			})?
		}
		_ => Err(anyhow::anyhow!("Unhandled txn OP type"))?,
	};
	let res_commit_ctx_bytes = actor_statemachine::commit(CommitRequest { ctx: context_bytes })?;
	if res_commit_ctx_bytes.is_empty() {
		info!("*********  Commit succesfully. the ctx is empty. it is supposed to be empty");
	}
	Ok(())
}
fn health(_req: codec::core::HealthRequest) -> HandlerResult<()> {
	info!("health call from simple actor");
	Ok(())
}
