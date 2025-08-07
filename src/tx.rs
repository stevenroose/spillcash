use bitcoin::{opcodes, taproot, ScriptBuf};
use bitcoin::secp256k1::{self, PublicKey, XOnlyPublicKey};


lazy_static::lazy_static! {
	/// Global secp context.
	pub static ref SECP: secp256k1::Secp256k1<secp256k1::All> = secp256k1::Secp256k1::new();
}


pub const EXPIRY: usize = 144*30;


// pub fn delayed_sign(delay_blocks: usize, pubkey: XOnlyPublicKey) -> ScriptBuf {
// 	let csv = bitcoin::Sequence::from_height(delay_blocks as u16);
// 	bitcoin::Script::builder()
// 		.push_int(csv.to_consensus_u32() as i64)
// 		.push_opcode(opcodes::all::OP_CSV)
// 		.push_opcode(opcodes::all::OP_DROP)
// 		.push_x_only_key(&pubkey)
// 		.push_opcode(opcodes::all::OP_CHECKSIG)
// 		.into_script()
// }
//
// pub fn create_address(server: PublicKey, user: PublicKey) {
// 	let agg_pk = musig::combine_keys(keys)
// 	taproot::TaprootBuilder::new()
// 		.add_leaf(0, delayed_sign(EXPIRY, user)).unwrap()
// 		.finalize(&SECP, agg_pk).unwrap()
//
// }
