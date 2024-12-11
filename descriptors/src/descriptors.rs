//! # Descriptors.rs
//!
//! Vulkan descriptor sets.
//!
//! The descriptors live in GPU memory.
//! The CPU writes them, and the GPU reads them from shaders.
//!
//! Animats
//! December, 2024.
///
use log;
use anyhow::{Error};
use ash::{vk};
use vk::Handle;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// The types of descriptor tables. These live in the GPU.
enum DescriptorTableType {
    StorageBuffer = 0,
    SampledImage = 1,
    StorageImage = 2,
}

impl DescriptorTableType {
    /// Return all the types
    fn all_types() -> impl Iterator<Item = Self> {
        [Self::StorageBuffer, Self::SampledImage, Self::StorageImage].into_iter()
    }

    fn set_index(self) -> u32 {
        self as u32
    }

    fn from_set_index(set_index: u32) -> Self {
        match set_index {
            0 => Self::StorageBuffer,
            1 => Self::SampledImage,
            _ => panic!("invalid set index"),
        }
    }

    /// Local name to Vulkann ame.
    fn to_vk(self) -> vk::DescriptorType {
        match self {
            DescriptorTableType::StorageBuffer => vk::DescriptorType::STORAGE_BUFFER,
            DescriptorTableType::SampledImage => vk::DescriptorType::SAMPLED_IMAGE,
            DescriptorTableType::StorageImage => vk::DescriptorType::STORAGE_IMAGE,
        }
    }

    /// Display name
    fn name(self) -> &'static str {
        match self {
            DescriptorTableType::StorageBuffer => "storage_buffer",
            DescriptorTableType::SampledImage => "sampled_image",
            DescriptorTableType::StorageImage => "storage_image",
        }
    }

    fn max_count(self, gpu: &GpuInfo) -> u32 {
        let props = &gpu.properties.properties12;
        match self {
            DescriptorTableType::StorageBuffer => u32::min(
                props.max_descriptor_set_update_after_bind_storage_buffers,
                props.max_per_stage_descriptor_update_after_bind_storage_buffers,
            ),
            DescriptorTableType::SampledImage => u32::min(
                props.max_descriptor_set_update_after_bind_sampled_images,
                props.max_per_stage_descriptor_update_after_bind_sampled_images,
            ),
            DescriptorTableType::StorageImage => u32::min(
                props.max_descriptor_set_update_after_bind_storage_images,
                props.max_per_stage_descriptor_update_after_bind_storage_images,
            ),
        }
    }
}


/// Descriptors
pub struct Descriptors {
    /// The descriptor set
    descriptor_sets: Vec<vk::DescriptorSet>,
}

impl Descriptors {
    /// Create the descriptor tables.
    ///
    /// Loosely modeled after how Orbit does this.
    pub fn new() -> Result<Self, Error> {
        let pool_sizes: Vec<_> = DescriptorTableType::all_types()
            .map(|desc_ty| vk::DescriptorPoolSize {
                ty: desc_ty.to_vk(),
                descriptor_count: desc_ty.max_count(&gpu),
            })
            .collect();

        let pool_create_info = vk::DescriptorPoolCreateInfo::builder()
            .flags(vk::DescriptorPoolCreateFlags::UPDATE_AFTER_BIND)
            .max_sets(4)
            .pool_sizes(&pool_sizes);

        let descriptor_pool = unsafe {
            device
                .create_descriptor_pool(&pool_create_info, None)
                .unwrap()
        };

        set_debug_name(
            vk::DescriptorPool::TYPE,
            descriptor_pool.as_raw(),
            "bindless_descriptor_pool",
        );

        let descriptor_counts: Vec<_> = DescriptorTableType::all_types()
            .map(|ty| ty.max_count(&gpu))
            .collect();

        let mut variable_count = vk::DescriptorSetVariableDescriptorCountAllocateInfo::builder()
            .descriptor_counts(&descriptor_counts);

        let alloc_info = vk::DescriptorSetAllocateInfo::builder()
            .descriptor_pool(descriptor_pool)
            .set_layouts(&descriptor_layouts)
            .push_next(&mut variable_count);

        let descriptor_sets = unsafe { device.allocate_descriptor_sets(&alloc_info).unwrap() };

        let names = [
            "buffer_descriptor_set",
            "sampled_image_descriptor_set",
            "storage_image_descriptor_index",
        ];

        for (i, descriptor_set) in descriptor_sets.iter().enumerate() {
            set_debug_name(vk::DescriptorSet::TYPE, descriptor_set.as_raw(), names[i]);
        }

        log::info!("created descriptor table");
        
        Ok(Self {
            descriptor_sets
        })
    }
}
