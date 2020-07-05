use std::collections::HashMap;
use std::ops::Index;
use std::path::{Path, PathBuf};

use ggez::GameResult;

pub struct Assets<A> {
    base_path: PathBuf,
    assets: HashMap<String, A>,
}

impl<A> Assets<A> {
    pub fn load(
        path: &Path,
        extensions: &[&str],
        mut func: impl FnMut(&str) -> GameResult<A>,
    ) -> GameResult<Assets<A>> {
        let mut assets = HashMap::new();

        for p in glob::glob(path.join("**/*").to_str().unwrap()).unwrap() {
            let pu = p.unwrap();
            if !extensions
                .iter()
                .any(|ext| pu.to_str().unwrap().ends_with(*ext))
            {
                continue;
            }
            let name = pu
                .to_str()
                .unwrap()
                .replace(path.to_str().unwrap(), "")
                .replace("\\", "/");
            assets.insert(name.clone(), func(name.as_str())?);
        }

        Ok(Assets {
            base_path: path.to_owned(),
            assets
        })
    }

    pub fn get(&self, key: &str) -> &A {
        &self.assets[key]
    }

    pub fn get_from_pattern<'a>(&'a self, pattern: &str) -> Vec<&'a A> {
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

impl<A> Index<&str> for Assets<A> {
    type Output = A;

    fn index(&self, index: &str) -> &Self::Output {
        &self.assets[index]
    }
}
