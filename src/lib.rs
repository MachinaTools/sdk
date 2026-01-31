use anchor_lang::prelude::*;

declare_id!("Mach1na111111111111111111111111111111111");

#[program]
pub mod machina {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.authority = ctx.accounts.authority.key();
        state.bump = ctx.bumps.state;
        Ok(())
    }

    pub fn update(ctx: Context<Update>, value: u64) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.value = value;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 8 + 1,
        seeds = [b"state"],
        bump
    )]
    pub state: Account<'info, State>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(
        mut,
        seeds = [b"state"],
        bump = state.bump,
        has_one = authority
    )]
    pub state: Account<'info, State>,
    pub authority: Signer<'info>,
}

#[account]
pub struct State {
    pub authority: Pubkey,
    pub value: u64,
    pub bump: u8,
}
