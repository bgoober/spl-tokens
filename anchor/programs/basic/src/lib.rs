use anchor_lang::prelude::*;

mod context;

use context::*;

mod state;


declare_id!("FQNLwZFnZEC6F6DLQ7cJX8EeK5D5SjDDgNRdAJh27fU5");

#[program]
pub mod basic {
    use super::*;

    pub fn init_token(ctx: Context<Init>) -> Result<()> {
        let name = "MyToken";
        let symbol = "MTK";
        let uri = "https://google.com";
        ctx.accounts.init_token(name.to_string(), symbol.to_string(), uri.to_string())?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
