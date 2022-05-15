use anchor_lang::prelude::*;

declare_id!("8bvhVr9Zk8kbi6b4VU5PHGcVEri8KaXHKAAfZHgXgCVj");

#[program]
pub mod myepicproject {
  use super::*;
  pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> Result <()> {
    let base_account = &mut ctx.accounts.base_account;
    base_account.total_cert = 0;
    Ok(())
  }

  // The function now accepts a cert_obj param from the user. We also reference the user from the Context
  pub fn add_gif(ctx: Context<AddCert>, cert_obj: String) -> Result <()> {
    let base_account = &mut ctx.accounts.base_account;
    let user = &mut ctx.accounts.user;

	// Build the struct.
    let item = ItemStruct {
      cert_obj: cert_obj.to_string(),
      user_address: *user.to_account_info().key,
    };
		
	// Add it to the cert_list vector.
    base_account.cert_list.push(item);
    base_account.total_cert += 1;
    Ok(())
  }
}

#[derive(Accounts)]
pub struct StartStuffOff<'info> {
  #[account(init, payer = user, space = 9000000000)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
  pub system_program: Program <'info, System>,
}

// Add the signer who calls the AddCert method to the struct so that we can save it
#[derive(Accounts)]
pub struct AddCert<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
}

// Create a custom struct for us to work with.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub cert_obj: String,
    pub user_address: Pubkey,
}

#[account]
pub struct BaseAccount {
    pub total_cert: u64,
	// Attach a Vector of type ItemStruct to the account.
    pub cert_list: Vec<ItemStruct>,
}