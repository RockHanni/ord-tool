use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::time::Duration;
use bitcoin::{BlockHash, Transaction, Txid};
use bitcoin::consensus::{Decodable, encode, ReadExt};
use bitcoin::hashes::hex;
use bitcoin::hashes::hex::{FromHex, HexIterator};
use bitcoin::psbt::serialize::Deserialize;
use bitcoincore_rpc::{Auth, Client, RpcApi};
use bitcoincore_rpc::bitcoincore_rpc_json::GetMempoolEntryResult;
use bitcoincore_rpc::json::GetTransactionResult;
use jsonrpsee_core::client::ClientT;
use jsonrpsee_core::rpc_params;
use jsonrpsee_http_client::HttpClientBuilder;
use tokio::time;


#[tokio::main]
async fn main() {
    let mut f = File::open("conf").unwrap();
    let mut url = Default::default();
    f.read_to_string(&mut url).unwrap();
    let see_client = HttpClientBuilder::default().max_response_size(50 * 1024 * 1024).build(&url).unwrap();
    // let block_hash: BlockHash = see_client.request("getblockhash", rpc_params!(775538)).await.unwrap();
    // println!("best block hash: {}", block_hash);


    // let block_info: GetBlockResult = see_client.request("getblock", rpc_params!(block_hash)).await.unwrap();
    // for txid in block_info.tx {
    // let _tx_hex: String = see_client.request("getrawtransaction", rpc_params!("e2637e853f8942ffb3385f6debf565b14aea2629b576492fceac61ac1de42f81",false, block_hash)).await.unwrap();
    // let tx: Transaction = deserialize_hex(&tx_hex).unwrap();

    ///
    /// advance_into_inscription_envelope in ord used to check where the inscription is using ord.
    // let vec = ord::Inscription::from_transaction(&tx);
    // for s in vec {
    //     println!("{:?}", s.inscription.content_type());
    //     println!("{:?}", s.inscription.media());
    //     println!("{:?}", s.inscription.body());
    //     println!("{:?}", s.tx_in_index);
    //
    //     let image_data = s.inscription.body().unwrap();
    //
    //     let t = s.inscription.content_type().unwrap().split('/').into_iter().map(|w| w.to_string()).collect::<Vec<String>>();
    //     let name = format!("output.{}", t[1]);
    //     let mut output_file = File::create(&name).unwrap();
    //     output_file.write(image_data).unwrap();
    // }
    ///
    //
    let rpc = Client::new(&url,
                          Auth::UserPass("a".to_string(),
                                         "a".to_string())).unwrap();

    let tx_ids: Vec<Txid> = see_client.request("getrawmempool", rpc_params!()).await.unwrap();
    for tx_id in tx_ids.iter() {
        let tx_hex: String = see_client.request("getrawtransaction", rpc_params!(tx_id,false, None::<&bitcoin::BlockHash>)).await.unwrap();
        let tx: Transaction = deserialize_hex(&tx_hex).unwrap();
        let tx_ins_vec = ord::Inscription::from_transaction(&tx);
        println!("len: {}", tx_ins_vec.len());
        if tx_ins_vec.len() > 0 {
            for (idx, ts) in tx_ins_vec.into_iter().enumerate() {
                println!("{:?}", ts.inscription.content_type());
                println!("{:?}", ts.inscription.media());
                println!("{:?}", ts.inscription.body());
                println!("{:?}", ts.tx_in_index);

                let body = ts.inscription.body().unwrap();

                let t = ts.inscription.content_type().unwrap().split('/').into_iter().map(|w| w.to_string()).collect::<Vec<String>>();
                let name = format!("output.{}.{}", idx, t[1]);
                let mut output_file = File::create(&name).unwrap();
                output_file.write(body).unwrap();
            }
        }
        time::sleep(Duration::from_millis(500));
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