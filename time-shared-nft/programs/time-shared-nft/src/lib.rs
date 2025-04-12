use anchor_lang::prelude::*;

declare_id!("DE1wx1MeAQeZQCepnd2Ukd5o2S7v6JJaRfuX1G3EG1rs");

#[program]
pub mod time_shared_nft {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
