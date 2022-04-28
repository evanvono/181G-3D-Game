use crate::animation;
use crate::assets::{self, Assets};
use crate::camera::Camera;
use crate::input;
use crate::input::Input;
use crate::renderer;
use crate::renderer::flat::NUM_CLUES;
use crate::vulkan::Vulkan;
use crate::Isometry3;
use color_eyre::eyre::Result;
use std::rc::Rc;
use vulkano::query::{
    QueryControlFlags, QueryPool, QueryPoolCreateInfo, QueryResultFlags, QueryType,
};
use winit::event::MouseButton;
use winit::event::{Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

pub trait GameThing: 'static {}

pub trait World {
    fn update(&mut self, inp: &input::Input, assets: &mut assets::Assets);
    fn render(&mut self, assets: &mut assets::Assets, render_state: &mut renderer::RenderState);
    fn handle_query_pool_results(&mut self, query_pool_results: &[u32; NUM_CLUES]);
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
            title: "Capture the Evidence".to_string(),
        }
    }
}
use thunderdome::{Arena, Index};
pub struct GameObjectRef(Index);
pub struct TransformParent {
    parent: GameObjectRef,
    local_to_global: Isometry3,
    global_to_local: Isometry3,
}

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
    query_results: [u32; NUM_CLUES]
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
            query_results: [0u32; NUM_CLUES]
        }
    }
    pub fn set_camera(&mut self, cam: Camera) {
        self.render_states = [
            crate::renderer::RenderState::new(cam),
            crate::renderer::RenderState::new(cam),
        ]
    }
    pub fn play(mut self, f: impl Fn(&mut Self) + 'static) -> Result<()> {
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
                    event:
                        WindowEvent::MouseInput {
                            state: button_state,
                            button: MouseButton::Left,
                            ..
                        },
                    ..
                } => self.input.handle_left_mouse_event(button_state),
                Event::DeviceEvent {
                    event: winit::event::DeviceEvent::MouseMotion { delta },
                    ..
                } => self.input.handle_cursor_motion(delta),
                Event::WindowEvent {
                    event: WindowEvent::CursorMoved { position, .. },
                    ..
                } => self.input.handle_cursor_moved_event(position),
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
                }
                Event::WindowEvent {
                    event:
                        WindowEvent::MouseInput {
                            state: button_state,
                            button: MouseButton::Left,
                            ..
                        },
                    ..
                } => self.input.handle_left_mouse_event(button_state),
                Event::DeviceEvent {
                    event: winit::event::DeviceEvent::MouseMotion { delta },
                    ..
                } => self.input.handle_cursor_motion(delta),
                Event::WindowEvent {
                    event: WindowEvent::CursorMoved { position, .. },
                    ..
                } => self.input.handle_cursor_moved_event(position),
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
                                // let results = self.get_query_pool_results();
                                w.handle_query_pool_results(&self.query_results);
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

        unsafe {
            builder
                .reset_query_pool(self.flat_renderer.query_pool.clone(), 0..NUM_CLUES as u32)
                .unwrap()
                .begin_render_pass(
                    vulkan.framebuffers[image_num].clone(),
                    SubpassContents::Inline,
                    vec![[0.0, 0.0, 0.0, 0.0].into(), (0.0).into()],
                )
                .unwrap()
                .set_viewport(0, [vulkan.viewport.clone()]);

            self.skinned_renderer.draw(&mut builder);
            self.sprites_renderer.draw(&mut builder);
            self.textured_renderer.draw(&mut builder);
            self.flat_renderer.draw(&mut builder);

            builder.end_render_pass().unwrap();
        }
        let command_buffer = builder.build().unwrap();

        self.flat_renderer.query_pool
                .queries_range(0..NUM_CLUES as u32)
                .unwrap()
                .get_results(
                    &mut self.query_results,
                    QueryResultFlags {
                        // Block the function call until the results are available.
                        // Note: if not all the queries have actually been executed, then this
                        // will wait forever for something that never happens!
                        wait: false,

                        // Blocking and waiting will never give partial results.
                        partial: false,

                        // Blocking and waiting will ensure the results are always available after
                        // the function returns.
                        //
                        // If you disable waiting, then this can be used to include the
                        // availability of each query's results. You need one extra element per
                        // query in your `query_results` buffer for this. This element will
                        // be filled with a zero/nonzero value indicating availability.
                        with_availability: false,
                    },
                )
                .unwrap();

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
    fn get_query_pool_results(&self) -> [u32; NUM_CLUES] {
        let mut query_results = [0u32; NUM_CLUES];

        self.flat_renderer
            .query_pool
            .queries_range(0..NUM_CLUES as u32)
            .unwrap()
            .get_results(
                &mut query_results,
                QueryResultFlags {
                    // Block the function call until the results are available.
                    // Note: if not all the queries have actually been executed, then this
                    // will wait forever for something that never happens!
                    wait: true,

                    // Blocking and waiting will never give partial results.
                    partial: false,

                    // Blocking and waiting will ensure the results are always available after
                    // the function returns.
                    //
                    // If you disable waiting, then this can be used to include the
                    // availability of each query's results. You need one extra element per
                    // query in your `query_results` buffer for this. This element will
                    // be filled with a zero/nonzero value indicating availability.
                    with_availability: false,
                },
            )
            .unwrap();
        query_results
    }
}
