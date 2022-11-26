use bevy::{prelude::*, utils::HashMap};
use gpu_evolve_texture::{DisplayEvolveTexture, PrimaryEvolveTexture, SecondaryEvolveTexture};
use std::io::{Read, Write};
use anyhow::{anyhow, Context};
use bevy_asset::LoadState;
use bevy_render::render_resource::TextureFormat;

fn construct_processor_shader(
    fmt_path: impl AsRef<std::path::Path>,
    fields_path: impl AsRef<std::path::Path>,
    processor_path: impl AsRef<std::path::Path>,
    fields_names_path: impl AsRef<std::path::Path>,
) -> anyhow::Result<()> {
    fn write_with_buffer(
        buffer: &mut String,
        path: impl AsRef<std::path::Path>,
        file: &mut std::fs::File
    ) -> anyhow::Result<()> {
        buffer.clear();
        let mut field_file = std::fs::File::open(path.as_ref()).with_context(|| {
            format!("Failed to open field file ({})", path.as_ref().display())
        })?;

        field_file.read_to_string(buffer).with_context(|| {
            format!("Failed to read field file ({})", path.as_ref().display())
        })?;

        file.write_all(buffer.as_bytes()).with_context(|| {
            format!("Failed to write to output shader file while reading field file ({})", path.as_ref().display())
        })?;

        Ok(())
    }

    let mut processor_file = std::fs::File::create(processor_path.as_ref()).with_context(|| {
        format!("Failed to create output shader file ({})", processor_path.as_ref().display())
    })?;
    let mut fields_names_file = std::fs::File::create(fields_names_path.as_ref()).with_context(|| {
        format!("Failed to create fields names file ({})", fields_names_path.as_ref().display())
    })?;

    let mut buffer = String::default();

    let mut fields_names = Vec::<String>::default();

    write_with_buffer(
        &mut buffer,
        fmt_path.as_ref().join("processor_shader_vars"),
        &mut processor_file,
    )?;

    for entry in std::fs::read_dir(fields_path.as_ref()).with_context(|| {
        format!("Failed to locate fields directory ({})", fields_path.as_ref().display())
    })? {
        let entry = entry.with_context(|| {
            "Failed to get entry while reading fields directory"
        })?;

        if entry.metadata().with_context(|| {
            "Failed to get entry metadata while reading fields directory"
        })?.is_file() {
            let name = entry.file_name().to_string_lossy().split('.').next().ok_or_else(|| {
                anyhow!("It appears that entry's file name is missing ({})", entry.path().display())
            })?.to_owned();

            let name_lower = name.to_lowercase();
            let binding = fields_names.len() + 1;

            processor_file.write_all(format!(
                "// ----------\n// {name_lower}\n// ----------\n"
            ).as_bytes()).with_context(|| {
                "Failed to write resource binding to output file while processing entry in fields directory"
            })?;

            processor_file.write_all(format!(
                "@group(0) @binding({binding}) var<storage, read_write> {name_lower} : array<f32>;\n"
            ).as_bytes()).with_context(|| {
                "Failed to write resource binding to output file while processing entry in fields directory"
            })?;

            write_with_buffer(
                &mut buffer,
                entry.path(),
                &mut processor_file
            )?;

            fields_names.push(name.to_lowercase());
        }
    }

    write_with_buffer(
        &mut buffer,
        fmt_path.as_ref().join("processor_shader_init_begin"),
        &mut processor_file,
    )?;

    write_with_buffer(
        &mut buffer,
        fmt_path.as_ref().join("processor_shader_init_end"),
        &mut processor_file,
    )?;

    write_with_buffer(
        &mut buffer,
        fmt_path.as_ref().join("processor_shader_process_begin"),
        &mut processor_file,
    )?;

    for field_name in fields_names.iter() {
        processor_file.write_all(
            format!("    let color = color + process_{field_name}(location, color);\n").as_bytes()
        )?;

        fields_names_file.write_all(field_name.as_bytes())?;
        fields_names_file.write_all(b"\n")?;
    }

    write_with_buffer(
        &mut buffer,
        fmt_path.as_ref().join("processor_shader_process_end"),
        &mut processor_file,
    )?;

    Ok(())
}

struct FieldMap {
    pub map: HashMap<String, usize>,

}

fn main() -> anyhow::Result<()> {
    let cwd = std::env::current_dir()?;

    construct_processor_shader(
        cwd.join("assets/fmt"),
        cwd.join("assets/shaders/fields"),
        cwd.join("assets/shaders/processor.wgsl"),
        cwd.join("assets/shaders/fields_names.fnames"),
    )?;

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(gpu_evolve_texture::EvolveTexturePlugin)
        .add_startup_system(setup)
        .add_system(make_evolve_texture)
        .run();

    Ok(())
}

struct ImgHandle(Handle<Image>);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(Camera2dBundle::default());

    let img = asset_server.load("img.png");

    commands.insert_resource(ImgHandle(img));
}

fn make_evolve_texture(
    asset_server: Res<AssetServer>,
    handle: Res<ImgHandle>,
    mut images: ResMut<Assets<Image>>,
    mut once: Local<bool>,
    mut commands: Commands
) {
    if *once {
        return
    }

    if asset_server.get_load_state(handle.0.clone()) == LoadState::Loaded {
        *once = true;
        let img = images.get(&handle.0).unwrap().convert(TextureFormat::Rgba8Unorm).expect("Failed");
        let handle = images.add(img);

        commands.insert_resource(PrimaryEvolveTexture(Some(handle.clone())));
        commands.insert_resource(SecondaryEvolveTexture(Some(handle.clone())));
        commands.insert_resource(DisplayEvolveTexture(Some(handle.clone())));

        commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(320., 240.)),
                ..default()
            },
            texture: handle.clone(),
            ..default()
        });
    }
}