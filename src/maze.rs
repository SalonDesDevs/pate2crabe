pub enum Tile{
    Wall,
    Ground,
}

pub struct Maze{
    /// (width, height)
    dim: (usize, usize),
    tiles: Vec<Tile>
}