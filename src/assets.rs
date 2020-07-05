use std::collections::HashMap;
use std::ops::Index;
use std::path::Path;

use ggez::GameResult;

pub fn load_assets<A>(
    path: &Path,
    extensions: &[&str],
    mut func: impl FnMut(&str) -> GameResult<A>,
) -> GameResult<Assets<A>> {
    let mut assets = HashMap::new();
    for p in glob::glob((path.display().to_string() + "/**/*").as_str()).unwrap() {
        let pu = p.unwrap();
        if extensions
            .iter()
            .all(|ext| !pu.to_str().unwrap().ends_with(*ext))
        {
            continue;
        }
        let name = pu
            .display()
            .to_string()
            .replace(path.display().to_string().as_str(), "")
            .replace("\\", "/");
        assets.insert(name.clone(), func(name.as_str())?);
    }

    println!("{:?}", assets.keys());

    Ok(Assets { assets })
}

pub struct Assets<A> {
    assets: HashMap<String, A>,
}

impl<A> Assets<A> {
    pub fn get(&self, key: &str) -> &A {
        &self.assets[key]
    }
}

impl<A> Index<&str> for Assets<A> {
    type Output = A;

    fn index(&self, index: &str) -> &Self::Output {
        &self.assets[index]
    }
}
