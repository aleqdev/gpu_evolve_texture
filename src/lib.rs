pub mod bind_group;
pub mod pipeline;
pub mod state;
pub mod node;
pub mod descriptor;

pub use bind_group::*;
pub use pipeline::*;
pub use state::*;
pub use node::*;
pub use descriptor::*;

use bevy_app::{App, Plugin};
use bevy_asset::Handle;
use bevy_ecs::prelude::{Commands, Res};
use bevy_ecs::system::ResMut;
use bevy_render::extract_resource::{ExtractResource, ExtractResourcePlugin};
use bevy_render::render_asset::RenderAssets;
use bevy_render::render_resource::{
    BindGroupDescriptor, BindGroupEntry, BindingResource
};
use bevy_render::renderer::RenderDevice;
use bevy_render::texture::Image;
use bevy_render::{RenderApp, RenderStage};
use bevy_render::render_graph::RenderGraph;

// VOID (0., 0., 0., 255.)
// WOOD (150., 120., 90., 255.)
// FIRE (255., 120., 30., 255.)

#[derive(Default, Clone, ExtractResource)]
pub struct PrimaryEvolveTexture(pub Option<Handle<Image>>);

#[derive(Default, Clone, ExtractResource)]
pub struct SecondaryEvolveTexture(pub Option<Handle<Image>>);

#[derive(Default, Clone, ExtractResource)]
pub struct DisplayEvolveTexture(pub Option<Handle<Image>>);

pub struct EvolveTexturePlugin;

impl Plugin for EvolveTexturePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PrimaryEvolveTexture>();
        app.init_resource::<SecondaryEvolveTexture>();
        app.init_resource::<DisplayEvolveTexture>();

        app.add_plugin(ExtractResourcePlugin::<PrimaryEvolveTexture>::default());
        app.add_plugin(ExtractResourcePlugin::<SecondaryEvolveTexture>::default());
        app.add_plugin(ExtractResourcePlugin::<DisplayEvolveTexture>::default());

        let render_app = app.sub_app_mut(RenderApp)
            .init_resource::<EvolveTexturePipeline>()
            .init_resource::<EvolveTextureBindGroup>()
            .add_system_to_stage(RenderStage::Queue, queue_bind_group);

        let mut render_graph = render_app.world.resource_mut::<RenderGraph>();
        render_graph.add_node("evolve_texture", EvolveTextureNode::default());
    }
}

fn queue_bind_group(
    mut commands: Commands,
    pipeline: Res<EvolveTexturePipeline>,
    gpu_images: Res<RenderAssets<Image>>,
    primary_image: Res<PrimaryEvolveTexture>,
    secondary_image: Res<SecondaryEvolveTexture>,
    display_image: Res<DisplayEvolveTexture>,
    render_device: Res<RenderDevice>,
) {
    if primary_image.0.is_none()
        || secondary_image.0.is_none()
        || display_image.0.is_none()
    {
        return;
    }

    let primary_view = &gpu_images[primary_image.0.as_ref().unwrap()];

    let secondary_view = &gpu_images[secondary_image.0.as_ref().unwrap()];

    let display_view = &gpu_images[display_image.0.as_ref().unwrap()];

    let bind = render_device.create_bind_group(&BindGroupDescriptor {
        label: None,
        layout: &pipeline.bind_group_layout,
        entries: &[
            BindGroupEntry {
                binding: 0,
                resource: BindingResource::TextureView(&primary_view.texture_view),
            },
            BindGroupEntry {
                binding: 1,
                resource: BindingResource::TextureView(&secondary_view.texture_view),
            },
            BindGroupEntry {
                binding: 2,
                resource: BindingResource::TextureView(&display_view.texture_view),
            },
        ],
    });

    commands.insert_resource(EvolveTextureBindGroup(Some(bind)));
}
