use std::fs::File;
use std::io::{Read, Write};
use bitcoin::{BlockHash, Transaction};
use bitcoin::consensus::{Decodable, ReadExt};
use bitcoin::hashes::hex::HexIterator;
use jsonrpsee_core::client::ClientT;
use jsonrpsee_core::rpc_params;
use jsonrpsee_http_client::HttpClientBuilder;


#[tokio::main]
async fn main() {
    let mut f = File::open("conf").unwrap();
    let mut url = Default::default();
    f.read_to_string(&mut url).unwrap();
    let see_client = HttpClientBuilder::default().build(url).unwrap();
    let block_hash: BlockHash = see_client.request("getblockhash", rpc_params!(775538)).await.unwrap();
    println!("best block hash: {}", block_hash);


    // let block_info: GetBlockResult = see_client.request("getblock", rpc_params!(block_hash)).await.unwrap();
    // for txid in block_info.tx {
    let tx_hex: String = see_client.request("getrawtransaction", rpc_params!("e2637e853f8942ffb3385f6debf565b14aea2629b576492fceac61ac1de42f81",false, block_hash)).await.unwrap();
    let tx: Transaction = deserialize_hex(&tx_hex).unwrap();
    let vec = ord::Inscription::from_transaction(&tx);
    for s in vec {
        println!("{:?}", s.inscription.content_type());
        println!("{:?}", s.inscription.media());
        println!("{:?}", s.inscription.body());
        println!("{:?}", s.tx_in_index);

        let image_data = s.inscription.body().unwrap();

        let t = s.inscription.content_type().unwrap().split('/').into_iter().map(|w| w.to_string()).collect::<Vec<String>>();
        let name = format!("output.{}", t[1]);
        let mut output_file = File::create(&name).unwrap();
        output_file.write(image_data).unwrap();
    }
    // }
}


fn deserialize_hex<T: Decodable>(hex: &str) -> anyhow::Result<T> {
    let mut reader = HexIterator::new(&hex).map_err(|e| anyhow::Error::msg(e.to_string()))?;
    let object = Decodable::consensus_decode(&mut reader).map_err(|e| anyhow::Error::msg(e.to_string()))?;
    if reader.read_u8().is_ok() {
        Err(anyhow::Error::msg(bitcoin::consensus::encode::Error::ParseFailed(
            "data not consumed entirely when explicitly deserializing",
        )))
    } else {
        Ok(object)
    }
}