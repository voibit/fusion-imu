use core::mem::MaybeUninit;
use fusion_imu_sys as sys;
use crate::Vector;

/// Gyroscope offset algorithm structure.
pub struct FusionOffset {
    inner: sys::FusionBias,
}


impl FusionOffset {
    /// Create a new `FusionOffset` instance.
    /// Arguments:
    /// `sample_rate` samplerate in Hz.
    /// 
    pub fn new(sample_rate: f32) -> Self {
        let mut bias = MaybeUninit::uninit();
        let mut setting = unsafe {sys::fusionBiasDefaultSettings};
        setting.sampleRate = sample_rate;

        unsafe {
            sys::FusionBiasInitialise(bias.as_mut_ptr());
            sys::FusionBiasSetSettings(bias.as_mut_ptr(),&setting as _);
            FusionOffset {
                inner: bias.assume_init(),
            }
        }
    }

    /// Updates the gyroscope offset algorithm and returns the corrected
    /// gyroscope measurement. Values are in degrees per second.
    pub fn update(&mut self, gyroscope: Vector) -> Vector {
        unsafe {
            sys::FusionBiasUpdate(&mut self.inner as *mut sys::FusionBias, gyroscope.into())
                .into()
        }
    }
}
