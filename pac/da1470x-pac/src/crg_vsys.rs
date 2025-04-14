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
#[doc = r"CRG_VSYS registers"]
unsafe impl ::core::marker::Send for super::CrgVsys {}
unsafe impl ::core::marker::Sync for super::CrgVsys {}
impl super::CrgVsys {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn vsys_gen_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::VsysGenCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::VsysGenCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn vsys_gen_irq_clear_reg(
        &self,
    ) -> &'static crate::common::Reg<self::VsysGenIrqClearReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::VsysGenIrqClearReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn vsys_gen_irq_mask_reg(
        &self,
    ) -> &'static crate::common::Reg<self::VsysGenIrqMaskReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::VsysGenIrqMaskReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn vsys_gen_irq_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::VsysGenIrqStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::VsysGenIrqStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VsysGenCtrlReg_SPEC;
impl crate::sealed::RegSpec for VsysGenCtrlReg_SPEC {
    type DataType = u32;
}

pub type VsysGenCtrlReg = crate::RegValueT<VsysGenCtrlReg_SPEC>;

impl VsysGenCtrlReg {
    #[inline(always)]
    pub fn force_vbat_vsys_sw(
        self,
    ) -> crate::common::RegisterField<23, 0x3, 1, 0, u8, u8, VsysGenCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<23,0x3,1,0,u8,u8,VsysGenCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn force_ldo_enable(
        self,
    ) -> crate::common::RegisterField<21, 0x3, 1, 0, u8, u8, VsysGenCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<21,0x3,1,0,u8,u8,VsysGenCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_temp_protect_mode(
        self,
    ) -> crate::common::RegisterField<19, 0x3, 1, 0, u8, u8, VsysGenCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<19,0x3,1,0,u8,u8,VsysGenCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn en_headroom(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, VsysGenCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18,1,0,VsysGenCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn curlim_offset_trim(
        self,
    ) -> crate::common::RegisterField<13, 0x1f, 1, 0, u8, u8, VsysGenCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1f,1,0,u8,u8,VsysGenCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn curlim_gain_trim(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, u8, VsysGenCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1f,1,0,u8,u8,VsysGenCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn curlim_set(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, u8, VsysGenCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x7f,1,0,u8,u8,VsysGenCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn en_curlim(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, VsysGenCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,VsysGenCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for VsysGenCtrlReg {
    #[inline(always)]
    fn default() -> VsysGenCtrlReg {
        <crate::RegValueT<VsysGenCtrlReg_SPEC> as RegisterValue<_>>::new(397549)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VsysGenIrqClearReg_SPEC;
impl crate::sealed::RegSpec for VsysGenIrqClearReg_SPEC {
    type DataType = u32;
}

pub type VsysGenIrqClearReg = crate::RegValueT<VsysGenIrqClearReg_SPEC>;

impl VsysGenIrqClearReg {
    #[inline(always)]
    pub fn vbus_low_drive_irq_clear(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, VsysGenIrqClearReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1,1,0,VsysGenIrqClearReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_vsys_high_temp_irq_clear(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, VsysGenIrqClearReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0,1,0,VsysGenIrqClearReg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for VsysGenIrqClearReg {
    #[inline(always)]
    fn default() -> VsysGenIrqClearReg {
        <crate::RegValueT<VsysGenIrqClearReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VsysGenIrqMaskReg_SPEC;
impl crate::sealed::RegSpec for VsysGenIrqMaskReg_SPEC {
    type DataType = u32;
}

pub type VsysGenIrqMaskReg = crate::RegValueT<VsysGenIrqMaskReg_SPEC>;

impl VsysGenIrqMaskReg {
    #[inline(always)]
    pub fn vbus_low_drive_irq_mask(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, VsysGenIrqMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,VsysGenIrqMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_vsys_high_temp_irq_mask(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, VsysGenIrqMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,VsysGenIrqMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for VsysGenIrqMaskReg {
    #[inline(always)]
    fn default() -> VsysGenIrqMaskReg {
        <crate::RegValueT<VsysGenIrqMaskReg_SPEC> as RegisterValue<_>>::new(2)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VsysGenIrqStatusReg_SPEC;
impl crate::sealed::RegSpec for VsysGenIrqStatusReg_SPEC {
    type DataType = u32;
}

pub type VsysGenIrqStatusReg = crate::RegValueT<VsysGenIrqStatusReg_SPEC>;

impl VsysGenIrqStatusReg {
    #[inline(always)]
    pub fn vbus_low_drive_irq_status(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, VsysGenIrqStatusReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<1,1,0,VsysGenIrqStatusReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_vsys_high_temp_irq_status(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, VsysGenIrqStatusReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<0,1,0,VsysGenIrqStatusReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for VsysGenIrqStatusReg {
    #[inline(always)]
    fn default() -> VsysGenIrqStatusReg {
        <crate::RegValueT<VsysGenIrqStatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
