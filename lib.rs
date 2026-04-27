use anchor_lang::prelude::*;

declare_id!("");

#[program]
pub mod libros {
    use super::*;

    pub fn crear_usuario(ctx: Context<CrearUsuario>, nombre: String) -> Result<()> {
        let usuario = &mut ctx.accounts.usuario;
        usuario.owner = ctx.accounts.owner.key();
        usuario.nombre = nombre;
        usuario.libros = Vec::new();
        Ok(())
    }

    pub fn agregar_libro(
        ctx: Context<ModificarUsuario>,
        titulo: String,
        autor: String
    ) -> Result<()> {

        let usuario = &mut ctx.accounts.usuario;

        require!(
            usuario.owner == ctx.accounts.owner.key(),
            ErrorLibro::NoOwner
        );

        usuario.libros.push(Libro {
            titulo,
            autor,
            leido: false,
        });

        Ok(())
    }

    pub fn marcar_leido(ctx: Context<ModificarUsuario>, index: u8) -> Result<()> {
        let usuario = &mut ctx.accounts.usuario;

        require!(
            usuario.owner == ctx.accounts.owner.key(),
            ErrorLibro::NoOwner
        );

        require!(
            (index as usize) < usuario.libros.len(),
            ErrorLibro::NoExiste
        );

        usuario.libros[index as usize].leido = true;

        Ok(())
    }

    pub fn eliminar_libro(ctx: Context<ModificarUsuario>, index: u8) -> Result<()> {
        let usuario = &mut ctx.accounts.usuario;

        require!(
            usuario.owner == ctx.accounts.owner.key(),
            ErrorLibro::NoOwner
        );

        require!(
            (index as usize) < usuario.libros.len(),
            ErrorLibro::NoExiste
        );

        usuario.libros.remove(index as usize);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CrearUsuario<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = 600,
        seeds = [b"usuario", owner.key().as_ref()],
        bump
    )]
    pub usuario: Account<'info, Usuario>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModificarUsuario<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mut)]
    pub usuario: Account<'info, Usuario>,
}

#[account]
pub struct Usuario {
    pub owner: Pubkey,
    pub nombre: String,
    pub libros: Vec<Libro>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Libro {
    pub titulo: String,
    pub autor: String,
    pub leido: bool,
}

#[error_code]
pub enum ErrorLibro {
    #[msg("No eres propietario")]
    NoOwner,

    #[msg("El libro no existe")]
    NoExiste,
}
