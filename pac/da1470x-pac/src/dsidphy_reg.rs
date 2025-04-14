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
#[doc = r"DSIDPHY_REG registers"]
unsafe impl ::core::marker::Send for super::DsidphyReg {}
unsafe impl ::core::marker::Sync for super::DsidphyReg {}
impl super::DsidphyReg {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn dphy_bist_dc_out_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DphyBistDcOutReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DphyBistDcOutReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dphy_bist_enbl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DphyBistEnblReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DphyBistEnblReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dphy_bist_pattern_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DphyBistPatternReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DphyBistPatternReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dphy_clk_data_lane_prog_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DphyClkDataLaneProgReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DphyClkDataLaneProgReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dphy_global_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DphyGlobalCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DphyGlobalCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dphy_pll_control_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DphyPllControlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DphyPllControlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dphy_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DphyStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DphyStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dphy_tx_rcal_reg(
        &self,
    ) -> &'static crate::common::Reg<self::DphyTxRcalReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::DphyTxRcalReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dsi2_dphy_clk_rst_n_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dsi2DphyClkRstNCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dsi2DphyClkRstNCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dsi2_error_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dsi2ErrorStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dsi2ErrorStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dsi2_interrupt_en_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dsi2InterruptEnReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dsi2InterruptEnReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dsi2_memctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dsi2MemctrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dsi2MemctrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dsi2_trigger_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dsi2TriggerReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dsi2TriggerReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[inline(always)]
    pub const fn dsi2_ulps_cfg_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Dsi2UlpsCfgReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Dsi2UlpsCfgReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DphyBistDcOutReg_SPEC;
impl crate::sealed::RegSpec for DphyBistDcOutReg_SPEC {
    type DataType = u32;
}

pub type DphyBistDcOutReg = crate::RegValueT<DphyBistDcOutReg_SPEC>;

impl DphyBistDcOutReg {
    #[inline(always)]
    pub fn dphy_bist_dc_out(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, DphyBistDcOutReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,DphyBistDcOutReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for DphyBistDcOutReg {
    #[inline(always)]
    fn default() -> DphyBistDcOutReg {
        <crate::RegValueT<DphyBistDcOutReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DphyBistEnblReg_SPEC;
impl crate::sealed::RegSpec for DphyBistEnblReg_SPEC {
    type DataType = u32;
}

pub type DphyBistEnblReg = crate::RegValueT<DphyBistEnblReg_SPEC>;

impl DphyBistEnblReg {
    #[inline(always)]
    pub fn dphy_bist_enbl(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, DphyBistEnblReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,DphyBistEnblReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DphyBistEnblReg {
    #[inline(always)]
    fn default() -> DphyBistEnblReg {
        <crate::RegValueT<DphyBistEnblReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DphyBistPatternReg_SPEC;
impl crate::sealed::RegSpec for DphyBistPatternReg_SPEC {
    type DataType = u32;
}

pub type DphyBistPatternReg = crate::RegValueT<DphyBistPatternReg_SPEC>;

impl DphyBistPatternReg {
    #[inline(always)]
    pub fn dphy_bist_pattern(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        DphyBistPatternReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            DphyBistPatternReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for DphyBistPatternReg {
    #[inline(always)]
    fn default() -> DphyBistPatternReg {
        <crate::RegValueT<DphyBistPatternReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DphyClkDataLaneProgReg_SPEC;
impl crate::sealed::RegSpec for DphyClkDataLaneProgReg_SPEC {
    type DataType = u32;
}

pub type DphyClkDataLaneProgReg = crate::RegValueT<DphyClkDataLaneProgReg_SPEC>;

impl DphyClkDataLaneProgReg {
    #[inline(always)]
    pub fn uc_prg_hs_trail(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1f,
        1,
        0,
        u8,
        u8,
        DphyClkDataLaneProgReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1f,
            1,
            0,
            u8,
            u8,
            DphyClkDataLaneProgReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn uc_prg_hs_zero(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x7f,
        1,
        0,
        u8,
        u8,
        DphyClkDataLaneProgReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x7f,
            1,
            0,
            u8,
            u8,
            DphyClkDataLaneProgReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn uc_prg_hs_prepare(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, DphyClkDataLaneProgReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<13,1,0,DphyClkDataLaneProgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn u_prg_hs_trail(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1f,
        1,
        0,
        u8,
        u8,
        DphyClkDataLaneProgReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1f,
            1,
            0,
            u8,
            u8,
            DphyClkDataLaneProgReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn u_prg_hs_zero(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x3f,
        1,
        0,
        u8,
        u8,
        DphyClkDataLaneProgReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x3f,
            1,
            0,
            u8,
            u8,
            DphyClkDataLaneProgReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn u_prg_hs_prepare(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        u8,
        u8,
        DphyClkDataLaneProgReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            u8,
            u8,
            DphyClkDataLaneProgReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for DphyClkDataLaneProgReg {
    #[inline(always)]
    fn default() -> DphyClkDataLaneProgReg {
        <crate::RegValueT<DphyClkDataLaneProgReg_SPEC> as RegisterValue<_>>::new(12781064)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DphyGlobalCtrlReg_SPEC;
impl crate::sealed::RegSpec for DphyGlobalCtrlReg_SPEC {
    type DataType = u32;
}

pub type DphyGlobalCtrlReg = crate::RegValueT<DphyGlobalCtrlReg_SPEC>;

impl DphyGlobalCtrlReg {
    #[inline(always)]
    pub fn ulps_pll_ctrl(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, DphyGlobalCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,DphyGlobalCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ulps_phy_ctrl(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, DphyGlobalCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,DphyGlobalCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lock_byp(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, DphyGlobalCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,DphyGlobalCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn auto_pd_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, DphyGlobalCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,DphyGlobalCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn phy_pd(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, DphyGlobalCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,DphyGlobalCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pll_pd(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, DphyGlobalCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,DphyGlobalCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DphyGlobalCtrlReg {
    #[inline(always)]
    fn default() -> DphyGlobalCtrlReg {
        <crate::RegValueT<DphyGlobalCtrlReg_SPEC> as RegisterValue<_>>::new(55)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DphyPllControlReg_SPEC;
impl crate::sealed::RegSpec for DphyPllControlReg_SPEC {
    type DataType = u32;
}

pub type DphyPllControlReg = crate::RegValueT<DphyPllControlReg_SPEC>;

impl DphyPllControlReg {
    #[inline(always)]
    pub fn pll_lock_latch(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, DphyPllControlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20,1,0,DphyPllControlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pll_bypass(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, DphyPllControlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19,1,0,DphyPllControlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pll_tst(
        self,
    ) -> crate::common::RegisterField<
        15,
        0xf,
        1,
        0,
        u8,
        u8,
        DphyPllControlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0xf,
            1,
            0,
            u8,
            u8,
            DphyPllControlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pll_cm(
        self,
    ) -> crate::common::RegisterField<
        7,
        0xff,
        1,
        0,
        u8,
        u8,
        DphyPllControlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0xff,
            1,
            0,
            u8,
            u8,
            DphyPllControlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pll_cn(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1f,
        1,
        0,
        u8,
        u8,
        DphyPllControlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1f,
            1,
            0,
            u8,
            u8,
            DphyPllControlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn pll_co(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, DphyPllControlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            u8,
            u8,
            DphyPllControlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for DphyPllControlReg {
    #[inline(always)]
    fn default() -> DphyPllControlReg {
        <crate::RegValueT<DphyPllControlReg_SPEC> as RegisterValue<_>>::new(325629)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DphyStatusReg_SPEC;
impl crate::sealed::RegSpec for DphyStatusReg_SPEC {
    type DataType = u32;
}

pub type DphyStatusReg = crate::RegValueT<DphyStatusReg_SPEC>;

impl DphyStatusReg {
    #[inline(always)]
    pub fn pll_lock(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, DphyStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,DphyStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for DphyStatusReg {
    #[inline(always)]
    fn default() -> DphyStatusReg {
        <crate::RegValueT<DphyStatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DphyTxRcalReg_SPEC;
impl crate::sealed::RegSpec for DphyTxRcalReg_SPEC {
    type DataType = u32;
}

pub type DphyTxRcalReg = crate::RegValueT<DphyTxRcalReg_SPEC>;

impl DphyTxRcalReg {
    #[inline(always)]
    pub fn tx_rcal(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, DphyTxRcalReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,DphyTxRcalReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DphyTxRcalReg {
    #[inline(always)]
    fn default() -> DphyTxRcalReg {
        <crate::RegValueT<DphyTxRcalReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsi2DphyClkRstNCtrlReg_SPEC;
impl crate::sealed::RegSpec for Dsi2DphyClkRstNCtrlReg_SPEC {
    type DataType = u32;
}

pub type Dsi2DphyClkRstNCtrlReg = crate::RegValueT<Dsi2DphyClkRstNCtrlReg_SPEC>;

impl Dsi2DphyClkRstNCtrlReg {
    #[inline(always)]
    pub fn dphy_tx_esc_clk_div(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Dsi2DphyClkRstNCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<1,1,0,Dsi2DphyClkRstNCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dsi2_reset_n(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Dsi2DphyClkRstNCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<0,1,0,Dsi2DphyClkRstNCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dsi2DphyClkRstNCtrlReg {
    #[inline(always)]
    fn default() -> Dsi2DphyClkRstNCtrlReg {
        <crate::RegValueT<Dsi2DphyClkRstNCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsi2ErrorStatusReg_SPEC;
impl crate::sealed::RegSpec for Dsi2ErrorStatusReg_SPEC {
    type DataType = u32;
}

pub type Dsi2ErrorStatusReg = crate::RegValueT<Dsi2ErrorStatusReg_SPEC>;

impl Dsi2ErrorStatusReg {
    #[inline(always)]
    pub fn row_access_error(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Dsi2ErrorStatusReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<11,1,0,Dsi2ErrorStatusReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn host_bta_timeout(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Dsi2ErrorStatusReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<10,1,0,Dsi2ErrorStatusReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ls_rx_timeout(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Dsi2ErrorStatusReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,Dsi2ErrorStatusReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn hs_tx_timeout(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Dsi2ErrorStatusReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,Dsi2ErrorStatusReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn crc_error(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Dsi2ErrorStatusReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,Dsi2ErrorStatusReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ecc_two_bit_error(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Dsi2ErrorStatusReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,Dsi2ErrorStatusReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ecc_one_bit_error(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Dsi2ErrorStatusReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,Dsi2ErrorStatusReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vid_error_blanking(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Dsi2ErrorStatusReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,Dsi2ErrorStatusReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vid_error_fifo_underflow(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Dsi2ErrorStatusReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,Dsi2ErrorStatusReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vid_error_fifo_overflow(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Dsi2ErrorStatusReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,Dsi2ErrorStatusReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vid_error_sync_pulse(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Dsi2ErrorStatusReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,Dsi2ErrorStatusReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vid_error(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Dsi2ErrorStatusReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Dsi2ErrorStatusReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dsi2ErrorStatusReg {
    #[inline(always)]
    fn default() -> Dsi2ErrorStatusReg {
        <crate::RegValueT<Dsi2ErrorStatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsi2InterruptEnReg_SPEC;
impl crate::sealed::RegSpec for Dsi2InterruptEnReg_SPEC {
    type DataType = u32;
}

pub type Dsi2InterruptEnReg = crate::RegValueT<Dsi2InterruptEnReg_SPEC>;

impl Dsi2InterruptEnReg {
    #[inline(always)]
    pub fn row_access_error_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Dsi2InterruptEnReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<11,1,0,Dsi2InterruptEnReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn host_bta_timeout_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Dsi2InterruptEnReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<10,1,0,Dsi2InterruptEnReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lp_rx_timeout_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Dsi2InterruptEnReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,Dsi2InterruptEnReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn hs_tx_timeout_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Dsi2InterruptEnReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,Dsi2InterruptEnReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn crc_error_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Dsi2InterruptEnReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,Dsi2InterruptEnReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ecc_two_bit_error_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Dsi2InterruptEnReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,Dsi2InterruptEnReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ecc_one_bit_error_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Dsi2InterruptEnReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,Dsi2InterruptEnReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vid_error_blanking_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Dsi2InterruptEnReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,Dsi2InterruptEnReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vid_error_fifo_underflow_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Dsi2InterruptEnReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,Dsi2InterruptEnReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vid_error_fifo_overflow_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Dsi2InterruptEnReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,Dsi2InterruptEnReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vid_error_sync_pulse_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Dsi2InterruptEnReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,Dsi2InterruptEnReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vid_error_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Dsi2InterruptEnReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Dsi2InterruptEnReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dsi2InterruptEnReg {
    #[inline(always)]
    fn default() -> Dsi2InterruptEnReg {
        <crate::RegValueT<Dsi2InterruptEnReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsi2MemctrlReg_SPEC;
impl crate::sealed::RegSpec for Dsi2MemctrlReg_SPEC {
    type DataType = u32;
}

pub type Dsi2MemctrlReg = crate::RegValueT<Dsi2MemctrlReg_SPEC>;

impl Dsi2MemctrlReg {
    #[inline(always)]
    pub fn dsi2_ms(
        self,
    ) -> crate::common::RegisterField<1, 0xf, 1, 0, u8, u8, Dsi2MemctrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0xf,1,0,u8,u8,Dsi2MemctrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dsi2_mse(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Dsi2MemctrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Dsi2MemctrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dsi2MemctrlReg {
    #[inline(always)]
    fn default() -> Dsi2MemctrlReg {
        <crate::RegValueT<Dsi2MemctrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsi2TriggerReg_SPEC;
impl crate::sealed::RegSpec for Dsi2TriggerReg_SPEC {
    type DataType = u32;
}

pub type Dsi2TriggerReg = crate::RegValueT<Dsi2TriggerReg_SPEC>;

impl Dsi2TriggerReg {
    #[inline(always)]
    pub fn trigger_data(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, Dsi2TriggerReg_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,Dsi2TriggerReg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Dsi2TriggerReg {
    #[inline(always)]
    fn default() -> Dsi2TriggerReg {
        <crate::RegValueT<Dsi2TriggerReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsi2UlpsCfgReg_SPEC;
impl crate::sealed::RegSpec for Dsi2UlpsCfgReg_SPEC {
    type DataType = u32;
}

pub type Dsi2UlpsCfgReg = crate::RegValueT<Dsi2UlpsCfgReg_SPEC>;

impl Dsi2UlpsCfgReg {
    #[inline(always)]
    pub fn ulps_active(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, Dsi2UlpsCfgReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,Dsi2UlpsCfgReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ulps_clk_active(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Dsi2UlpsCfgReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,Dsi2UlpsCfgReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ulps_enable(
        self,
    ) -> crate::common::RegisterField<1, 0x3, 1, 0, u8, u8, Dsi2UlpsCfgReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x3,1,0,u8,u8,Dsi2UlpsCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ulps_clk_enable(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Dsi2UlpsCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Dsi2UlpsCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dsi2UlpsCfgReg {
    #[inline(always)]
    fn default() -> Dsi2UlpsCfgReg {
        <crate::RegValueT<Dsi2UlpsCfgReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
