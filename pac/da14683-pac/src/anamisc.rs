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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:16:08 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"ANAMISC registers"]
unsafe impl ::core::marker::Send for super::Anamisc {}
unsafe impl ::core::marker::Sync for super::Anamisc {}
impl super::Anamisc {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn charger_ctrl1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChargerCtrl1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChargerCtrl1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn charger_ctrl2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChargerCtrl2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChargerCtrl2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(10usize),
            )
        }
    }

    #[inline(always)]
    pub const fn charger_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ChargerStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ChargerStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn clk_ref_cnt_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ClkRefCntReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ClkRefCntReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(98usize),
            )
        }
    }

    #[inline(always)]
    pub const fn clk_ref_sel_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ClkRefSelReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ClkRefSelReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(96usize),
            )
        }
    }

    #[inline(always)]
    pub const fn clk_ref_val_h_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ClkRefValHReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ClkRefValHReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(102usize),
            )
        }
    }

    #[inline(always)]
    pub const fn clk_ref_val_l_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ClkRefValLReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ClkRefValLReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(100usize),
            )
        }
    }

    #[inline(always)]
    pub const fn soc_add2ch_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SocAdd2ChReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SocAdd2ChReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(70usize),
            )
        }
    }

    #[inline(always)]
    pub const fn soc_charge_avg_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SocChargeAvgReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SocChargeAvgReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }

    #[inline(always)]
    pub const fn soc_charge_cntr1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SocChargeCntr1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SocChargeCntr1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(72usize),
            )
        }
    }

    #[inline(always)]
    pub const fn soc_charge_cntr2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SocChargeCntr2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SocChargeCntr2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(74usize),
            )
        }
    }

    #[inline(always)]
    pub const fn soc_charge_cntr3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SocChargeCntr3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SocChargeCntr3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(76usize),
            )
        }
    }

    #[inline(always)]
    pub const fn soc_ctrl1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SocCtrl1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SocCtrl1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[inline(always)]
    pub const fn soc_ctrl2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SocCtrl2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SocCtrl2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(66usize),
            )
        }
    }

    #[inline(always)]
    pub const fn soc_ctrl3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SocCtrl3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SocCtrl3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(68usize),
            )
        }
    }

    #[inline(always)]
    pub const fn soc_ext_in_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SocExtInReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SocExtInReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(84usize),
            )
        }
    }

    #[inline(always)]
    pub const fn soc_ext_out_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SocExtOutReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SocExtOutReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(86usize),
            )
        }
    }

    #[inline(always)]
    pub const fn soc_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SocStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SocStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(82usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChargerCtrl1Reg_SPEC;
impl crate::sealed::RegSpec for ChargerCtrl1Reg_SPEC {
    type DataType = u16;
}

pub type ChargerCtrl1Reg = crate::RegValueT<ChargerCtrl1Reg_SPEC>;

impl ChargerCtrl1Reg {
    #[inline(always)]
    pub fn die_temp_disable(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, ChargerCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,ChargerCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn die_temp_set(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, u8, ChargerCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,u8,u8,ChargerCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn charge_cur(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, ChargerCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xf,1,0,u8,u8,ChargerCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ntc_low_disable(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, ChargerCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,ChargerCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ntc_disable(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, ChargerCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,ChargerCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn charge_on(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, ChargerCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,ChargerCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn charge_level(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, ChargerCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,ChargerCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ChargerCtrl1Reg {
    #[inline(always)]
    fn default() -> ChargerCtrl1Reg {
        <crate::RegValueT<ChargerCtrl1Reg_SPEC> as RegisterValue<_>>::new(8192)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChargerCtrl2Reg_SPEC;
impl crate::sealed::RegSpec for ChargerCtrl2Reg_SPEC {
    type DataType = u16;
}

pub type ChargerCtrl2Reg = crate::RegValueT<ChargerCtrl2Reg_SPEC>;

impl ChargerCtrl2Reg {
    #[inline(always)]
    pub fn charger_test(
        self,
    ) -> crate::common::RegisterField<13, 0x7, 1, 0, u8, u8, ChargerCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x7,1,0,u8,u8,ChargerCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn current_offset_trim(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, u8, ChargerCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1f,1,0,u8,u8,ChargerCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn charger_vfloat_adj(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, ChargerCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,ChargerCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn current_gain_trim(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, ChargerCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,ChargerCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ChargerCtrl2Reg {
    #[inline(always)]
    fn default() -> ChargerCtrl2Reg {
        <crate::RegValueT<ChargerCtrl2Reg_SPEC> as RegisterValue<_>>::new(3847)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChargerStatusReg_SPEC;
impl crate::sealed::RegSpec for ChargerStatusReg_SPEC {
    type DataType = u16;
}

pub type ChargerStatusReg = crate::RegValueT<ChargerStatusReg_SPEC>;

impl ChargerStatusReg {
    #[inline(always)]
    pub fn charger_tmode_prot(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, ChargerStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,ChargerStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn charger_battemp_high(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, ChargerStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,ChargerStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn charger_battemp_ok(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, ChargerStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,ChargerStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn charger_battemp_low(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, ChargerStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,ChargerStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn end_of_charge(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ChargerStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,ChargerStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn charger_cv_mode(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, ChargerStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,ChargerStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn charger_cc_mode(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ChargerStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,ChargerStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for ChargerStatusReg {
    #[inline(always)]
    fn default() -> ChargerStatusReg {
        <crate::RegValueT<ChargerStatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkRefCntReg_SPEC;
impl crate::sealed::RegSpec for ClkRefCntReg_SPEC {
    type DataType = u16;
}

pub type ClkRefCntReg = crate::RegValueT<ClkRefCntReg_SPEC>;

impl ClkRefCntReg {
    #[inline(always)]
    pub fn ref_cnt_val(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, ClkRefCntReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            ClkRefCntReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for ClkRefCntReg {
    #[inline(always)]
    fn default() -> ClkRefCntReg {
        <crate::RegValueT<ClkRefCntReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkRefSelReg_SPEC;
impl crate::sealed::RegSpec for ClkRefSelReg_SPEC {
    type DataType = u16;
}

pub type ClkRefSelReg = crate::RegValueT<ClkRefSelReg_SPEC>;

impl ClkRefSelReg {
    #[inline(always)]
    pub fn ref_cal_start(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ClkRefSelReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,ClkRefSelReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ref_clk_sel(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, ClkRefSelReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,ClkRefSelReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ClkRefSelReg {
    #[inline(always)]
    fn default() -> ClkRefSelReg {
        <crate::RegValueT<ClkRefSelReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkRefValHReg_SPEC;
impl crate::sealed::RegSpec for ClkRefValHReg_SPEC {
    type DataType = u16;
}

pub type ClkRefValHReg = crate::RegValueT<ClkRefValHReg_SPEC>;

impl ClkRefValHReg {
    #[inline(always)]
    pub fn xtal_cnt_val(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, ClkRefValHReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            ClkRefValHReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for ClkRefValHReg {
    #[inline(always)]
    fn default() -> ClkRefValHReg {
        <crate::RegValueT<ClkRefValHReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkRefValLReg_SPEC;
impl crate::sealed::RegSpec for ClkRefValLReg_SPEC {
    type DataType = u16;
}

pub type ClkRefValLReg = crate::RegValueT<ClkRefValLReg_SPEC>;

impl ClkRefValLReg {
    #[inline(always)]
    pub fn xtal_cnt_val(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, ClkRefValLReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            ClkRefValLReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for ClkRefValLReg {
    #[inline(always)]
    fn default() -> ClkRefValLReg {
        <crate::RegValueT<ClkRefValLReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SocAdd2ChReg_SPEC;
impl crate::sealed::RegSpec for SocAdd2ChReg_SPEC {
    type DataType = u16;
}

pub type SocAdd2ChReg = crate::RegValueT<SocAdd2ChReg_SPEC>;

impl SocAdd2ChReg {
    #[inline(always)]
    pub fn soc_add2ch(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, SocAdd2ChReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            SocAdd2ChReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SocAdd2ChReg {
    #[inline(always)]
    fn default() -> SocAdd2ChReg {
        <crate::RegValueT<SocAdd2ChReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SocChargeAvgReg_SPEC;
impl crate::sealed::RegSpec for SocChargeAvgReg_SPEC {
    type DataType = u16;
}

pub type SocChargeAvgReg = crate::RegValueT<SocChargeAvgReg_SPEC>;

impl SocChargeAvgReg {
    #[inline(always)]
    pub fn charge_avg(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        SocChargeAvgReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            SocChargeAvgReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SocChargeAvgReg {
    #[inline(always)]
    fn default() -> SocChargeAvgReg {
        <crate::RegValueT<SocChargeAvgReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SocChargeCntr1Reg_SPEC;
impl crate::sealed::RegSpec for SocChargeCntr1Reg_SPEC {
    type DataType = u16;
}

pub type SocChargeCntr1Reg = crate::RegValueT<SocChargeCntr1Reg_SPEC>;

impl SocChargeCntr1Reg {
    #[inline(always)]
    pub fn charge_cnt1(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        SocChargeCntr1Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            SocChargeCntr1Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SocChargeCntr1Reg {
    #[inline(always)]
    fn default() -> SocChargeCntr1Reg {
        <crate::RegValueT<SocChargeCntr1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SocChargeCntr2Reg_SPEC;
impl crate::sealed::RegSpec for SocChargeCntr2Reg_SPEC {
    type DataType = u16;
}

pub type SocChargeCntr2Reg = crate::RegValueT<SocChargeCntr2Reg_SPEC>;

impl SocChargeCntr2Reg {
    #[inline(always)]
    pub fn charge_cnt2(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        SocChargeCntr2Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            SocChargeCntr2Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SocChargeCntr2Reg {
    #[inline(always)]
    fn default() -> SocChargeCntr2Reg {
        <crate::RegValueT<SocChargeCntr2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SocChargeCntr3Reg_SPEC;
impl crate::sealed::RegSpec for SocChargeCntr3Reg_SPEC {
    type DataType = u16;
}

pub type SocChargeCntr3Reg = crate::RegValueT<SocChargeCntr3Reg_SPEC>;

impl SocChargeCntr3Reg {
    #[inline(always)]
    pub fn charge_cnt3(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, SocChargeCntr3Reg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            SocChargeCntr3Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SocChargeCntr3Reg {
    #[inline(always)]
    fn default() -> SocChargeCntr3Reg {
        <crate::RegValueT<SocChargeCntr3Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SocCtrl1Reg_SPEC;
impl crate::sealed::RegSpec for SocCtrl1Reg_SPEC {
    type DataType = u16;
}

pub type SocCtrl1Reg = crate::RegValueT<SocCtrl1Reg_SPEC>;

impl SocCtrl1Reg {
    #[inline(always)]
    pub fn soc_cint(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, u8, SocCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x3,1,0,u8,u8,SocCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn soc_bias(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, u8, SocCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,u8,u8,SocCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn soc_clk(
        self,
    ) -> crate::common::RegisterField<9, 0x7, 1, 0, u8, u8, SocCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x7,1,0,u8,u8,SocCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn soc_lpf(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, SocCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,SocCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn soc_idac(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, SocCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,SocCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn soc_sign(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, SocCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,SocCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn soc_gpio(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, SocCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,SocCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn soc_mute(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, SocCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,SocCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn soc_reset_avg(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, SocCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,SocCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn soc_reset_charge(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, SocCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,SocCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn soc_enable(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, SocCtrl1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,SocCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for SocCtrl1Reg {
    #[inline(always)]
    fn default() -> SocCtrl1Reg {
        <crate::RegValueT<SocCtrl1Reg_SPEC> as RegisterValue<_>>::new(55424)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SocCtrl2Reg_SPEC;
impl crate::sealed::RegSpec for SocCtrl2Reg_SPEC {
    type DataType = u16;
}

pub type SocCtrl2Reg = crate::RegValueT<SocCtrl2Reg_SPEC>;

impl SocCtrl2Reg {
    #[inline(always)]
    pub fn soc_dynavg(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, SocCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,SocCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn soc_maw(
        self,
    ) -> crate::common::RegisterField<12, 0x7, 1, 0, u8, u8, SocCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x7,1,0,u8,u8,SocCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn soc_cmireg_enable(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, SocCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,SocCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn soc_chop(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, u8, SocCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x7,1,0,u8,u8,SocCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn soc_icm(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, SocCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,u8,u8,SocCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn soc_dcycle(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, SocCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,SocCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn soc_scycle(
        self,
    ) -> crate::common::RegisterField<2, 0x7, 1, 0, u8, u8, SocCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x7,1,0,u8,u8,SocCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn soc_rvi(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, SocCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,SocCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for SocCtrl2Reg {
    #[inline(always)]
    fn default() -> SocCtrl2Reg {
        <crate::RegValueT<SocCtrl2Reg_SPEC> as RegisterValue<_>>::new(30570)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SocCtrl3Reg_SPEC;
impl crate::sealed::RegSpec for SocCtrl3Reg_SPEC {
    type DataType = u16;
}

pub type SocCtrl3Reg = crate::RegValueT<SocCtrl3Reg_SPEC>;

impl SocCtrl3Reg {
    #[inline(always)]
    pub fn soc_vcmi(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, SocCtrl3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,SocCtrl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn soc_dynhys(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, SocCtrl3Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,SocCtrl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn soc_dyntarg(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, SocCtrl3Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,SocCtrl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn soc_vsat(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, SocCtrl3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,SocCtrl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for SocCtrl3Reg {
    #[inline(always)]
    fn default() -> SocCtrl3Reg {
        <crate::RegValueT<SocCtrl3Reg_SPEC> as RegisterValue<_>>::new(17)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SocExtInReg_SPEC;
impl crate::sealed::RegSpec for SocExtInReg_SPEC {
    type DataType = u16;
}

pub type SocExtInReg = crate::RegValueT<SocExtInReg_SPEC>;

impl SocExtInReg {
    #[inline(always)]
    pub fn soc_ext_idac_en(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, SocExtInReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,SocExtInReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn soc_ext_scycle_en(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, SocExtInReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,SocExtInReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn soc_nr_scycle(
        self,
    ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, u8, SocExtInReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x7,1,0,u8,u8,SocExtInReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn soc_rdac_dis(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, SocExtInReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,SocExtInReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn soc_idac_sign(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, SocExtInReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,SocExtInReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn soc_idac_val(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, SocExtInReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1ff,1,0,u16,u16,SocExtInReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for SocExtInReg {
    #[inline(always)]
    fn default() -> SocExtInReg {
        <crate::RegValueT<SocExtInReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SocExtOutReg_SPEC;
impl crate::sealed::RegSpec for SocExtOutReg_SPEC {
    type DataType = u16;
}

pub type SocExtOutReg = crate::RegValueT<SocExtOutReg_SPEC>;

impl SocExtOutReg {
    #[inline(always)]
    pub fn soc_ctrl_event(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, SocExtOutReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,SocExtOutReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn soc_state(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, SocExtOutReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,SocExtOutReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn soc_rising_comp(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, SocExtOutReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,SocExtOutReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn soc_pos_comp(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, SocExtOutReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,SocExtOutReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn soc_lowlim_comp(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, SocExtOutReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,SocExtOutReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn soc_high_lim(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, SocExtOutReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,SocExtOutReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for SocExtOutReg {
    #[inline(always)]
    fn default() -> SocExtOutReg {
        <crate::RegValueT<SocExtOutReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SocStatusReg_SPEC;
impl crate::sealed::RegSpec for SocStatusReg_SPEC {
    type DataType = u16;
}

pub type SocStatusReg = crate::RegValueT<SocStatusReg_SPEC>;

impl SocStatusReg {
    #[inline(always)]
    pub fn soc_int_locked(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, SocStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,SocStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn soc_int_overload(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, SocStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,SocStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for SocStatusReg {
    #[inline(always)]
    fn default() -> SocStatusReg {
        <crate::RegValueT<SocStatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
