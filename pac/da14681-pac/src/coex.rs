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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:15:56 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"COEX registers"]
unsafe impl ::core::marker::Send for super::Coex {}
unsafe impl ::core::marker::Sync for super::Coex {}
impl super::Coex {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn coex_ble_pti_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CoexBlePtiReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CoexBlePtiReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn coex_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CoexCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CoexCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn coex_diag_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CoexDiagReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CoexDiagReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn coex_ftdf_pti_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CoexFtdfPtiReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CoexFtdfPtiReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(10usize),
            )
        }
    }

    #[inline(always)]
    pub const fn coex_int_mask_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CoexIntMaskReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CoexIntMaskReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn coex_int_stat_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CoexIntStatReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CoexIntStatReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[inline(always)]
    pub const fn coex_pri10_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CoexPri10Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CoexPri10Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[inline(always)]
    pub const fn coex_pri11_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CoexPri11Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CoexPri11Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(38usize),
            )
        }
    }

    #[inline(always)]
    pub const fn coex_pri12_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CoexPri12Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CoexPri12Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[inline(always)]
    pub const fn coex_pri13_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CoexPri13Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CoexPri13Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(42usize),
            )
        }
    }

    #[inline(always)]
    pub const fn coex_pri14_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CoexPri14Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CoexPri14Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[inline(always)]
    pub const fn coex_pri15_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CoexPri15Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CoexPri15Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(46usize),
            )
        }
    }

    #[inline(always)]
    pub const fn coex_pri16_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CoexPri16Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CoexPri16Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[inline(always)]
    pub const fn coex_pri17_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CoexPri17Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CoexPri17Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(50usize),
            )
        }
    }

    #[inline(always)]
    pub const fn coex_pri1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CoexPri1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CoexPri1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(18usize),
            )
        }
    }

    #[inline(always)]
    pub const fn coex_pri2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CoexPri2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CoexPri2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[inline(always)]
    pub const fn coex_pri3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CoexPri3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CoexPri3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(22usize),
            )
        }
    }

    #[inline(always)]
    pub const fn coex_pri4_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CoexPri4Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CoexPri4Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[inline(always)]
    pub const fn coex_pri5_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CoexPri5Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CoexPri5Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(26usize),
            )
        }
    }

    #[inline(always)]
    pub const fn coex_pri6_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CoexPri6Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CoexPri6Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[inline(always)]
    pub const fn coex_pri7_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CoexPri7Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CoexPri7Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(30usize),
            )
        }
    }

    #[inline(always)]
    pub const fn coex_pri8_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CoexPri8Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CoexPri8Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[inline(always)]
    pub const fn coex_pri9_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CoexPri9Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CoexPri9Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(34usize),
            )
        }
    }

    #[inline(always)]
    pub const fn coex_stat_reg(
        &self,
    ) -> &'static crate::common::Reg<self::CoexStatReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::CoexStatReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CoexBlePtiReg_SPEC;
impl crate::sealed::RegSpec for CoexBlePtiReg_SPEC {
    type DataType = u16;
}

pub type CoexBlePtiReg = crate::RegValueT<CoexBlePtiReg_SPEC>;

impl CoexBlePtiReg {
    #[inline(always)]
    pub fn coex_ble_pti(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, CoexBlePtiReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,CoexBlePtiReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CoexBlePtiReg {
    #[inline(always)]
    fn default() -> CoexBlePtiReg {
        <crate::RegValueT<CoexBlePtiReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CoexCtrlReg_SPEC;
impl crate::sealed::RegSpec for CoexCtrlReg_SPEC {
    type DataType = u16;
}

pub type CoexCtrlReg = crate::RegValueT<CoexCtrlReg_SPEC>;

impl CoexCtrlReg {
    #[inline(always)]
    pub fn ignore_ble(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, CoexCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,CoexCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ignore_ftdf(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, CoexCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,CoexCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ignore_ext(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, CoexCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,CoexCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sel_ble_radio_busy(
        self,
    ) -> crate::common::RegisterField<11, 0x3, 1, 0, u8, u8, CoexCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x3,1,0,u8,u8,CoexCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sel_ble_wlan_tx_rx(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, CoexCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,CoexCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sel_ftdf_pti(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, CoexCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,CoexCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sel_ftdf_cca(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, CoexCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,CoexCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sel_coex_diag(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, u8, u8, CoexCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x3,1,0,u8,u8,CoexCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn smart_act_impl(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, CoexCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,CoexCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn prging_arbiter(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, CoexCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,CoexCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CoexCtrlReg {
    #[inline(always)]
    fn default() -> CoexCtrlReg {
        <crate::RegValueT<CoexCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CoexDiagReg_SPEC;
impl crate::sealed::RegSpec for CoexDiagReg_SPEC {
    type DataType = u16;
}

pub type CoexDiagReg = crate::RegValueT<CoexDiagReg_SPEC>;

impl CoexDiagReg {
    #[inline(always)]
    pub fn coex_diag_mon(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, CoexDiagReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,CoexDiagReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for CoexDiagReg {
    #[inline(always)]
    fn default() -> CoexDiagReg {
        <crate::RegValueT<CoexDiagReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CoexFtdfPtiReg_SPEC;
impl crate::sealed::RegSpec for CoexFtdfPtiReg_SPEC {
    type DataType = u16;
}

pub type CoexFtdfPtiReg = crate::RegValueT<CoexFtdfPtiReg_SPEC>;

impl CoexFtdfPtiReg {
    #[inline(always)]
    pub fn coex_ftdf_pti(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, CoexFtdfPtiReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,CoexFtdfPtiReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CoexFtdfPtiReg {
    #[inline(always)]
    fn default() -> CoexFtdfPtiReg {
        <crate::RegValueT<CoexFtdfPtiReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CoexIntMaskReg_SPEC;
impl crate::sealed::RegSpec for CoexIntMaskReg_SPEC {
    type DataType = u16;
}

pub type CoexIntMaskReg = crate::RegValueT<CoexIntMaskReg_SPEC>;

impl CoexIntMaskReg {
    #[inline(always)]
    pub fn coex_irq_on_decision_sw(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, CoexIntMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,CoexIntMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn coex_irq_on_start_mid(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, CoexIntMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,CoexIntMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn coex_irq_on_closing_brk(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, CoexIntMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,CoexIntMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn coex_irq_on_radio_busy_f(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, CoexIntMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,CoexIntMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn coex_irq_on_radio_busy_r(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, CoexIntMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,CoexIntMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn coex_irq_on_ble_active_f(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, CoexIntMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,CoexIntMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn coex_irq_on_ble_active_r(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, CoexIntMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,CoexIntMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn coex_irq_on_ftdf_active_f(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, CoexIntMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,CoexIntMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn coex_irq_on_ftdf_active_r(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, CoexIntMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,CoexIntMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn coex_irq_on_ext_act_f(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, CoexIntMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,CoexIntMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn coex_irq_on_ext_act_r(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, CoexIntMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,CoexIntMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn coex_irq_on_smart_pri_f(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, CoexIntMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,CoexIntMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn coex_irq_on_smart_pri_r(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, CoexIntMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,CoexIntMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn coex_irq_on_smart_act_f(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, CoexIntMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,CoexIntMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn coex_irq_on_smart_act_r(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, CoexIntMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,CoexIntMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn coex_irq_mask(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, CoexIntMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,CoexIntMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CoexIntMaskReg {
    #[inline(always)]
    fn default() -> CoexIntMaskReg {
        <crate::RegValueT<CoexIntMaskReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CoexIntStatReg_SPEC;
impl crate::sealed::RegSpec for CoexIntStatReg_SPEC {
    type DataType = u16;
}

pub type CoexIntStatReg = crate::RegValueT<CoexIntStatReg_SPEC>;

impl CoexIntStatReg {
    #[inline(always)]
    pub fn coex_irq_on_decision_sw(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, CoexIntStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15,1,0,CoexIntStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn coex_irq_on_start_mid(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, CoexIntStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14,1,0,CoexIntStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn coex_irq_on_closing_brk(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, CoexIntStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13,1,0,CoexIntStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn coex_irq_on_radio_busy_f(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, CoexIntStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12,1,0,CoexIntStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn coex_irq_on_radio_busy_r(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, CoexIntStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11,1,0,CoexIntStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn coex_irq_on_ble_active_f(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, CoexIntStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,CoexIntStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn coex_irq_on_ble_active_r(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, CoexIntStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9,1,0,CoexIntStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn coex_irq_on_ftdf_active_f(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, CoexIntStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,CoexIntStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn coex_irq_on_ftdf_active_r(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, CoexIntStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,CoexIntStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn coex_irq_on_ext_act_f(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, CoexIntStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,CoexIntStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn coex_irq_on_ext_act_r(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, CoexIntStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,CoexIntStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn coex_irq_on_smart_pri_f(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, CoexIntStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,CoexIntStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn coex_irq_on_smart_pri_r(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, CoexIntStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,CoexIntStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn coex_irq_on_smart_act_f(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, CoexIntStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,CoexIntStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn coex_irq_on_smart_act_r(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, CoexIntStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,CoexIntStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn coex_irq_stat(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, CoexIntStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,CoexIntStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for CoexIntStatReg {
    #[inline(always)]
    fn default() -> CoexIntStatReg {
        <crate::RegValueT<CoexIntStatReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CoexPri10Reg_SPEC;
impl crate::sealed::RegSpec for CoexPri10Reg_SPEC {
    type DataType = u16;
}

pub type CoexPri10Reg = crate::RegValueT<CoexPri10Reg_SPEC>;

impl CoexPri10Reg {
    #[inline(always)]
    pub fn coex_pri_mac(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, u8, CoexPri10Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x3,1,0,u8,u8,CoexPri10Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn coex_pri_pti(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, CoexPri10Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,CoexPri10Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CoexPri10Reg {
    #[inline(always)]
    fn default() -> CoexPri10Reg {
        <crate::RegValueT<CoexPri10Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CoexPri11Reg_SPEC;
impl crate::sealed::RegSpec for CoexPri11Reg_SPEC {
    type DataType = u16;
}

pub type CoexPri11Reg = crate::RegValueT<CoexPri11Reg_SPEC>;

impl CoexPri11Reg {
    #[inline(always)]
    pub fn coex_pri_mac(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, u8, CoexPri11Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x3,1,0,u8,u8,CoexPri11Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn coex_pri_pti(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, CoexPri11Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,CoexPri11Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CoexPri11Reg {
    #[inline(always)]
    fn default() -> CoexPri11Reg {
        <crate::RegValueT<CoexPri11Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CoexPri12Reg_SPEC;
impl crate::sealed::RegSpec for CoexPri12Reg_SPEC {
    type DataType = u16;
}

pub type CoexPri12Reg = crate::RegValueT<CoexPri12Reg_SPEC>;

impl CoexPri12Reg {
    #[inline(always)]
    pub fn coex_pri_mac(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, u8, CoexPri12Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x3,1,0,u8,u8,CoexPri12Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn coex_pri_pti(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, CoexPri12Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,CoexPri12Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CoexPri12Reg {
    #[inline(always)]
    fn default() -> CoexPri12Reg {
        <crate::RegValueT<CoexPri12Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CoexPri13Reg_SPEC;
impl crate::sealed::RegSpec for CoexPri13Reg_SPEC {
    type DataType = u16;
}

pub type CoexPri13Reg = crate::RegValueT<CoexPri13Reg_SPEC>;

impl CoexPri13Reg {
    #[inline(always)]
    pub fn coex_pri_mac(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, u8, CoexPri13Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x3,1,0,u8,u8,CoexPri13Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn coex_pri_pti(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, CoexPri13Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,CoexPri13Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CoexPri13Reg {
    #[inline(always)]
    fn default() -> CoexPri13Reg {
        <crate::RegValueT<CoexPri13Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CoexPri14Reg_SPEC;
impl crate::sealed::RegSpec for CoexPri14Reg_SPEC {
    type DataType = u16;
}

pub type CoexPri14Reg = crate::RegValueT<CoexPri14Reg_SPEC>;

impl CoexPri14Reg {
    #[inline(always)]
    pub fn coex_pri_mac(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, u8, CoexPri14Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x3,1,0,u8,u8,CoexPri14Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn coex_pri_pti(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, CoexPri14Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,CoexPri14Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CoexPri14Reg {
    #[inline(always)]
    fn default() -> CoexPri14Reg {
        <crate::RegValueT<CoexPri14Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CoexPri15Reg_SPEC;
impl crate::sealed::RegSpec for CoexPri15Reg_SPEC {
    type DataType = u16;
}

pub type CoexPri15Reg = crate::RegValueT<CoexPri15Reg_SPEC>;

impl CoexPri15Reg {
    #[inline(always)]
    pub fn coex_pri_mac(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, u8, CoexPri15Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x3,1,0,u8,u8,CoexPri15Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn coex_pri_pti(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, CoexPri15Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,CoexPri15Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CoexPri15Reg {
    #[inline(always)]
    fn default() -> CoexPri15Reg {
        <crate::RegValueT<CoexPri15Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CoexPri16Reg_SPEC;
impl crate::sealed::RegSpec for CoexPri16Reg_SPEC {
    type DataType = u16;
}

pub type CoexPri16Reg = crate::RegValueT<CoexPri16Reg_SPEC>;

impl CoexPri16Reg {
    #[inline(always)]
    pub fn coex_pri_mac(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, u8, CoexPri16Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x3,1,0,u8,u8,CoexPri16Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn coex_pri_pti(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, CoexPri16Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,CoexPri16Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CoexPri16Reg {
    #[inline(always)]
    fn default() -> CoexPri16Reg {
        <crate::RegValueT<CoexPri16Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CoexPri17Reg_SPEC;
impl crate::sealed::RegSpec for CoexPri17Reg_SPEC {
    type DataType = u16;
}

pub type CoexPri17Reg = crate::RegValueT<CoexPri17Reg_SPEC>;

impl CoexPri17Reg {
    #[inline(always)]
    pub fn coex_pri_mac(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, u8, CoexPri17Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x3,1,0,u8,u8,CoexPri17Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn coex_pri_pti(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, CoexPri17Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,CoexPri17Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CoexPri17Reg {
    #[inline(always)]
    fn default() -> CoexPri17Reg {
        <crate::RegValueT<CoexPri17Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CoexPri1Reg_SPEC;
impl crate::sealed::RegSpec for CoexPri1Reg_SPEC {
    type DataType = u16;
}

pub type CoexPri1Reg = crate::RegValueT<CoexPri1Reg_SPEC>;

impl CoexPri1Reg {
    #[inline(always)]
    pub fn coex_pri_mac(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, u8, CoexPri1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x3,1,0,u8,u8,CoexPri1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn coex_pri_pti(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, CoexPri1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,CoexPri1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CoexPri1Reg {
    #[inline(always)]
    fn default() -> CoexPri1Reg {
        <crate::RegValueT<CoexPri1Reg_SPEC> as RegisterValue<_>>::new(24)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CoexPri2Reg_SPEC;
impl crate::sealed::RegSpec for CoexPri2Reg_SPEC {
    type DataType = u16;
}

pub type CoexPri2Reg = crate::RegValueT<CoexPri2Reg_SPEC>;

impl CoexPri2Reg {
    #[inline(always)]
    pub fn coex_pri_mac(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, u8, CoexPri2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x3,1,0,u8,u8,CoexPri2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn coex_pri_pti(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, CoexPri2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,CoexPri2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CoexPri2Reg {
    #[inline(always)]
    fn default() -> CoexPri2Reg {
        <crate::RegValueT<CoexPri2Reg_SPEC> as RegisterValue<_>>::new(8)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CoexPri3Reg_SPEC;
impl crate::sealed::RegSpec for CoexPri3Reg_SPEC {
    type DataType = u16;
}

pub type CoexPri3Reg = crate::RegValueT<CoexPri3Reg_SPEC>;

impl CoexPri3Reg {
    #[inline(always)]
    pub fn coex_pri_mac(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, u8, CoexPri3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x3,1,0,u8,u8,CoexPri3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn coex_pri_pti(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, CoexPri3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,CoexPri3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CoexPri3Reg {
    #[inline(always)]
    fn default() -> CoexPri3Reg {
        <crate::RegValueT<CoexPri3Reg_SPEC> as RegisterValue<_>>::new(16)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CoexPri4Reg_SPEC;
impl crate::sealed::RegSpec for CoexPri4Reg_SPEC {
    type DataType = u16;
}

pub type CoexPri4Reg = crate::RegValueT<CoexPri4Reg_SPEC>;

impl CoexPri4Reg {
    #[inline(always)]
    pub fn coex_pri_mac(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, u8, CoexPri4Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x3,1,0,u8,u8,CoexPri4Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn coex_pri_pti(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, CoexPri4Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,CoexPri4Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CoexPri4Reg {
    #[inline(always)]
    fn default() -> CoexPri4Reg {
        <crate::RegValueT<CoexPri4Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CoexPri5Reg_SPEC;
impl crate::sealed::RegSpec for CoexPri5Reg_SPEC {
    type DataType = u16;
}

pub type CoexPri5Reg = crate::RegValueT<CoexPri5Reg_SPEC>;

impl CoexPri5Reg {
    #[inline(always)]
    pub fn coex_pri_mac(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, u8, CoexPri5Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x3,1,0,u8,u8,CoexPri5Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn coex_pri_pti(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, CoexPri5Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,CoexPri5Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CoexPri5Reg {
    #[inline(always)]
    fn default() -> CoexPri5Reg {
        <crate::RegValueT<CoexPri5Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CoexPri6Reg_SPEC;
impl crate::sealed::RegSpec for CoexPri6Reg_SPEC {
    type DataType = u16;
}

pub type CoexPri6Reg = crate::RegValueT<CoexPri6Reg_SPEC>;

impl CoexPri6Reg {
    #[inline(always)]
    pub fn coex_pri_mac(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, u8, CoexPri6Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x3,1,0,u8,u8,CoexPri6Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn coex_pri_pti(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, CoexPri6Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,CoexPri6Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CoexPri6Reg {
    #[inline(always)]
    fn default() -> CoexPri6Reg {
        <crate::RegValueT<CoexPri6Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CoexPri7Reg_SPEC;
impl crate::sealed::RegSpec for CoexPri7Reg_SPEC {
    type DataType = u16;
}

pub type CoexPri7Reg = crate::RegValueT<CoexPri7Reg_SPEC>;

impl CoexPri7Reg {
    #[inline(always)]
    pub fn coex_pri_mac(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, u8, CoexPri7Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x3,1,0,u8,u8,CoexPri7Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn coex_pri_pti(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, CoexPri7Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,CoexPri7Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CoexPri7Reg {
    #[inline(always)]
    fn default() -> CoexPri7Reg {
        <crate::RegValueT<CoexPri7Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CoexPri8Reg_SPEC;
impl crate::sealed::RegSpec for CoexPri8Reg_SPEC {
    type DataType = u16;
}

pub type CoexPri8Reg = crate::RegValueT<CoexPri8Reg_SPEC>;

impl CoexPri8Reg {
    #[inline(always)]
    pub fn coex_pri_mac(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, u8, CoexPri8Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x3,1,0,u8,u8,CoexPri8Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn coex_pri_pti(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, CoexPri8Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,CoexPri8Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CoexPri8Reg {
    #[inline(always)]
    fn default() -> CoexPri8Reg {
        <crate::RegValueT<CoexPri8Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CoexPri9Reg_SPEC;
impl crate::sealed::RegSpec for CoexPri9Reg_SPEC {
    type DataType = u16;
}

pub type CoexPri9Reg = crate::RegValueT<CoexPri9Reg_SPEC>;

impl CoexPri9Reg {
    #[inline(always)]
    pub fn coex_pri_mac(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, u8, CoexPri9Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x3,1,0,u8,u8,CoexPri9Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn coex_pri_pti(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, CoexPri9Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,CoexPri9Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for CoexPri9Reg {
    #[inline(always)]
    fn default() -> CoexPri9Reg {
        <crate::RegValueT<CoexPri9Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CoexStatReg_SPEC;
impl crate::sealed::RegSpec for CoexStatReg_SPEC {
    type DataType = u16;
}

pub type CoexStatReg = crate::RegValueT<CoexStatReg_SPEC>;

impl CoexStatReg {
    #[inline(always)]
    pub fn ignore_ble_stat(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, CoexStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15,1,0,CoexStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ignore_ftdf_stat(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, CoexStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14,1,0,CoexStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ignore_ext_stat(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, CoexStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13,1,0,CoexStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn coex_radio_busy(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, CoexStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12,1,0,CoexStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ext_act1(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, CoexStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11,1,0,CoexStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ext_act0(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, CoexStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,CoexStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn smart_pri(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, CoexStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9,1,0,CoexStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn smart_act(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, CoexStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,CoexStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn coex_decision_closing(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, CoexStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,CoexStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn coex_decision(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, u8, u8, CoexStatReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<5,0x3,1,0,u8,u8,CoexStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn coex_decision_ptr(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, CoexStatReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,CoexStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for CoexStatReg {
    #[inline(always)]
    fn default() -> CoexStatReg {
        <crate::RegValueT<CoexStatReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
