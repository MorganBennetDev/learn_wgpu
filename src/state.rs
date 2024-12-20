use crate::camera::{self, CameraController, CameraUniform};
use crate::instance::{Instance, InstanceRaw};
use crate::light::LightUniform;
use crate::model::{DrawModel, Model, ModelVertex, Vertex};
use crate::renderer::Renderer;
use crate::{renderer, resources};
use crate::{camera::Camera, texture};
use cgmath::prelude::*;
use wgpu::util::DeviceExt;
use winit::event::{ElementState, KeyEvent, MouseButton};
use winit::keyboard::PhysicalKey;
use winit::{dpi::PhysicalPosition, event::WindowEvent, window::Window};

pub struct State<'a> {
    pub renderer: renderer::Renderer<'a>,
    pub window: &'a Window,
    pub mouse_pressed: bool,
}

impl<'a> State<'a> {
    // Creating some of the wgpu types requires async code
    pub async fn new(window: &'a Window) -> State<'a> {
        Self {
            renderer: renderer::Renderer::new(&window).await,
            window,
            mouse_pressed: false,
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(key),
                        state,
                        ..
                    },
                ..
            } => self.renderer.camera_controller.process_keyboard(*key, *state),
            WindowEvent::MouseWheel { delta, .. } => {
                self.renderer.camera_controller.process_scroll(delta);
                true
            }
            WindowEvent::MouseInput {
                button: MouseButton::Left,
                state,
                ..
            } => {
                self.mouse_pressed = *state == ElementState::Pressed;
                true
            }
            _ => false,
        }
    }
}
