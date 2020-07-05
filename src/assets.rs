use std::collections::HashMap;
use std::ops::Index;
use std::path::Path;

use ggez::{Context, GameResult};
use ggez::graphics::Image;

pub fn load_assets(ctx: &mut Context, path: &Path) -> GameResult<Assets> {
    let mut assets = HashMap::new();
    for p in glob::glob((path.display().to_string() + "/**/*.png").as_str()).unwrap() {
        let image = p
            .unwrap()
            .display()
            .to_string()
            .replace(path.display().to_string().as_str(), "")
            .replace("\\", "/");
        assets.insert(image.clone(), Image::new(ctx, image.as_str())?);
    }

    Ok(Assets {
        assets
    })
}

pub struct Assets {
    assets: HashMap<String, Image>,
}

impl Assets {
    pub fn get(&self, key: &str) -> &Image {
        &self.assets[key]
    }
}

impl Index<&str> for Assets {
    type Output = Image;

    fn index(&self, index: &str) -> &Self::Output {
        &self.assets[index]
    }
}
