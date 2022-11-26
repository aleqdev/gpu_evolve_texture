use bevy_ecs::prelude::World;
use bevy_render::render_graph::{Node, NodeRunError, RenderGraphContext};
use bevy_render::render_resource::{CachedPipelineState, ComputePassDescriptor, PipelineCache};
use bevy_render::renderer::RenderContext;
use crate::{DisplayEvolveTexture, EvolveTextureBindGroup, EvolveTexturePipeline, EvolveTextureState, PrimaryEvolveTexture, SecondaryEvolveTexture};

#[derive(Default)]
pub struct EvolveTextureNode {
    pub state: EvolveTextureState
}

impl Node for EvolveTextureNode {
    fn update(&mut self, world: &mut World) {
        let pipeline = world.resource::<EvolveTexturePipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();

        match self.state {
            EvolveTextureState::Loading => {
                if let CachedPipelineState::Ok(_) =
                pipeline_cache.get_compute_pipeline_state(pipeline.init_pipeline)
                {
                    self.state = EvolveTextureState::Init;
                }
            }
            EvolveTextureState::Init => {
                if let CachedPipelineState::Ok(_) =
                pipeline_cache.get_compute_pipeline_state(pipeline.process_pipeline)
                {
                    self.state = EvolveTextureState::Process;
                }
            }
            EvolveTextureState::Process => {}
        }
    }

    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), NodeRunError> {
        if self.state == EvolveTextureState::Loading {
            return Ok(())
        }

        let bind_group = &world.resource::<EvolveTextureBindGroup>().0;

        if bind_group.is_none() {return Ok(())}
        let bind_group = bind_group.as_ref().unwrap();

        let pipeline_cache = world.resource::<PipelineCache>();
        let pipeline = world.resource::<EvolveTexturePipeline>();

        let mut pass = render_context
            .command_encoder
            .begin_compute_pass(&ComputePassDescriptor::default());

        pass.set_bind_group(0, bind_group, &[]);

        match self.state {
            EvolveTextureState::Loading => unreachable!(),
            EvolveTextureState::Init => {
                println!("init");
                let init_pipeline = pipeline_cache
                    .get_compute_pipeline(pipeline.init_pipeline)
                    .unwrap();
                pass.set_pipeline(init_pipeline);
                pass.dispatch_workgroups(320 / 8, 240 / 8, 1);
            }
            EvolveTextureState::Process => {
                println!("process");
                let update_pipeline = pipeline_cache
                    .get_compute_pipeline(pipeline.process_pipeline)
                    .unwrap();
                pass.set_pipeline(update_pipeline);
                pass.dispatch_workgroups(320 / 8, 240 / 8, 1);
            }
        }

        Ok(())
    }
}