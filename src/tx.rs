use bitcoin::secp256k1::{self, Keypair, Message, PublicKey, XOnlyPublicKey};
use bitcoin::taproot::TaprootSpendInfo;
use bitcoin::{
    Address, Amount, CompressedPublicKey, Network, OutPoint, ScriptBuf, Sequence, Transaction,
    TxIn, TxOut, Witness, absolute, opcodes, sighash, taproot,
};

use crate::musig;

lazy_static::lazy_static! {
    /// Global secp context.
    pub static ref SECP: secp256k1::Secp256k1<secp256k1::All> = secp256k1::Secp256k1::new();
}

pub const EXPIRY: usize = 144 * 30;

pub fn delayed_sign(delay_blocks: usize, pubkey: XOnlyPublicKey) -> ScriptBuf {
    let csv = bitcoin::Sequence::from_height(delay_blocks as u16);
    bitcoin::Script::builder()
        .push_int(csv.to_consensus_u32() as i64)
        .push_opcode(opcodes::all::OP_CSV)
        .push_opcode(opcodes::all::OP_DROP)
        .push_x_only_key(&pubkey)
        .push_opcode(opcodes::all::OP_CHECKSIG)
        .into_script()
}

pub fn multisig(server: PublicKey, user: PublicKey) -> ScriptBuf {
    bitcoin::Script::builder()
        .push_x_only_key(&server.into())
        .push_opcode(opcodes::all::OP_CHECKSIG)
        .push_x_only_key(&user.into())
        .push_opcode(opcodes::all::OP_CHECKSIG)
        .into_script()
}

pub fn create_taproot(server: PublicKey, user: PublicKey) -> TaprootSpendInfo {
    let agg_pk = musig::combine_keys([user, server]);
    taproot::TaprootBuilder::new()
        .add_leaf(1, delayed_sign(EXPIRY, user.into()))
        .unwrap()
        .add_leaf(1, multisig(server, user))
        .unwrap()
        .finalize(&SECP, agg_pk)
        .unwrap()
}

pub fn create_spk(server: PublicKey, user: PublicKey) -> ScriptBuf {
	let tr = create_taproot(server, user);
	ScriptBuf::new_p2tr_tweaked(tr.output_key())
}

pub fn create_address(server: PublicKey, user: PublicKey) -> Address {
	Address::p2tr_tweaked(create_taproot(server, user).output_key(), Network::Regtest)
}

pub fn update(
    user: Keypair,
    server_amount: Amount,
    user_amount: Amount,
    server: PublicKey,
    channel: &Transaction,
) -> Transaction {
    let mut tx = Transaction {
        version: bitcoin::transaction::Version::TWO,
        lock_time: absolute::LockTime::ZERO,
        input: vec![TxIn {
            previous_output: OutPoint::new(channel.compute_txid(), 0),
            script_sig: ScriptBuf::new(),
            sequence: Sequence::ZERO,
            witness: { Witness::new() },
        }],
        output: vec![
            // server
            TxOut {
                value: server_amount,
                script_pubkey: ScriptBuf::new_p2pkh(&CompressedPublicKey(server).pubkey_hash()),
            },
            // server
            TxOut {
                value: user_amount,
                script_pubkey: ScriptBuf::new_p2pkh(
                    &CompressedPublicKey(user.public_key()).pubkey_hash(),
                ),
            },
        ],
    };

    let mut shc = sighash::SighashCache::new(&tx);

    let leaf_hash = taproot::TapLeafHash::from_script(
        &multisig(server, user.public_key()),
        taproot::LeafVersion::TapScript,
    );
    let sighash = shc
        .taproot_script_spend_signature_hash(
            0,
            &sighash::Prevouts::All(&[&channel.output[0]]),
            leaf_hash,
            bitcoin::TapSighashType::Default,
        )
        .unwrap();

    let sig = SECP.sign_schnorr(&Message::from_digest_slice(&sighash[..]).unwrap(), &user);

    tx.input[0].witness.push(&sig[..]);

    tx
}

pub fn tx_json(tx: &Transaction) -> String {
	serde_json::to_string_pretty(&hal::GetInfo::get_info(tx, Network::Regtest)).unwrap()
}
