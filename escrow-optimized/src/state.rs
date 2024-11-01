/*

Escrow

1. Maker (32 bytes)
2. maker_ta_b (32 bytes)
3. mint_a (32 bytes)
4. mint_b (32 bytes) // for RPC lookup
5. amount_b (8 bytes)

*/

pub struct Escrow(*const u8);

impl Escrow {
    pub fn maker(&self) -> Pubkey {
        unsafe { *(self.0 as *const Pubkey); }
    }

    pub fn maker_ta_b(&self) -> Pubkey {
        unsafe { *(self.0.add(32) as *const Pubkey) }
    }
    

    pub fn mint_a(&self) -> Pubkey {
        unsafe { *(self.0.add(64) as *const Pubkey) }
    }

    pub fn mint_b(&self) -> Pubkey {
        unsafe { *(self.0.add(96) as *const Pubkey) }
    }

    pub fn amount_b(&self) -> Pubkey {
        unsafe { *(self.0.add(128) as *const Pubkey) }
    }
}