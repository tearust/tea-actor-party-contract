#[macro_use]
extern crate log;
use serde::{Serialize, Deserialize};
use bincode::Result as SerdeResult;
use wascc_actor::prelude::codec::messaging::BrokerMessage;
use wascc_actor::prelude::*;
use wascc_actor::HandlerResult;
use sample::SampleTxn;
use prost::Message;
use tea_actor_utility::actor_statemachine;
use interface::{TOKEN_ID_TEA, Balance, Tsid,};
use token_state::token_context::TokenContext;
use vmh_codec::message::structs_proto::tokenstate::*;
actor_handlers! {
	codec::messaging::OP_DELIVER_MESSAGE => handle_message,
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
		["replica", "txnexec"] => return handle_txn_exec(&msg.body),
		 _ => (),
	};
	Ok(vec![])
}

fn handle_system_init() -> anyhow::Result<()> {
	info!("tea party contract actor system init...");
	Ok(())
}

fn helper_get_state_tsid()->HandlerResult<Tsid>{
	let tsid_bytes: Vec<u8> = actor_statemachine::query_state_tsid()?;
	let tsid: Tsid = bincode::deserialize(&tsid_bytes)?;
	Ok(tsid)
}

fn handle_txn_exec(tsid_txn_bytes: &[u8])-> HandlerResult<Vec<u8>>{
	let (tsid, txn_bytes):(Tsid, Vec<u8>) = bincode::deserialize(tsid_txn_bytes)?;
	let sample_txn: SampleTxn = bincode::deserialize(&txn_bytes)?;
	let base: Tsid = helper_get_state_tsid()?;
	info!("decode the sample_txn {:?}", &sample_txn);
	let context_bytes = match sample_txn {
		SampleTxn::Topup{acct, amt} =>{
			info!("acct, amt: {:?}, {:?}", &acct, &amt);
			let ctx = TokenContext::new(tsid, base, TOKEN_ID_TEA);
			let ctx_bytes = bincode::serialize(&ctx)?;
			let to: u32 = acct;
			let amt: Vec<u8> = bincode::serialize(&amt)?;
			actor_statemachine::topup(TopupRequest{
				ctx: ctx_bytes,
				to,
				amt,
			})?
			
		},
		SampleTxn::Transfer_tea{from, to, amt} => {
			info!("TransferTea from to amt: {:?},{:?},{:?}", &from, &to, &amt);
			let ctx = TokenContext::new(tsid, base, TOKEN_ID_TEA);
			let ctx_bytes = bincode::serialize(&ctx)?;
			let amt: Vec<u8> = bincode::serialize(&amt)?;
			actor_statemachine::mov(MoveRequest{
				ctx: ctx_bytes,
				from,
				to,
				amt,
			})?
		},
		SampleTxn::PostMessage{from, ttl} => {
			info!("PostMessage from ttl: {:?},{:?}", &from, &ttl);
			let cost_please_add_app_logic_here = (ttl/1024u32) as Balance;
			let to_please_add_app_logic_here_should_be_bonding_curve = 0u32;
			let ctx = TokenContext::new(tsid, base, TOKEN_ID_TEA);
			let ctx_bytes = bincode::serialize(&ctx)?;
			let amt: Vec<u8> = bincode::serialize(&cost_please_add_app_logic_here)?;
			actor_statemachine::mov(MoveRequest{
				ctx: ctx_bytes,
				from,
				to: to_please_add_app_logic_here_should_be_bonding_curve,
				amt,
			})?
		},
		SampleTxn::PrivateMessage{from, to, ttl} => {
			info!("PrivateMessage from ttl: {:?},{:?},{:?}", &from, &to, &ttl);
			let cost_please_add_app_logic_here = (ttl/1024u32) as Balance;
			let to_please_add_app_logic_here_should_be_bonding_curve = 0u32;
			let ctx = TokenContext::new(tsid, base, TOKEN_ID_TEA);
			let ctx_bytes = bincode::serialize(&ctx)?;
			let amt: Vec<u8> = bincode::serialize(&cost_please_add_app_logic_here)?;
			actor_statemachine::mov(MoveRequest{
				ctx: ctx_bytes,
				from,
				to: to_please_add_app_logic_here_should_be_bonding_curve,
				amt,
			})?
		},
		_ =>Err(anyhow::anyhow!("Unhandled txn OP type"))?,
	};
	// let context: TokenContext = bincode::deserialize(&context_bytes)?;
	let should_be_empty = actor_statemachine::commit(CommitRequest{
		ctx: context_bytes
	})?;
	Ok(should_be_empty)
}
fn health(_req: codec::core::HealthRequest) -> HandlerResult<()> {
	info!("health call from simple actor");
	Ok(())
}
