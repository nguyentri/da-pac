/*
DISCLAIMER
This software is supplied by Renesas Electronics Corporation and is only intended for use with Renesas products.
No other uses are authorized. This software is owned by Renesas Electronics Corporation and is protected under all
applicable laws, including copyright laws.
THIS SOFTWARE IS PROVIDED "AS IS" AND RENESAS MAKES NO WARRANTIES REGARDING THIS SOFTWARE, WHETHER EXPRESS, IMPLIED
OR STATUTORY, INCLUDING BUT NOT LIMITED TO WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
NON-INFRINGEMENT.  ALL SUCH WARRANTIES ARE EXPRESSLY DISCLAIMED.TO THE MAXIMUM EXTENT PERMITTED NOT PROHIBITED BY
LAW, NEITHER RENESAS ELECTRONICS CORPORATION NOR ANY OF ITS AFFILIATED COMPANIES SHALL BE LIABLE FOR ANY DIRECT,
INDIRECT, SPECIAL, INCIDENTAL OR CONSEQUENTIAL DAMAGES FOR ANY REASON RELATED TO THIS SOFTWARE, EVEN IF RENESAS OR
ITS AFFILIATES HAVE BEEN ADVISED OF THE POSSIBILITY OF SUCH DAMAGES.
Renesas reserves the right, without notice, to make changes to this software and to discontinue the availability
of this software. By using this software, you agree to the additional terms and conditions found by accessing the
following link:
http://www.renesas.com/disclaimer

*/
// Generated from SVD 1.2, with svd2pac 0.4.0 on Sat, 12 Apr 2025 22:14:28 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"CRG_GPU registers"]
unsafe impl ::core::marker::Send for super::CrgGpu {}
unsafe impl ::core::marker::Sync for super::CrgGpu {}
impl super::CrgGpu {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    #[doc = "Control register for clocks in PD_GPU"]
    #[inline(always)]
    pub const fn clk_gpu_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ClkGpuReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ClkGpuReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkGpuReg_SPEC;
impl crate::sealed::RegSpec for ClkGpuReg_SPEC {
    type DataType = u32;
}
#[doc = "Control register for clocks in PD_GPU"]
pub type ClkGpuReg = crate::RegValueT<ClkGpuReg_SPEC>;

impl ClkGpuReg {
    #[doc = ""]
    #[inline(always)]
    pub fn mipi_d_phy_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, ClkGpuReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,ClkGpuReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn mipi_phy_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ClkGpuReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,ClkGpuReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn mipi_dsi_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, ClkGpuReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,ClkGpuReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn gpu_enable(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ClkGpuReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,ClkGpuReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ClkGpuReg {
    #[inline(always)]
    fn default() -> ClkGpuReg {
        <crate::RegValueT<ClkGpuReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
