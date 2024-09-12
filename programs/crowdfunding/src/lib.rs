use anchor_lang::prelude::*;

declare_id!("9bgEVsGwtv9yKXHTyBwPVT4pkmbwukV8YHGH2rrJjX8G");

#[program]
pub mod crowdfunding {
    use anchor_lang::solana_program::entrypoint::ProgramResult;

    use super::*;

    //创建众筹账户指令
    pub fn create(ctx:Context<Create>,name:String,description:String) -> ProgramResult{
        let campaign = &mut ctx.accounts.campaign;
        campaign.name = name;
        campaign.description = description;
        campaign.amount_donated = 0;
        campaign.admin = *ctx.accounts.user.key;
        Ok(())
    }

    //创建取钱指令
    pub fn withdrew(ctx:Context<Withdraw>,amount:u64) -> ProgramResult{
        let campaign =&mut ctx.accounts.campaign;
        let user =&mut ctx.accounts.user;

        //判断一下campaign Account的管理员和传入的user是否是同一个
        if campaign.admin != *user.key{
            return Err(ProgramError::IncorrectProgramId);
        }

        //计算租金
        let rent_balance = Rent::get()?.minimum_balance(campaign.to_account_info().data_len());

        //判断一下campaign Account的是否有足够的余额取钱
        if **campaign.to_account_info().lamports.borrow() - rent_balance < amount {
            return Err(ProgramError::InsufficientFunds);
        }

        //将众筹账户的余额减少
        **campaign.to_account_info().try_borrow_mut_lamports()? -=amount;
        //将用户账户的余额增加
        **user.to_account_info().try_borrow_mut_lamports()? +=amount;

        Ok(())
    }

    //创建捐款指令
    pub fn donate(ctx:Context<Donate>,amount:u64) -> ProgramResult{
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.user.key(),//付款方
            &ctx.accounts.campaign.key(),//收款方
            amount //数量
        );

        //调用创建的指令
        let _ = anchor_lang::solana_program::program::invoke(&ix, &[
            ctx.accounts.user.to_account_info(),
            ctx.accounts.campaign.to_account_info(),
        ]);

        (&mut ctx.accounts.campaign).amount_donated += amount;

        Ok(())
    }

}

#[derive(Accounts)]
pub struct Create<'info>{
    #[account(init,payer=user,space=9000,seeds=[b"COMPAIGN_DEMO".as_ref(),user.key().as_ref()],bump)]
    pub campaign:Account<'info,Campaign>,
    #[account(mut)]
    pub user:Signer<'info>,
    pub system_program:Program<'info,System>
}

#[derive(Accounts)]
pub struct Withdraw<'info>{
    #[account(mut)]
    pub campaign:Account<'info,Campaign>,
    #[account(mut)]
    pub user:Signer<'info>,
}

#[derive(Accounts)]
pub struct Donate<'info>{
    #[account(mut)]
    pub campaign:Account<'info,Campaign>,
    #[account(mut)]
    pub user:Signer<'info>,
    pub system_program:Program<'info,System>
}

#[account]
pub struct Campaign{
    pub admin:Pubkey,
    pub name:String,
    pub description:String,
    pub amount_donated:u64
}