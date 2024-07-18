use anchor_lang::prelude::*;

declare_id!("FQNLwZFnZEC6F6DLQ7cJX8EeK5D5SjDDgNRdAJh27fU5");

#[program]
pub mod basic {
    use super::*;

    pub fn greet(_ctx: Context<Initialize>) -> Result<()> {
        msg!("GM!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
