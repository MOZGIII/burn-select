use std::marker::PhantomData;

pub struct Default<Element>(PhantomData<Element>);

#[cfg(all(feature = "burn-wgpu", target_os = "macos"))]
type DefaultWgpuDevice = burn_wgpu::Metal;
#[cfg(all(feature = "burn-wgpu", not(target_os = "macos")))]
type DefaultWgpuDevice = burn_wgpu::Vulkan;

impl super::Config for Default<f32> {
    #[cfg(feature = "burn-tch")]
    type TchElement = f32;

    #[cfg(feature = "burn-wgpu")]
    type WgpuDevice = DefaultWgpuDevice;
    #[cfg(feature = "burn-wgpu")]
    type WgpuFloatElement = f32;
    #[cfg(feature = "burn-wgpu")]
    type WgpuIntElement = i32;

    #[cfg(feature = "burn-ndarray")]
    type NdarrayElement = f32;
}

#[cfg(not(feature = "burn-wgpu"))]
#[cfg(not(feature = "burn-ndarray"))]
impl super::Config for Default<burn_tensor::f16> {
    #[cfg(feature = "burn-tch")]
    type TchElement = burn_tensor::f16;
}

#[cfg(not(feature = "burn-wgpu"))]
#[cfg(not(feature = "burn-ndarray"))]
impl super::Config for Default<burn_tensor::bf16> {
    #[cfg(feature = "burn-tch")]
    type TchElement = burn_tensor::bf16;
}
