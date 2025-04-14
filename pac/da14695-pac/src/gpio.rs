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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:16:21 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"GPIO registers"]
unsafe impl ::core::marker::Send for super::Gpio {}
unsafe impl ::core::marker::Sync for super::Gpio {}
impl super::Gpio {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn gpio_clk_sel_reg(
        &self,
    ) -> &'static crate::common::Reg<self::GpioClkSelReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::GpioClkSelReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(252usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p0_00_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P000ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P000ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p0_01_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P001ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P001ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p0_02_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P002ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P002ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p0_03_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P003ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P003ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p0_04_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P004ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P004ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p0_05_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P005ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P005ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p0_06_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P006ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P006ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p0_07_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P007ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P007ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p0_08_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P008ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P008ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p0_09_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P009ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P009ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p0_10_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P010ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P010ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p0_11_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P011ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P011ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(68usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p0_12_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P012ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P012ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(72usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p0_13_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P013ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P013ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(76usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p0_14_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P014ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P014ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p0_15_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P015ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P015ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(84usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p0_16_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P016ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P016ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(88usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p0_17_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P017ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P017ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(92usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p0_18_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P018ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P018ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(96usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p0_19_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P019ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P019ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(100usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p0_20_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P020ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P020ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(104usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p0_21_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P021ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P021ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(108usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p0_22_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P022ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P022ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(112usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p0_23_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P023ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P023ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(116usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p0_24_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P024ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P024ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(120usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p0_25_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P025ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P025ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(124usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p0_26_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P026ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P026ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(128usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p0_27_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P027ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P027ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(132usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p0_28_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P028ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P028ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(136usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p0_29_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P029ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P029ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(140usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p0_30_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P030ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P030ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(144usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p0_31_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P031ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P031ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(148usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p0_data_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P0DataReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P0DataReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p0_padpwr_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P0PadpwrCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P0PadpwrCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(244usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p0_reset_data_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P0ResetDataReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P0ResetDataReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p0_set_data_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P0SetDataReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P0SetDataReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p1_00_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P100ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P100ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(152usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p1_01_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P101ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P101ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(156usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p1_02_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P102ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P102ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(160usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p1_03_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P103ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P103ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(164usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p1_04_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P104ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P104ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(168usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p1_05_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P105ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P105ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(172usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p1_06_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P106ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P106ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(176usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p1_07_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P107ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P107ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(180usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p1_08_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P108ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P108ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(184usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p1_09_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P109ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P109ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(188usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p1_10_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P110ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P110ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(192usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p1_11_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P111ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P111ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(196usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p1_12_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P112ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P112ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(200usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p1_13_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P113ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P113ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(204usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p1_14_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P114ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P114ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(208usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p1_15_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P115ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P115ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(212usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p1_16_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P116ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P116ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(216usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p1_17_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P117ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P117ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(220usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p1_18_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P118ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P118ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(224usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p1_19_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P119ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P119ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(228usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p1_20_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P120ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P120ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(232usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p1_21_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P121ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P121ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(236usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p1_22_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P122ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P122ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(240usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p1_data_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P1DataReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P1DataReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p1_padpwr_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P1PadpwrCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P1PadpwrCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(248usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p1_reset_data_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P1ResetDataReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P1ResetDataReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p1_set_data_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P1SetDataReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P1SetDataReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn pad_weak_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PadWeakCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PadWeakCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(256usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpioClkSelReg_SPEC;
impl crate::sealed::RegSpec for GpioClkSelReg_SPEC {
    type DataType = u32;
}

pub type GpioClkSelReg = crate::RegValueT<GpioClkSelReg_SPEC>;

impl GpioClkSelReg {
    #[inline(always)]
    pub fn divn_output_en(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, GpioClkSelReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,GpioClkSelReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rc32m_output_en(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, GpioClkSelReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,GpioClkSelReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn xtal32m_output_en(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, GpioClkSelReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,GpioClkSelReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rcx_output_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, GpioClkSelReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,GpioClkSelReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rc32k_output_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, GpioClkSelReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,GpioClkSelReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn xtal32k_output_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, GpioClkSelReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,GpioClkSelReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn func_clock_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, GpioClkSelReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,GpioClkSelReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn func_clock_sel(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, GpioClkSelReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,GpioClkSelReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for GpioClkSelReg {
    #[inline(always)]
    fn default() -> GpioClkSelReg {
        <crate::RegValueT<GpioClkSelReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P000ModeReg_SPEC;
impl crate::sealed::RegSpec for P000ModeReg_SPEC {
    type DataType = u32;
}

pub type P000ModeReg = crate::RegValueT<P000ModeReg_SPEC>;

impl P000ModeReg {
    #[inline(always)]
    pub fn ppod(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, P000ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,P000ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P000ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P000ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, P000ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,P000ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P000ModeReg {
    #[inline(always)]
    fn default() -> P000ModeReg {
        <crate::RegValueT<P000ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P001ModeReg_SPEC;
impl crate::sealed::RegSpec for P001ModeReg_SPEC {
    type DataType = u32;
}

pub type P001ModeReg = crate::RegValueT<P001ModeReg_SPEC>;

impl P001ModeReg {
    #[inline(always)]
    pub fn ppod(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, P001ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,P001ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P001ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P001ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, P001ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,P001ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P001ModeReg {
    #[inline(always)]
    fn default() -> P001ModeReg {
        <crate::RegValueT<P001ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P002ModeReg_SPEC;
impl crate::sealed::RegSpec for P002ModeReg_SPEC {
    type DataType = u32;
}

pub type P002ModeReg = crate::RegValueT<P002ModeReg_SPEC>;

impl P002ModeReg {
    #[inline(always)]
    pub fn ppod(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, P002ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,P002ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P002ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P002ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, P002ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,P002ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P002ModeReg {
    #[inline(always)]
    fn default() -> P002ModeReg {
        <crate::RegValueT<P002ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P003ModeReg_SPEC;
impl crate::sealed::RegSpec for P003ModeReg_SPEC {
    type DataType = u32;
}

pub type P003ModeReg = crate::RegValueT<P003ModeReg_SPEC>;

impl P003ModeReg {
    #[inline(always)]
    pub fn ppod(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, P003ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,P003ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P003ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P003ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, P003ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,P003ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P003ModeReg {
    #[inline(always)]
    fn default() -> P003ModeReg {
        <crate::RegValueT<P003ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P004ModeReg_SPEC;
impl crate::sealed::RegSpec for P004ModeReg_SPEC {
    type DataType = u32;
}

pub type P004ModeReg = crate::RegValueT<P004ModeReg_SPEC>;

impl P004ModeReg {
    #[inline(always)]
    pub fn ppod(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, P004ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,P004ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P004ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P004ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, P004ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,P004ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P004ModeReg {
    #[inline(always)]
    fn default() -> P004ModeReg {
        <crate::RegValueT<P004ModeReg_SPEC> as RegisterValue<_>>::new(256)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P005ModeReg_SPEC;
impl crate::sealed::RegSpec for P005ModeReg_SPEC {
    type DataType = u32;
}

pub type P005ModeReg = crate::RegValueT<P005ModeReg_SPEC>;

impl P005ModeReg {
    #[inline(always)]
    pub fn ppod(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, P005ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,P005ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P005ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P005ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, P005ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,P005ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P005ModeReg {
    #[inline(always)]
    fn default() -> P005ModeReg {
        <crate::RegValueT<P005ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P006ModeReg_SPEC;
impl crate::sealed::RegSpec for P006ModeReg_SPEC {
    type DataType = u32;
}

pub type P006ModeReg = crate::RegValueT<P006ModeReg_SPEC>;

impl P006ModeReg {
    #[inline(always)]
    pub fn ppod(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, P006ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,P006ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P006ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P006ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, P006ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,P006ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P006ModeReg {
    #[inline(always)]
    fn default() -> P006ModeReg {
        <crate::RegValueT<P006ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P007ModeReg_SPEC;
impl crate::sealed::RegSpec for P007ModeReg_SPEC {
    type DataType = u32;
}

pub type P007ModeReg = crate::RegValueT<P007ModeReg_SPEC>;

impl P007ModeReg {
    #[inline(always)]
    pub fn ppod(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, P007ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,P007ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P007ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P007ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, P007ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,P007ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P007ModeReg {
    #[inline(always)]
    fn default() -> P007ModeReg {
        <crate::RegValueT<P007ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P008ModeReg_SPEC;
impl crate::sealed::RegSpec for P008ModeReg_SPEC {
    type DataType = u32;
}

pub type P008ModeReg = crate::RegValueT<P008ModeReg_SPEC>;

impl P008ModeReg {
    #[inline(always)]
    pub fn ppod(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, P008ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,P008ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P008ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P008ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, P008ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,P008ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P008ModeReg {
    #[inline(always)]
    fn default() -> P008ModeReg {
        <crate::RegValueT<P008ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P009ModeReg_SPEC;
impl crate::sealed::RegSpec for P009ModeReg_SPEC {
    type DataType = u32;
}

pub type P009ModeReg = crate::RegValueT<P009ModeReg_SPEC>;

impl P009ModeReg {
    #[inline(always)]
    pub fn ppod(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, P009ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,P009ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P009ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P009ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, P009ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,P009ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P009ModeReg {
    #[inline(always)]
    fn default() -> P009ModeReg {
        <crate::RegValueT<P009ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P010ModeReg_SPEC;
impl crate::sealed::RegSpec for P010ModeReg_SPEC {
    type DataType = u32;
}

pub type P010ModeReg = crate::RegValueT<P010ModeReg_SPEC>;

impl P010ModeReg {
    #[inline(always)]
    pub fn ppod(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, P010ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,P010ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P010ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P010ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, P010ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,P010ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P010ModeReg {
    #[inline(always)]
    fn default() -> P010ModeReg {
        <crate::RegValueT<P010ModeReg_SPEC> as RegisterValue<_>>::new(256)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P011ModeReg_SPEC;
impl crate::sealed::RegSpec for P011ModeReg_SPEC {
    type DataType = u32;
}

pub type P011ModeReg = crate::RegValueT<P011ModeReg_SPEC>;

impl P011ModeReg {
    #[inline(always)]
    pub fn ppod(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, P011ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,P011ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P011ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P011ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, P011ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,P011ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P011ModeReg {
    #[inline(always)]
    fn default() -> P011ModeReg {
        <crate::RegValueT<P011ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P012ModeReg_SPEC;
impl crate::sealed::RegSpec for P012ModeReg_SPEC {
    type DataType = u32;
}

pub type P012ModeReg = crate::RegValueT<P012ModeReg_SPEC>;

impl P012ModeReg {
    #[inline(always)]
    pub fn ppod(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, P012ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,P012ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P012ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P012ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, P012ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,P012ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P012ModeReg {
    #[inline(always)]
    fn default() -> P012ModeReg {
        <crate::RegValueT<P012ModeReg_SPEC> as RegisterValue<_>>::new(256)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P013ModeReg_SPEC;
impl crate::sealed::RegSpec for P013ModeReg_SPEC {
    type DataType = u32;
}

pub type P013ModeReg = crate::RegValueT<P013ModeReg_SPEC>;

impl P013ModeReg {
    #[inline(always)]
    pub fn ppod(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, P013ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,P013ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P013ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P013ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, P013ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,P013ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P013ModeReg {
    #[inline(always)]
    fn default() -> P013ModeReg {
        <crate::RegValueT<P013ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P014ModeReg_SPEC;
impl crate::sealed::RegSpec for P014ModeReg_SPEC {
    type DataType = u32;
}

pub type P014ModeReg = crate::RegValueT<P014ModeReg_SPEC>;

impl P014ModeReg {
    #[inline(always)]
    pub fn ppod(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, P014ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,P014ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P014ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P014ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, P014ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,P014ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P014ModeReg {
    #[inline(always)]
    fn default() -> P014ModeReg {
        <crate::RegValueT<P014ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P015ModeReg_SPEC;
impl crate::sealed::RegSpec for P015ModeReg_SPEC {
    type DataType = u32;
}

pub type P015ModeReg = crate::RegValueT<P015ModeReg_SPEC>;

impl P015ModeReg {
    #[inline(always)]
    pub fn ppod(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, P015ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,P015ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P015ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P015ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, P015ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,P015ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P015ModeReg {
    #[inline(always)]
    fn default() -> P015ModeReg {
        <crate::RegValueT<P015ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P016ModeReg_SPEC;
impl crate::sealed::RegSpec for P016ModeReg_SPEC {
    type DataType = u32;
}

pub type P016ModeReg = crate::RegValueT<P016ModeReg_SPEC>;

impl P016ModeReg {
    #[inline(always)]
    pub fn ppod(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, P016ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,P016ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P016ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P016ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, P016ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,P016ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P016ModeReg {
    #[inline(always)]
    fn default() -> P016ModeReg {
        <crate::RegValueT<P016ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P017ModeReg_SPEC;
impl crate::sealed::RegSpec for P017ModeReg_SPEC {
    type DataType = u32;
}

pub type P017ModeReg = crate::RegValueT<P017ModeReg_SPEC>;

impl P017ModeReg {
    #[inline(always)]
    pub fn ppod(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, P017ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,P017ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P017ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P017ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, P017ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,P017ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P017ModeReg {
    #[inline(always)]
    fn default() -> P017ModeReg {
        <crate::RegValueT<P017ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P018ModeReg_SPEC;
impl crate::sealed::RegSpec for P018ModeReg_SPEC {
    type DataType = u32;
}

pub type P018ModeReg = crate::RegValueT<P018ModeReg_SPEC>;

impl P018ModeReg {
    #[inline(always)]
    pub fn ppod(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, P018ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,P018ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P018ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P018ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, P018ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,P018ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P018ModeReg {
    #[inline(always)]
    fn default() -> P018ModeReg {
        <crate::RegValueT<P018ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P019ModeReg_SPEC;
impl crate::sealed::RegSpec for P019ModeReg_SPEC {
    type DataType = u32;
}

pub type P019ModeReg = crate::RegValueT<P019ModeReg_SPEC>;

impl P019ModeReg {
    #[inline(always)]
    pub fn ppod(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, P019ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,P019ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P019ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P019ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, P019ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,P019ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P019ModeReg {
    #[inline(always)]
    fn default() -> P019ModeReg {
        <crate::RegValueT<P019ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P020ModeReg_SPEC;
impl crate::sealed::RegSpec for P020ModeReg_SPEC {
    type DataType = u32;
}

pub type P020ModeReg = crate::RegValueT<P020ModeReg_SPEC>;

impl P020ModeReg {
    #[inline(always)]
    pub fn ppod(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, P020ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,P020ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P020ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P020ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, P020ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,P020ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P020ModeReg {
    #[inline(always)]
    fn default() -> P020ModeReg {
        <crate::RegValueT<P020ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P021ModeReg_SPEC;
impl crate::sealed::RegSpec for P021ModeReg_SPEC {
    type DataType = u32;
}

pub type P021ModeReg = crate::RegValueT<P021ModeReg_SPEC>;

impl P021ModeReg {
    #[inline(always)]
    pub fn ppod(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, P021ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,P021ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P021ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P021ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, P021ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,P021ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P021ModeReg {
    #[inline(always)]
    fn default() -> P021ModeReg {
        <crate::RegValueT<P021ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P022ModeReg_SPEC;
impl crate::sealed::RegSpec for P022ModeReg_SPEC {
    type DataType = u32;
}

pub type P022ModeReg = crate::RegValueT<P022ModeReg_SPEC>;

impl P022ModeReg {
    #[inline(always)]
    pub fn ppod(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, P022ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,P022ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P022ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P022ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, P022ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,P022ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P022ModeReg {
    #[inline(always)]
    fn default() -> P022ModeReg {
        <crate::RegValueT<P022ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P023ModeReg_SPEC;
impl crate::sealed::RegSpec for P023ModeReg_SPEC {
    type DataType = u32;
}

pub type P023ModeReg = crate::RegValueT<P023ModeReg_SPEC>;

impl P023ModeReg {
    #[inline(always)]
    pub fn ppod(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, P023ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,P023ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P023ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P023ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, P023ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,P023ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P023ModeReg {
    #[inline(always)]
    fn default() -> P023ModeReg {
        <crate::RegValueT<P023ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P024ModeReg_SPEC;
impl crate::sealed::RegSpec for P024ModeReg_SPEC {
    type DataType = u32;
}

pub type P024ModeReg = crate::RegValueT<P024ModeReg_SPEC>;

impl P024ModeReg {
    #[inline(always)]
    pub fn ppod(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, P024ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,P024ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P024ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P024ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, P024ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,P024ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P024ModeReg {
    #[inline(always)]
    fn default() -> P024ModeReg {
        <crate::RegValueT<P024ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P025ModeReg_SPEC;
impl crate::sealed::RegSpec for P025ModeReg_SPEC {
    type DataType = u32;
}

pub type P025ModeReg = crate::RegValueT<P025ModeReg_SPEC>;

impl P025ModeReg {
    #[inline(always)]
    pub fn ppod(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, P025ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,P025ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P025ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P025ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, P025ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,P025ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P025ModeReg {
    #[inline(always)]
    fn default() -> P025ModeReg {
        <crate::RegValueT<P025ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P026ModeReg_SPEC;
impl crate::sealed::RegSpec for P026ModeReg_SPEC {
    type DataType = u32;
}

pub type P026ModeReg = crate::RegValueT<P026ModeReg_SPEC>;

impl P026ModeReg {
    #[inline(always)]
    pub fn ppod(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, P026ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,P026ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P026ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P026ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, P026ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,P026ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P026ModeReg {
    #[inline(always)]
    fn default() -> P026ModeReg {
        <crate::RegValueT<P026ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P027ModeReg_SPEC;
impl crate::sealed::RegSpec for P027ModeReg_SPEC {
    type DataType = u32;
}

pub type P027ModeReg = crate::RegValueT<P027ModeReg_SPEC>;

impl P027ModeReg {
    #[inline(always)]
    pub fn ppod(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, P027ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,P027ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P027ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P027ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, P027ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,P027ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P027ModeReg {
    #[inline(always)]
    fn default() -> P027ModeReg {
        <crate::RegValueT<P027ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P028ModeReg_SPEC;
impl crate::sealed::RegSpec for P028ModeReg_SPEC {
    type DataType = u32;
}

pub type P028ModeReg = crate::RegValueT<P028ModeReg_SPEC>;

impl P028ModeReg {
    #[inline(always)]
    pub fn ppod(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, P028ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,P028ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P028ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P028ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, P028ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,P028ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P028ModeReg {
    #[inline(always)]
    fn default() -> P028ModeReg {
        <crate::RegValueT<P028ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P029ModeReg_SPEC;
impl crate::sealed::RegSpec for P029ModeReg_SPEC {
    type DataType = u32;
}

pub type P029ModeReg = crate::RegValueT<P029ModeReg_SPEC>;

impl P029ModeReg {
    #[inline(always)]
    pub fn ppod(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, P029ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,P029ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P029ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P029ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, P029ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,P029ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P029ModeReg {
    #[inline(always)]
    fn default() -> P029ModeReg {
        <crate::RegValueT<P029ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P030ModeReg_SPEC;
impl crate::sealed::RegSpec for P030ModeReg_SPEC {
    type DataType = u32;
}

pub type P030ModeReg = crate::RegValueT<P030ModeReg_SPEC>;

impl P030ModeReg {
    #[inline(always)]
    pub fn ppod(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, P030ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,P030ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P030ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P030ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, P030ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,P030ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P030ModeReg {
    #[inline(always)]
    fn default() -> P030ModeReg {
        <crate::RegValueT<P030ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P031ModeReg_SPEC;
impl crate::sealed::RegSpec for P031ModeReg_SPEC {
    type DataType = u32;
}

pub type P031ModeReg = crate::RegValueT<P031ModeReg_SPEC>;

impl P031ModeReg {
    #[inline(always)]
    pub fn ppod(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, P031ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,P031ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P031ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P031ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, P031ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,P031ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P031ModeReg {
    #[inline(always)]
    fn default() -> P031ModeReg {
        <crate::RegValueT<P031ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P0DataReg_SPEC;
impl crate::sealed::RegSpec for P0DataReg_SPEC {
    type DataType = u32;
}

pub type P0DataReg = crate::RegValueT<P0DataReg_SPEC>;

impl P0DataReg {
    #[inline(always)]
    pub fn p0_data(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        P0DataReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            P0DataReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P0DataReg {
    #[inline(always)]
    fn default() -> P0DataReg {
        <crate::RegValueT<P0DataReg_SPEC> as RegisterValue<_>>::new(5136)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P0PadpwrCtrlReg_SPEC;
impl crate::sealed::RegSpec for P0PadpwrCtrlReg_SPEC {
    type DataType = u32;
}

pub type P0PadpwrCtrlReg = crate::RegValueT<P0PadpwrCtrlReg_SPEC>;

impl P0PadpwrCtrlReg {
    #[inline(always)]
    pub fn p0_out_ctrl(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3ffffff,
        1,
        0,
        u32,
        u32,
        P0PadpwrCtrlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x3ffffff,
            1,
            0,
            u32,
            u32,
            P0PadpwrCtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P0PadpwrCtrlReg {
    #[inline(always)]
    fn default() -> P0PadpwrCtrlReg {
        <crate::RegValueT<P0PadpwrCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P0ResetDataReg_SPEC;
impl crate::sealed::RegSpec for P0ResetDataReg_SPEC {
    type DataType = u32;
}

pub type P0ResetDataReg = crate::RegValueT<P0ResetDataReg_SPEC>;

impl P0ResetDataReg {
    #[inline(always)]
    pub fn p0_reset(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        P0ResetDataReg_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            P0ResetDataReg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P0ResetDataReg {
    #[inline(always)]
    fn default() -> P0ResetDataReg {
        <crate::RegValueT<P0ResetDataReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P0SetDataReg_SPEC;
impl crate::sealed::RegSpec for P0SetDataReg_SPEC {
    type DataType = u32;
}

pub type P0SetDataReg = crate::RegValueT<P0SetDataReg_SPEC>;

impl P0SetDataReg {
    #[inline(always)]
    pub fn p0_set(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        P0SetDataReg_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            P0SetDataReg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P0SetDataReg {
    #[inline(always)]
    fn default() -> P0SetDataReg {
        <crate::RegValueT<P0SetDataReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P100ModeReg_SPEC;
impl crate::sealed::RegSpec for P100ModeReg_SPEC {
    type DataType = u32;
}

pub type P100ModeReg = crate::RegValueT<P100ModeReg_SPEC>;

impl P100ModeReg {
    #[inline(always)]
    pub fn ppod(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, P100ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,P100ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P100ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P100ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, P100ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,P100ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P100ModeReg {
    #[inline(always)]
    fn default() -> P100ModeReg {
        <crate::RegValueT<P100ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P101ModeReg_SPEC;
impl crate::sealed::RegSpec for P101ModeReg_SPEC {
    type DataType = u32;
}

pub type P101ModeReg = crate::RegValueT<P101ModeReg_SPEC>;

impl P101ModeReg {
    #[inline(always)]
    pub fn ppod(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, P101ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,P101ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P101ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P101ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, P101ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,P101ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P101ModeReg {
    #[inline(always)]
    fn default() -> P101ModeReg {
        <crate::RegValueT<P101ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P102ModeReg_SPEC;
impl crate::sealed::RegSpec for P102ModeReg_SPEC {
    type DataType = u32;
}

pub type P102ModeReg = crate::RegValueT<P102ModeReg_SPEC>;

impl P102ModeReg {
    #[inline(always)]
    pub fn ppod(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, P102ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,P102ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P102ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P102ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, P102ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,P102ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P102ModeReg {
    #[inline(always)]
    fn default() -> P102ModeReg {
        <crate::RegValueT<P102ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P103ModeReg_SPEC;
impl crate::sealed::RegSpec for P103ModeReg_SPEC {
    type DataType = u32;
}

pub type P103ModeReg = crate::RegValueT<P103ModeReg_SPEC>;

impl P103ModeReg {
    #[inline(always)]
    pub fn ppod(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, P103ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,P103ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P103ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P103ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, P103ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,P103ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P103ModeReg {
    #[inline(always)]
    fn default() -> P103ModeReg {
        <crate::RegValueT<P103ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P104ModeReg_SPEC;
impl crate::sealed::RegSpec for P104ModeReg_SPEC {
    type DataType = u32;
}

pub type P104ModeReg = crate::RegValueT<P104ModeReg_SPEC>;

impl P104ModeReg {
    #[inline(always)]
    pub fn ppod(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, P104ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,P104ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P104ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P104ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, P104ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,P104ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P104ModeReg {
    #[inline(always)]
    fn default() -> P104ModeReg {
        <crate::RegValueT<P104ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P105ModeReg_SPEC;
impl crate::sealed::RegSpec for P105ModeReg_SPEC {
    type DataType = u32;
}

pub type P105ModeReg = crate::RegValueT<P105ModeReg_SPEC>;

impl P105ModeReg {
    #[inline(always)]
    pub fn ppod(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, P105ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,P105ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P105ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P105ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, P105ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,P105ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P105ModeReg {
    #[inline(always)]
    fn default() -> P105ModeReg {
        <crate::RegValueT<P105ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P106ModeReg_SPEC;
impl crate::sealed::RegSpec for P106ModeReg_SPEC {
    type DataType = u32;
}

pub type P106ModeReg = crate::RegValueT<P106ModeReg_SPEC>;

impl P106ModeReg {
    #[inline(always)]
    pub fn ppod(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, P106ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,P106ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P106ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P106ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, P106ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,P106ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P106ModeReg {
    #[inline(always)]
    fn default() -> P106ModeReg {
        <crate::RegValueT<P106ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P107ModeReg_SPEC;
impl crate::sealed::RegSpec for P107ModeReg_SPEC {
    type DataType = u32;
}

pub type P107ModeReg = crate::RegValueT<P107ModeReg_SPEC>;

impl P107ModeReg {
    #[inline(always)]
    pub fn ppod(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, P107ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,P107ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P107ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P107ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, P107ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,P107ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P107ModeReg {
    #[inline(always)]
    fn default() -> P107ModeReg {
        <crate::RegValueT<P107ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P108ModeReg_SPEC;
impl crate::sealed::RegSpec for P108ModeReg_SPEC {
    type DataType = u32;
}

pub type P108ModeReg = crate::RegValueT<P108ModeReg_SPEC>;

impl P108ModeReg {
    #[inline(always)]
    pub fn ppod(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, P108ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,P108ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P108ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P108ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, P108ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,P108ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P108ModeReg {
    #[inline(always)]
    fn default() -> P108ModeReg {
        <crate::RegValueT<P108ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P109ModeReg_SPEC;
impl crate::sealed::RegSpec for P109ModeReg_SPEC {
    type DataType = u32;
}

pub type P109ModeReg = crate::RegValueT<P109ModeReg_SPEC>;

impl P109ModeReg {
    #[inline(always)]
    pub fn ppod(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, P109ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,P109ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P109ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P109ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, P109ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,P109ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P109ModeReg {
    #[inline(always)]
    fn default() -> P109ModeReg {
        <crate::RegValueT<P109ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P110ModeReg_SPEC;
impl crate::sealed::RegSpec for P110ModeReg_SPEC {
    type DataType = u32;
}

pub type P110ModeReg = crate::RegValueT<P110ModeReg_SPEC>;

impl P110ModeReg {
    #[inline(always)]
    pub fn ppod(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, P110ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,P110ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P110ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P110ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, P110ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,P110ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P110ModeReg {
    #[inline(always)]
    fn default() -> P110ModeReg {
        <crate::RegValueT<P110ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P111ModeReg_SPEC;
impl crate::sealed::RegSpec for P111ModeReg_SPEC {
    type DataType = u32;
}

pub type P111ModeReg = crate::RegValueT<P111ModeReg_SPEC>;

impl P111ModeReg {
    #[inline(always)]
    pub fn ppod(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, P111ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,P111ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P111ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P111ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, P111ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,P111ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P111ModeReg {
    #[inline(always)]
    fn default() -> P111ModeReg {
        <crate::RegValueT<P111ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P112ModeReg_SPEC;
impl crate::sealed::RegSpec for P112ModeReg_SPEC {
    type DataType = u32;
}

pub type P112ModeReg = crate::RegValueT<P112ModeReg_SPEC>;

impl P112ModeReg {
    #[inline(always)]
    pub fn ppod(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, P112ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,P112ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P112ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P112ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, P112ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,P112ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P112ModeReg {
    #[inline(always)]
    fn default() -> P112ModeReg {
        <crate::RegValueT<P112ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P113ModeReg_SPEC;
impl crate::sealed::RegSpec for P113ModeReg_SPEC {
    type DataType = u32;
}

pub type P113ModeReg = crate::RegValueT<P113ModeReg_SPEC>;

impl P113ModeReg {
    #[inline(always)]
    pub fn ppod(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, P113ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,P113ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P113ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P113ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, P113ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,P113ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P113ModeReg {
    #[inline(always)]
    fn default() -> P113ModeReg {
        <crate::RegValueT<P113ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P114ModeReg_SPEC;
impl crate::sealed::RegSpec for P114ModeReg_SPEC {
    type DataType = u32;
}

pub type P114ModeReg = crate::RegValueT<P114ModeReg_SPEC>;

impl P114ModeReg {
    #[inline(always)]
    pub fn ppod(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, P114ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,P114ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P114ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P114ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, P114ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,P114ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P114ModeReg {
    #[inline(always)]
    fn default() -> P114ModeReg {
        <crate::RegValueT<P114ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P115ModeReg_SPEC;
impl crate::sealed::RegSpec for P115ModeReg_SPEC {
    type DataType = u32;
}

pub type P115ModeReg = crate::RegValueT<P115ModeReg_SPEC>;

impl P115ModeReg {
    #[inline(always)]
    pub fn ppod(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, P115ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,P115ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P115ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P115ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, P115ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,P115ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P115ModeReg {
    #[inline(always)]
    fn default() -> P115ModeReg {
        <crate::RegValueT<P115ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P116ModeReg_SPEC;
impl crate::sealed::RegSpec for P116ModeReg_SPEC {
    type DataType = u32;
}

pub type P116ModeReg = crate::RegValueT<P116ModeReg_SPEC>;

impl P116ModeReg {
    #[inline(always)]
    pub fn ppod(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, P116ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,P116ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P116ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P116ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, P116ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,P116ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P116ModeReg {
    #[inline(always)]
    fn default() -> P116ModeReg {
        <crate::RegValueT<P116ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P117ModeReg_SPEC;
impl crate::sealed::RegSpec for P117ModeReg_SPEC {
    type DataType = u32;
}

pub type P117ModeReg = crate::RegValueT<P117ModeReg_SPEC>;

impl P117ModeReg {
    #[inline(always)]
    pub fn ppod(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, P117ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,P117ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P117ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P117ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, P117ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,P117ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P117ModeReg {
    #[inline(always)]
    fn default() -> P117ModeReg {
        <crate::RegValueT<P117ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P118ModeReg_SPEC;
impl crate::sealed::RegSpec for P118ModeReg_SPEC {
    type DataType = u32;
}

pub type P118ModeReg = crate::RegValueT<P118ModeReg_SPEC>;

impl P118ModeReg {
    #[inline(always)]
    pub fn ppod(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, P118ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,P118ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P118ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P118ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, P118ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,P118ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P118ModeReg {
    #[inline(always)]
    fn default() -> P118ModeReg {
        <crate::RegValueT<P118ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P119ModeReg_SPEC;
impl crate::sealed::RegSpec for P119ModeReg_SPEC {
    type DataType = u32;
}

pub type P119ModeReg = crate::RegValueT<P119ModeReg_SPEC>;

impl P119ModeReg {
    #[inline(always)]
    pub fn ppod(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, P119ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,P119ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P119ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P119ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, P119ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,P119ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P119ModeReg {
    #[inline(always)]
    fn default() -> P119ModeReg {
        <crate::RegValueT<P119ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P120ModeReg_SPEC;
impl crate::sealed::RegSpec for P120ModeReg_SPEC {
    type DataType = u32;
}

pub type P120ModeReg = crate::RegValueT<P120ModeReg_SPEC>;

impl P120ModeReg {
    #[inline(always)]
    pub fn ppod(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, P120ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,P120ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P120ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P120ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, P120ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,P120ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P120ModeReg {
    #[inline(always)]
    fn default() -> P120ModeReg {
        <crate::RegValueT<P120ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P121ModeReg_SPEC;
impl crate::sealed::RegSpec for P121ModeReg_SPEC {
    type DataType = u32;
}

pub type P121ModeReg = crate::RegValueT<P121ModeReg_SPEC>;

impl P121ModeReg {
    #[inline(always)]
    pub fn ppod(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, P121ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,P121ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P121ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P121ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, P121ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,P121ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P121ModeReg {
    #[inline(always)]
    fn default() -> P121ModeReg {
        <crate::RegValueT<P121ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P122ModeReg_SPEC;
impl crate::sealed::RegSpec for P122ModeReg_SPEC {
    type DataType = u32;
}

pub type P122ModeReg = crate::RegValueT<P122ModeReg_SPEC>;

impl P122ModeReg {
    #[inline(always)]
    pub fn ppod(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, P122ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,P122ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P122ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P122ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, P122ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,P122ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P122ModeReg {
    #[inline(always)]
    fn default() -> P122ModeReg {
        <crate::RegValueT<P122ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P1DataReg_SPEC;
impl crate::sealed::RegSpec for P1DataReg_SPEC {
    type DataType = u32;
}

pub type P1DataReg = crate::RegValueT<P1DataReg_SPEC>;

impl P1DataReg {
    #[inline(always)]
    pub fn p1_data(
        self,
    ) -> crate::common::RegisterField<0, 0x7fffff, 1, 0, u32, u32, P1DataReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7fffff,1,0,u32,u32,P1DataReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P1DataReg {
    #[inline(always)]
    fn default() -> P1DataReg {
        <crate::RegValueT<P1DataReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P1PadpwrCtrlReg_SPEC;
impl crate::sealed::RegSpec for P1PadpwrCtrlReg_SPEC {
    type DataType = u32;
}

pub type P1PadpwrCtrlReg = crate::RegValueT<P1PadpwrCtrlReg_SPEC>;

impl P1PadpwrCtrlReg {
    #[inline(always)]
    pub fn p1_out_ctrl(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7fffff,
        1,
        0,
        u32,
        u32,
        P1PadpwrCtrlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7fffff,
            1,
            0,
            u32,
            u32,
            P1PadpwrCtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P1PadpwrCtrlReg {
    #[inline(always)]
    fn default() -> P1PadpwrCtrlReg {
        <crate::RegValueT<P1PadpwrCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P1ResetDataReg_SPEC;
impl crate::sealed::RegSpec for P1ResetDataReg_SPEC {
    type DataType = u32;
}

pub type P1ResetDataReg = crate::RegValueT<P1ResetDataReg_SPEC>;

impl P1ResetDataReg {
    #[inline(always)]
    pub fn p1_reset(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7fffff,
        1,
        0,
        u32,
        u32,
        P1ResetDataReg_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x7fffff,
            1,
            0,
            u32,
            u32,
            P1ResetDataReg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P1ResetDataReg {
    #[inline(always)]
    fn default() -> P1ResetDataReg {
        <crate::RegValueT<P1ResetDataReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P1SetDataReg_SPEC;
impl crate::sealed::RegSpec for P1SetDataReg_SPEC {
    type DataType = u32;
}

pub type P1SetDataReg = crate::RegValueT<P1SetDataReg_SPEC>;

impl P1SetDataReg {
    #[inline(always)]
    pub fn p1_set(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7fffff,
        1,
        0,
        u32,
        u32,
        P1SetDataReg_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x7fffff,
            1,
            0,
            u32,
            u32,
            P1SetDataReg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P1SetDataReg {
    #[inline(always)]
    fn default() -> P1SetDataReg {
        <crate::RegValueT<P1SetDataReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadWeakCtrlReg_SPEC;
impl crate::sealed::RegSpec for PadWeakCtrlReg_SPEC {
    type DataType = u32;
}

pub type PadWeakCtrlReg = crate::RegValueT<PadWeakCtrlReg_SPEC>;

impl PadWeakCtrlReg {
    #[inline(always)]
    pub fn p1_09_lowdrv(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, PadWeakCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,PadWeakCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn p1_06_lowdrv(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, PadWeakCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,PadWeakCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn p1_02_lowdrv(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, PadWeakCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,PadWeakCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn p1_01_lowdrv(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, PadWeakCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,PadWeakCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn p1_00_lowdrv(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, PadWeakCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,PadWeakCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn p0_27_lowdrv(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, PadWeakCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,PadWeakCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn p0_26_lowdrv(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, PadWeakCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,PadWeakCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn p0_25_lowdrv(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, PadWeakCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,PadWeakCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn p0_18_lowdrv(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, PadWeakCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,PadWeakCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn p0_17_lowdrv(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, PadWeakCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,PadWeakCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn p0_16_lowdrv(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, PadWeakCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,PadWeakCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn p0_07_lowdrv(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, PadWeakCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,PadWeakCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn p0_06_lowdrv(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, PadWeakCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,PadWeakCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PadWeakCtrlReg {
    #[inline(always)]
    fn default() -> PadWeakCtrlReg {
        <crate::RegValueT<PadWeakCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
