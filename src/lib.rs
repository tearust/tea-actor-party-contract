// #[macro_use]
// extern crate serde_derive;
#[macro_use]
extern crate log;

use wascc_actor::prelude::codec::messaging::BrokerMessage;
use wascc_actor::prelude::*;
use wascc_actor::HandlerResult;
// use prost::Message;
// use vmh_codec::{
// 	error::DISCARD_MESSAGE_ERROR,
// 	message::structs_proto::{layer1, rpc},
// 	rpc::adapter::AdapterDispatchType,
// };

const BINDING_NAME: &str = "tea_party_contract";
const MY_ACTOR_NAME: &'static str = "TEA_PARTY_CONTRACT_HANDLER";
/// Block height duration of update runtime activity
const UPDATE_NODE_PROFILE_DURATION: u32 = 100;
const LAST_UPDATED_BLOCK_HEIGHT_KEY: &str = "last_updated_block_height";
const MINER_INFO_ITEM_KEY: &str = "miner_info_item_key";

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
		// ["reply", MY_ACTOR_NAME, uuid] => action::result_handler(&msg, uuid)?,
		// [OUTBOUND_RES_SUBJECT_PREFIX, tea_codec::ACTOR_PUBKEY_SIMPLE, ref_seq] => {
		// 	handle_outbound_response(ref_seq, &msg)?
		// }
		// ["adapter", section] => {
		// 	return handle_adapter_request(msg.body.as_slice(), section);
		// }
		// ["ra", "actor", "response"] => handle_ra_response(&msg)?,
		 _ => (),
	};
	Ok(vec![])
}

fn handle_system_init() -> anyhow::Result<()> {
	info!("tea party contract actor system init...");
	Ok(())
}


fn health(_req: codec::core::HealthRequest) -> HandlerResult<()> {
	info!("health call from simple actor");
	Ok(())
}

