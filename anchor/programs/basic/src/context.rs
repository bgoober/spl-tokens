use anchor_lang::{prelude::*, solana_program::pubkey};
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{create_metadata_accounts_v3, mpl_token_metadata::types::DataV2, CreateMetadataAccountsV3, Metadata},
    token::{Mint, Token},
};

const ADMIN_PUBKEY: Pubkey = pubkey!("4QPAeQG6CTq2zMJAVCJnzY9hciQteaMkgBmcyGL7Vrwp");


#[derive(Accounts)]
pub struct Init<'info> {
    #[account(
        mut,
        address = ADMIN_PUBKEY
    )]
    pub admin: Signer<'info>,

    // The PDA is both the address of the mint account and the mint authority
    #[account(
        init,
        seeds = [b"reward"],
        bump,
        payer = admin,
        mint::decimals = 9,
        mint::authority = reward_token_mint,

    )]
    pub reward_token_mint: Account<'info, Mint>,

    ///CHECK: Using "address" constraint to validate metadata account address
    #[account(
        mut,
        address=find_metadata_account(&reward_token_mint.key()).0
    )]
    pub metadata_account: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> Init<'info> {
    pub fn init_token(&mut self, name: String, symbol: String, uri: String) -> Result<()> {
        // Create metadata account
        let cpi_context = CpiContext::new(
            self.token_metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                metadata: self.metadata_account.to_account_info(),
                mint: self.mint.to_account_info(),
                mint_authority: self.authority.to_account_info(),
                update_authority: self.authority.to_account_info(),
                payer: self.payer.to_account_info(),
                system_program: self.system_program.to_account_info(),
                rent: self.rent.to_account_info(),
            },
        );

        let data_v2 = DataV2 {
            name,
            symbol,
            uri,
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        };

        create_metadata_accounts_v3(cpi_context, data_v2, false, true, None)?;

        Ok(())
    }
}
