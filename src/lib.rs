#[macro_use]
extern crate log;
use serde::{Serialize, Deserialize};
use bincode::Result as SerdeResult;
use wascc_actor::prelude::codec::messaging::BrokerMessage;
use wascc_actor::prelude::*;
use wascc_actor::HandlerResult;
use sample::SampleTxn;
use prost::Message;
use tea_actor_utility::actor_statemachine::*;
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

fn handle_txn_exec(txn_bytes: &[u8])-> HandlerResult<Vec<u8>>{
	let sample_txn = bincode::deserialize(txn_bytes)?;
	info!("decode the sample_txn {:?}", &sample_txn);
	

	Ok(txn_bytes.to_vec())
}
fn health(_req: codec::core::HealthRequest) -> HandlerResult<()> {
	info!("health call from simple actor");
	Ok(())
}
