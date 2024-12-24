use std::collections::HashMap;

pub struct Compute {
    pub pipeline: wgpu::ComputePipeline,
    pub texture: wgpu::Texture,
    pub width: u32,
    pub height: u32
}

const WORKGROUP_SIZE: u32 = 16;

impl Compute {
    pub fn new(device: &wgpu::Device, config: wgpu::SurfaceConfiguration) -> Compute {
        let compute_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("compute shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/compute.wgsl").into())
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("compute bind group layout"),
            entries: &[
                // Input
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false
                    },
                    count: None
                },
                // Output
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::StorageTexture {
                        view_dimension: wgpu::TextureViewDimension::D2,
                        format: wgpu::TextureFormat::Rgba8Unorm,
                        access: wgpu::StorageTextureAccess::WriteOnly
                    },
                    count: None
                }
            ]
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("compute pipeline layout"),
            bind_group_layouts: &[ &bind_group_layout ],
            push_constant_ranges: &[]
        });

        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("compute pipeline"),
            layout: Some(&pipeline_layout),
            module: &compute_module,
            compilation_options: wgpu::PipelineCompilationOptions {
                constants: &HashMap::new(),
                zero_initialize_workgroup_memory: false,
                vertex_pulling_transform: false
            },
            cache: None,
            entry_point: "main"
        });

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            size: wgpu::Extent3d { width: config.width, height: config.height, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::STORAGE_BINDING,
            label: Some("postrender texture"),
            view_formats: &[]
        });
        
        return Compute {
            pipeline,
            texture,
            width: config.width,
            height: config.height
        };
    }

    pub fn resize(&mut self, device: &wgpu::Device, new_size: winit::dpi::PhysicalSize<u32>) {
        self.width = new_size.width;
        self.height = new_size.height;
        self.texture = device.create_texture(&wgpu::TextureDescriptor {
            size: wgpu::Extent3d { width: self.width, height: self.height, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::STORAGE_BINDING,
            label: Some("postrender texture"),
            view_formats: &[]
        });
    }

    pub fn run(&mut self, input_view: &wgpu::TextureView, device: &wgpu::Device, queue: &wgpu::Queue) -> wgpu::TextureView {
        let output_view = self.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("compute bind group"),
            layout: &self.pipeline.get_bind_group_layout(0),
            entries: &[
                // Input
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(input_view),
                },
                // Output
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&output_view),
                },
            ],
        });

        let mut encoder = device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("compute encoder"),
            });

        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("compute pass"),
                timestamp_writes: None
            });

            compute_pass.set_pipeline(&self.pipeline);
            compute_pass.set_bind_group(0, &bind_group, &[]);
            compute_pass.dispatch_workgroups((self.width + WORKGROUP_SIZE - 1) / WORKGROUP_SIZE, (self.height + WORKGROUP_SIZE - 1) / WORKGROUP_SIZE, 1);
        }

        queue.submit(std::iter::once(encoder.finish()));

        return output_view;
    }
}