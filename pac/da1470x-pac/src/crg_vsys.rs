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
#[doc = r"CRG_VSYS registers"]
unsafe impl ::core::marker::Send for super::CrgVsys {}
unsafe impl ::core::marker::Sync for super::CrgVsys {}
impl super::CrgVsys {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    #[doc = ""]
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

    #[doc = ""]
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

    #[doc = ""]
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

    #[doc = ""]
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
#[doc = ""]
pub type VsysGenCtrlReg = crate::RegValueT<VsysGenCtrlReg_SPEC>;

impl VsysGenCtrlReg {
    #[doc = "0x0,0x1: VBAT_VSYS set to ideal diode\n0x2: Forces VBAT_VSYS switch to be opened\n0x3: Forces VBAT_VSYS switch to be closed"]
    #[inline(always)]
    pub fn force_vbat_vsys_sw(
        self,
    ) -> crate::common::RegisterField<23, 0x3, 1, 0, u8, VsysGenCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<23,0x3,1,0,u8, VsysGenCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0x0,0x1: LDO VSYS will be enabled when COMP_VBUS_OK & COMP_VBUS_ABOVE_VSYS.\n0x2: LDO_VSYS will be disabled regardless of VBUS state\n0x3: LDO_VSYS will be enabled regardless of VBUS state"]
    #[inline(always)]
    pub fn force_ldo_enable(
        self,
    ) -> crate::common::RegisterField<21, 0x3, 1, 0, u8, VsysGenCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<21,0x3,1,0,u8, VsysGenCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0x0: LDO_VSYS is muted (disabled) when temperature is too high, when temperature is back to operating conditions ldo is unmuted\n0x1: LDO_VSYS is muted (disabled) when temperature is too high, LDO_VSYS_HIGH_TEMP_IRQ must be cleared to allow ldo to be unmuted. See VSYS_GEN_IRQ_CLEAR_REG.\n0x2: LDO_VSYS is not muted (disabled) when temperature is too high\n0x3: Force LDO_VSYS mute"]
    #[inline(always)]
    pub fn ldo_temp_protect_mode(
        self,
    ) -> crate::common::RegisterField<19, 0x3, 1, 0, u8, VsysGenCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<19,0x3,1,0,u8, VsysGenCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enables the voltage headroom loop in the LDO_VSYS."]
    #[inline(always)]
    pub fn en_headroom(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, VsysGenCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18,1,0,VsysGenCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "For adjusting the offset of the curlim range ( +/- 78 mA)\n0x0 = maximum positive offset.\n0x10 = minimal offset ( reset value).\n0x1F = maximum negative offset."]
    #[inline(always)]
    pub fn curlim_offset_trim(
        self,
    ) -> crate::common::RegisterField<13, 0x1f, 1, 0, u8, VsysGenCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1f,1,0,u8, VsysGenCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "For adjusting the gain of the curlim range ( +/- 20%)\n0x0 = maximum gain.\n0x10 = nominal gain ( reset value).\n0x1F = minimum gain."]
    #[inline(always)]
    pub fn curlim_gain_trim(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, VsysGenCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1f,1,0,u8, VsysGenCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Sets the level of the LDO_VSYS current limiter in 10 mA steps. It is reset when vbus is not plugged (COMP_VBUS_PLUGIN == 0).\n0x0 = 1270 mA\n0x1 = 1260 mA\n...\n0x76 = 90 mA ( reset value)"]
    #[inline(always)]
    pub fn curlim_set(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, VsysGenCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x7f,1,0,u8, VsysGenCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enables the current limiter in the LDO_VSYS"]
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
#[doc = ""]
pub type VsysGenIrqClearReg = crate::RegValueT<VsysGenIrqClearReg_SPEC>;

impl VsysGenIrqClearReg {
    #[doc = "Clears VBUS_LOW_DRIVE_IRQ"]
    #[inline(always)]
    pub fn vbus_low_drive_irq_clear(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, VsysGenIrqClearReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1,1,0,VsysGenIrqClearReg_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clears LDO_VSYS_HIGH_TEMP_IRQ"]
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
#[doc = ""]
pub type VsysGenIrqMaskReg = crate::RegValueT<VsysGenIrqMaskReg_SPEC>;

impl VsysGenIrqMaskReg {
    #[doc = "Masks VBUS_LOW_DRIVE_IRQ interrupt. It is reset when vbus is not plugged in (COMP_VBUS_PLUGIN == 0). This is because when vbus is just plugged in, it is likely to bounce; therefore IRQ is masked to avoid false triggering. Some time after vbus plugin, software might increase ldo_vsys current limit and should set this mask to 0 (after clearing IRQ)."]
    #[inline(always)]
    pub fn vbus_low_drive_irq_mask(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, VsysGenIrqMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,VsysGenIrqMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Masks LDO_VSYS_HIGH_TEMP_IRQ interrupt"]
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
#[doc = ""]
pub type VsysGenIrqStatusReg = crate::RegValueT<VsysGenIrqStatusReg_SPEC>;

impl VsysGenIrqStatusReg {
    #[doc = "Indicates vbus drive strength is not enough to keep vbus up with the set limit of ldo_vbus. Consider lowering CURLIM_SET in VSYS_GEN_CTRL_REG."]
    #[inline(always)]
    pub fn vbus_low_drive_irq_status(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, VsysGenIrqStatusReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<1,1,0,VsysGenIrqStatusReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Indicates that a high temperature has been detected at ldo_vsys"]
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
