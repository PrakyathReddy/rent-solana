use anchor_lang::prelude::*;
use anchor_lang::system_program;

declare_id!("2cRCb4Bm3pe937zKDuNqX6dZJwuy1FtK3A4o2KTSmnPs");

#[program]
pub mod rent_solana {
    use super::*;

    pub fn create_system_account(
        ctx: Context<CreateSystemAccount>,
        address_data: AddressData,
    ) -> Result<()> {
        msg!("Program invoked, creating system account...");
        msg!(
            "New public key will be: {}",
            &ctx.accounts.new_account.key().to_string()
        );

        // Determine the minimum necessary rent by calculating the account size
        let account_span = (address_data.try_to_vec()?).len();
        let lamports_required = (Rent::get()?).minimum_balance(account_span);

        msg!("Account span is: {}", &account_span);
        msg!("Lamports required: {}", &lamports_required);

        system_program::create_account(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::CreateAccount {
                    from: ctx.accounts.payer.to_account_info(),
                    to: ctx.accounts.new_account.to_account_info(),
                },
            ),
            lamports_required,
            account_span as u64,
            &ctx.accounts.system_program.key(),
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateSystemAccount<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub new_account: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug)]
pub struct AddressData {
    name: String,
    address: String,
}
