use crate::input::Input;
use crate::Isometry3;
use crate::animation;
use crate::assets::{self, Assets};
use crate::renderer;
use crate::vulkan::Vulkan;
use winit::event::MouseButton;
use color_eyre::eyre::Result;
use std::rc::Rc;
use winit::event::{Event, WindowEvent, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use crate::input;
use crate::camera::Camera;


pub trait GameThing:'static {}

pub trait World {
    fn update(&mut self, inp: &input::Input, assets: &mut assets::Assets);
    fn render(&mut self, assets: &mut assets::Assets, render_state: &mut renderer::RenderState);
    fn paused(&self) -> bool;
    fn pause(&mut self);
    fn unpause(&mut self);
}

pub struct WindowSettings {
    pub w: usize,
    pub h: usize,
    pub title: String,
}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            w: 1024,
            h: 768,
            title: "Engine Window".to_string(),
        }
    }
}
use thunderdome::{Arena,Index};
pub struct GameObjectRef(Index);
pub struct TransformParent {
    parent:GameObjectRef,
    local_to_global:Isometry3,
    global_to_local:Isometry3
}
/*
pub struct Engine {
    textures:HashMap<TextureRef,Texture>,
    next_texture:usize,
    meshes:HashMap<MeshRef,Mesh>,
    next_mesh:usize,
    event_loop:Option<EventLoop<()>>,
    pub camera:Camera,
    objects:Arena<GameObject>,
    // just an example: we'll store parent links out of band to illustrate,
    // and make the indices line up
    parents:Arena<TransformParent>,
    vulkan:Vulkan,
    input:Input,
    tex_mesh_renderer:TexturedMeshRenderer
}*/

pub struct Engine {
    assets: Assets,
    event_loop: Option<EventLoop<()>>,
    vulkan: Vulkan,
    input: input::Input,
    // 1 is new, 0 is old
    render_states: [crate::renderer::RenderState; 2],
    interpolated_state: crate::renderer::RenderState,
    skinned_renderer: crate::renderer::skinned::Renderer,
    sprites_renderer: crate::renderer::sprites::Renderer,
    textured_renderer: crate::renderer::textured::Renderer,
    flat_renderer: crate::renderer::flat::Renderer,
    dt: f64,
    acc: f64,
    last_frame: std::time::Instant,
}

impl Engine {
    pub fn new(ws: WindowSettings, dt: f64) -> Self {
        use crate::types::Vec3;
        let event_loop = EventLoop::new();
        let wb = WindowBuilder::new()
            .with_inner_size(winit::dpi::LogicalSize::new(ws.w as f32, ws.h as f32))
            .with_title(ws.title);
        let input = input::Input::new();
        let default_cam =
            Camera::look_at(Vec3::new(0., 0., 0.), Vec3::new(0., 0., 1.), Vec3::unit_y());
        let mut vulkan = Vulkan::new(wb, &event_loop);
        Self {
            assets: Assets::new(),
            skinned_renderer: crate::renderer::skinned::Renderer::new(&mut vulkan),
            sprites_renderer: crate::renderer::sprites::Renderer::new(&mut vulkan),
            textured_renderer: crate::renderer::textured::Renderer::new(&mut vulkan),
            flat_renderer: crate::renderer::flat::Renderer::new(&mut vulkan),
            vulkan,
            render_states: [
                crate::renderer::RenderState::new(default_cam),
                crate::renderer::RenderState::new(default_cam),
            ],
            interpolated_state: crate::renderer::RenderState::new(default_cam),
            dt,
            event_loop: Some(event_loop),
            input,
            acc: 0.0,
            last_frame: std::time::Instant::now(),
        }
    }
    pub fn set_camera(&mut self, cam:Camera) {
        self.render_states = [
            crate::renderer::RenderState::new(cam),
            crate::renderer::RenderState::new(cam),
        ]
    }
    pub fn play(mut self, f:impl Fn(&mut Self) + 'static) -> Result<()> {
        let ev = self.event_loop.take().unwrap();
        ev.run(move |event, _, control_flow| {
            match event {
                // Nested match patterns are pretty useful---see if you can figure out what's going on in this match.
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    *control_flow = ControlFlow::Exit;
                }
                Event::WindowEvent {
                    event: WindowEvent::Resized(_),
                    ..
                } => {
                    self.vulkan.recreate_swapchain = true;
                }
                // NewEvents: Let's start processing events.
                Event::NewEvents(_) => {}
                // WindowEvent->KeyboardInput: Keyboard input!
                Event::WindowEvent {
                    event:
                        WindowEvent::KeyboardInput {
                            input: in_event, ..
                        },
                    ..
                } => {
                    self.input.handle_key_event(in_event);
                }
                Event::WindowEvent {
                    event: WindowEvent::MouseInput {
                        state: button_state,
                        button: MouseButton::Left,
                        ..
                    }, 
                    ..
                } => {
                    self.input.handle_left_mouse_event(button_state)
                }
                Event::DeviceEvent {
                    event: winit::event::DeviceEvent::MouseMotion { delta }
                    , ..
                } => {
                    self.input.handle_cursor_motion(delta)
                }
                Event::WindowEvent {
                    event: WindowEvent::CursorMoved {
                        position,
                        ..
                    }, 
                    ..
                } => {
                    self.input.handle_cursor_moved_event(position)
                }
                Event::MainEventsCleared => {
                    // track DT, accumulator, ...
                    {
                        f(&mut self);
                        self.input.next_frame();
                    }
                    self.render3d();
                }
                _ => (),
            }
        });
    }
    pub fn play_world(mut self, mut w: impl World + 'static) -> Result<()> {
        let ev = self.event_loop.take().unwrap();
        self.last_frame = std::time::Instant::now();
        ev.run(move |event, _, control_flow| {
            match event {
                // Nested match patterns are pretty useful---see if you can figure out what's going on in this match.
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    *control_flow = ControlFlow::Exit;
                }
                Event::WindowEvent {
                    event: WindowEvent::Resized(_),
                    ..
                } => {
                    self.vulkan.recreate_swapchain = true;
                }
                // NewEvents: Let's start processing events.
                Event::NewEvents(_) => {}
                // WindowEvent->KeyboardInput: Keyboard input!
                Event::WindowEvent {
                    event:
                        WindowEvent::KeyboardInput {
                            input: in_event, ..
                        },
                    ..
                } => {
                    self.input.handle_key_event(in_event);
                    if self.input.is_key_pressed(VirtualKeyCode::Key0) {
                        w.pause();
                    }
                    else if self.input.is_key_pressed(VirtualKeyCode::Key1) {
                        w.unpause();
                    }
                }
                Event::WindowEvent {
                    event: WindowEvent::MouseInput {
                        state: button_state,
                        button: MouseButton::Left,
                        ..
                    }, 
                    ..
                } => {
                    self.input.handle_left_mouse_event(button_state)
                }
                Event::DeviceEvent {
                    event: winit::event::DeviceEvent::MouseMotion { delta }
                    , ..
                } => {
                    if !w.paused(){
                        self.input.handle_cursor_motion(delta)
                    }
                    
                }
                Event::WindowEvent {
                    event: WindowEvent::CursorMoved {
                        position,
                        ..
                    }, 
                    ..
                } => {
                    if !w.paused(){
                        self.input.handle_cursor_moved_event(position)
                    }
                }
                Event::MainEventsCleared => {
                    // track DT, accumulator, ...
                    {
                        self.acc += self.last_frame.elapsed().as_secs_f64();
                        self.last_frame = std::time::Instant::now();
                        while self.acc >= self.dt {
                            w.update(&self.input, &mut self.assets);
                            self.input.next_frame();
                            if self.acc <= self.dt * 2.0 {
                                self.render_states[0].clear();
                                w.render(&mut self.assets, &mut self.render_states[0]);
                                self.render_states.swap(0, 1);
                            }
                            self.acc -= self.dt;
                        }
                    }
                    self.render3d();
                }
                _ => (),
            }
        });
    }
    fn render3d(&mut self) {
        use vulkano::command_buffer::{
            AutoCommandBufferBuilder, CommandBufferUsage, SubpassContents,
        };

        let vulkan = &mut self.vulkan;
        vulkan.recreate_swapchain_if_necessary();
        let image_num = vulkan.get_next_image();
        if image_num.is_none() {
            return;
        }
        let image_num = image_num.unwrap();
        let mut builder = AutoCommandBufferBuilder::primary(
            vulkan.device.clone(),
            vulkan.queue.family(),
            CommandBufferUsage::OneTimeSubmit,
        )
        .unwrap();
        let r = (self.acc / self.dt) as f32;
        // let r = 1.0;
        let ar = vulkan.viewport.dimensions[0] / vulkan.viewport.dimensions[1];
        self.interpolated_state.camera_mut().set_ratio(ar);
        for rs in self.render_states.iter_mut() {
            rs.camera_mut().set_ratio(ar);
        }
        self.interpolated_state
            .interpolate_from(&self.render_states[0], &self.render_states[1], r);

        self.skinned_renderer.prepare(
            &self.interpolated_state,
            &self.assets,
            &self.interpolated_state.camera,
        );
        self.sprites_renderer.prepare(
            &self.interpolated_state,
            &self.assets,
            &self.interpolated_state.camera,
        );
        self.flat_renderer.prepare(
            &self.interpolated_state,
            &self.assets,
            &self.interpolated_state.camera,
        );
        self.textured_renderer.prepare(
            &self.interpolated_state,
            &self.assets,
            &self.interpolated_state.camera,
        );

        builder
            .begin_render_pass(
                vulkan.framebuffers[image_num].clone(),
                SubpassContents::Inline,
                vec![[0.0, 0.0, 0.0, 0.0].into(), (0.0).into()],
            )
            .unwrap()
            .set_viewport(0, [vulkan.viewport.clone()]);

        self.skinned_renderer.draw(&mut builder);
        self.sprites_renderer.draw(&mut builder);
        self.flat_renderer.draw(&mut builder);
        self.textured_renderer.draw(&mut builder);

        builder.end_render_pass().unwrap();

        let command_buffer = builder.build().unwrap();
        vulkan.execute_commands(command_buffer, image_num);
    }
    pub fn load_texture(&mut self, path: &std::path::Path) -> Result<assets::TextureRef> {
        self.assets.load_texture(path, &mut self.vulkan)
    }
    pub fn load_skinned(
        &mut self,
        path: &std::path::Path,
        node_root: &[&str],
    ) -> Result<Vec<assets::MeshRef<renderer::skinned::Mesh>>> {
        self.assets.load_skinned(path, node_root, &mut self.vulkan)
    }
    pub fn load_textured(
        &mut self,
        path: &std::path::Path,
    ) -> Result<Vec<assets::MeshRef<renderer::textured::Mesh>>> {
        self.assets.load_textured(path, &mut self.vulkan)
    }
    pub fn load_anim(
        &mut self,
        path: &std::path::Path,
        mesh: assets::MeshRef<renderer::skinned::Mesh>,
        settings: animation::AnimationSettings,
        which: &str,
    ) -> Result<assets::AnimRef> {
        self.assets.load_anim(path, mesh, settings, which)
    }
    pub fn create_skinned_model(
        &self,
        meshes: Vec<assets::MeshRef<renderer::skinned::Mesh>>,
        textures: Vec<assets::TextureRef>,
    ) -> Rc<renderer::skinned::Model> {
        assert_eq!(meshes.len(), textures.len());
        Rc::new(renderer::skinned::Model::new(meshes, textures))
    }
    pub fn create_textured_model(
        &self,
        meshes: Vec<assets::MeshRef<renderer::textured::Mesh>>,
        textures: Vec<assets::TextureRef>,
    ) -> Rc<renderer::textured::Model> {
        assert_eq!(meshes.len(), textures.len());
        Rc::new(renderer::textured::Model::new(meshes, textures))
    }
    pub fn load_flat(&mut self, path: &std::path::Path) -> Result<Rc<renderer::flat::Model>> {
        self.assets.load_flat(path, &mut self.vulkan)
    }
    pub fn get_inputs(&self) -> Input {
        self.input.clone()
    }
}

/*
impl Engine {
    pub fn new(ws:WindowSettings) -> Self {
        let event_loop = EventLoop::new();
        let wb = WindowBuilder::new()
            .with_inner_size(winit::dpi::LogicalSize::new(ws.w as f32, ws.h as f32))
            .with_title(ws.title);
        let input = Input::new();
        let mut vulkan = Vulkan::new(wb, &event_loop);
        Self {
            tex_mesh_renderer:TexturedMeshRenderer::new(&mut vulkan),
            vulkan,
            event_loop:Some(event_loop),
            camera:Camera::look_at(Vec3::new(0.,0.,0.), Vec3::new(0.,0.,1.), Vec3::unit_y()),
            objects:Arena::new(),
            parents:Arena::new(),
            input,
            next_texture:0,
            next_mesh:0,
            textures:HashMap::new(),
            meshes:HashMap::new()
        }
    }
    pub fn set_camera(&mut self, cam:Camera) {
        self.camera = cam;
    }
    pub fn create_game_object(&mut self, model:Option<&Model>, trf:Isometry3, data:Box<dyn GameThing>, parent:Option<GameObjectRef>) -> GameObjectRef {
        let obj=self.objects.insert(GameObject{transform:trf,model:model.cloned(),data});
        if let Some(parent) = parent {
            let parent_go = &self.objects[parent.0];
            let parent_trfs = self.parents.get(parent.0);
            let local_to_global = parent_trfs
                .map(|p| p.local_to_global * parent_go.transform)
                .unwrap_or(parent_go.transform);
            let global_to_local = parent_trfs
                .map(|p| p.global_to_local * parent_go.transform.inversed())
                .unwrap_or_else(|| parent_go.transform.inversed());
            self.parents.insert_at(
                obj,
                TransformParent{
                    parent,
                    local_to_global,
                    global_to_local
                }
            );
        }
        GameObjectRef(obj)
    }
    pub fn remove_object(&mut self, go:GameObjectRef) {
        self.objects.remove(go.0);
        self.parents.remove(go.0);
    }
    pub fn objects_mut(&mut self) -> impl Iterator<Item=(GameObjectRef, &mut GameObject)> {
        self.objects.iter_mut().map(|(idx,go)| (GameObjectRef(idx),go))
    }
    pub fn load_texture(&mut self, path: &std::path::Path) -> Result<TextureRef> {
        let img = Image::from_file(path)?;
        let tid = self.next_texture;
        self.next_texture+=1;
        let (vulk_img, fut) = ImmutableImage::from_iter(
            img.as_slice().iter().copied(),
            vulkano::image::ImageDimensions::Dim2d {
                width: img.sz.x,
                height: img.sz.y,
                array_layers: 1,
            },
            vulkano::image::MipmapsCount::One,
            vulkano::format::Format::R8G8B8A8_SRGB,
            self.vulkan.queue.clone(),
        )?;
        // fancy!
        let old_fut = self.vulkan.previous_frame_end.take();
        self.vulkan.previous_frame_end = match old_fut {
            None => Some(Box::new(fut)),
            Some(old_fut) => Some(Box::new(old_fut.join(fut))),
        };
        self.textures.insert(TextureRef(tid), Texture{image:img, texture:vulk_img});
        Ok(TextureRef(tid))
    }
    pub fn load_mesh(&mut self, path: &std::path::Path, scale:f32) -> Result<MeshRef> {
        let mid = self.next_mesh;
        self.next_mesh+=1;

        use russimp::scene::{PostProcess,Scene};
        let mut scene = Scene::from_file(
            path.to_str().ok_or_else(|| eyre!("Mesh path can't be converted to string: {:?}",path))?,
            vec![PostProcess::Triangulate, PostProcess::JoinIdenticalVertices, PostProcess::FlipUVs])?;
        let mesh = scene.meshes.swap_remove(0);
        let verts = &mesh.vertices;
        let uvs = mesh.texture_coords.first().ok_or_else(|| eyre!("Mesh fbx has no texture coords: {:?}",path))?.as_ref();
        let uvs = uvs.ok_or_else(|| eyre!("Mesh fbx doesn't specify texture coords: {:?}",path))?;
        ensure!(mesh.faces[0].0.len()==3,"Mesh face has too many indices: {:?}",mesh.faces[0]);
        // This is safe to allow because we need an ExactSizeIterator of faces
        #[allow(clippy::needless_collect)]
        let faces:Vec<u32> = mesh.faces.iter().flat_map(|v| { v.0.iter().copied()}).collect();
        let (vb,vb_fut) = vulkano::buffer::ImmutableBuffer::from_iter(
            verts.iter().zip(uvs.iter()).map(|(pos,uv)| VertexUV{position:[pos.x*scale,pos.y*scale,pos.z*scale], uv:[uv.x,uv.y]}),
            vulkano::buffer::BufferUsage::vertex_buffer(),
            self.vulkan.queue.clone()
        )?;
        let (ib,ib_fut) = vulkano::buffer::ImmutableBuffer::from_iter(
            faces.into_iter(),
            vulkano::buffer::BufferUsage::index_buffer(),
            self.vulkan.queue.clone()
        )?;
        let load_fut = vb_fut.join(ib_fut);
        let old_fut = self.vulkan.previous_frame_end.take();
        self.vulkan.previous_frame_end = match old_fut {
            None => Some(Box::new(load_fut)),
            Some(old_fut) => Some(Box::new(old_fut.join(load_fut))),
        };
        self.meshes.insert(MeshRef(mid), Mesh{mesh,verts:vb,idx:ib});
        Ok(MeshRef(mid))
    }
    pub fn create_model(&self, mesh:&MeshRef, texture:&TextureRef) -> Model {
        Model{mesh:*mesh,texture:*texture}
    }
    pub fn play(mut self, f:impl Fn(&mut Self) + 'static) -> Result<()> {
        let ev = self.event_loop.take().unwrap();
        ev.run(move |event, _, control_flow| {
            match event {
                // Nested match patterns are pretty useful---see if you can figure out what's going on in this match.
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    *control_flow = ControlFlow::Exit;
                }
                Event::WindowEvent {
                    event: WindowEvent::Resized(_),
                    ..
                } => {
                    self.vulkan.recreate_swapchain = true;
                }
                // NewEvents: Let's start processing events.
                Event::NewEvents(_) => {}
                // WindowEvent->KeyboardInput: Keyboard input!
                Event::WindowEvent {
                    event:
                        WindowEvent::KeyboardInput {
                            input: in_event, ..
                        },
                    ..
                } => {
                    self.input.handle_key_event(in_event);
                }
                Event::WindowEvent {
                    event: WindowEvent::MouseInput {
                        state: button_state,
                        button: MouseButton::Left,
                        ..
                    }, 
                    ..
                } => {
                    self.input.handle_left_mouse_event(button_state)
                }
                Event::DeviceEvent {
                    event: winit::event::DeviceEvent::MouseMotion { delta }
                    , ..
                } => {
                    self.input.handle_cursor_motion(delta)
                }
                Event::WindowEvent {
                    event: WindowEvent::CursorMoved {
                        position,
                        ..
                    }, 
                    ..
                } => {
                    self.input.handle_cursor_moved_event(position)
                }
                Event::MainEventsCleared => {
                    // track DT, accumulator, ...
                    {
                        f(&mut self);
                        self.input.next_frame();
                    }
                    self.render3d();
                }
                _ => (),
            }
        });
    }
    fn render3d(&mut self) {
        use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, SubpassContents};

        let vulkan = &mut self.vulkan;
        vulkan.recreate_swapchain_if_necessary();
        let image_num = vulkan.get_next_image();
        if image_num.is_none() { return; }
        let image_num = image_num.unwrap();
        let mut builder = AutoCommandBufferBuilder::primary(
            vulkan.device.clone(),
            vulkan.queue.family(),
            CommandBufferUsage::OneTimeSubmit,
        )
            .unwrap();

        for (_id,obj) in self.objects.iter() {
            if let Some(model) = obj.model {
                let mesh = &self.meshes[&model.mesh];
                let tex = &self.textures[&model.texture];
                self.tex_mesh_renderer.push_model(model, mesh, tex, obj.transform);
            }
        }
        self.tex_mesh_renderer.prepare_draw(&self.camera);

        builder
            .begin_render_pass(
                vulkan.framebuffers[image_num].clone(),
                SubpassContents::Inline,
                vec![[0.0, 0.0, 0.0, 0.0].into(), (1.0).into()]
            )
            .unwrap()
            .set_viewport(0, [vulkan.viewport.clone()]);

        self.tex_mesh_renderer.draw(&mut builder);

        builder.end_render_pass().unwrap();

        let command_buffer = builder.build().unwrap();
        vulkan.execute_commands(command_buffer, image_num);
    }

    pub fn get_inputs(&self) -> Input {
        self.input.clone()
    }
}

pub struct GameObject {
    model:Option<Model>,
    transform:Isometry3,
    data:Box<dyn GameThing>
}
impl GameObject {
    pub fn move_by(&mut self, vec:Vec3) {
        self.transform.append_translation(vec);
    }
    pub fn data_mut<T:GameThing>(&mut self) -> &mut T {
        use std::any::Any;
        let dat = &mut self.data as &mut dyn Any;
        let dat:&mut T = dat.downcast_mut::<T>().expect("Invalid thing data!");
        dat
    }
}
use std::sync::Arc;
use vulkano::buffer::ImmutableBuffer;
use crate::image::Image;
use vulkano::image::immutable::ImmutableImage;

use bytemuck::{Pod, Zeroable};
#[repr(C)]
#[derive(Default, Debug, Clone, Copy, Pod, Zeroable)]
pub struct VertexUV {
    pub position: [f32; 3],
    pub uv: [f32; 2]
}
vulkano::impl_vertex!(VertexUV, position, uv);

pub struct Mesh {
    pub mesh:russimp::mesh::Mesh,
    pub verts:Arc<ImmutableBuffer<[VertexUV]>>,
    pub idx:Arc<ImmutableBuffer<[u32]>>
}
pub struct Texture {
    pub image:Image,
    pub texture:Arc<ImmutableImage>
}

#[derive(Clone,Copy,PartialEq,Eq,Hash)]
pub struct Model {
    pub mesh:MeshRef,
    pub texture:TextureRef
}

// string_interner
#[derive(Clone,Copy,PartialEq,Eq,Hash)]
pub struct MeshRef(usize);
#[derive(Clone,Copy,PartialEq,Eq,Hash)]
pub struct TextureRef(usize);
*/