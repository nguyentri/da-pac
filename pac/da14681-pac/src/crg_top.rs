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
#[doc = r"CRG_TOP registers"]
unsafe impl ::core::marker::Send for super::CrgTop {}
unsafe impl ::core::marker::Sync for super::CrgTop {}
impl super::CrgTop {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn ana_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::AnaStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::AnaStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(42usize),
            )
        }
    }

    #[inline(always)]
    pub const fn bandgap_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BandgapReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BandgapReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[inline(always)]
    pub const fn bod_ctrl2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BodCtrl2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BodCtrl2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(54usize),
            )
        }
    }

    #[inline(always)]
    pub const fn bod_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BodCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BodCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[inline(always)]
    pub const fn bod_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BodStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BodStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[inline(always)]
    pub const fn clk_16m_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Clk16MReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Clk16MReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(34usize),
            )
        }
    }

    #[inline(always)]
    pub const fn clk_32k_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Clk32KReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Clk32KReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[inline(always)]
    pub const fn clk_amba_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ClkAmbaReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ClkAmbaReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn clk_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ClkCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ClkCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(10usize),
            )
        }
    }

    #[inline(always)]
    pub const fn clk_freq_trim_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ClkFreqTrimReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ClkFreqTrimReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[inline(always)]
    pub const fn clk_radio_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ClkRadioReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ClkRadioReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn clk_rcx20k_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ClkRcx20KReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ClkRcx20KReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[inline(always)]
    pub const fn clk_tmr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ClkTmrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ClkTmrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ldo_ctrl1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LdoCtrl1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LdoCtrl1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(58usize),
            )
        }
    }

    #[inline(always)]
    pub const fn ldo_ctrl2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LdoCtrl2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LdoCtrl2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pmu_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PmuCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PmuCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn sleep_timer_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SleepTimerReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SleepTimerReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(62usize),
            )
        }
    }

    #[inline(always)]
    pub const fn sys_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SysCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SysCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(18usize),
            )
        }
    }

    #[inline(always)]
    pub const fn sys_stat_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SysStatReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SysStatReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[inline(always)]
    pub const fn vbus_irq_clear_reg(
        &self,
    ) -> &'static crate::common::Reg<self::VbusIrqClearReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::VbusIrqClearReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(50usize),
            )
        }
    }

    #[inline(always)]
    pub const fn vbus_irq_mask_reg(
        &self,
    ) -> &'static crate::common::Reg<self::VbusIrqMaskReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::VbusIrqMaskReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[inline(always)]
    pub const fn xtalrdy_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::XtalrdyCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::XtalrdyCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AnaStatusReg_SPEC;
impl crate::sealed::RegSpec for AnaStatusReg_SPEC {
    type DataType = u16;
}

pub type AnaStatusReg = crate::RegValueT<AnaStatusReg_SPEC>;

impl AnaStatusReg {
    #[inline(always)]
    pub fn comp_1v8_pa_high(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn comp_1v8_flash_high(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn comp_v33_high(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn comp_vbus_low(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn comp_vbus_high(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_1v8_flash_ok(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_1v8_pa_ok(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_core_ok(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn comp_vdd_high(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bandgap_ok(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_supply_usb_ok(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_supply_vbat_ok(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn newbat(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vbus_available(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn comp_vbat_ok(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_radio_ok(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, AnaStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,AnaStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for AnaStatusReg {
    #[inline(always)]
    fn default() -> AnaStatusReg {
        <crate::RegValueT<AnaStatusReg_SPEC> as RegisterValue<_>>::new(144)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BandgapReg_SPEC;
impl crate::sealed::RegSpec for BandgapReg_SPEC {
    type DataType = u16;
}

pub type BandgapReg = crate::RegValueT<BandgapReg_SPEC>;

impl BandgapReg {
    #[inline(always)]
    pub fn bypass_cold_boot_disable(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, BandgapReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,BandgapReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_sleep_trim(
        self,
    ) -> crate::common::RegisterField<10, 0xf, 1, 0, u8, u8, BandgapReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0xf,1,0,u8,u8,BandgapReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bgr_itrim(
        self,
    ) -> crate::common::RegisterField<5, 0x1f, 1, 0, u8, u8, BandgapReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1f,1,0,u8,u8,BandgapReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bgr_trim(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, BandgapReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,BandgapReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BandgapReg {
    #[inline(always)]
    fn default() -> BandgapReg {
        <crate::RegValueT<BandgapReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BodCtrl2Reg_SPEC;
impl crate::sealed::RegSpec for BodCtrl2Reg_SPEC {
    type DataType = u16;
}

pub type BodCtrl2Reg = crate::RegValueT<BodCtrl2Reg_SPEC>;

impl BodCtrl2Reg {
    #[inline(always)]
    pub fn bod_vbat_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, BodCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,BodCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bod_1v8_flash_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, BodCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,BodCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bod_1v8_pa_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, BodCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,BodCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bod_v33_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, BodCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,BodCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bod_vdd_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, BodCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,BodCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bod_reset_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, BodCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,BodCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BodCtrl2Reg {
    #[inline(always)]
    fn default() -> BodCtrl2Reg {
        <crate::RegValueT<BodCtrl2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BodCtrlReg_SPEC;
impl crate::sealed::RegSpec for BodCtrlReg_SPEC {
    type DataType = u16;
}

pub type BodCtrlReg = crate::RegValueT<BodCtrlReg_SPEC>;

impl BodCtrlReg {
    #[inline(always)]
    pub fn bod_vdd_lvl(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, u8, BodCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,BodCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bod_v33_trim(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, BodCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,BodCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bod_1v8_pa_trim(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, BodCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,BodCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bod_1v8_flash_trim(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, u8, BodCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3,1,0,u8,u8,BodCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bod_vdd_trim(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, BodCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,BodCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BodCtrlReg {
    #[inline(always)]
    fn default() -> BodCtrlReg {
        <crate::RegValueT<BodCtrlReg_SPEC> as RegisterValue<_>>::new(1877)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BodStatusReg_SPEC;
impl crate::sealed::RegSpec for BodStatusReg_SPEC {
    type DataType = u16;
}

pub type BodStatusReg = crate::RegValueT<BodStatusReg_SPEC>;

impl BodStatusReg {
    #[inline(always)]
    pub fn bod_vbat_low(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, BodStatusReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,BodStatusReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bod_v33_low(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, BodStatusReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,BodStatusReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bod_1v8_flash_low(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, BodStatusReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,BodStatusReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bod_1v8_pa_low(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, BodStatusReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,BodStatusReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bod_vdd_low(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, BodStatusReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,BodStatusReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BodStatusReg {
    #[inline(always)]
    fn default() -> BodStatusReg {
        <crate::RegValueT<BodStatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clk16MReg_SPEC;
impl crate::sealed::RegSpec for Clk16MReg_SPEC {
    type DataType = u16;
}

pub type Clk16MReg = crate::RegValueT<Clk16MReg_SPEC>;

impl Clk16MReg {
    #[inline(always)]
    pub fn rc16m_startup_disable(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Clk16MReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,Clk16MReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn xtal16_hpass_flt_en(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Clk16MReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,Clk16MReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn xtal16_spike_flt_bypass(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Clk16MReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,Clk16MReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn xtal16_amp_trim(
        self,
    ) -> crate::common::RegisterField<10, 0x7, 1, 0, u8, u8, Clk16MReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x7,1,0,u8,u8,Clk16MReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn xtal16_ext_clk_enable(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Clk16MReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,Clk16MReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn xtal16_max_current(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Clk16MReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,Clk16MReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn xtal16_cur_set(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, u8, Clk16MReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x7,1,0,u8,u8,Clk16MReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rc16m_trim(
        self,
    ) -> crate::common::RegisterField<1, 0xf, 1, 0, u8, u8, Clk16MReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0xf,1,0,u8,u8,Clk16MReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rc16m_enable(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Clk16MReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Clk16MReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Clk16MReg {
    #[inline(always)]
    fn default() -> Clk16MReg {
        <crate::RegValueT<Clk16MReg_SPEC> as RegisterValue<_>>::new(3072)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clk32KReg_SPEC;
impl crate::sealed::RegSpec for Clk32KReg_SPEC {
    type DataType = u16;
}

pub type Clk32KReg = crate::RegValueT<Clk32KReg_SPEC>;

impl Clk32KReg {
    #[inline(always)]
    pub fn xtal32k_disable_ampreg(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Clk32KReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,Clk32KReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rc32k_trim(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, Clk32KReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8,u8,Clk32KReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rc32k_enable(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Clk32KReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,Clk32KReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn xtal32k_cur(
        self,
    ) -> crate::common::RegisterField<3, 0xf, 1, 0, u8, u8, Clk32KReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0xf,1,0,u8,u8,Clk32KReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn xtal32k_rbias(
        self,
    ) -> crate::common::RegisterField<1, 0x3, 1, 0, u8, u8, Clk32KReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x3,1,0,u8,u8,Clk32KReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn xtal32k_enable(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Clk32KReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Clk32KReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Clk32KReg {
    #[inline(always)]
    fn default() -> Clk32KReg {
        <crate::RegValueT<Clk32KReg_SPEC> as RegisterValue<_>>::new(1948)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkAmbaReg_SPEC;
impl crate::sealed::RegSpec for ClkAmbaReg_SPEC {
    type DataType = u16;
}

pub type ClkAmbaReg = crate::RegValueT<ClkAmbaReg_SPEC>;

impl ClkAmbaReg {
    #[inline(always)]
    pub fn qspi_enable(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, ClkAmbaReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,ClkAmbaReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn qspi_div(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, u8, ClkAmbaReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,u8,u8,ClkAmbaReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn otp_enable(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, ClkAmbaReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,ClkAmbaReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn trng_clk_enable(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, ClkAmbaReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,ClkAmbaReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ecc_clk_enable(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, ClkAmbaReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,ClkAmbaReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn aes_clk_enable(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, ClkAmbaReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,ClkAmbaReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pclk_div(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, ClkAmbaReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,ClkAmbaReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn hclk_div(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, ClkAmbaReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,ClkAmbaReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ClkAmbaReg {
    #[inline(always)]
    fn default() -> ClkAmbaReg {
        <crate::RegValueT<ClkAmbaReg_SPEC> as RegisterValue<_>>::new(34)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkCtrlReg_SPEC;
impl crate::sealed::RegSpec for ClkCtrlReg_SPEC {
    type DataType = u16;
}

pub type ClkCtrlReg = crate::RegValueT<ClkCtrlReg_SPEC>;

impl ClkCtrlReg {
    #[inline(always)]
    pub fn running_at_pll96m(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, ClkCtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15,1,0,ClkCtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn running_at_xtal16m(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, ClkCtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14,1,0,ClkCtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn running_at_rc16m(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, ClkCtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13,1,0,ClkCtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn running_at_32k(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, ClkCtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12,1,0,ClkCtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn clk32k_source(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, ClkCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,ClkCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn divn_sync_level(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, ClkCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,ClkCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn divn_xtal32m_mode(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, ClkCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,ClkCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pll_div2(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, ClkCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,ClkCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn usb_clk_src(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, ClkCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,ClkCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn xtal32m_mode(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, ClkCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,ClkCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn xtal16m_disable(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ClkCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,ClkCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sys_clk_sel(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, ClkCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,ClkCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ClkCtrlReg {
    #[inline(always)]
    fn default() -> ClkCtrlReg {
        <crate::RegValueT<ClkCtrlReg_SPEC> as RegisterValue<_>>::new(8193)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkFreqTrimReg_SPEC;
impl crate::sealed::RegSpec for ClkFreqTrimReg_SPEC {
    type DataType = u16;
}

pub type ClkFreqTrimReg = crate::RegValueT<ClkFreqTrimReg_SPEC>;

impl ClkFreqTrimReg {
    #[inline(always)]
    pub fn coarse_adj(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, u8, ClkFreqTrimReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,ClkFreqTrimReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn fine_adj(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, ClkFreqTrimReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,ClkFreqTrimReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ClkFreqTrimReg {
    #[inline(always)]
    fn default() -> ClkFreqTrimReg {
        <crate::RegValueT<ClkFreqTrimReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkRadioReg_SPEC;
impl crate::sealed::RegSpec for ClkRadioReg_SPEC {
    type DataType = u16;
}

pub type ClkRadioReg = crate::RegValueT<ClkRadioReg_SPEC>;

impl ClkRadioReg {
    #[inline(always)]
    pub fn ftdf_mac_enable(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, ClkRadioReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,ClkRadioReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ftdf_mac_div(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, ClkRadioReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,ClkRadioReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ble_enable(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, ClkRadioReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,ClkRadioReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ble_lp_reset(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, ClkRadioReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,ClkRadioReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ble_div(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, ClkRadioReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,ClkRadioReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfcu_enable(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, ClkRadioReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,ClkRadioReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfcu_div(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, ClkRadioReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,ClkRadioReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ClkRadioReg {
    #[inline(always)]
    fn default() -> ClkRadioReg {
        <crate::RegValueT<ClkRadioReg_SPEC> as RegisterValue<_>>::new(64)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkRcx20KReg_SPEC;
impl crate::sealed::RegSpec for ClkRcx20KReg_SPEC {
    type DataType = u16;
}

pub type ClkRcx20KReg = crate::RegValueT<ClkRcx20KReg_SPEC>;

impl ClkRcx20KReg {
    #[inline(always)]
    pub fn rcx20k_enable(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, ClkRcx20KReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,ClkRcx20KReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rcx20k_lowf(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, ClkRcx20KReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,ClkRcx20KReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rcx20k_bias(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, ClkRcx20KReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,ClkRcx20KReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rcx20k_ntc(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, ClkRcx20KReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,ClkRcx20KReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rcx20k_trim(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, ClkRcx20KReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,ClkRcx20KReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ClkRcx20KReg {
    #[inline(always)]
    fn default() -> ClkRcx20KReg {
        <crate::RegValueT<ClkRcx20KReg_SPEC> as RegisterValue<_>>::new(376)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkTmrReg_SPEC;
impl crate::sealed::RegSpec for ClkTmrReg_SPEC {
    type DataType = u16;
}

pub type ClkTmrReg = crate::RegValueT<ClkTmrReg_SPEC>;

impl ClkTmrReg {
    #[inline(always)]
    pub fn p06_tmr1_pwm_mode(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, ClkTmrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,ClkTmrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn wakeupct_enable(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, ClkTmrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,ClkTmrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn breath_enable(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, ClkTmrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,ClkTmrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmr2_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, ClkTmrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,ClkTmrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmr2_enable(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, ClkTmrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,ClkTmrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmr2_div(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, ClkTmrReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,ClkTmrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmr1_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, ClkTmrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,ClkTmrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmr1_enable(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, ClkTmrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,ClkTmrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmr1_div(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, ClkTmrReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,ClkTmrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmr0_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, ClkTmrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,ClkTmrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmr0_enable(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ClkTmrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,ClkTmrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmr0_div(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, ClkTmrReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,ClkTmrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ClkTmrReg {
    #[inline(always)]
    fn default() -> ClkTmrReg {
        <crate::RegValueT<ClkTmrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LdoCtrl1Reg_SPEC;
impl crate::sealed::RegSpec for LdoCtrl1Reg_SPEC {
    type DataType = u16;
}

pub type LdoCtrl1Reg = crate::RegValueT<LdoCtrl1Reg_SPEC>;

impl LdoCtrl1Reg {
    #[inline(always)]
    pub fn ldo_radio_enable(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, LdoCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,LdoCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_radio_setvdd(
        self,
    ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, u8, LdoCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x7,1,0,u8,u8,LdoCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_core_setvdd(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, u8, LdoCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,LdoCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_supply_usb_level(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, LdoCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,LdoCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_supply_vbat_level(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, LdoCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,LdoCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_vbat_ret_level(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, u8, LdoCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3,1,0,u8,u8,LdoCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_core_curlim(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, LdoCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,LdoCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for LdoCtrl1Reg {
    #[inline(always)]
    fn default() -> LdoCtrl1Reg {
        <crate::RegValueT<LdoCtrl1Reg_SPEC> as RegisterValue<_>>::new(87)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LdoCtrl2Reg_SPEC;
impl crate::sealed::RegSpec for LdoCtrl2Reg_SPEC {
    type DataType = u16;
}

pub type LdoCtrl2Reg = crate::RegValueT<LdoCtrl2Reg_SPEC>;

impl LdoCtrl2Reg {
    #[inline(always)]
    pub fn ldo_1v8_pa_ret_disable(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, LdoCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,LdoCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_1v8_flash_ret_disable(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, LdoCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,LdoCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_vbat_ret_disable(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, LdoCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,LdoCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_1v8_pa_on(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, LdoCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,LdoCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_1v8_flash_on(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, LdoCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,LdoCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_3v3_on(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, LdoCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,LdoCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_1v2_on(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, LdoCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,LdoCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for LdoCtrl2Reg {
    #[inline(always)]
    fn default() -> LdoCtrl2Reg {
        <crate::RegValueT<LdoCtrl2Reg_SPEC> as RegisterValue<_>>::new(15)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PmuCtrlReg_SPEC;
impl crate::sealed::RegSpec for PmuCtrlReg_SPEC {
    type DataType = u16;
}

pub type PmuCtrlReg = crate::RegValueT<PmuCtrlReg_SPEC>;

impl PmuCtrlReg {
    #[inline(always)]
    pub fn retain_eccram(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn retain_cache(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn enable_clkless(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn retain_ram(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, u8, PmuCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1f,1,0,u8,u8,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn otp_copy_div(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, PmuCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn reset_on_wakeup(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ftdf_sleep(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ble_sleep(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn radio_sleep(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn periph_sleep(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, PmuCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,PmuCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PmuCtrlReg {
    #[inline(always)]
    fn default() -> PmuCtrlReg {
        <crate::RegValueT<PmuCtrlReg_SPEC> as RegisterValue<_>>::new(15)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SleepTimerReg_SPEC;
impl crate::sealed::RegSpec for SleepTimerReg_SPEC {
    type DataType = u16;
}

pub type SleepTimerReg = crate::RegValueT<SleepTimerReg_SPEC>;

impl SleepTimerReg {
    #[inline(always)]
    pub fn sleep_timer(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        SleepTimerReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            SleepTimerReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SleepTimerReg {
    #[inline(always)]
    fn default() -> SleepTimerReg {
        <crate::RegValueT<SleepTimerReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SysCtrlReg_SPEC;
impl crate::sealed::RegSpec for SysCtrlReg_SPEC {
    type DataType = u16;
}

pub type SysCtrlReg = crate::RegValueT<SysCtrlReg_SPEC>;

impl SysCtrlReg {
    #[inline(always)]
    pub fn sw_reset(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, SysCtrlReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<15,1,0,SysCtrlReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn remap_intvect(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, SysCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,SysCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn otp_copy(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, SysCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,SysCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn qspi_init(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, SysCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,SysCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dev_phase(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, SysCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,SysCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cacheram_mux(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, SysCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,SysCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn timeout_disable(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, SysCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,SysCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dra_off(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, SysCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,SysCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn debugger_enable(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, SysCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,SysCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn otpc_reset_req(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, SysCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,SysCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pad_latch_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, SysCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,SysCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn remap_rams(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, u8, SysCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x3,1,0,u8,u8,SysCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn remap_adr0(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, SysCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,SysCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for SysCtrlReg {
    #[inline(always)]
    fn default() -> SysCtrlReg {
        <crate::RegValueT<SysCtrlReg_SPEC> as RegisterValue<_>>::new(32)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SysStatReg_SPEC;
impl crate::sealed::RegSpec for SysStatReg_SPEC {
    type DataType = u16;
}

pub type SysStatReg = crate::RegValueT<SysStatReg_SPEC>;

impl SysStatReg {
    #[inline(always)]
    pub fn ftdf_is_up(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ftdf_is_down(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ble_is_up(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ble_is_down(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn xtal16_trim_ready(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dbg_is_active(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn per_is_up(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn per_is_down(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rad_is_up(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rad_is_down(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, SysStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,SysStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for SysStatReg {
    #[inline(always)]
    fn default() -> SysStatReg {
        <crate::RegValueT<SysStatReg_SPEC> as RegisterValue<_>>::new(1349)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VbusIrqClearReg_SPEC;
impl crate::sealed::RegSpec for VbusIrqClearReg_SPEC {
    type DataType = u16;
}

pub type VbusIrqClearReg = crate::RegValueT<VbusIrqClearReg_SPEC>;

impl VbusIrqClearReg {
    #[inline(always)]
    pub fn vbus_irq_clear(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        VbusIrqClearReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            VbusIrqClearReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for VbusIrqClearReg {
    #[inline(always)]
    fn default() -> VbusIrqClearReg {
        <crate::RegValueT<VbusIrqClearReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VbusIrqMaskReg_SPEC;
impl crate::sealed::RegSpec for VbusIrqMaskReg_SPEC {
    type DataType = u16;
}

pub type VbusIrqMaskReg = crate::RegValueT<VbusIrqMaskReg_SPEC>;

impl VbusIrqMaskReg {
    #[inline(always)]
    pub fn vbus_irq_en_rise(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, VbusIrqMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,VbusIrqMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vbus_irq_en_fall(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, VbusIrqMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,VbusIrqMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for VbusIrqMaskReg {
    #[inline(always)]
    fn default() -> VbusIrqMaskReg {
        <crate::RegValueT<VbusIrqMaskReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct XtalrdyCtrlReg_SPEC;
impl crate::sealed::RegSpec for XtalrdyCtrlReg_SPEC {
    type DataType = u16;
}

pub type XtalrdyCtrlReg = crate::RegValueT<XtalrdyCtrlReg_SPEC>;

impl XtalrdyCtrlReg {
    #[inline(always)]
    pub fn xtalrdy_cnt(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, XtalrdyCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,XtalrdyCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for XtalrdyCtrlReg {
    #[inline(always)]
    fn default() -> XtalrdyCtrlReg {
        <crate::RegValueT<XtalrdyCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
