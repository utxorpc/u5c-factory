//! Maps the Cardano blocks in `../blocks` to UTxO RPC messages and writes the vectors.

use pallas_utxorpc::{v1alpha, v1beta, LedgerContext, TxoRef, UtxoMap};
use prost::Message;
use std::{fmt::Write, fs, path::Path};

#[derive(Clone)]
struct NoLedger;

impl LedgerContext for NoLedger {
    fn get_utxos(&self, _refs: &[TxoRef]) -> Option<UtxoMap> {
        None
    }

    fn get_slot_timestamp(&self, _slot: u64) -> Option<u64> {
        None
    }
}

fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");

    let mut names: Vec<_> = fs::read_dir(root.join("blocks"))
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .collect();
    names.sort();

    fs::create_dir_all(root.join("v1alpha")).unwrap();
    fs::create_dir_all(root.join("v1beta")).unwrap();

    let mut index = String::from("name\tera\tprotocol\tslot\thash\ttx_count\tfirst_tx_hash\n");

    for path in names {
        let name = path.file_stem().unwrap().to_str().unwrap().to_string();
        let block = hex::decode(fs::read_to_string(&path).unwrap().trim()).unwrap();
        let parsed = pallas_traverse::MultiEraBlock::decode(&block).unwrap();
        let txs = parsed.txs();

        let protocol = name.rsplit_once("-v").unwrap().1;
        let first_tx = txs.first().map(|tx| tx.hash().to_string());

        writeln!(
            index,
            "{name}\t{}\t{protocol}\t{}\t{}\t{}\t{}",
            parsed.era(),
            parsed.slot(),
            parsed.hash(),
            txs.len(),
            first_tx.as_deref().unwrap_or("-"),
        )
        .unwrap();

        let alpha = v1alpha::Mapper::new(NoLedger).map_block_cbor(&block);
        let beta = v1beta::Mapper::new(NoLedger).map_block_cbor(&block);

        fs::write(root.join(format!("v1alpha/{name}.pb")), alpha.encode_to_vec()).unwrap();
        fs::write(root.join(format!("v1beta/{name}.pb")), beta.encode_to_vec()).unwrap();
    }

    fs::write(root.join("vectors.tsv"), index).unwrap();
}
