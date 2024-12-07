use parity_scale_codec::{Decode, Encode};

#[derive(Debug, Encode, Decode)]
struct Client {
    id: u32,
    name: String,
}

fn main() {
    let client = Client {
        id: 1,
        name: "John".to_string(),
    };

    println!("Client: {:?}", client);

    let encoded = client.encode();
    println!("Encoded: {:?}", encoded);

    let decoded = Client::decode(&mut encoded.as_slice());
    println!("Decoded: {:?}", decoded);
}
