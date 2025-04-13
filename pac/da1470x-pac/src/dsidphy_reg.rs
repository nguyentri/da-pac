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
#[doc = r"DSIDPHY_REG registers"]
unsafe impl ::core::marker::Send for super::DsidphyReg {}
unsafe impl ::core::marker::Sync for super::DsidphyReg {}
impl super::DsidphyReg {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    #[doc = ""]
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

    #[doc = ""]
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

    #[doc = ""]
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

    #[doc = ""]
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

    #[doc = ""]
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

    #[doc = "PLL control"]
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

    #[doc = ""]
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

    #[doc = "On-chip termination control"]
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

    #[doc = ""]
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

    #[doc = ""]
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

    #[doc = ""]
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

    #[doc = ""]
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

    #[doc = ""]
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

    #[doc = ""]
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
#[doc = ""]
pub type DphyBistDcOutReg = crate::RegValueT<DphyBistDcOutReg_SPEC>;

impl DphyBistDcOutReg {
    #[doc = "Output signal used in the DC test modes."]
    #[inline(always)]
    pub fn dphy_bist_dc_out(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, DphyBistDcOutReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8, DphyBistDcOutReg_SPEC,crate::common::R>::from_register(self,0)
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
#[doc = ""]
pub type DphyBistEnblReg = crate::RegValueT<DphyBistEnblReg_SPEC>;

impl DphyBistEnblReg {
    #[doc = "Six-bit signal that enables the testing modes\n\\[000000\\] Testing Disabled\n\\[000001\\] HS-TX Sequence Test (1)\n\\[000010\\] HS-TX Sequence Test (2)\n\\[000011\\] HS-TX Sequence PRBS Test (1)\n\\[000100\\] HS-TX Sequence PRBS Test (2)\n\\[001001\\] HS-TX Test (1)\n\\[001010\\] HS-TX Test (2)\n\\[001011\\] HS-TX PRBS Test (1)\n\\[001100\\] HS-TX PRBS Test (2)\n\\[001101\\] HS-TX SDI Test\n\\[010000\\] LP-TX LPDT Test\n\\[010001\\] LP-TX DC Test\n\\[010011\\] LP-RX DC Tests\n\\[010100\\] LP-CD DC Tests"]
    #[inline(always)]
    pub fn dphy_bist_enbl(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, DphyBistEnblReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8, DphyBistEnblReg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = ""]
pub type DphyBistPatternReg = crate::RegValueT<DphyBistPatternReg_SPEC>;

impl DphyBistPatternReg {
    #[doc = "This is the programmable test pattern used by BIST pattern generator and pattern matcher. The TEST_PATTERN bus value will be used as the payload data during the test, the HS SYNC token shouldnt be part of the TEST_PATTERN"]
    #[inline(always)]
    pub fn dphy_bist_pattern(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = ""]
pub type DphyClkDataLaneProgReg = crate::RegValueT<DphyClkDataLaneProgReg_SPEC>;

impl DphyClkDataLaneProgReg {
    #[doc = "Clock Lane: Bits used to program T_CLK_TRAIL time in the end of high speed transmission mode"]
    #[inline(always)]
    pub fn uc_prg_hs_trail(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1f,
        1,
        0,
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
            DphyClkDataLaneProgReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Clock Lane: Bits used to program T_CLK_ZERO time in the beginning of high speed transmission mode."]
    #[inline(always)]
    pub fn uc_prg_hs_zero(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x7f,
        1,
        0,
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
            DphyClkDataLaneProgReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Clock Lane: Bit used to program T_CLK_PREPARE time in the beginning of high speed transmission mode."]
    #[inline(always)]
    pub fn uc_prg_hs_prepare(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, DphyClkDataLaneProgReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<13,1,0,DphyClkDataLaneProgReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Data Lane: Bits used to program T_HS_TRAIL time in the end of high speed transmission mode"]
    #[inline(always)]
    pub fn u_prg_hs_trail(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1f,
        1,
        0,
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
            DphyClkDataLaneProgReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Data Lane: Bits used to program T_HS_ZERO time in the beginning of high speed transmission mode."]
    #[inline(always)]
    pub fn u_prg_hs_zero(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x3f,
        1,
        0,
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
            DphyClkDataLaneProgReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Data Lane: Bits used to program T_HS_PREPARE time in the beginning of high speed transmission mode"]
    #[inline(always)]
    pub fn u_prg_hs_prepare(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
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
#[doc = ""]
pub type DphyGlobalCtrlReg = crate::RegValueT<DphyGlobalCtrlReg_SPEC>;

impl DphyGlobalCtrlReg {
    #[doc = "When the PHY enters ULPS, this input determines if the PLL will be powered down or not.\n1b0: Feature disabled\n1b1: Feature enabled \\[Default\\]"]
    #[inline(always)]
    pub fn ulps_pll_ctrl(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, DphyGlobalCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,DphyGlobalCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "When the PHY enters ULPS, this input determines if the PHY will be powered down or not.\n1b0: Feature disabled\n1b1: Feature enabled \\[Default\\]"]
    #[inline(always)]
    pub fn ulps_phy_ctrl(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, DphyGlobalCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,DphyGlobalCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "When clock lane exits from ULPS, this input determines if the PLL LOCK signal will be used to gate the TxByteClkHS\n1b0: PLL LOCK signal will gate TxByteClkHS clock \\[Default\\]\n1b1: PLL LOCK signal will not gate TxByteClkHS clock, CIL based counter will be used to gate the TxByteClkHS"]
    #[inline(always)]
    pub fn lock_byp(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, DphyGlobalCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,DphyGlobalCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Powers down inactive lanes reported by CFG_NUM_LANES input bus.\n1b0: inactive lanes are powered up and driving LP11.\n1b1: inactive lanes are powered down \\[Default\\]"]
    #[inline(always)]
    pub fn auto_pd_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, DphyGlobalCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,DphyGlobalCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Power Down input for PHY. When high, all blocks are powered down"]
    #[inline(always)]
    pub fn phy_pd(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, DphyGlobalCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,DphyGlobalCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Power-down signal When high, the PLL is powered down"]
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
#[doc = "PLL control"]
pub type DphyPllControlReg = crate::RegValueT<DphyPllControlReg_SPEC>;

impl DphyPllControlReg {
    #[doc = "Enable signal to use latched LOCK signal\n1b0: LOCK<= Normal LOCK signal \\[Default\\]\n1b1: LOCK<= Latched LOCK signal"]
    #[inline(always)]
    pub fn pll_lock_latch(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, DphyPllControlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20,1,0,DphyPllControlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enable signal to bypass PLL:\n1b0: CLKOUT<= CLKREF * ( M / ( N * O ))\n1b1: CLKOUT<= CLKEXT"]
    #[inline(always)]
    pub fn pll_bypass(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, DphyPllControlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19,1,0,DphyPllControlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Test Pins. Should be connected to chip configuration registers.\n4\'b1001: TST_PLL \\[Default\\] at normal operation"]
    #[inline(always)]
    pub fn pll_tst(
        self,
    ) -> crate::common::RegisterField<15, 0xf, 1, 0, u8, DphyPllControlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0xf,1,0,u8, DphyPllControlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Control M (feedback) divider"]
    #[inline(always)]
    pub fn pll_cm(
        self,
    ) -> crate::common::RegisterField<7, 0xff, 1, 0, u8, DphyPllControlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0xff,1,0,u8, DphyPllControlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Control N (input) divider"]
    #[inline(always)]
    pub fn pll_cn(
        self,
    ) -> crate::common::RegisterField<2, 0x1f, 1, 0, u8, DphyPllControlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1f,1,0,u8, DphyPllControlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Control O (output) divider"]
    #[inline(always)]
    pub fn pll_co(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, DphyPllControlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8, DphyPllControlReg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = ""]
pub type DphyStatusReg = crate::RegValueT<DphyStatusReg_SPEC>;

impl DphyStatusReg {
    #[doc = "Lock Detect output. Asserted when PLL is frequency locked"]
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
#[doc = "On-chip termination control"]
pub type DphyTxRcalReg = crate::RegValueT<DphyTxRcalReg_SPEC>;

impl DphyTxRcalReg {
    #[doc = "On-chip termination control bits for manual calibration of HS-TX. Only active when BYPASS_RCAL asserted.\n1b0: 20% higher than mid-range. Highest impedance setting.\n1b1: Mid-range impedance setting."]
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
#[doc = ""]
pub type Dsi2DphyClkRstNCtrlReg = crate::RegValueT<Dsi2DphyClkRstNCtrlReg_SPEC>;

impl Dsi2DphyClkRstNCtrlReg {
    #[doc = "0: div/4, required with RCHS of 64MHz\n1: div/6, required with RCHS of 92MHz"]
    #[inline(always)]
    pub fn dphy_tx_esc_clk_div(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Dsi2DphyClkRstNCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<1,1,0,Dsi2DphyClkRstNCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Hold the DSI2 in reset. This should be set to \'b0 when configuring the DSI2 to avoid metastability"]
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
#[doc = ""]
pub type Dsi2ErrorStatusReg = crate::RegValueT<Dsi2ErrorStatusReg_SPEC>;

impl Dsi2ErrorStatusReg {
    #[doc = "There was a row access error in the memory. A read and a write took place at the same cycle. Reason could be that the FIFO was almost full"]
    #[inline(always)]
    pub fn row_access_error(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Dsi2ErrorStatusReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<11,1,0,Dsi2ErrorStatusReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Host BTA timeout has occurred."]
    #[inline(always)]
    pub fn host_bta_timeout(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Dsi2ErrorStatusReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<10,1,0,Dsi2ErrorStatusReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Low Power RX timeout has occurred"]
    #[inline(always)]
    pub fn ls_rx_timeout(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Dsi2ErrorStatusReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,Dsi2ErrorStatusReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "High Speed transmit has timed out."]
    #[inline(always)]
    pub fn hs_tx_timeout(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Dsi2ErrorStatusReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,Dsi2ErrorStatusReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "CRC calculated on the received data does not match the CRC the transmitter sent at the end of the packet."]
    #[inline(always)]
    pub fn crc_error(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Dsi2ErrorStatusReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,Dsi2ErrorStatusReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Single bit error in the packet header was detected and corrected."]
    #[inline(always)]
    pub fn ecc_two_bit_error(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Dsi2ErrorStatusReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,Dsi2ErrorStatusReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Two packet header bit errors were detected and not corrected"]
    #[inline(always)]
    pub fn ecc_one_bit_error(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Dsi2ErrorStatusReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,Dsi2ErrorStatusReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Only asserted if cfg_vid_video_mode is set to Burst Mode and cfg_vid_bllp_mode is set to 1b1. Asserted if not enough blanking time between end of a video line and the next vid_hsync. If video line transmission to the controller is occurring when a new vid_hsync is detected, this error signal will be asserted. This may indicate that VID_CLK is too fast, or that the cfg_vid_* parameters are incorrectly set."]
    #[inline(always)]
    pub fn vid_error_blanking(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Dsi2ErrorStatusReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,Dsi2ErrorStatusReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Asserted if internal video FIFO underflows. The video rate might be lower than the MIPI rate. This may indicate that VID_CLK is too slow, or that the cfg_vid_* parameters are incorrectly set."]
    #[inline(always)]
    pub fn vid_error_fifo_underflow(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Dsi2ErrorStatusReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,Dsi2ErrorStatusReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Asserted if internal video FIFO overflows. The video rate might be higher than the MIPI rate or the delay for MIPI is long enough that the FIFO overflows. This may indicate that VID_CLK is too fast, or that the cfg_vid_* parameters are incorrectly set."]
    #[inline(always)]
    pub fn vid_error_fifo_overflow(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Dsi2ErrorStatusReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,Dsi2ErrorStatusReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Only asserted if cfg_vid_video_mode is set to Non-Burst mode with Sync Pulses. New vid_hsync was detected while still sending blanking from last video line. This can occur if video rate is set to higher than MIPI rate and there is not enough cycles to transmit the video blanking after transmitting the video line. This may indicate that VID_CLK is too fast, or that the cfg_vid_* parameters are incorrectly set."]
    #[inline(always)]
    pub fn vid_error_sync_pulse(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Dsi2ErrorStatusReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,Dsi2ErrorStatusReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Asserted if an error occurs during operation. Signal is asserted if any of the other vid_error signals (sync_pulse, fifo_overflow, fifo_underflow, or blanking) are asserted. Signal can be used as an enable to latch the other vid_error signals."]
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
#[doc = ""]
pub type Dsi2InterruptEnReg = crate::RegValueT<Dsi2InterruptEnReg_SPEC>;

impl Dsi2InterruptEnReg {
    #[doc = "Enable Row Access Error IRQ"]
    #[inline(always)]
    pub fn row_access_error_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Dsi2InterruptEnReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<11,1,0,Dsi2InterruptEnReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enable BTA timeout IRQ"]
    #[inline(always)]
    pub fn host_bta_timeout_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Dsi2InterruptEnReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<10,1,0,Dsi2InterruptEnReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enable Low Power RX timeout IRQ"]
    #[inline(always)]
    pub fn lp_rx_timeout_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Dsi2InterruptEnReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,Dsi2InterruptEnReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enable High Speed TX timeout IRQ"]
    #[inline(always)]
    pub fn hs_tx_timeout_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Dsi2InterruptEnReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,Dsi2InterruptEnReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enable CRC error IRQ"]
    #[inline(always)]
    pub fn crc_error_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Dsi2InterruptEnReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,Dsi2InterruptEnReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enable ECC 2 bit error IRQ"]
    #[inline(always)]
    pub fn ecc_two_bit_error_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Dsi2InterruptEnReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,Dsi2InterruptEnReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enable ECC 1 bit error IRQ"]
    #[inline(always)]
    pub fn ecc_one_bit_error_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Dsi2InterruptEnReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,Dsi2InterruptEnReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enable VID blanking error IRQ"]
    #[inline(always)]
    pub fn vid_error_blanking_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Dsi2InterruptEnReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,Dsi2InterruptEnReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enable VID FIFO underflow IRQ"]
    #[inline(always)]
    pub fn vid_error_fifo_underflow_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Dsi2InterruptEnReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,Dsi2InterruptEnReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enable VID FIFO overflow IRQ"]
    #[inline(always)]
    pub fn vid_error_fifo_overflow_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Dsi2InterruptEnReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,Dsi2InterruptEnReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enable VID sync pulse error IRQ"]
    #[inline(always)]
    pub fn vid_error_sync_pulse_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Dsi2InterruptEnReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,Dsi2InterruptEnReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enable VID error IRQ"]
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
#[doc = ""]
pub type Dsi2MemctrlReg = crate::RegValueT<Dsi2MemctrlReg_SPEC>;

impl Dsi2MemctrlReg {
    #[doc = ""]
    #[inline(always)]
    pub fn dsi2_ms(
        self,
    ) -> crate::common::RegisterField<1, 0xf, 1, 0, u8, Dsi2MemctrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0xf,1,0,u8, Dsi2MemctrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
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
#[doc = ""]
pub type Dsi2TriggerReg = crate::RegValueT<Dsi2TriggerReg_SPEC>;

impl Dsi2TriggerReg {
    #[doc = "Transmit trigger.\nThe format of trigger_send is as follows:\n1b00 = Trigger 0 (Reset-Trigger)\n1b01 = Trigger 1 (\\[Reserved\\])\n1b10 = Trigger 2 (\\[Reserved\\])\n1b11 = Trigger 3 (\\[Reserved\\])"]
    #[inline(always)]
    pub fn trigger_data(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Dsi2TriggerReg_SPEC, crate::common::W> {
        crate::common::RegisterField::<0,0x3,1,0,u8, Dsi2TriggerReg_SPEC,crate::common::W>::from_register(self,0)
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
#[doc = ""]
pub type Dsi2UlpsCfgReg = crate::RegValueT<Dsi2UlpsCfgReg_SPEC>;

impl Dsi2UlpsCfgReg {
    #[doc = "Ultra Low Power State status. Bits assert high to indicate that the corresponding data lanes are in Ultra Low Power State.\nBit \\[0\\] Data lane 0\nBit \\[1\\] Data Lane 1"]
    #[inline(always)]
    pub fn ulps_active(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, Dsi2UlpsCfgReg_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0x3,1,0,u8, Dsi2UlpsCfgReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Ultra Low Power State clock lane status. Asserted high to indicate that the clock lane is in Ultra Low Power State."]
    #[inline(always)]
    pub fn ulps_clk_active(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Dsi2UlpsCfgReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,Dsi2UlpsCfgReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Ultra-Low Power State enable. When asserted (active high) the controller instructs the PHY to put the corresponding data lane into Ultra Low Power State.\nBit \\[0\\] Data lane 0\nBit \\[1\\] Data Lane 1"]
    #[inline(always)]
    pub fn ulps_enable(
        self,
    ) -> crate::common::RegisterField<1, 0x3, 1, 0, u8, Dsi2UlpsCfgReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x3,1,0,u8, Dsi2UlpsCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Ultra-Low Power State enable for clock lane. When set (1\'b1) the controller instructs the PHY to put the clock lane into Ultra Low Power State."]
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
