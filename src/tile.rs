use ggez::graphics::Image;

#[derive(Debug, Clone)]
pub enum Tile {
    Wall(Option<Image>),
    Ground,
}

impl Tile {
    pub fn is_wall(&self) -> bool {
        matches!(self, Tile::Wall(_))
    }
}
