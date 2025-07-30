use std::sync::Arc;

use egui_wgpu::{wgpu, Renderer};
use egui_winit::State;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

async fn run() {
    let event_loop = EventLoop::new().unwrap();
    let window = Arc::new(
        WindowBuilder::new()
            .with_title("CADSMITH")
            .with_inner_size(winit::dpi::PhysicalSize::new(800, 600))
            .build(&event_loop)
            .unwrap(),
    );

    let instance = wgpu::Instance::default();
    let surface = instance.create_surface(&*window).unwrap();

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        })
        .await
        .unwrap();

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
            },
            None,
        )
        .await
        .unwrap();

    let capabilities = surface.get_capabilities(&adapter);
    let format = capabilities.formats[0];
    let mut config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format,
        width: window.inner_size().width,
        height: window.inner_size().height,
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode: capabilities.alpha_modes[0],
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    };
    surface.configure(&device, &config);

    let egui_context = egui::Context::default();
    let mut egui_state = State::new(
        egui_context.clone(),
        egui::ViewportId::ROOT,
        &*window,
        None,
        None,
    );

    let mut egui_renderer = Renderer::new(&device, format, None, 1);

    let window_clone = window.clone();
    let _ = event_loop.run(move |event, elwt| {
        let window = &*window_clone;
        elwt.set_control_flow(ControlFlow::Poll);

        match event {
            Event::WindowEvent { event, .. } => {
                if !egui_state.on_window_event(window, &event).consumed {
                    match event {
                        WindowEvent::CloseRequested => elwt.exit(),
                        WindowEvent::Resized(size) => {
                            config.width = size.width;
                            config.height = size.height;
                            surface.configure(&device, &config);
                        }
                        WindowEvent::RedrawRequested => {
                            let output = surface.get_current_texture().unwrap();
                            let view =
                                output.texture.create_view(&wgpu::TextureViewDescriptor::default());

                            let mut encoder = device.create_command_encoder(
                                &wgpu::CommandEncoderDescriptor {
                                    label: Some("Render Encoder"),
                                },
                            );

                            let screen_descriptor = egui_wgpu::ScreenDescriptor {
                                size_in_pixels: [config.width, config.height],
                                pixels_per_point: window.scale_factor() as f32,
                            };

                            // println!(
                            //     "ScreenDescriptor: width={}, height={}, pixels_per_point={}",
                            //     screen_descriptor.size_in_pixels[0],
                            //     screen_descriptor.size_in_pixels[1],
                            //     screen_descriptor.pixels_per_point,
                            // );
                            
                            let raw_input = egui_state.take_egui_input(window);
                            let full_output = egui_context.run(raw_input, |ctx| {
                                let toolbar_height = 40.0; 

                                egui::TopBottomPanel::top("top_toolbar").show(ctx, |ui| {
                                    ui.horizontal(|ui| {
                                        ui.label("🛠 Toolbar:");
                                        ui.button("New");
                                        ui.button("Save");
                                        ui.button("Undo");
                                    });
                                });

                                egui::SidePanel::right("right_panel").show(ctx, |ui| {
                                    ui.vertical(|ui| {
                                        ui.label("🛠 Right Panel:");
                                    });
                                });


                                egui::Window::new("Workspace")
                                    .default_pos(egui::pos2(0.0, 0.0))
                                    .movable(true) // 👈 disables dragging
                                    .resizable(true) // (optional) disables resizing
                                    .title_bar(false) // (optional) hides the title bar
                                    .default_pos(egui::pos2(100.0, 100.0)) // 👈 fixed screen position
                                    // .frame(egui::Frame::none()) // (optional) removes padding/frame
                                    .show(ctx, |ui| {
                                        let (response, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::click());

                                        let offset = response.rect.min.to_vec2(); // 👈 window top-left corner
                                        
                                        if let Some(pos) = ctx.input(|i| i.pointer.hover_pos()) {
                                            ctx.memory_mut(|m| {
                                                m.data.insert_temp("cursor_pos".into(), pos - offset); // 👈 relative to window
                                            });
                                        }

                                        if response.clicked() {
                                            if let Some(pos) = response.interact_pointer_pos() {
                                                ctx.memory_mut(|m| {
                                                    if let Some(start) = m.data.get_temp::<egui::Pos2>("line_start".into()) {
                                                        let mut lines = m.data.get_temp::<Vec<(egui::Pos2, egui::Pos2)>>("lines".into()).unwrap_or_default();
                                                        // lines.push((start, pos));
                                                        lines.push((start, pos - offset)); // 👈 store relative end
                                                        m.data.insert_temp("lines".into(), lines);
                                                        m.data.remove::<egui::Pos2>("line_start".into());
                                                        m.data.remove::<egui::Pos2>("cursor_pos".into());
                                                    } else {
                                                        // m.data.insert_temp("line_start".into(), pos);
                                                        m.data.insert_temp("line_start".into(), pos - offset); // 👈 relative to window
                                                    }
                                                });
                                            }
                                        }

                                        // Draw finalized lines
                                        let lines = ctx.memory(|m| m.data.get_temp::<Vec<(egui::Pos2, egui::Pos2)>>("lines".into()).unwrap_or_default());
                                        for (start, end) in lines {
                                            painter.line_segment([start + offset, end + offset], egui::Stroke::new(2.0, egui::Color32::YELLOW));
                                        }

                                        // Draw preview line
                                        let maybe_start = ctx.memory(|m| m.data.get_temp::<egui::Pos2>("line_start".into()));
                                        let maybe_cursor = ctx.memory(|m| m.data.get_temp::<egui::Pos2>("cursor_pos".into()));
                                        if let (Some(start), Some(cursor)) = (maybe_start, maybe_cursor) {
                                            painter.line_segment([start + offset, cursor + offset], egui::Stroke::new(1.0, egui::Color32::LIGHT_GRAY));
                                        }
                                    });
                            });

                            egui_state.handle_platform_output(window, full_output.platform_output.clone());

                            let clipped_primitives = egui_context.tessellate(
                                full_output.shapes,
                                screen_descriptor.pixels_per_point,
                            );

                            for (id, image_delta) in &full_output.textures_delta.set {
                                egui_renderer.update_texture(&device, &queue, *id, image_delta);
                            }

                            egui_renderer.update_buffers(
                                &device,
                                &queue,
                                &mut encoder,
                                &clipped_primitives,
                                &screen_descriptor,
                            );

                            {
                                let mut render_pass = encoder.begin_render_pass(
                                    &wgpu::RenderPassDescriptor {
                                        label: Some("Render Pass"),
                                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                            view: &view,
                                            resolve_target: None,
                                            ops: wgpu::Operations {
                                                load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                                                store: wgpu::StoreOp::Store,
                                            },
                                        })],
                                        depth_stencil_attachment: None,
                                        timestamp_writes: None,
                                        occlusion_query_set: None,
                                    },
                                );

                                // egui_renderer.update_textures(&device, &queue, &full_output.textures_delta);

                                egui_renderer.render(
                                    &mut render_pass,
                                    &clipped_primitives,
                                    &screen_descriptor,
                                );
                            }

                            queue.submit(Some(encoder.finish()));

                            output.present();

                            for id in &full_output.textures_delta.free {
                                egui_renderer.free_texture(id);
                            }
                        }
                        _ => {}
                    }
                }
            }
            Event::AboutToWait => {
                window.request_redraw();
            }
            _ => {}
        }
    });
}

fn main() {
    println!("frame started");
    pollster::block_on(run());
}
