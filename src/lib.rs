#[macro_use]
extern crate log;

use wascc_actor::prelude::codec::messaging::BrokerMessage;
use wascc_actor::prelude::*;
use wascc_actor::HandlerResult;
// use prost::Message;

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
	// just echo the input back for communication testing
	Ok(txn_bytes.to_vec())
}
fn health(_req: codec::core::HealthRequest) -> HandlerResult<()> {
	info!("health call from simple actor");
	Ok(())
}
