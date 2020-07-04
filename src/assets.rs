use ggez::graphics::Image;
use ggez::{Context, GameResult};
use std::path::Path;

pub struct Assets {
    pub wall_corn_bot_lft: Image,
    pub wall_corn_bot_rgt: Image,
    pub wall_corn_top_lft: Image,
    pub wall_corn_top_rgt: Image,
    pub wall_crss_all: Image,
    pub wall_crss_hori_bot: Image,
    pub wall_crss_hori_top: Image,
    pub wall_crss_vert_lft: Image,
    pub wall_crss_vert_rgt: Image,
    pub wall_hori_lft: Image,
    pub wall_hori_mid: Image,
    pub wall_hori_rgt: Image,
    pub wall_vert_bot: Image,
    pub wall_vert_mid: Image,
    pub wall_vert_top: Image,
    pub grass: Image,
    pub idle_1: Image,
}

pub fn load_assets(ctx: &mut Context) -> GameResult<Assets> {
    Ok(Assets {
        wall_corn_bot_lft: Image::new(ctx, "/wall_corn_bot_lft.png")?,
        wall_corn_bot_rgt: Image::new(ctx, "/wall_corn_bot_rgt.png")?,
        wall_corn_top_lft: Image::new(ctx, "/wall_corn_top_lft.png")?,
        wall_corn_top_rgt: Image::new(ctx, "/wall_corn_top_rgt.png")?,
        wall_crss_all: Image::new(ctx, "/wall_crss_all.png")?,
        wall_crss_hori_bot: Image::new(ctx, "/wall_crss_hori_bot.png")?,
        wall_crss_hori_top: Image::new(ctx, "/wall_crss_hori_top.png")?,
        wall_crss_vert_lft: Image::new(ctx, "/wall_crss_vert_lft.png")?,
        wall_crss_vert_rgt: Image::new(ctx, "/wall_crss_vert_rgt.png")?,
        wall_hori_lft: Image::new(ctx, "/wall_hori_lft.png")?,
        wall_hori_mid: Image::new(ctx, "/wall_hori_mid.png")?,
        wall_hori_rgt: Image::new(ctx, "/wall_hori_rgt.png")?,
        wall_vert_bot: Image::new(ctx, "/wall_vert_bot.png")?,
        wall_vert_mid: Image::new(ctx, "/wall_vert_mid.png")?,
        wall_vert_top: Image::new(ctx, "/wall_vert_top.png")?,
        grass: Image::new(ctx, "/grass.png")?,
        idle_1: Image::new(ctx, "/idle_1.png")?,
    })
}
