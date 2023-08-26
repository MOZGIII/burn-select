pub mod config;

#[derive(Debug, Clone, enum_kinds::EnumKind)]
#[enum_kind(
    BackendKind,
    cfg_attr(feature = "clap", derive(clap::ValueEnum)),
    cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))
)]
pub enum Backend<Config: self::Config> {
    #[cfg(feature = "burn-tch")]
    Tch(burn_tch::TchBackend<Config::TchElement>),
    #[cfg(feature = "burn-wgpu")]
    Wgpu(
        burn_wgpu::WgpuBackend<
            Config::WgpuDevice,
            Config::WgpuFloatElement,
            Config::WgpuIntElement,
        >,
    ),
    #[cfg(feature = "burn-ndarray")]
    Ndarray(burn_ndarray::NdArrayBackend<Config::NdarrayElement>),
}

pub trait Config {
    #[cfg(feature = "burn-tch")]
    type TchElement: burn_tch::TchElement;

    #[cfg(feature = "burn-wgpu")]
    type WgpuDevice: burn_wgpu::GraphicsApi;
    #[cfg(feature = "burn-wgpu")]
    type WgpuFloatElement: burn_wgpu::FloatElement;
    #[cfg(feature = "burn-wgpu")]
    type WgpuIntElement: burn_wgpu::IntElement;

    #[cfg(feature = "burn-ndarray")]
    type NdarrayElement: burn_ndarray::FloatNdArrayElement;
}
