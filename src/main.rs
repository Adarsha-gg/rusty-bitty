use::std::io::Read;
use serde::{Serialize, Serializer};

#[derive(Debug, Serialize)]

struct Input{
    txid: String,
    output_index: u32,
    script_sig: String,
    sequence: u32
}

trait BitcoinValue{
    fn to_btc(&self) -> f64;
}

impl BitcoinValue for Amount{
    fn to_btc(&self) -> f64{
        self.0 as f64/100_000_000.0
    }
}

#[derive(Debug)]
struct Amount(u64);


#[derive(Debug, Serialize)]
struct Output{
    #[serde(serialize_with = "as_btc")]
    amount: Amount,
    script_pubkey: String,
}

fn as_btc<S:Serializer, T:BitcoinValue>(t: &T, s: S) -> Result<S::Ok, S::Error>{
    let btc = t.to_btc();
    s.serialize_f64(btc)

}

#[derive(Debug, Serialize)]
struct Transaction{
    version:u32,
    inputs: Vec<Input>,
    output: Vec<Output>
}


fn main() {
    let txn_hash = "010000000242d5c1d6f7308bbe95c0f6e1301dd73a8da77d2155b0773bc297ac47f9cd7380010000006a4730440220771361aae55e84496b9e7b06e0a53dd122a1425f85840af7a52b20fa329816070220221dd92132e82ef9c133cb1a106b64893892a11acf2cfa1adb7698dcdc02f01b0121030077be25dc482e7f4abad60115416881fe4ef98af33c924cd8b20ca4e57e8bd5feffffff75c87cc5f3150eefc1c04c0246e7e0b370e64b17d6226c44b333a6f4ca14b49c000000006b483045022100e0d85fece671d367c8d442a96230954cdda4b9cf95e9edc763616d05d93e944302202330d520408d909575c5f6976cc405b3042673b601f4f2140b2e4d447e671c47012103c43afccd37aae7107f5a43f5b7b223d034e7583b77c8cd1084d86895a7341abffeffffff02ebb10f00000000001976a9144ef88a0b04e3ad6d1888da4be260d6735e0d308488ac508c1e000000000017a91476c0c8f2fc403c5edaea365f6a284317b9cdf7258700000000";
    let txn_bytes = hex::decode(txn_hash).unwrap();
    let mut byte_slice  = txn_bytes.as_slice();
    let ok = read_u32(&mut byte_slice);
    let compact_size_count = read_compact_size(&mut byte_slice) ;
    
    let mut inputs = vec![];

    for _ in 0..compact_size_count{
        let txid = read_txn_id(&mut byte_slice);
        let output_index = read_u32(&mut byte_slice);
        let script_sig = read_script(&mut byte_slice);
        let sequence = read_u32(&mut byte_slice);   

        inputs.push(Input{
            txid,
            output_index,
            script_sig,
            sequence
        })
    }

    let output_count = read_compact_size(&mut byte_slice);
    let mut output = vec![];

    for _ in 0..output_count{
        let  amount = read_amount(&mut byte_slice);
        let  script_pubkey =read_script(&mut byte_slice);

        output.push(Output{
            amount: amount,
            script_pubkey: script_pubkey
        });
    }


    let mut transaction = vec![];
    transaction.push(Transaction{
        version: ok,
        inputs: inputs,
        output: output
        
    });
    
    println!("{}",serde_json::to_string_pretty(&transaction).unwrap()); 
}

fn read_script(slice:&mut &[u8]) -> String{
    let script_size = read_compact_size(slice) as usize;
    let mut buffer = vec!(0_u8; script_size);
    slice.read(&mut buffer[..]).unwrap();
    hex::encode(buffer)

}

fn read_u32(slice:&mut&[u8]) -> u32 {
    let mut buffer = [0; 4];
    slice.read(&mut buffer).unwrap();   
    let indian = u32::from_le_bytes(buffer);
    return indian;
}

fn read_amount(slice:&mut &[u8]) -> Amount{
    let mut buffer = [0;8];
    slice.read(&mut buffer).unwrap();
    let indian = u64::from_le_bytes(buffer);
    Amount(indian)
}

fn read_txn_id(slice:&mut &[u8]) -> String{
    let mut buffer = [0; 32];
    slice.read(&mut buffer).unwrap();
    buffer.reverse();
    hex::encode(buffer)

}

fn read_compact_size(slice:&mut&[u8]) -> u64{
    let mut compact_size = [0_u8; 1];
    slice.read(&mut compact_size).unwrap();
    match compact_size[0]{
        0..=252=> compact_size[0] as u64,
        254 => {
            let mut buffer = [0; 2];
        slice.read(&mut buffer).unwrap();
        u16::from_le_bytes(buffer) as u64
        },
        255 => {
            let mut buffer = [0; 4];
            slice.read(&mut buffer).unwrap();
            u32::from_le_bytes(buffer) as u64
        },
        253 =>{
            let mut buffer = [0; 8];
        slice.read(&mut buffer).unwrap();
        u64::from_le_bytes(buffer)
        }
    }   
}

#[cfg(test)]
mod test{
    use crate::read_compact_size;
    // use super::read_compact_size;

    #[test]
    fn test_read_compact_size(){
        let mut bytes = [1_u8].as_slice();
        let compact = read_compact_size(&mut bytes);
        assert!(compact == 1);
        assert_eq!(compact, 1_u64);
        println!("{:?}",compact);
    
    
    }


}




//   // These 3 things do the same thing
//   let version_bytes= <[u8; 4]>::try_from(&txn_bytes[0..4]).unwrap();
    
//   let into_version_bytes: [u8; 4] = (&txn_bytes[0..4]).try_into().unwrap();
  
//   let mut array = [0_u8; 4];
//   array.copy_from_slice(&txn_bytes[0..4]);
  
// fn read_compact_size(slice:&mut&[u8]) -> u64{
//     let mut compact_size = [0_u8; 1];
//     slice.read(&mut compact_size).unwrap();
//     if (0..253).contains(&compact_size[0]){
//         compact_size[0] as u64
//     }
//     else if compact_size[0] == 253 {
//         let mut buffer = [0; 2];
//         slice.read(&mut buffer).unwrap();
//         u16::from_le_bytes(buffer) as u64
//     }
//     else if compact_size[0] == 254 {
//         let mut buffer = [0; 4];
//         slice.read(&mut buffer).unwrap();
//         u32::from_le_bytes(buffer) as u64
//     }
//     else{
//         let mut buffer = [0; 8];
//         slice.read(&mut buffer).unwrap();
//         u64::from_le_bytes(buffer)
//     }

// impl fmt::Debug for Input{
//     fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result{
//         f.debug_struct("Input")
//          .field("txid", &self.txid)
//          .field("output_index", &self.output_index)
//          .field("script_sig", &self.script_sig)
//          .field("sequence", &self.sequence)
//          .finish()
     
//     }   
// }

// let json_inputs = serde_json::to_string_pretty(&transaction).unwrap();