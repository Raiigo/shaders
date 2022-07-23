use bevy::{
    ecs::system::{lifetimeless::SRes, SystemParamItem},
    math::Vec2,
    prelude::{
        default, shape, App, Assets, Color, Commands, Mesh, OrthographicCameraBundle, ResMut, Handle, Shader, AssetServer,
    },
    reflect::TypeUuid,
    render::{
        render_asset::{PrepareAssetError, RenderAsset},
        render_resource::{
            BindGroup, BindGroupDescriptor, BindGroupLayout, BindGroupLayoutDescriptor,
        },
        renderer::RenderDevice,
    },
    sprite::{
        ColorMaterial, Material2d, Material2dPipeline, Material2dPlugin, MaterialMesh2dBundle,
    },
    DefaultPlugins,
};

#[derive(TypeUuid, Clone)]
#[uuid = "cf76d061-581b-44c9-a95c-817ef5a8fb60"]
struct MyMaterial;

struct MyMaterialGPU {
    bind_group: BindGroup,
}

impl RenderAsset for MyMaterial {
    type ExtractedAsset = MyMaterial;

    type PreparedAsset = MyMaterialGPU;

    type Param = (SRes<RenderDevice>, SRes<Material2dPipeline<MyMaterial>>);

    fn extract_asset(&self) -> MyMaterial {
        return self.clone();
    }

    fn prepare_asset(
        extracted_asset: MyMaterial,
        (render_device, material_2d_pipeline): &mut SystemParamItem<Self::Param>,
    ) -> Result<MyMaterialGPU, PrepareAssetError<Self::ExtractedAsset>> {
        Ok(MyMaterialGPU {
            bind_group: render_device.create_bind_group(&BindGroupDescriptor {
                label: None,
                layout: &material_2d_pipeline.material2d_layout,
                entries: &[],
            }),
        })
    }
}

impl Material2d for MyMaterial {
    fn bind_group(material: &MyMaterialGPU) -> &BindGroup {
        &material.bind_group
    }

    fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
        render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: None,
            entries: &[],
        })
    }

    fn fragment_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        Some(asset_server.load("my_material.wgsl"))
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(Material2dPlugin::<MyMaterial>::default())
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<MyMaterial>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let mut quad_mesh = Mesh::from(shape::Quad {
        size: Vec2::new(100.0, 100.0),
        flip: false,
    });
    // quad_mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, vec![[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 0.0]]);

    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes
            .add(quad_mesh)
            .into(),
        material: materials.add(MyMaterial),
        ..default()
    });
}
