use anchor_lang::prelude::*;
mod constants;
use constants::*;

declare_id!("CXF96zxKJLY8ZCmkr2YSTKs7ASkCDAeJffysMNogzfWU");

#[program]
pub mod anchor_student_intro_program {
    use super::*;

    pub fn add_student(ctx: Context<AddStudent>, name: String, introduction: String) -> Result<()> {
        msg!("Student Account Created");
        msg!("Name: {}", name);
        msg!("Introduction: {}", introduction);

        let student = &mut ctx.accounts.student;
        student.student_address = ctx.accounts.initializer.key();
        student.name = name;
        student.introduction = introduction;

        Ok(())
    }

    pub fn update_intro(
        ctx: Context<UpdateIntro>,
        _name: String,
        introduction: String,
    ) -> Result<()> {
        msg!("Updating Student Account");
        msg!("Introduction: {}", introduction);

        let student = &mut ctx.accounts.student;
        student.introduction = introduction;

        Ok(())
    }

    pub fn delete_student(_ctx: Context<DeleteStudent>, name: String) -> Result<()> {
        msg!("Student {} deleted", name);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(name: String, introduction: String)]
pub struct AddStudent<'info> {
    #[account(
            init,
            seeds = [name.as_bytes(), initializer.key().as_ref()],
            bump,
            payer = initializer,
            space = StudentAccountState::INIT_SPACE + name.len() + introduction.len(),
        )]
    pub student: Account<'info, StudentAccountState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(name: String, introduction: String)]
pub struct UpdateIntro<'info> {
    #[account(
            mut,
            seeds = [name.as_bytes(), initializer.key().as_ref()],
            bump,
            realloc = StudentAccountState::INIT_SPACE + name.len() + introduction.len(),
            realloc::payer = initializer,
            realloc::zero = true
        )]
    pub student: Account<'info, StudentAccountState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct DeleteStudent<'info> {
    #[account(
            mut,
            seeds = [name.as_bytes(), initializer.key().as_ref()],
            bump,
            close = initializer
        )]
    pub student: Account<'info, StudentAccountState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct StudentAccountState {
    pub student_address: Pubkey,
    pub name: String,
    pub introduction: String,
}

impl Space for StudentAccountState {
    const INIT_SPACE: usize =
        ANCHOR_DISCRIMINATOR + PUBKEY_SIZE + STRING_LENGTH_PREFIX + STRING_LENGTH_PREFIX;
}
