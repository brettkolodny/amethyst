use std::{thread::sleep, time::Duration};

use amethyst::{
    assets::{
        Asset, AssetStorage, Format, Handle, Loader, ProcessingState, ProgressCounter, Source,
        TypeUuid,
    },
    error::{format_err, Error, ResultExt},
    prelude::*,
    utils::application_root_dir,
};
use amethyst_assets::{
    register_asset_type, register_importer, AssetProcessorSystem, DefaultLoader, LoaderBundle,
    RonFormat,
};
use amethyst_rendy::{types::DefaultBackend, RenderingBundle};
use log::info;
use ron::de::Deserializer;
use serde::{Deserialize, Serialize};

/// Custom asset representing an energy blast.
#[derive(Clone, Debug, Default, Deserialize, Serialize, TypeUuid)]
#[uuid = "a016abff-623d-48cf-a6e4-e76e069fe843"]
pub struct EnergyBlast {
    /// How much HP to subtract.
    pub hp_damage: u32,
    /// How much MP to subtract.
    pub mp_damage: u32,
}

impl Asset for EnergyBlast {
    type Data = Self;

    fn name() -> &'static str {
        "EnergyBlast"
    }
}

pub struct LoadingState {
    /// Handle to the energy blast.
    energy_blast_handle: Option<Handle<EnergyBlast>>,
}

/// Format for loading from `.mylang` files.
#[derive(Clone, Debug, Default, Deserialize, Serialize, TypeUuid)]
#[uuid = "1aacd480-2eb5-4e02-8ed4-daaf33245a45"]
pub struct MyLangFormat(RonFormat<EnergyBlast>);

impl Format<EnergyBlast> for MyLangFormat {
    fn name(&self) -> &'static str {
        "MyLangEnergyBlast"
    }

    fn import_simple(&self, bytes: Vec<u8>) -> Result<EnergyBlast, Error> {
        println!("Importing a mylang file to EnergyBlast");
        self.0.import_simple(bytes)
    }
}

register_asset_type!(EnergyBlast => EnergyBlast; AssetProcessorSystem<EnergyBlast>);

impl SimpleState for LoadingState {
    fn on_start(&mut self, data: StateData<'_, GameData>) {
        let loader = data.resources.get::<DefaultLoader>().unwrap();
        self.energy_blast_handle = Some(loader.load("energy_blast.mylang"));
    }

    fn update(&mut self, data: &mut StateData<'_, GameData>) -> SimpleTrans {
        let energy_blast_assets = data.resources.get::<AssetStorage<EnergyBlast>>().unwrap();
        if let Some(energy_blast) =
            energy_blast_assets.get(self.energy_blast_handle.as_ref().unwrap())
        {
            println!("Loaded energy blast: {:?}", energy_blast);
            Trans::Quit
        } else {
            Trans::None
        }
    }
}

fn main() -> amethyst::Result<()> {
    let config = amethyst::LoggerConfig {
        log_file: Some(std::path::PathBuf::from("asset_loading.log")),
        level_filter: amethyst::LogLevelFilter::Info,
        module_levels: vec![
            (
                "amethyst_assets".to_string(),
                amethyst::LogLevelFilter::Debug,
            ),
            (
                "atelier_daemon".to_string(),
                amethyst::LogLevelFilter::Debug,
            ),
            (
                "atelier_loader".to_string(),
                amethyst::LogLevelFilter::Trace,
            ),
        ],
        ..Default::default()
    };
    amethyst::start_logger(config);

    let app_root = application_root_dir()?;
    let assets_dir = app_root.join("examples/asset_custom/assets/");

    let mut builder = DispatcherBuilder::default();

    builder.add_bundle(LoaderBundle);
    builder.add_bundle(RenderingBundle::<DefaultBackend>::new());

    let game = Application::new(
        assets_dir,
        LoadingState {
            energy_blast_handle: None,
        },
        builder,
    )?;

    game.run();
    Ok(())
}
