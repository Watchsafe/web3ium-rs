use alloy_consensus::TxEnvelope;
use alloy_primitives::hex;
use alloy_rlp::Decodable;

pub fn decode_raw_tx(_tx: &str) -> Result<TxEnvelope, Box<dyn std::error::Error>> {
    let raw_tx = hex::decode(_tx).unwrap();
    let res = TxEnvelope::decode(&mut raw_tx.as_slice()).unwrap();
    Ok(res)
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn test_decode_legacy_tx() {
       // Legacy transaction raw data
       let legacy_tx = "0xf8a91e85032c9797e982d3ea94ec53bf9167f50cdeb3ae105f56099aaab9061f8380b844095ea7b3000000000000000000000000163a5ec5e9c32238d075e2d829fe9fa87451e3b70000000000000000000000000000000000000000000000000de0b6b3a764000025a0437a7c1077dd8fb77c434756f486346c564556e0ea65e59428643b91b7184632a070df9c281661b23f4e7547015a9382c9a8c8e23393733eb9550b6630528a4005";
       
       let tx = decode_raw_tx(legacy_tx).unwrap();
       println!("Legacy transaction decoded: {:#?}", tx);
   }

   #[test]
   fn test_decode_eip1559_tx() {
       // EIP-1559 transaction raw data
       let eip1559_tx = "0x02f8b001018450775d80850324a9a70082d3ea94ec53bf9167f50cdeb3ae105f56099aaab9061f8380b844095ea7b3000000000000000000000000163a5ec5e9c32238d075e2d829fe9fa87451e3b70000000000000000000000000000000000000000000000000de0b6b3a7640000c001a098421643be02def45744834741859d065b20dfe814001dcc54f521626281a5e0a03fe4c9d2cb0a473865efe0ebee2cf5288aaa54dedf5093430a88ac5c167e5d90";
       
       let tx = decode_raw_tx(eip1559_tx).unwrap();
       println!("EIP-1559 transaction decoded: {:#?}", tx);
   }

   #[test]
   fn test_decode_with_0x_prefix() {
       let legacy_tx = "0xf8691e850324a9a70082d3ea94ec53bf9167f50cdeb3ae105f56099aaab9061f8380b844095ea7b3000000000000000000000000163a5ec5e9c32238d075e2d829fe9fa87451e3b70000000000000000000000000000000000000000000000000de0b6b3a7640000018080";
       
       let tx = decode_raw_tx(legacy_tx).unwrap();
       println!("Legacy transaction with 0x prefix decoded: {:#?}", tx);
   }
}