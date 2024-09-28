use anchor_lang::prelude::*;

declare_id!("8ckgTy9QCDvrirvAxCvBD9ZRMnuBdyyuuCtaPQAXNBWS");

#[program]
pub mod kamala_horse {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
