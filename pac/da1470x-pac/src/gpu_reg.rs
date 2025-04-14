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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:16:41 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"GPU_REG registers"]
unsafe impl ::core::marker::Send for super::GpuReg {}
unsafe impl ::core::marker::Sync for super::GpuReg {}
impl super::GpuReg {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn gpu_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::GpuCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GpuCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpuCtrlReg_SPEC;
impl crate::sealed::RegSpec for GpuCtrlReg_SPEC {
    type DataType = u32;
}

pub type GpuCtrlReg = crate::RegValueT<GpuCtrlReg_SPEC>;

impl GpuCtrlReg {
    #[inline(always)]
    pub fn pwrs_b(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, GpuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,GpuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn gpu_ram_ms(
        self,
    ) -> crate::common::RegisterField<2, 0xf, 1, 0, u8, u8, GpuCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0xf,1,0,u8,u8,GpuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn gpu_ram_mse(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, GpuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,GpuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn gpu_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, GpuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,GpuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GpuCtrlReg {
    #[inline(always)]
    fn default() -> GpuCtrlReg {
        <crate::RegValueT<GpuCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
