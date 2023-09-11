use anchor_lang::prelude::*;

declare_id!("2cRCb4Bm3pe937zKDuNqX6dZJwuy1FtK3A4o2KTSmnPs");

#[program]
pub mod rent_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
