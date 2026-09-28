use bevy::pbr::{Material, MaterialPipeline, MaterialPipelineKey};
use bevy::prelude::*;
use bevy::render::mesh::MeshVertexBufferLayoutRef;
use bevy::render::render_resource::{
    AsBindGroup, PrimitiveTopology, RenderPipelineDescriptor, ShaderRef,
    SpecializedMeshPipelineError,
};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct GlobeMaterial {
    #[uniform(0)]
    pub base_color: LinearRgba,
    #[uniform(0)]
    pub atmosphere_color: LinearRgba,
}

impl Material for GlobeMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/globe_point.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/globe_point.wgsl".into()
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayoutRef,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.primitive.topology = PrimitiveTopology::PointList;
        descriptor.primitive.cull_mode = None;
        Ok(())
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct AstroDotMaterial {
    #[uniform(0)]
    pub color: LinearRgba,
    #[uniform(0)]
    pub size_px: f32,
}

impl Material for AstroDotMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/astro_dot.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/astro_dot.wgsl".into()
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayoutRef,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.primitive.topology = PrimitiveTopology::TriangleList;
        descriptor.primitive.cull_mode = None;
        Ok(())
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct OrbitMaterial {
    #[uniform(0)]
    pub color: LinearRgba,
}

impl Material for OrbitMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/orbit_curve.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/orbit_curve.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayoutRef,
        key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.primitive.topology = key.mesh_key.primitive_topology();
        descriptor.primitive.cull_mode = None;
        Ok(())
    }
}
