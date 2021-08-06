use bevy::{
    asset::{AssetLoader, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
};
pub use pyxel;
use pyxel::Pyxel;
use serde::Deserialize;

#[derive(Debug, Deserialize, TypeUuid)]
#[uuid = "9da4b419-2d5f-4def-a63b-bcc1f43254d4"]
pub struct PyxelFile {
    /// The file's name
    pub file_name: String,
    /// The path inside your assets folder
    pub path_inside_assets_folder: String,
    pub data: Pyxel,
}

#[derive(Default)]
pub struct PyxelFileLoader;
impl AssetLoader for PyxelFileLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::asset::BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let data = pyxel::load_from_memory(bytes)?;
            let custom_asset = PyxelFile {
                path_inside_assets_folder: load_context.path().to_str().unwrap().to_string(),
                file_name: load_context
                    .path()
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
                data,
            };

            load_context.set_default_asset(LoadedAsset::new(custom_asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["pyxel"]
    }
}

pub struct BevyPyxelPlugin;

impl Plugin for BevyPyxelPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<PyxelFile>()
            .init_asset_loader::<PyxelFileLoader>();
    }
}
