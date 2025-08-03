use std::{any::Any, path::Path, sync::Arc};

use crate::NeutonResult;

/// Trait for all assets in the engine.
/// This trait provides methods to access the asset's path and to downcast it to its concrete type.
pub trait Asset: Any + Send + Sync {
    fn as_any(&self) -> &dyn Any;
}

/// Trait for assets that can be loaded from a file.
/// This trait extends the `Asset` trait and provides a method to load the asset from a file path.
/// Implement this trait for any asset type that can be loaded from a file.
#[allow(dead_code)]
pub trait LoadableAsset: Asset + Sized {
    /// Load the asset from the specified path.
    fn load<P: AsRef<Path>>(path: P) -> NeutonResult<Self>;

    /// Returns the path of the asset.
    fn path(&self) -> &Path;
}

impl<T: Asset> Asset for Box<T> {
    fn as_any(&self) -> &dyn Any {
        self.as_ref().as_any()
    }
}

pub struct AssetManager {
    assets: Vec<Arc<dyn Asset>>,
}

#[allow(dead_code)]
impl AssetManager {
    pub fn new() -> Self {
        Self { assets: Vec::new() }
    }

    pub fn load_asset<T>(&mut self, path: impl AsRef<Path>) -> NeutonResult<Arc<T>>
    where
        T: LoadableAsset + 'static,
    {
        let asset = T::load(path)?;
        let arc = Arc::new(asset);
        self.assets.push(arc.clone() as Arc<dyn Asset>);
        Ok(arc)
    }

    pub fn get_asset<T: Asset + 'static>(&self, index: usize) -> Option<Arc<T>> {
        self.assets.get(index).and_then(|a| {
            let arc: Arc<dyn Asset> = Arc::clone(a);
            Arc::downcast::<T>(arc).ok()
        })
    }
}
