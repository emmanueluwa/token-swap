use anchor_lang::prelude::*;
//allows working on older and new token extension program
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface, TransferChecked, transfer_checked};

//helper function for transferring tokens
pub fn transfer_tokens<'info> (
    from: &InterfaceAccount<'info, 
    TokenAccount>, to: &InterfaceAccount<'info, 
    TokenAccount>, 
    amount: &u64,
    mint: &InterfaceAccount<'info, Mint>,
    authority: &Signer<'info>,
    token_program: &Interface<'info, TokenInterface>
) -> Result<()> {
    let transfer_accounts_options = TransferChecked {
        from: from.to_account_info(),
        mint: mint.to_account_info(),
        to: to.to_account_info(),
        authority: authority.to_account_info()
    };

    //creating cross program invocation context to interact with token program
    let cpi_context = CpiContext::new(
        token_program.to_account_info(), 
        transfer_accounts_options
    );

    transfer_checked(cpi_context, *amount, mint.decimals)
}