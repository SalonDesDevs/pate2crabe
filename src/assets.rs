use std::collections::HashMap;
use std::ops::Index;
use std::path::{Path, PathBuf};

use ggez::{Context, GameResult};
use ggez::graphics::Image;

pub struct Assets {
    base_path: PathBuf,
    assets: HashMap<String, Image>,
}

impl Assets {
    pub fn load(ctx: &mut Context, path: &Path) -> GameResult<Assets> {
        let mut assets = HashMap::new();

        for p in glob::glob(path.join("**/*.png").to_str().unwrap()).unwrap() {
            let image = p
                .unwrap()
                .display()
                .to_string()
                .replace(path.to_str().unwrap(), "")
                .replace("\\", "/");
            assets.insert(image.clone(), Image::new(ctx, image.as_str())?);
        }

        Ok(Assets {
            base_path: path.to_owned(),
            assets
        })
    }

    pub fn get(&self, key: &str) -> &Image {
        &self.assets[key]
    }

    pub fn get_from_pattern<'a>(&'a self, pattern: &str) -> Vec<&'a Image> {
        let mut matching_assets = Vec::new();
        for p in glob::glob(self.base_path.join(pattern).to_str().unwrap()).unwrap() {
            matching_assets.push(&self.assets[
                &p
                .unwrap()
                .to_str()
                .unwrap()
                .replace(self.base_path.to_str().unwrap(), "")
                .replace("\\", "/")
            ]);
        }
        matching_assets
    }
}

impl Index<&str> for Assets {
    type Output = Image;

    fn index(&self, index: &str) -> &Self::Output {
        &self.assets[index]
    }
}
