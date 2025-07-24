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
// Generated from SVD 1.2, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:44:12 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"adplldig registers"]
unsafe impl ::core::marker::Send for super::Adplldig {}
unsafe impl ::core::marker::Sync for super::Adplldig {}
impl super::Adplldig {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn adpll_acc_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::AdpllAccCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::AdpllAccCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(100usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adpll_anatst_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::AdpllAnatstCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::AdpllAnatstCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(96usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adpll_anatst_rd_reg(
        &self,
    ) -> &'static crate::common::Reg<self::AdpllAnatstRdReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::AdpllAnatstRdReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(148usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adpll_ana_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::AdpllAnaCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::AdpllAnaCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adpll_attr_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::AdpllAttrCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::AdpllAttrCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adpll_cn_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::AdpllCnCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::AdpllCnCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adpll_dcoamp_cal_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::AdpllDcoampCalCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::AdpllDcoampCalCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adpll_dco_rd_reg(
        &self,
    ) -> &'static crate::common::Reg<self::AdpllDcoRdReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::AdpllDcoRdReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(128usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adpll_div_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::AdpllDivCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::AdpllDivCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adpll_fif_ctrl1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::AdpllFifCtrl1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::AdpllFifCtrl1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adpll_fif_ctrl2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::AdpllFifCtrl2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::AdpllFifCtrl2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adpll_freqmeas_rd_reg(
        &self,
    ) -> &'static crate::common::Reg<self::AdpllFreqmeasRdReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::AdpllFreqmeasRdReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(124usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adpll_fsm_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::AdpllFsmCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::AdpllFsmCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adpll_init_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::AdpllInitCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::AdpllInitCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adpll_kdco_cal_ctrl1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::AdpllKdcoCalCtrl1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::AdpllKdcoCalCtrl1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adpll_kdco_cal_ctrl2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::AdpllKdcoCalCtrl2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::AdpllKdcoCalCtrl2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adpll_kdco_rd_reg(
        &self,
    ) -> &'static crate::common::Reg<self::AdpllKdcoRdReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::AdpllKdcoRdReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(132usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adpll_kdtctdc_cal_ctrl1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::AdpllKdtctdcCalCtrl1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::AdpllKdtctdcCalCtrl1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adpll_kdtctdc_cal_ctrl2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::AdpllKdtctdcCalCtrl2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::AdpllKdtctdcCalCtrl2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adpll_kdtc_rd_reg(
        &self,
    ) -> &'static crate::common::Reg<self::AdpllKdtcRdReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::AdpllKdtcRdReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(136usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adpll_lf_ctrl1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::AdpllLfCtrl1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::AdpllLfCtrl1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adpll_lf_ctrl2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::AdpllLfCtrl2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::AdpllLfCtrl2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adpll_misc_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::AdpllMiscCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::AdpllMiscCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(72usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adpll_mon_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::AdpllMonCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::AdpllMonCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(68usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adpll_overrule_ctrl1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::AdpllOverruleCtrl1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::AdpllOverruleCtrl1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adpll_overrule_ctrl2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::AdpllOverruleCtrl2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::AdpllOverruleCtrl2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(84usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adpll_overrule_ctrl3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::AdpllOverruleCtrl3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::AdpllOverruleCtrl3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(88usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adpll_pllfcwdt_rd_reg(
        &self,
    ) -> &'static crate::common::Reg<self::AdpllPllfcwdtRdReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::AdpllPllfcwdtRdReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(144usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adpll_rfpt_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::AdpllRfptCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::AdpllRfptCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(92usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adpll_sdmod_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::AdpllSdmodCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::AdpllSdmodCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adpll_tunestate_rd_reg(
        &self,
    ) -> &'static crate::common::Reg<self::AdpllTunestateRdReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::AdpllTunestateRdReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(140usize),
            )
        }
    }

    #[inline(always)]
    pub const fn adpll_txmod_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::AdpllTxmodCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::AdpllTxmodCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdpllAccCtrlReg_SPEC;
impl crate::sealed::RegSpec for AdpllAccCtrlReg_SPEC {
    type DataType = u32;
}

pub type AdpllAccCtrlReg = crate::RegValueT<AdpllAccCtrlReg_SPEC>;

impl AdpllAccCtrlReg {
    #[inline(always)]
    pub fn en_cmf_avg(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, AdpllAccCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,AdpllAccCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn clip_mod_tune_0_tx(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1fff,
        1,
        0,
        u16,
        u16,
        AdpllAccCtrlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1fff,
            1,
            0,
            u16,
            u16,
            AdpllAccCtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn clip_mod_tune_0_rx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1fff,
        1,
        0,
        u16,
        u16,
        AdpllAccCtrlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1fff,
            1,
            0,
            u16,
            u16,
            AdpllAccCtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for AdpllAccCtrlReg {
    #[inline(always)]
    fn default() -> AdpllAccCtrlReg {
        <crate::RegValueT<AdpllAccCtrlReg_SPEC> as RegisterValue<_>>::new(2147483647)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdpllAnatstCtrlReg_SPEC;
impl crate::sealed::RegSpec for AdpllAnatstCtrlReg_SPEC {
    type DataType = u32;
}

pub type AdpllAnatstCtrlReg = crate::RegValueT<AdpllAnatstCtrlReg_SPEC>;

impl AdpllAnatstCtrlReg {
    #[inline(always)]
    pub fn anatstspare(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        u16,
        u16,
        AdpllAnatstCtrlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            AdpllAnatstCtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn anatsten(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        AdpllAnatstCtrlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            AdpllAnatstCtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for AdpllAnatstCtrlReg {
    #[inline(always)]
    fn default() -> AdpllAnatstCtrlReg {
        <crate::RegValueT<AdpllAnatstCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdpllAnatstRdReg_SPEC;
impl crate::sealed::RegSpec for AdpllAnatstRdReg_SPEC {
    type DataType = u32;
}

pub type AdpllAnatstRdReg = crate::RegValueT<AdpllAnatstRdReg_SPEC>;

impl AdpllAnatstRdReg {
    #[inline(always)]
    pub fn anatstspare_in(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        AdpllAnatstRdReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            AdpllAnatstRdReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for AdpllAnatstRdReg {
    #[inline(always)]
    fn default() -> AdpllAnatstRdReg {
        <crate::RegValueT<AdpllAnatstRdReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdpllAnaCtrlReg_SPEC;
impl crate::sealed::RegSpec for AdpllAnaCtrlReg_SPEC {
    type DataType = u32;
}

pub type AdpllAnaCtrlReg = crate::RegValueT<AdpllAnaCtrlReg_SPEC>;

impl AdpllAnaCtrlReg {
    #[inline(always)]
    pub fn dtc_ldo_dmy(
        self,
    ) -> crate::common::RegisterField<27, 0x3, 1, 0, u8, u8, AdpllAnaCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<27,0x3,1,0,u8,u8,AdpllAnaCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vpasettle(
        self,
    ) -> crate::common::RegisterField<24, 0x3, 1, 0, u8, u8, AdpllAnaCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x3,1,0,u8,u8,AdpllAnaCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tdc_offset(
        self,
    ) -> crate::common::RegisterField<16, 0x3f, 1, 0, u8, u8, AdpllAnaCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            16,
            0x3f,
            1,
            0,
            u8,
            u8,
            AdpllAnaCtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dtc_en(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, AdpllAnaCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,AdpllAnaCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dtcoffset(
        self,
    ) -> crate::common::RegisterField<8, 0x7f, 1, 0, u8, u8, AdpllAnaCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x7f,1,0,u8,u8,AdpllAnaCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tgldeten(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, AdpllAnaCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,AdpllAnaCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn en_ckdcomod(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, AdpllAnaCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,AdpllAnaCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn inv_ckdcomod(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, AdpllAnaCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,AdpllAnaCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn inv_ckphv(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, AdpllAnaCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,AdpllAnaCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn inv_cktdc(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, AdpllAnaCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,AdpllAnaCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tdc_inv(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, AdpllAnaCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,AdpllAnaCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tdc_ckvin_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, AdpllAnaCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,AdpllAnaCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tdc_dtcin_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, AdpllAnaCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,AdpllAnaCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for AdpllAnaCtrlReg {
    #[inline(always)]
    fn default() -> AdpllAnaCtrlReg {
        <crate::RegValueT<AdpllAnaCtrlReg_SPEC> as RegisterValue<_>>::new(34963579)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdpllAttrCtrlReg_SPEC;
impl crate::sealed::RegSpec for AdpllAttrCtrlReg_SPEC {
    type DataType = u32;
}

pub type AdpllAttrCtrlReg = crate::RegValueT<AdpllAttrCtrlReg_SPEC>;

impl AdpllAttrCtrlReg {
    #[inline(always)]
    pub fn pwr_mode_tx(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, AdpllAttrCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,AdpllAttrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pwr_mode_rx(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, AdpllAttrCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,AdpllAttrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for AdpllAttrCtrlReg {
    #[inline(always)]
    fn default() -> AdpllAttrCtrlReg {
        <crate::RegValueT<AdpllAttrCtrlReg_SPEC> as RegisterValue<_>>::new(3)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdpllCnCtrlReg_SPEC;
impl crate::sealed::RegSpec for AdpllCnCtrlReg_SPEC {
    type DataType = u32;
}

pub type AdpllCnCtrlReg = crate::RegValueT<AdpllCnCtrlReg_SPEC>;

impl AdpllCnCtrlReg {
    #[inline(always)]
    pub fn ch0(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1fff,
        1,
        0,
        u16,
        u16,
        AdpllCnCtrlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1fff,
            1,
            0,
            u16,
            u16,
            AdpllCnCtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sgn(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, AdpllCnCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,AdpllCnCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cs(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, AdpllCnCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,AdpllCnCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cn(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, u8, AdpllCnCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7f,1,0,u8,u8,AdpllCnCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for AdpllCnCtrlReg {
    #[inline(always)]
    fn default() -> AdpllCnCtrlReg {
        <crate::RegValueT<AdpllCnCtrlReg_SPEC> as RegisterValue<_>>::new(157417728)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdpllDcoampCalCtrlReg_SPEC;
impl crate::sealed::RegSpec for AdpllDcoampCalCtrlReg_SPEC {
    type DataType = u32;
}

pub type AdpllDcoampCalCtrlReg = crate::RegValueT<AdpllDcoampCalCtrlReg_SPEC>;

impl AdpllDcoampCalCtrlReg {
    #[inline(always)]
    pub fn dcoampic_lp_tx(
        self,
    ) -> crate::common::RegisterField<
        28,
        0xf,
        1,
        0,
        u8,
        u8,
        AdpllDcoampCalCtrlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0xf,
            1,
            0,
            u8,
            u8,
            AdpllDcoampCalCtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dcoampic_lp_rx(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xf,
        1,
        0,
        u8,
        u8,
        AdpllDcoampCalCtrlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0xf,
            1,
            0,
            u8,
            u8,
            AdpllDcoampCalCtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dcoampic_hp_tx(
        self,
    ) -> crate::common::RegisterField<
        20,
        0xf,
        1,
        0,
        u8,
        u8,
        AdpllDcoampCalCtrlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0xf,
            1,
            0,
            u8,
            u8,
            AdpllDcoampCalCtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dcoampic_hp_rx(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xf,
        1,
        0,
        u8,
        u8,
        AdpllDcoampCalCtrlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xf,
            1,
            0,
            u8,
            u8,
            AdpllDcoampCalCtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dcoamptm(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, AdpllDcoampCalCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<7,1,0,AdpllDcoampCalCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ampcalen(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, AdpllDcoampCalCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<6,1,0,AdpllDcoampCalCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn kmedium(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x7,
        1,
        0,
        u8,
        u8,
        AdpllDcoampCalCtrlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x7,
            1,
            0,
            u8,
            u8,
            AdpllDcoampCalCtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn kcoarse(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        u8,
        u8,
        AdpllDcoampCalCtrlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            u8,
            u8,
            AdpllDcoampCalCtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for AdpllDcoampCalCtrlReg {
    #[inline(always)]
    fn default() -> AdpllDcoampCalCtrlReg {
        <crate::RegValueT<AdpllDcoampCalCtrlReg_SPEC> as RegisterValue<_>>::new(2004287505)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdpllDcoRdReg_SPEC;
impl crate::sealed::RegSpec for AdpllDcoRdReg_SPEC {
    type DataType = u32;
}

pub type AdpllDcoRdReg = crate::RegValueT<AdpllDcoRdReg_SPEC>;

impl AdpllDcoRdReg {
    #[inline(always)]
    pub fn dcoamp(
        self,
    ) -> crate::common::RegisterField<26, 0xf, 1, 0, u8, u8, AdpllDcoRdReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<26,0xf,1,0,u8,u8,AdpllDcoRdReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcomod(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1fff,
        1,
        0,
        u16,
        u16,
        AdpllDcoRdReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            13,
            0x1fff,
            1,
            0,
            u16,
            u16,
            AdpllDcoRdReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dcofine(
        self,
    ) -> crate::common::RegisterField<7, 0x3f, 1, 0, u8, u8, AdpllDcoRdReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<7,0x3f,1,0,u8,u8,AdpllDcoRdReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcomedium(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, u8, AdpllDcoRdReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<4,0x7,1,0,u8,u8,AdpllDcoRdReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcocoarse(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, AdpllDcoRdReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,AdpllDcoRdReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for AdpllDcoRdReg {
    #[inline(always)]
    fn default() -> AdpllDcoRdReg {
        <crate::RegValueT<AdpllDcoRdReg_SPEC> as RegisterValue<_>>::new(469762055)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdpllDivCtrlReg_SPEC;
impl crate::sealed::RegSpec for AdpllDivCtrlReg_SPEC {
    type DataType = u32;
}

pub type AdpllDivCtrlReg = crate::RegValueT<AdpllDivCtrlReg_SPEC>;

impl AdpllDivCtrlReg {
    #[inline(always)]
    pub fn txdiv_trim(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1ff,
        1,
        0,
        u16,
        u16,
        AdpllDivCtrlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1ff,
            1,
            0,
            u16,
            u16,
            AdpllDivCtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rxdiv_trim(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1ff,
        1,
        0,
        u16,
        u16,
        AdpllDivCtrlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1ff,
            1,
            0,
            u16,
            u16,
            AdpllDivCtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rxdiv_fb_en_tx(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, AdpllDivCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,AdpllDivCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rxdiv_fb_en_rx(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, AdpllDivCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,AdpllDivCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn fbdiv_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, AdpllDivCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,AdpllDivCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for AdpllDivCtrlReg {
    #[inline(always)]
    fn default() -> AdpllDivCtrlReg {
        <crate::RegValueT<AdpllDivCtrlReg_SPEC> as RegisterValue<_>>::new(26803971)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdpllFifCtrl1Reg_SPEC;
impl crate::sealed::RegSpec for AdpllFifCtrl1Reg_SPEC {
    type DataType = u32;
}

pub type AdpllFifCtrl1Reg = crate::RegValueT<AdpllFifCtrl1Reg_SPEC>;

impl AdpllFifCtrl1Reg {
    #[inline(always)]
    pub fn fifrx_1m(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3fff,
        1,
        0,
        u16,
        u16,
        AdpllFifCtrl1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3fff,
            1,
            0,
            u16,
            u16,
            AdpllFifCtrl1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for AdpllFifCtrl1Reg {
    #[inline(always)]
    fn default() -> AdpllFifCtrl1Reg {
        <crate::RegValueT<AdpllFifCtrl1Reg_SPEC> as RegisterValue<_>>::new(1024)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdpllFifCtrl2Reg_SPEC;
impl crate::sealed::RegSpec for AdpllFifCtrl2Reg_SPEC {
    type DataType = u32;
}

pub type AdpllFifCtrl2Reg = crate::RegValueT<AdpllFifCtrl2Reg_SPEC>;

impl AdpllFifCtrl2Reg {
    #[inline(always)]
    pub fn fiftx(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x3fff,
        1,
        0,
        u16,
        u16,
        AdpllFifCtrl2Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x3fff,
            1,
            0,
            u16,
            u16,
            AdpllFifCtrl2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fifrx_offset(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3fff,
        1,
        0,
        u16,
        u16,
        AdpllFifCtrl2Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3fff,
            1,
            0,
            u16,
            u16,
            AdpllFifCtrl2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for AdpllFifCtrl2Reg {
    #[inline(always)]
    fn default() -> AdpllFifCtrl2Reg {
        <crate::RegValueT<AdpllFifCtrl2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdpllFreqmeasRdReg_SPEC;
impl crate::sealed::RegSpec for AdpllFreqmeasRdReg_SPEC {
    type DataType = u32;
}

pub type AdpllFreqmeasRdReg = crate::RegValueT<AdpllFreqmeasRdReg_SPEC>;

impl AdpllFreqmeasRdReg {
    #[inline(always)]
    pub fn measdone_out(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, AdpllFreqmeasRdReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29,1,0,AdpllFreqmeasRdReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn qualmondet(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, AdpllFreqmeasRdReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26,1,0,AdpllFreqmeasRdReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tdcbub(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, AdpllFreqmeasRdReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25,1,0,AdpllFreqmeasRdReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn phvsa0(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, AdpllFreqmeasRdReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24,1,0,AdpllFreqmeasRdReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn phvsa1(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, AdpllFreqmeasRdReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23,1,0,AdpllFreqmeasRdReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn freqdiff(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7fffff,
        1,
        0,
        u32,
        u32,
        AdpllFreqmeasRdReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x7fffff,
            1,
            0,
            u32,
            u32,
            AdpllFreqmeasRdReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for AdpllFreqmeasRdReg {
    #[inline(always)]
    fn default() -> AdpllFreqmeasRdReg {
        <crate::RegValueT<AdpllFreqmeasRdReg_SPEC> as RegisterValue<_>>::new(553648128)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdpllFsmCtrlReg_SPEC;
impl crate::sealed::RegSpec for AdpllFsmCtrlReg_SPEC {
    type DataType = u32;
}

pub type AdpllFsmCtrlReg = crate::RegValueT<AdpllFsmCtrlReg_SPEC>;

impl AdpllFsmCtrlReg {
    #[inline(always)]
    pub fn tvpasettle(
        self,
    ) -> crate::common::RegisterField<24, 0x3f, 1, 0, u8, u8, AdpllFsmCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            24,
            0x3f,
            1,
            0,
            u8,
            u8,
            AdpllFsmCtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tsettle(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, u8, AdpllFsmCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0xf,1,0,u8,u8,AdpllFsmCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tpasettle(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, u8, AdpllFsmCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xf,1,0,u8,u8,AdpllFsmCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmod(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, u8, AdpllFsmCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0xf,1,0,u8,u8,AdpllFsmCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tfine(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, AdpllFsmCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xf,1,0,u8,u8,AdpllFsmCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmedium(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, AdpllFsmCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,AdpllFsmCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tcoarse(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, AdpllFsmCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,AdpllFsmCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for AdpllFsmCtrlReg {
    #[inline(always)]
    fn default() -> AdpllFsmCtrlReg {
        <crate::RegValueT<AdpllFsmCtrlReg_SPEC> as RegisterValue<_>>::new(150515332)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdpllInitCtrlReg_SPEC;
impl crate::sealed::RegSpec for AdpllInitCtrlReg_SPEC {
    type DataType = u32;
}

pub type AdpllInitCtrlReg = crate::RegValueT<AdpllInitCtrlReg_SPEC>;

impl AdpllInitCtrlReg {
    #[inline(always)]
    pub fn dcomodic(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1fff,
        1,
        0,
        u16,
        u16,
        AdpllInitCtrlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1fff,
            1,
            0,
            u16,
            u16,
            AdpllInitCtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dcofineic(
        self,
    ) -> crate::common::RegisterField<8, 0x3f, 1, 0, u8, u8, AdpllInitCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            8,
            0x3f,
            1,
            0,
            u8,
            u8,
            AdpllInitCtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dcomediumic(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, u8, AdpllInitCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x7,1,0,u8,u8,AdpllInitCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcocoarseic(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, AdpllInitCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,AdpllInitCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for AdpllInitCtrlReg {
    #[inline(always)]
    fn default() -> AdpllInitCtrlReg {
        <crate::RegValueT<AdpllInitCtrlReg_SPEC> as RegisterValue<_>>::new(5)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdpllKdcoCalCtrl1Reg_SPEC;
impl crate::sealed::RegSpec for AdpllKdcoCalCtrl1Reg_SPEC {
    type DataType = u32;
}

pub type AdpllKdcoCalCtrl1Reg = crate::RegValueT<AdpllKdcoCalCtrl1Reg_SPEC>;

impl AdpllKdcoCalCtrl1Reg {
    #[inline(always)]
    pub fn kdcolf_in_1m(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        AdpllKdcoCalCtrl1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            AdpllKdcoCalCtrl1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn kdcohfic_1m(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        AdpllKdcoCalCtrl1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            AdpllKdcoCalCtrl1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for AdpllKdcoCalCtrl1Reg {
    #[inline(always)]
    fn default() -> AdpllKdcoCalCtrl1Reg {
        <crate::RegValueT<AdpllKdcoCalCtrl1Reg_SPEC> as RegisterValue<_>>::new(39835)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdpllKdcoCalCtrl2Reg_SPEC;
impl crate::sealed::RegSpec for AdpllKdcoCalCtrl2Reg_SPEC {
    type DataType = u32;
}

pub type AdpllKdcoCalCtrl2Reg = crate::RegValueT<AdpllKdcoCalCtrl2Reg_SPEC>;

impl AdpllKdcoCalCtrl2Reg {
    #[inline(always)]
    pub fn kdcoestdev(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x3,
        1,
        0,
        u8,
        u8,
        AdpllKdcoCalCtrl2Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x3,
            1,
            0,
            u8,
            u8,
            AdpllKdcoCalCtrl2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn kdcocaltx(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, AdpllKdcoCalCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<29,1,0,AdpllKdcoCalCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn kdcocalrx(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, AdpllKdcoCalCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<28,1,0,AdpllKdcoCalCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn kdcolfcalen(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, AdpllKdcoCalCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<27,1,0,AdpllKdcoCalCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tkdcocal(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x7,
        1,
        0,
        u8,
        u8,
        AdpllKdcoCalCtrl2Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x7,
            1,
            0,
            u8,
            u8,
            AdpllKdcoCalCtrl2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn kdcocn_ic(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7f,
        1,
        0,
        u8,
        u8,
        AdpllKdcoCalCtrl2Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7f,
            1,
            0,
            u8,
            u8,
            AdpllKdcoCalCtrl2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn kmod_alpha_1m(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        u8,
        u8,
        AdpllKdcoCalCtrl2Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            u8,
            u8,
            AdpllKdcoCalCtrl2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for AdpllKdcoCalCtrl2Reg {
    #[inline(always)]
    fn default() -> AdpllKdcoCalCtrl2Reg {
        <crate::RegValueT<AdpllKdcoCalCtrl2Reg_SPEC> as RegisterValue<_>>::new(1276379146)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdpllKdcoRdReg_SPEC;
impl crate::sealed::RegSpec for AdpllKdcoRdReg_SPEC {
    type DataType = u32;
}

pub type AdpllKdcoRdReg = crate::RegValueT<AdpllKdcoRdReg_SPEC>;

impl AdpllKdcoRdReg {
    #[inline(always)]
    pub fn cal_kdcocal(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, AdpllKdcoRdReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23,1,0,AdpllKdcoRdReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn kdcocn(
        self,
    ) -> crate::common::RegisterField<16, 0x7f, 1, 0, u8, u8, AdpllKdcoRdReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0x7f,1,0,u8,u8,AdpllKdcoRdReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn kdco_hf_out(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, AdpllKdcoRdReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,AdpllKdcoRdReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn kdco_hf_int(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, AdpllKdcoRdReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,AdpllKdcoRdReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for AdpllKdcoRdReg {
    #[inline(always)]
    fn default() -> AdpllKdcoRdReg {
        <crate::RegValueT<AdpllKdcoRdReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdpllKdtctdcCalCtrl1Reg_SPEC;
impl crate::sealed::RegSpec for AdpllKdtctdcCalCtrl1Reg_SPEC {
    type DataType = u32;
}

pub type AdpllKdtctdcCalCtrl1Reg = crate::RegValueT<AdpllKdtctdcCalCtrl1Reg_SPEC>;

impl AdpllKdtctdcCalCtrl1Reg {
    #[inline(always)]
    pub fn kdtcic(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1ff,
        1,
        0,
        u16,
        u16,
        AdpllKdtctdcCalCtrl1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1ff,
            1,
            0,
            u16,
            u16,
            AdpllKdtctdcCalCtrl1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn kdtccn_ic(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7f,
        1,
        0,
        u8,
        u8,
        AdpllKdtctdcCalCtrl1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7f,
            1,
            0,
            u8,
            u8,
            AdpllKdtctdcCalCtrl1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn kdtc_pipeline_bypass(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, AdpllKdtctdcCalCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<15,1,0,AdpllKdtctdcCalCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ktdc_in(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1ff,
        1,
        0,
        u16,
        u16,
        AdpllKdtctdcCalCtrl1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1ff,
            1,
            0,
            u16,
            u16,
            AdpllKdtctdcCalCtrl1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn kdtc_alpha(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3f,
        1,
        0,
        u8,
        u8,
        AdpllKdtctdcCalCtrl1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3f,
            1,
            0,
            u8,
            u8,
            AdpllKdtctdcCalCtrl1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for AdpllKdtctdcCalCtrl1Reg {
    #[inline(always)]
    fn default() -> AdpllKdtctdcCalCtrl1Reg {
        <crate::RegValueT<AdpllKdtctdcCalCtrl1Reg_SPEC> as RegisterValue<_>>::new(2147483647)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdpllKdtctdcCalCtrl2Reg_SPEC;
impl crate::sealed::RegSpec for AdpllKdtctdcCalCtrl2Reg_SPEC {
    type DataType = u32;
}

pub type AdpllKdtctdcCalCtrl2Reg = crate::RegValueT<AdpllKdtctdcCalCtrl2Reg_SPEC>;

impl AdpllKdtctdcCalCtrl2Reg {
    #[inline(always)]
    pub fn phrdly_extra(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, AdpllKdtctdcCalCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<15,1,0,AdpllKdtctdcCalCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tkdtccal(
        self,
    ) -> crate::common::RegisterField<
        11,
        0xf,
        1,
        0,
        u8,
        u8,
        AdpllKdtctdcCalCtrl2Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0xf,
            1,
            0,
            u8,
            u8,
            AdpllKdtctdcCalCtrl2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn phrdly(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x3,
        1,
        0,
        u8,
        u8,
        AdpllKdtctdcCalCtrl2Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x3,
            1,
            0,
            u8,
            u8,
            AdpllKdtctdcCalCtrl2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ktdccalen(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, AdpllKdtctdcCalCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<8,1,0,AdpllKdtctdcCalCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn kdtccallg(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x7,
        1,
        0,
        u8,
        u8,
        AdpllKdtctdcCalCtrl2Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x7,
            1,
            0,
            u8,
            u8,
            AdpllKdtctdcCalCtrl2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn kdtccal_inv(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, AdpllKdtctdcCalCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<3,1,0,AdpllKdtctdcCalCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn kdtccalmod1p(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, AdpllKdtctdcCalCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<2,1,0,AdpllKdtctdcCalCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn kdtccalmod(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, AdpllKdtctdcCalCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<1,1,0,AdpllKdtctdcCalCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn kdtccalen(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, AdpllKdtctdcCalCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<0,1,0,AdpllKdtctdcCalCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for AdpllKdtctdcCalCtrl2Reg {
    #[inline(always)]
    fn default() -> AdpllKdtctdcCalCtrl2Reg {
        <crate::RegValueT<AdpllKdtctdcCalCtrl2Reg_SPEC> as RegisterValue<_>>::new(32592)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdpllKdtcRdReg_SPEC;
impl crate::sealed::RegSpec for AdpllKdtcRdReg_SPEC {
    type DataType = u32;
}

pub type AdpllKdtcRdReg = crate::RegValueT<AdpllKdtcRdReg_SPEC>;

impl AdpllKdtcRdReg {
    #[inline(always)]
    pub fn cal_kdtccal(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, AdpllKdtcRdReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25,1,0,AdpllKdtcRdReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn kdtc_alpha_comp(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1ff,
        1,
        0,
        u16,
        u16,
        AdpllKdtcRdReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0x1ff,
            1,
            0,
            u16,
            u16,
            AdpllKdtcRdReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn kdtccn(
        self,
    ) -> crate::common::RegisterField<9, 0x7f, 1, 0, u8, u8, AdpllKdtcRdReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<9,0x7f,1,0,u8,u8,AdpllKdtcRdReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn kdtc_out(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, u16, AdpllKdtcRdReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            0,
            0x1ff,
            1,
            0,
            u16,
            u16,
            AdpllKdtcRdReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for AdpllKdtcRdReg {
    #[inline(always)]
    fn default() -> AdpllKdtcRdReg {
        <crate::RegValueT<AdpllKdtcRdReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdpllLfCtrl1Reg_SPEC;
impl crate::sealed::RegSpec for AdpllLfCtrl1Reg_SPEC {
    type DataType = u32;
}

pub type AdpllLfCtrl1Reg = crate::RegValueT<AdpllLfCtrl1Reg_SPEC>;

impl AdpllLfCtrl1Reg {
    #[inline(always)]
    pub fn finekz(
        self,
    ) -> crate::common::RegisterField<10, 0x3f, 1, 0, u8, u8, AdpllLfCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            10,
            0x3f,
            1,
            0,
            u8,
            u8,
            AdpllLfCtrl1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn finek(
        self,
    ) -> crate::common::RegisterField<5, 0x1f, 1, 0, u8, u8, AdpllLfCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1f,1,0,u8,u8,AdpllLfCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn finetau(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, AdpllLfCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,AdpllLfCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for AdpllLfCtrl1Reg {
    #[inline(always)]
    fn default() -> AdpllLfCtrl1Reg {
        <crate::RegValueT<AdpllLfCtrl1Reg_SPEC> as RegisterValue<_>>::new(33164)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdpllLfCtrl2Reg_SPEC;
impl crate::sealed::RegSpec for AdpllLfCtrl2Reg_SPEC {
    type DataType = u32;
}

pub type AdpllLfCtrl2Reg = crate::RegValueT<AdpllLfCtrl2Reg_SPEC>;

impl AdpllLfCtrl2Reg {
    #[inline(always)]
    pub fn rst_tau_en(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, AdpllLfCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30,1,0,AdpllLfCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn modkz(
        self,
    ) -> crate::common::RegisterField<24, 0x3f, 1, 0, u8, u8, AdpllLfCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            24,
            0x3f,
            1,
            0,
            u8,
            u8,
            AdpllLfCtrl2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn modk(
        self,
    ) -> crate::common::RegisterField<18, 0x3f, 1, 0, u8, u8, AdpllLfCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            18,
            0x3f,
            1,
            0,
            u8,
            u8,
            AdpllLfCtrl2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn modtau(
        self,
    ) -> crate::common::RegisterField<12, 0x3f, 1, 0, u8, u8, AdpllLfCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            12,
            0x3f,
            1,
            0,
            u8,
            u8,
            AdpllLfCtrl2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn modk_tune(
        self,
    ) -> crate::common::RegisterField<6, 0x3f, 1, 0, u8, u8, AdpllLfCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3f,1,0,u8,u8,AdpllLfCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn modtau_tune(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, AdpllLfCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,AdpllLfCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for AdpllLfCtrl2Reg {
    #[inline(always)]
    fn default() -> AdpllLfCtrl2Reg {
        <crate::RegValueT<AdpllLfCtrl2Reg_SPEC> as RegisterValue<_>>::new(480413598)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdpllMiscCtrlReg_SPEC;
impl crate::sealed::RegSpec for AdpllMiscCtrlReg_SPEC {
    type DataType = u32;
}

pub type AdpllMiscCtrlReg = crate::RegValueT<AdpllMiscCtrlReg_SPEC>;

impl AdpllMiscCtrlReg {
    #[inline(always)]
    pub fn phr_frac_preset_val(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xffff,
        1,
        0,
        u16,
        u16,
        AdpllMiscCtrlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xffff,
            1,
            0,
            u16,
            u16,
            AdpllMiscCtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn enfcwmod(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, AdpllMiscCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,AdpllMiscCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn enresidue(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, AdpllMiscCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,AdpllMiscCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn moddly(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, u8, AdpllMiscCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3,1,0,u8,u8,AdpllMiscCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn resdly(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, u8, AdpllMiscCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3,1,0,u8,u8,AdpllMiscCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dlyfcwdt(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, AdpllMiscCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,AdpllMiscCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for AdpllMiscCtrlReg {
    #[inline(always)]
    fn default() -> AdpllMiscCtrlReg {
        <crate::RegValueT<AdpllMiscCtrlReg_SPEC> as RegisterValue<_>>::new(218)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdpllMonCtrlReg_SPEC;
impl crate::sealed::RegSpec for AdpllMonCtrlReg_SPEC {
    type DataType = u32;
}

pub type AdpllMonCtrlReg = crate::RegValueT<AdpllMonCtrlReg_SPEC>;

impl AdpllMonCtrlReg {
    #[inline(always)]
    pub fn qualmonfrcen(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, AdpllMonCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30,1,0,AdpllMonCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn qualmontrhld(
        self,
    ) -> crate::common::RegisterField<24, 0x3f, 1, 0, u8, u8, AdpllMonCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            24,
            0x3f,
            1,
            0,
            u8,
            u8,
            AdpllMonCtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn qualmonwnd(
        self,
    ) -> crate::common::RegisterField<18, 0x3f, 1, 0, u8, u8, AdpllMonCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            18,
            0x3f,
            1,
            0,
            u8,
            u8,
            AdpllMonCtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn qualmonmod(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, u8, AdpllMonCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3,1,0,u8,u8,AdpllMonCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn hold_state(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, AdpllMonCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xf,1,0,u8,u8,AdpllMonCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfmeas_mode(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, AdpllMonCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,AdpllMonCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn enrfmeas(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, AdpllMonCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,AdpllMonCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmren(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, AdpllMonCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,AdpllMonCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tfreqmeas(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, AdpllMonCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,AdpllMonCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for AdpllMonCtrlReg {
    #[inline(always)]
    fn default() -> AdpllMonCtrlReg {
        <crate::RegValueT<AdpllMonCtrlReg_SPEC> as RegisterValue<_>>::new(16519043)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdpllOverruleCtrl1Reg_SPEC;
impl crate::sealed::RegSpec for AdpllOverruleCtrl1Reg_SPEC {
    type DataType = u32;
}

pub type AdpllOverruleCtrl1Reg = crate::RegValueT<AdpllOverruleCtrl1Reg_SPEC>;

impl AdpllOverruleCtrl1Reg {
    #[inline(always)]
    pub fn ovr_dtc_oh_wr(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x7f,
        1,
        0,
        u8,
        u8,
        AdpllOverruleCtrl1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x7f,
            1,
            0,
            u8,
            u8,
            AdpllOverruleCtrl1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovr_dtc_oh_sel(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, AdpllOverruleCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<24,1,0,AdpllOverruleCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ovr_dcoamp_wr(
        self,
    ) -> crate::common::RegisterField<
        17,
        0xf,
        1,
        0,
        u8,
        u8,
        AdpllOverruleCtrl1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0xf,
            1,
            0,
            u8,
            u8,
            AdpllOverruleCtrl1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovr_dcoamp_sel(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, AdpllOverruleCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<16,1,0,AdpllOverruleCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ovr_dcoamphold_wr(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, AdpllOverruleCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<13,1,0,AdpllOverruleCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ovr_dcoamphold_sel(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, AdpllOverruleCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<12,1,0,AdpllOverruleCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ovr_rdyfordiv_wr(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, AdpllOverruleCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<11,1,0,AdpllOverruleCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ovr_rdyfordiv_sel(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, AdpllOverruleCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<10,1,0,AdpllOverruleCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ovr_vpaen_wr(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, AdpllOverruleCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<9,1,0,AdpllOverruleCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ovr_vpaen_sel(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, AdpllOverruleCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<8,1,0,AdpllOverruleCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ovr_sresetn_wr(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, AdpllOverruleCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<7,1,0,AdpllOverruleCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ovr_sresetn_sel(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, AdpllOverruleCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<6,1,0,AdpllOverruleCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ovr_enpain_wr(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, AdpllOverruleCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<5,1,0,AdpllOverruleCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ovr_enpain_sel(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, AdpllOverruleCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<4,1,0,AdpllOverruleCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ovr_rxbit_wr(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, AdpllOverruleCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<3,1,0,AdpllOverruleCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ovr_rxbit_sel(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, AdpllOverruleCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<2,1,0,AdpllOverruleCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ovr_active_wr(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, AdpllOverruleCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<1,1,0,AdpllOverruleCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ovr_active_sel(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, AdpllOverruleCtrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<0,1,0,AdpllOverruleCtrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for AdpllOverruleCtrl1Reg {
    #[inline(always)]
    fn default() -> AdpllOverruleCtrl1Reg {
        <crate::RegValueT<AdpllOverruleCtrl1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdpllOverruleCtrl2Reg_SPEC;
impl crate::sealed::RegSpec for AdpllOverruleCtrl2Reg_SPEC {
    type DataType = u32;
}

pub type AdpllOverruleCtrl2Reg = crate::RegValueT<AdpllOverruleCtrl2Reg_SPEC>;

impl AdpllOverruleCtrl2Reg {
    #[inline(always)]
    pub fn ovr_dcomod_wr(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        u8,
        u8,
        AdpllOverruleCtrl2Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            u8,
            u8,
            AdpllOverruleCtrl2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovr_dcomod_sel(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, AdpllOverruleCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<23,1,0,AdpllOverruleCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ovr_dcofine_wr(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x3f,
        1,
        0,
        u8,
        u8,
        AdpllOverruleCtrl2Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x3f,
            1,
            0,
            u8,
            u8,
            AdpllOverruleCtrl2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovr_dcofine_sel(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, AdpllOverruleCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<16,1,0,AdpllOverruleCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ovr_dcomedium_wr(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x7,
        1,
        0,
        u8,
        u8,
        AdpllOverruleCtrl2Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x7,
            1,
            0,
            u8,
            u8,
            AdpllOverruleCtrl2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovr_dcomedium_sel(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, AdpllOverruleCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<8,1,0,AdpllOverruleCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ovr_dcocoarse_wr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0xf,
        1,
        0,
        u8,
        u8,
        AdpllOverruleCtrl2Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0xf,
            1,
            0,
            u8,
            u8,
            AdpllOverruleCtrl2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ovr_dcocoarse_sel(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, AdpllOverruleCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<0,1,0,AdpllOverruleCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for AdpllOverruleCtrl2Reg {
    #[inline(always)]
    fn default() -> AdpllOverruleCtrl2Reg {
        <crate::RegValueT<AdpllOverruleCtrl2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdpllOverruleCtrl3Reg_SPEC;
impl crate::sealed::RegSpec for AdpllOverruleCtrl3Reg_SPEC {
    type DataType = u32;
}

pub type AdpllOverruleCtrl3Reg = crate::RegValueT<AdpllOverruleCtrl3Reg_SPEC>;

impl AdpllOverruleCtrl3Reg {
    #[inline(always)]
    pub fn ovr_rxdiv_fb_en_wr(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, AdpllOverruleCtrl3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<7,1,0,AdpllOverruleCtrl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ovr_rxdiv_fb_en_sel(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, AdpllOverruleCtrl3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<6,1,0,AdpllOverruleCtrl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ovr_fbdiv_en_wr(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, AdpllOverruleCtrl3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<5,1,0,AdpllOverruleCtrl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ovr_fbdiv_en_sel(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, AdpllOverruleCtrl3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<4,1,0,AdpllOverruleCtrl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ovr_txdiv_en_wr(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, AdpllOverruleCtrl3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<3,1,0,AdpllOverruleCtrl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ovr_txdiv_en_sel(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, AdpllOverruleCtrl3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<2,1,0,AdpllOverruleCtrl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ovr_rxdiv_en_wr(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, AdpllOverruleCtrl3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<1,1,0,AdpllOverruleCtrl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ovr_rxdiv_en_sel(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, AdpllOverruleCtrl3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<0,1,0,AdpllOverruleCtrl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for AdpllOverruleCtrl3Reg {
    #[inline(always)]
    fn default() -> AdpllOverruleCtrl3Reg {
        <crate::RegValueT<AdpllOverruleCtrl3Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdpllPllfcwdtRdReg_SPEC;
impl crate::sealed::RegSpec for AdpllPllfcwdtRdReg_SPEC {
    type DataType = u32;
}

pub type AdpllPllfcwdtRdReg = crate::RegValueT<AdpllPllfcwdtRdReg_SPEC>;

impl AdpllPllfcwdtRdReg {
    #[inline(always)]
    pub fn pllfcwdt(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7fffff,
        1,
        0,
        u32,
        u32,
        AdpllPllfcwdtRdReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x7fffff,
            1,
            0,
            u32,
            u32,
            AdpllPllfcwdtRdReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for AdpllPllfcwdtRdReg {
    #[inline(always)]
    fn default() -> AdpllPllfcwdtRdReg {
        <crate::RegValueT<AdpllPllfcwdtRdReg_SPEC> as RegisterValue<_>>::new(2459648)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdpllRfptCtrlReg_SPEC;
impl crate::sealed::RegSpec for AdpllRfptCtrlReg_SPEC {
    type DataType = u32;
}

pub type AdpllRfptCtrlReg = crate::RegValueT<AdpllRfptCtrlReg_SPEC>;

impl AdpllRfptCtrlReg {
    #[inline(always)]
    pub fn rfpt_rate(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, AdpllRfptCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,AdpllRfptCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn inv_ckrfpt(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, AdpllRfptCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,AdpllRfptCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfpt_mux(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, AdpllRfptCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,AdpllRfptCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for AdpllRfptCtrlReg {
    #[inline(always)]
    fn default() -> AdpllRfptCtrlReg {
        <crate::RegValueT<AdpllRfptCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdpllSdmodCtrlReg_SPEC;
impl crate::sealed::RegSpec for AdpllSdmodCtrlReg_SPEC {
    type DataType = u32;
}

pub type AdpllSdmodCtrlReg = crate::RegValueT<AdpllSdmodCtrlReg_SPEC>;

impl AdpllSdmodCtrlReg {
    #[inline(always)]
    pub fn sdmmodetx(
        self,
    ) -> crate::common::RegisterField<3, 0x7, 1, 0, u8, u8, AdpllSdmodCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            3,
            0x7,
            1,
            0,
            u8,
            u8,
            AdpllSdmodCtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn sdmmoderx(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, AdpllSdmodCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            u8,
            u8,
            AdpllSdmodCtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for AdpllSdmodCtrlReg {
    #[inline(always)]
    fn default() -> AdpllSdmodCtrlReg {
        <crate::RegValueT<AdpllSdmodCtrlReg_SPEC> as RegisterValue<_>>::new(57)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdpllTunestateRdReg_SPEC;
impl crate::sealed::RegSpec for AdpllTunestateRdReg_SPEC {
    type DataType = u32;
}

pub type AdpllTunestateRdReg = crate::RegValueT<AdpllTunestateRdReg_SPEC>;

impl AdpllTunestateRdReg {
    #[inline(always)]
    pub fn tmrval(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3ff,
        1,
        0,
        u16,
        u16,
        AdpllTunestateRdReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x3ff,
            1,
            0,
            u16,
            u16,
            AdpllTunestateRdReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tune_state(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        u8,
        u8,
        AdpllTunestateRdReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            u8,
            u8,
            AdpllTunestateRdReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for AdpllTunestateRdReg {
    #[inline(always)]
    fn default() -> AdpllTunestateRdReg {
        <crate::RegValueT<AdpllTunestateRdReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdpllTxmodCtrlReg_SPEC;
impl crate::sealed::RegSpec for AdpllTxmodCtrlReg_SPEC {
    type DataType = u32;
}

pub type AdpllTxmodCtrlReg = crate::RegValueT<AdpllTxmodCtrlReg_SPEC>;

impl AdpllTxmodCtrlReg {
    #[inline(always)]
    pub fn inv_ckmodext(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, AdpllTxmodCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,AdpllTxmodCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tx_mode(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, u8, AdpllTxmodCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            6,
            0x3,
            1,
            0,
            u8,
            u8,
            AdpllTxmodCtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn eo_packet_dis(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, AdpllTxmodCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,AdpllTxmodCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn mod_index(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, u8, AdpllTxmodCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            2,
            0x3,
            1,
            0,
            u8,
            u8,
            AdpllTxmodCtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tx_data_inv(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, AdpllTxmodCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,AdpllTxmodCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bt_sel(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, AdpllTxmodCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,AdpllTxmodCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for AdpllTxmodCtrlReg {
    #[inline(always)]
    fn default() -> AdpllTxmodCtrlReg {
        <crate::RegValueT<AdpllTxmodCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
