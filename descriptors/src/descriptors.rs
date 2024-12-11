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

/// Descriptors
pub struct Descriptors {
    /// The descriptor set
    descriptor_sets: DescriptorSets
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

        log::info!("created device");
        
        Ok(Self {
            descriptor_sets
        })
    }
}
