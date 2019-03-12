use sha2::{Sha256, Digest};

pub struct Channel{
    address: AccountId, // find accountId implementation in system
    participants: [AccountId],
    nonce: i32
}

impl Channel{
    pub fn id(&self) -> i32 {
        let mut hasher = Sha256::new();
        hasher.input(self::channel_attributes_to_bytes);
        let result = hasher.result();

    }

    fn channel_attributes_to_bytes(&self)-> vec<u8>{
        //TODO finish method
    }
}