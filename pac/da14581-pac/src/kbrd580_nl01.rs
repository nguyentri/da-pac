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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:15:28 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"kbrd580_nl01 registers"]
unsafe impl ::core::marker::Send for super::Kbrd580Nl01 {}
unsafe impl ::core::marker::Sync for super::Kbrd580Nl01 {}
impl super::Kbrd580Nl01 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn gpio_debounce_reg(
        &self,
    ) -> &'static crate::common::Reg<self::GpioDebounceReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GpioDebounceReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gpio_int_level_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::GpioIntLevelCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GpioIntLevelCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gpio_irq0_in_sel_reg(
        &self,
    ) -> &'static crate::common::Reg<self::GpioIrq0InSelReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GpioIrq0InSelReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gpio_irq1_in_sel_reg(
        &self,
    ) -> &'static crate::common::Reg<self::GpioIrq1InSelReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GpioIrq1InSelReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gpio_irq2_in_sel_reg(
        &self,
    ) -> &'static crate::common::Reg<self::GpioIrq2InSelReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GpioIrq2InSelReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gpio_irq3_in_sel_reg(
        &self,
    ) -> &'static crate::common::Reg<self::GpioIrq3InSelReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GpioIrq3InSelReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gpio_irq4_in_sel_reg(
        &self,
    ) -> &'static crate::common::Reg<self::GpioIrq4InSelReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GpioIrq4InSelReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn gpio_reset_irq_reg(
        &self,
    ) -> &'static crate::common::Reg<self::GpioResetIrqReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GpioResetIrqReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(14usize),
            )
        }
    }

    #[inline(always)]
    pub const fn kbrd_irq_in_sel0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::KbrdIrqInSel0Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::KbrdIrqInSel0Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(18usize),
            )
        }
    }

    #[inline(always)]
    pub const fn kbrd_irq_in_sel1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::KbrdIrqInSel1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::KbrdIrqInSel1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[inline(always)]
    pub const fn kbrd_irq_in_sel2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::KbrdIrqInSel2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::KbrdIrqInSel2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(22usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpioDebounceReg_SPEC;
impl crate::sealed::RegSpec for GpioDebounceReg_SPEC {
    type DataType = u16;
}

pub type GpioDebounceReg = crate::RegValueT<GpioDebounceReg_SPEC>;

impl GpioDebounceReg {
    #[inline(always)]
    pub fn deb_enable_kbrd(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, GpioDebounceReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,GpioDebounceReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn deb_enable4(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, GpioDebounceReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,GpioDebounceReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn deb_enable3(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, GpioDebounceReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,GpioDebounceReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn deb_enable2(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, GpioDebounceReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,GpioDebounceReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn deb_enable1(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, GpioDebounceReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,GpioDebounceReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn deb_enable0(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, GpioDebounceReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,GpioDebounceReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn deb_value(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, GpioDebounceReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,GpioDebounceReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GpioDebounceReg {
    #[inline(always)]
    fn default() -> GpioDebounceReg {
        <crate::RegValueT<GpioDebounceReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpioIntLevelCtrlReg_SPEC;
impl crate::sealed::RegSpec for GpioIntLevelCtrlReg_SPEC {
    type DataType = u16;
}

pub type GpioIntLevelCtrlReg = crate::RegValueT<GpioIntLevelCtrlReg_SPEC>;

impl GpioIntLevelCtrlReg {
    #[inline(always)]
    pub fn edge_leveln4(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, GpioIntLevelCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<12,1,0,GpioIntLevelCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn edge_leveln3(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, GpioIntLevelCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<11,1,0,GpioIntLevelCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn edge_leveln2(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, GpioIntLevelCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<10,1,0,GpioIntLevelCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn edge_leveln1(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, GpioIntLevelCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<9,1,0,GpioIntLevelCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn edge_leveln0(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, GpioIntLevelCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<8,1,0,GpioIntLevelCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn input_level4(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, GpioIntLevelCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<4,1,0,GpioIntLevelCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn input_level3(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, GpioIntLevelCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<3,1,0,GpioIntLevelCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn input_level2(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, GpioIntLevelCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<2,1,0,GpioIntLevelCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn input_level1(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, GpioIntLevelCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<1,1,0,GpioIntLevelCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn input_level0(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, GpioIntLevelCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<0,1,0,GpioIntLevelCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GpioIntLevelCtrlReg {
    #[inline(always)]
    fn default() -> GpioIntLevelCtrlReg {
        <crate::RegValueT<GpioIntLevelCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpioIrq0InSelReg_SPEC;
impl crate::sealed::RegSpec for GpioIrq0InSelReg_SPEC {
    type DataType = u16;
}

pub type GpioIrq0InSelReg = crate::RegValueT<GpioIrq0InSelReg_SPEC>;

impl GpioIrq0InSelReg {
    #[inline(always)]
    pub fn kbrd_irq0_sel(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, GpioIrq0InSelReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x3f,
            1,
            0,
            u8,
            u8,
            GpioIrq0InSelReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for GpioIrq0InSelReg {
    #[inline(always)]
    fn default() -> GpioIrq0InSelReg {
        <crate::RegValueT<GpioIrq0InSelReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpioIrq1InSelReg_SPEC;
impl crate::sealed::RegSpec for GpioIrq1InSelReg_SPEC {
    type DataType = u16;
}

pub type GpioIrq1InSelReg = crate::RegValueT<GpioIrq1InSelReg_SPEC>;

impl GpioIrq1InSelReg {
    #[inline(always)]
    pub fn kbrd_irq1_sel(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, GpioIrq1InSelReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x3f,
            1,
            0,
            u8,
            u8,
            GpioIrq1InSelReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for GpioIrq1InSelReg {
    #[inline(always)]
    fn default() -> GpioIrq1InSelReg {
        <crate::RegValueT<GpioIrq1InSelReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpioIrq2InSelReg_SPEC;
impl crate::sealed::RegSpec for GpioIrq2InSelReg_SPEC {
    type DataType = u16;
}

pub type GpioIrq2InSelReg = crate::RegValueT<GpioIrq2InSelReg_SPEC>;

impl GpioIrq2InSelReg {
    #[inline(always)]
    pub fn kbrd_irq2_sel(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, GpioIrq2InSelReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x3f,
            1,
            0,
            u8,
            u8,
            GpioIrq2InSelReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for GpioIrq2InSelReg {
    #[inline(always)]
    fn default() -> GpioIrq2InSelReg {
        <crate::RegValueT<GpioIrq2InSelReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpioIrq3InSelReg_SPEC;
impl crate::sealed::RegSpec for GpioIrq3InSelReg_SPEC {
    type DataType = u16;
}

pub type GpioIrq3InSelReg = crate::RegValueT<GpioIrq3InSelReg_SPEC>;

impl GpioIrq3InSelReg {
    #[inline(always)]
    pub fn kbrd_irq3_sel(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, GpioIrq3InSelReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x3f,
            1,
            0,
            u8,
            u8,
            GpioIrq3InSelReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for GpioIrq3InSelReg {
    #[inline(always)]
    fn default() -> GpioIrq3InSelReg {
        <crate::RegValueT<GpioIrq3InSelReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpioIrq4InSelReg_SPEC;
impl crate::sealed::RegSpec for GpioIrq4InSelReg_SPEC {
    type DataType = u16;
}

pub type GpioIrq4InSelReg = crate::RegValueT<GpioIrq4InSelReg_SPEC>;

impl GpioIrq4InSelReg {
    #[inline(always)]
    pub fn kbrd_irq4_sel(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, GpioIrq4InSelReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x3f,
            1,
            0,
            u8,
            u8,
            GpioIrq4InSelReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for GpioIrq4InSelReg {
    #[inline(always)]
    fn default() -> GpioIrq4InSelReg {
        <crate::RegValueT<GpioIrq4InSelReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpioResetIrqReg_SPEC;
impl crate::sealed::RegSpec for GpioResetIrqReg_SPEC {
    type DataType = u16;
}

pub type GpioResetIrqReg = crate::RegValueT<GpioResetIrqReg_SPEC>;

impl GpioResetIrqReg {
    #[inline(always)]
    pub fn reset_kbrd_irq(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, GpioResetIrqReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5,1,0,GpioResetIrqReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reset_gpio4_irq(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, GpioResetIrqReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4,1,0,GpioResetIrqReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reset_gpio3_irq(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, GpioResetIrqReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3,1,0,GpioResetIrqReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reset_gpio2_irq(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, GpioResetIrqReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2,1,0,GpioResetIrqReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reset_gpio1_irq(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, GpioResetIrqReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1,1,0,GpioResetIrqReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reset_gpio0_irq(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, GpioResetIrqReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0,1,0,GpioResetIrqReg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for GpioResetIrqReg {
    #[inline(always)]
    fn default() -> GpioResetIrqReg {
        <crate::RegValueT<GpioResetIrqReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KbrdIrqInSel0Reg_SPEC;
impl crate::sealed::RegSpec for KbrdIrqInSel0Reg_SPEC {
    type DataType = u16;
}

pub type KbrdIrqInSel0Reg = crate::RegValueT<KbrdIrqInSel0Reg_SPEC>;

impl KbrdIrqInSel0Reg {
    #[inline(always)]
    pub fn kbrd_rel(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, KbrdIrqInSel0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,KbrdIrqInSel0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn kbrd_level(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, KbrdIrqInSel0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,KbrdIrqInSel0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn key_repeat(
        self,
    ) -> crate::common::RegisterField<8, 0x3f, 1, 0, u8, u8, KbrdIrqInSel0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            8,
            0x3f,
            1,
            0,
            u8,
            u8,
            KbrdIrqInSel0Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn kbrd_p07_en(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, KbrdIrqInSel0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,KbrdIrqInSel0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn kbrd_p06_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, KbrdIrqInSel0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,KbrdIrqInSel0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn kbrd_p05_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, KbrdIrqInSel0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,KbrdIrqInSel0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn kbrd_p04_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, KbrdIrqInSel0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,KbrdIrqInSel0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn kbrd_p03_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, KbrdIrqInSel0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,KbrdIrqInSel0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn kbrd_p02_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, KbrdIrqInSel0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,KbrdIrqInSel0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn kbrd_p01_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, KbrdIrqInSel0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,KbrdIrqInSel0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn kbrd_p00_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, KbrdIrqInSel0Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,KbrdIrqInSel0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for KbrdIrqInSel0Reg {
    #[inline(always)]
    fn default() -> KbrdIrqInSel0Reg {
        <crate::RegValueT<KbrdIrqInSel0Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KbrdIrqInSel1Reg_SPEC;
impl crate::sealed::RegSpec for KbrdIrqInSel1Reg_SPEC {
    type DataType = u16;
}

pub type KbrdIrqInSel1Reg = crate::RegValueT<KbrdIrqInSel1Reg_SPEC>;

impl KbrdIrqInSel1Reg {
    #[inline(always)]
    pub fn kbrd_p15_en(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, KbrdIrqInSel1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,KbrdIrqInSel1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn kbrd_p14_en(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, KbrdIrqInSel1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,KbrdIrqInSel1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn kbrd_p13_en(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, KbrdIrqInSel1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,KbrdIrqInSel1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn kbrd_p12_en(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, KbrdIrqInSel1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,KbrdIrqInSel1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn kbrd_p11_en(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, KbrdIrqInSel1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,KbrdIrqInSel1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn kbrd_p10_en(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, KbrdIrqInSel1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,KbrdIrqInSel1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn kbrd_p29_en(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, KbrdIrqInSel1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,KbrdIrqInSel1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn kbrd_p28_en(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, KbrdIrqInSel1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,KbrdIrqInSel1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn kbrd_p27_en(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, KbrdIrqInSel1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,KbrdIrqInSel1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn kbrd_p26_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, KbrdIrqInSel1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,KbrdIrqInSel1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn kbrd_p25_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, KbrdIrqInSel1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,KbrdIrqInSel1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn kbrd_p24_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, KbrdIrqInSel1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,KbrdIrqInSel1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn kbrd_p23_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, KbrdIrqInSel1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,KbrdIrqInSel1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn kbrd_p22_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, KbrdIrqInSel1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,KbrdIrqInSel1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn kbrd_p21_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, KbrdIrqInSel1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,KbrdIrqInSel1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn kbrd_p20_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, KbrdIrqInSel1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,KbrdIrqInSel1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for KbrdIrqInSel1Reg {
    #[inline(always)]
    fn default() -> KbrdIrqInSel1Reg {
        <crate::RegValueT<KbrdIrqInSel1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KbrdIrqInSel2Reg_SPEC;
impl crate::sealed::RegSpec for KbrdIrqInSel2Reg_SPEC {
    type DataType = u16;
}

pub type KbrdIrqInSel2Reg = crate::RegValueT<KbrdIrqInSel2Reg_SPEC>;

impl KbrdIrqInSel2Reg {
    #[inline(always)]
    pub fn kbrd_p37_en(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, KbrdIrqInSel2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,KbrdIrqInSel2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn kbrd_p36_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, KbrdIrqInSel2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,KbrdIrqInSel2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn kbrd_p35_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, KbrdIrqInSel2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,KbrdIrqInSel2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn kbrd_p34_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, KbrdIrqInSel2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,KbrdIrqInSel2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn kbrd_p33_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, KbrdIrqInSel2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,KbrdIrqInSel2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn kbrd_p32_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, KbrdIrqInSel2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,KbrdIrqInSel2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn kbrd_p31_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, KbrdIrqInSel2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,KbrdIrqInSel2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn kbrd_p30_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, KbrdIrqInSel2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,KbrdIrqInSel2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for KbrdIrqInSel2Reg {
    #[inline(always)]
    fn default() -> KbrdIrqInSel2Reg {
        <crate::RegValueT<KbrdIrqInSel2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}
