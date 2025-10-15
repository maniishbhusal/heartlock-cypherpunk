use anchor_lang::prelude::*;

declare_id!("8YYdRNeyP1EsMpiAFjTWzETyZ3Tutr4Taecq4htVbbfv");

#[program]
pub mod heartlock_cypherpunk {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
