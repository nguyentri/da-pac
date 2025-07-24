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
// Generated from SVD 1.2, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:44:57 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"KBSCAN registers"]
unsafe impl ::core::marker::Send for super::Kbscan {}
unsafe impl ::core::marker::Sync for super::Kbscan {}
impl super::Kbscan {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Keyboard scanner control 2 register"]
    #[inline(always)]
    pub const fn kbscn_ctrl2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::KbscnCtrl2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::KbscnCtrl2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[doc = "Keyboard scanner control register"]
    #[inline(always)]
    pub const fn kbscn_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::KbscnCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::KbscnCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Defines the debounce time for key press and release"]
    #[inline(always)]
    pub const fn kbscn_debounce_reg(
        &self,
    ) -> &'static crate::common::Reg<self::KbscnDebounceReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::KbscnDebounceReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[doc = "Defines the number of rows and columns of the matrix"]
    #[inline(always)]
    pub const fn kbscn_matrix_size_reg(
        &self,
    ) -> &'static crate::common::Reg<self::KbscnMatrixSizeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::KbscnMatrixSizeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Returns a key message from the message queue"]
    #[inline(always)]
    pub const fn kbscn_message_key_reg(
        &self,
    ) -> &'static crate::common::Reg<self::KbscnMessageKeyReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::KbscnMessageKeyReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(10usize),
            )
        }
    }

    #[doc = "Defines the keyboard mode for P00"]
    #[inline(always)]
    pub const fn kbscn_p00_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::KbscnP00ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::KbscnP00ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Defines the keyboard mode for P01"]
    #[inline(always)]
    pub const fn kbscn_p01_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::KbscnP01ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::KbscnP01ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(14usize),
            )
        }
    }

    #[doc = "Defines the keyboard mode for P02"]
    #[inline(always)]
    pub const fn kbscn_p02_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::KbscnP02ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::KbscnP02ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Defines the keyboard mode for P03"]
    #[inline(always)]
    pub const fn kbscn_p03_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::KbscnP03ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::KbscnP03ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(18usize),
            )
        }
    }

    #[doc = "Defines the keyboard mode for P04"]
    #[inline(always)]
    pub const fn kbscn_p04_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::KbscnP04ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::KbscnP04ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "Defines the keyboard mode for P05"]
    #[inline(always)]
    pub const fn kbscn_p05_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::KbscnP05ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::KbscnP05ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(22usize),
            )
        }
    }

    #[doc = "Defines the keyboard mode for P06"]
    #[inline(always)]
    pub const fn kbscn_p06_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::KbscnP06ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::KbscnP06ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "Defines the keyboard mode for P07"]
    #[inline(always)]
    pub const fn kbscn_p07_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::KbscnP07ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::KbscnP07ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(26usize),
            )
        }
    }

    #[doc = "Defines the keyboard mode for P10"]
    #[inline(always)]
    pub const fn kbscn_p10_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::KbscnP10ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::KbscnP10ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "Defines the keyboard mode for P11"]
    #[inline(always)]
    pub const fn kbscn_p11_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::KbscnP11ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::KbscnP11ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(30usize),
            )
        }
    }

    #[doc = "Defines the keyboard mode for P12"]
    #[inline(always)]
    pub const fn kbscn_p12_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::KbscnP12ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::KbscnP12ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "Defines the keyboard mode for P13"]
    #[inline(always)]
    pub const fn kbscn_p13_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::KbscnP13ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::KbscnP13ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(34usize),
            )
        }
    }

    #[doc = "Defines the keyboard mode for P14"]
    #[inline(always)]
    pub const fn kbscn_p14_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::KbscnP14ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::KbscnP14ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[doc = "Defines the keyboard mode for P15"]
    #[inline(always)]
    pub const fn kbscn_p15_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::KbscnP15ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::KbscnP15ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(38usize),
            )
        }
    }

    #[doc = "Defines the keyboard mode for P16"]
    #[inline(always)]
    pub const fn kbscn_p16_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::KbscnP16ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::KbscnP16ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[doc = "Defines the keyboard mode for P17"]
    #[inline(always)]
    pub const fn kbscn_p17_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::KbscnP17ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::KbscnP17ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(42usize),
            )
        }
    }

    #[doc = "Defines the keyboard mode for P20"]
    #[inline(always)]
    pub const fn kbscn_p20_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::KbscnP20ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::KbscnP20ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[doc = "Defines the keyboard mode for P21"]
    #[inline(always)]
    pub const fn kbscn_p21_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::KbscnP21ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::KbscnP21ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(46usize),
            )
        }
    }

    #[doc = "Defines the keyboard mode for P22"]
    #[inline(always)]
    pub const fn kbscn_p22_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::KbscnP22ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::KbscnP22ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[doc = "Defines the keyboard mode for P23"]
    #[inline(always)]
    pub const fn kbscn_p23_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::KbscnP23ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::KbscnP23ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(50usize),
            )
        }
    }

    #[doc = "Defines the keyboard mode for P24"]
    #[inline(always)]
    pub const fn kbscn_p24_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::KbscnP24ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::KbscnP24ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[doc = "Defines the keyboard mode for P30"]
    #[inline(always)]
    pub const fn kbscn_p30_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::KbscnP30ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::KbscnP30ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }

    #[doc = "Defines the keyboard mode for P31"]
    #[inline(always)]
    pub const fn kbscn_p31_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::KbscnP31ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::KbscnP31ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(62usize),
            )
        }
    }

    #[doc = "Defines the keyboard mode for P32"]
    #[inline(always)]
    pub const fn kbscn_p32_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::KbscnP32ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::KbscnP32ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[doc = "Defines the keyboard mode for P33"]
    #[inline(always)]
    pub const fn kbscn_p33_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::KbscnP33ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::KbscnP33ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(66usize),
            )
        }
    }

    #[doc = "Defines the keyboard mode for P34"]
    #[inline(always)]
    pub const fn kbscn_p34_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::KbscnP34ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::KbscnP34ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(68usize),
            )
        }
    }

    #[doc = "Defines the keyboard mode for P35"]
    #[inline(always)]
    pub const fn kbscn_p35_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::KbscnP35ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::KbscnP35ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(70usize),
            )
        }
    }

    #[doc = "Defines the keyboard mode for P36"]
    #[inline(always)]
    pub const fn kbscn_p36_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::KbscnP36ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::KbscnP36ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(72usize),
            )
        }
    }

    #[doc = "Defines the keyboard mode for P37"]
    #[inline(always)]
    pub const fn kbscn_p37_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::KbscnP37ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::KbscnP37ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(74usize),
            )
        }
    }

    #[doc = "Defines the keyboard mode for P40"]
    #[inline(always)]
    pub const fn kbscn_p40_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::KbscnP40ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::KbscnP40ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(76usize),
            )
        }
    }

    #[doc = "Defines the keyboard mode for P41"]
    #[inline(always)]
    pub const fn kbscn_p41_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::KbscnP41ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::KbscnP41ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(78usize),
            )
        }
    }

    #[doc = "Defines the keyboard mode for P42"]
    #[inline(always)]
    pub const fn kbscn_p42_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::KbscnP42ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::KbscnP42ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }

    #[doc = "Defines the keyboard mode for P43"]
    #[inline(always)]
    pub const fn kbscn_p43_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::KbscnP43ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::KbscnP43ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(82usize),
            )
        }
    }

    #[doc = "Defines the keyboard mode for P44"]
    #[inline(always)]
    pub const fn kbscn_p44_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::KbscnP44ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::KbscnP44ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(84usize),
            )
        }
    }

    #[doc = "Defines the keyboard mode for P45"]
    #[inline(always)]
    pub const fn kbscn_p45_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::KbscnP45ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::KbscnP45ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(86usize),
            )
        }
    }

    #[doc = "Defines the keyboard mode for P46"]
    #[inline(always)]
    pub const fn kbscn_p46_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::KbscnP46ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::KbscnP46ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(88usize),
            )
        }
    }

    #[doc = "Defines the keyboard mode for P47"]
    #[inline(always)]
    pub const fn kbscn_p47_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::KbscnP47ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::KbscnP47ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(90usize),
            )
        }
    }

    #[doc = "keyboard scanner Interrupt status register"]
    #[inline(always)]
    pub const fn kbscn_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::KbscnStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::KbscnStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KbscnCtrl2Reg_SPEC;
impl crate::sealed::RegSpec for KbscnCtrl2Reg_SPEC {
    type DataType = u16;
}

#[doc = "Keyboard scanner control 2 register"]
pub type KbscnCtrl2Reg = crate::RegValueT<KbscnCtrl2Reg_SPEC>;

impl KbscnCtrl2Reg {
    #[doc = "Define the row active time in keyboard clock cycles"]
    #[inline(always)]
    pub fn kbscn_row_active_time(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        KbscnCtrl2Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            KbscnCtrl2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for KbscnCtrl2Reg {
    #[inline(always)]
    fn default() -> KbscnCtrl2Reg {
        <crate::RegValueT<KbscnCtrl2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KbscnCtrlReg_SPEC;
impl crate::sealed::RegSpec for KbscnCtrlReg_SPEC {
    type DataType = u16;
}

#[doc = "Keyboard scanner control register"]
pub type KbscnCtrlReg = crate::RegValueT<KbscnCtrlReg_SPEC>;

impl KbscnCtrlReg {
    #[doc = "\'1\' reset fifo, read always \'0\'"]
    #[inline(always)]
    pub fn kbscn_reset_fifo(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, KbscnCtrlReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<14,1,0,KbscnCtrlReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Defines keyboard clk. \"00\" div/1, \"01\" div/4, \"10\" div/16, \"11\" div/64"]
    #[inline(always)]
    pub fn kbscn_clkdiv(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, u8, KbscnCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,u8,u8,KbscnCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "\'1\' After inactive time the keyboard scanner stops the key maxtrix scan"]
    #[inline(always)]
    pub fn kbscn_inactive_en(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, KbscnCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,KbscnCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Defines the inactive time in scan cycles. Value 0 is not allowed"]
    #[inline(always)]
    pub fn kbscn_inactive_time(
        self,
    ) -> crate::common::RegisterField<4, 0x7f, 1, 0, u8, u8, KbscnCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x7f,1,0,u8,u8,KbscnCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "\'1\' Enable IRQ for fifo over and under flow"]
    #[inline(always)]
    pub fn kbscn_irq_fifo_mask(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, KbscnCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,KbscnCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "\'1\' : Enable IRQ for inactive"]
    #[inline(always)]
    pub fn kbscn_irq_inactive_mask(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, KbscnCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,KbscnCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "\'1\' : Enable IRQ for message"]
    #[inline(always)]
    pub fn kbscn_irq_message_mask(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, KbscnCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,KbscnCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "\'1\' : Enable keyboard scanner, Auto clear when inactive enable and inactive case"]
    #[inline(always)]
    pub fn kbscn_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, KbscnCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,KbscnCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for KbscnCtrlReg {
    #[inline(always)]
    fn default() -> KbscnCtrlReg {
        <crate::RegValueT<KbscnCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KbscnDebounceReg_SPEC;
impl crate::sealed::RegSpec for KbscnDebounceReg_SPEC {
    type DataType = u16;
}

#[doc = "Defines the debounce time for key press and release"]
pub type KbscnDebounceReg = crate::RegValueT<KbscnDebounceReg_SPEC>;

impl KbscnDebounceReg {
    #[doc = "Defines the press debounce time in cycles of full matrix scan. One means no debounce, zero is reserved"]
    #[inline(always)]
    pub fn kbscn_debounce_press_time(
        self,
    ) -> crate::common::RegisterField<6, 0x3f, 1, 0, u8, u8, KbscnDebounceReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            6,
            0x3f,
            1,
            0,
            u8,
            u8,
            KbscnDebounceReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Defines the press debounce time in cycles of full matrix scan. One means no debounce, zero is reserved"]
    #[inline(always)]
    pub fn kbscn_debounce_release_time(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, KbscnDebounceReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x3f,
            1,
            0,
            u8,
            u8,
            KbscnDebounceReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for KbscnDebounceReg {
    #[inline(always)]
    fn default() -> KbscnDebounceReg {
        <crate::RegValueT<KbscnDebounceReg_SPEC> as RegisterValue<_>>::new(130)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KbscnMatrixSizeReg_SPEC;
impl crate::sealed::RegSpec for KbscnMatrixSizeReg_SPEC {
    type DataType = u16;
}

#[doc = "Defines the number of rows and columns of the matrix"]
pub type KbscnMatrixSizeReg = crate::RegValueT<KbscnMatrixSizeReg_SPEC>;

impl KbscnMatrixSizeReg {
    #[doc = "Defines the number of the columns of the keyboard matrix minus 1. Zero means number of columns 1"]
    #[inline(always)]
    pub fn kbscn_matrix_column(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1f,
        1,
        0,
        u8,
        u8,
        KbscnMatrixSizeReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1f,
            1,
            0,
            u8,
            u8,
            KbscnMatrixSizeReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Defines the number of the rows of the keyboard matrix minus 1. Zero means number of rows 1"]
    #[inline(always)]
    pub fn kbscn_matrix_row(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        u8,
        u8,
        KbscnMatrixSizeReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            u8,
            u8,
            KbscnMatrixSizeReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for KbscnMatrixSizeReg {
    #[inline(always)]
    fn default() -> KbscnMatrixSizeReg {
        <crate::RegValueT<KbscnMatrixSizeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KbscnMessageKeyReg_SPEC;
impl crate::sealed::RegSpec for KbscnMessageKeyReg_SPEC {
    type DataType = u16;
}

#[doc = "Returns a key message from the message queue"]
pub type KbscnMessageKeyReg = crate::RegValueT<KbscnMessageKeyReg_SPEC>;

impl KbscnMessageKeyReg {
    #[doc = "\'1\' : this message is the last of the group message, else \'0\'. When \'1\' bits 9:0 are all \'1\'"]
    #[inline(always)]
    pub fn kbscn_last_entry(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, KbscnMessageKeyReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,KbscnMessageKeyReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "\'0\' : New key state is release\n\'1\' : New key state is press"]
    #[inline(always)]
    pub fn kbscn_key_state(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, KbscnMessageKeyReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9,1,0,KbscnMessageKeyReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Defines the column id of key"]
    #[inline(always)]
    pub fn kbscn_keyid_column(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1f,
        1,
        0,
        u8,
        u8,
        KbscnMessageKeyReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1f,
            1,
            0,
            u8,
            u8,
            KbscnMessageKeyReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[doc = "Defines the row id of key"]
    #[inline(always)]
    pub fn kbscn_keyid_row(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, KbscnMessageKeyReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            u8,
            u8,
            KbscnMessageKeyReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for KbscnMessageKeyReg {
    #[inline(always)]
    fn default() -> KbscnMessageKeyReg {
        <crate::RegValueT<KbscnMessageKeyReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KbscnP00ModeReg_SPEC;
impl crate::sealed::RegSpec for KbscnP00ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "Defines the keyboard mode for P00"]
pub type KbscnP00ModeReg = crate::RegValueT<KbscnP00ModeReg_SPEC>;

impl KbscnP00ModeReg {
    #[doc = "\'1\' GPIO is enable for row or column"]
    #[inline(always)]
    pub fn kbscn_gpio_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, KbscnP00ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,KbscnP00ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "\'1\' GPIO is row, \'0\' GPIO is column"]
    #[inline(always)]
    pub fn kbscn_row(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, KbscnP00ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,KbscnP00ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Defines the row/column index that has to be connected"]
    #[inline(always)]
    pub fn kbscn_mode(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, KbscnP00ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,KbscnP00ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for KbscnP00ModeReg {
    #[inline(always)]
    fn default() -> KbscnP00ModeReg {
        <crate::RegValueT<KbscnP00ModeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KbscnP01ModeReg_SPEC;
impl crate::sealed::RegSpec for KbscnP01ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "Defines the keyboard mode for P01"]
pub type KbscnP01ModeReg = crate::RegValueT<KbscnP01ModeReg_SPEC>;

impl KbscnP01ModeReg {
    #[doc = "\'1\' GPIO is enable for row or column"]
    #[inline(always)]
    pub fn kbscn_gpio_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, KbscnP01ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,KbscnP01ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "\'1\' GPIO is row, \'0\' GPIO is column"]
    #[inline(always)]
    pub fn kbscn_row(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, KbscnP01ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,KbscnP01ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Defines the row/column index that has to be connected"]
    #[inline(always)]
    pub fn kbscn_mode(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, KbscnP01ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,KbscnP01ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for KbscnP01ModeReg {
    #[inline(always)]
    fn default() -> KbscnP01ModeReg {
        <crate::RegValueT<KbscnP01ModeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KbscnP02ModeReg_SPEC;
impl crate::sealed::RegSpec for KbscnP02ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "Defines the keyboard mode for P02"]
pub type KbscnP02ModeReg = crate::RegValueT<KbscnP02ModeReg_SPEC>;

impl KbscnP02ModeReg {
    #[doc = "\'1\' GPIO is enable for row or column"]
    #[inline(always)]
    pub fn kbscn_gpio_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, KbscnP02ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,KbscnP02ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "\'1\' GPIO is row, \'0\' GPIO is column"]
    #[inline(always)]
    pub fn kbscn_row(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, KbscnP02ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,KbscnP02ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Defines the row/column index that has to be connected"]
    #[inline(always)]
    pub fn kbscn_mode(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, KbscnP02ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,KbscnP02ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for KbscnP02ModeReg {
    #[inline(always)]
    fn default() -> KbscnP02ModeReg {
        <crate::RegValueT<KbscnP02ModeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KbscnP03ModeReg_SPEC;
impl crate::sealed::RegSpec for KbscnP03ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "Defines the keyboard mode for P03"]
pub type KbscnP03ModeReg = crate::RegValueT<KbscnP03ModeReg_SPEC>;

impl KbscnP03ModeReg {
    #[doc = "\'1\' GPIO is enable for row or column"]
    #[inline(always)]
    pub fn kbscn_gpio_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, KbscnP03ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,KbscnP03ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "\'1\' GPIO is row, \'0\' GPIO is column"]
    #[inline(always)]
    pub fn kbscn_row(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, KbscnP03ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,KbscnP03ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Defines the row/column index that has to be connected"]
    #[inline(always)]
    pub fn kbscn_mode(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, KbscnP03ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,KbscnP03ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for KbscnP03ModeReg {
    #[inline(always)]
    fn default() -> KbscnP03ModeReg {
        <crate::RegValueT<KbscnP03ModeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KbscnP04ModeReg_SPEC;
impl crate::sealed::RegSpec for KbscnP04ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "Defines the keyboard mode for P04"]
pub type KbscnP04ModeReg = crate::RegValueT<KbscnP04ModeReg_SPEC>;

impl KbscnP04ModeReg {
    #[doc = "\'1\' GPIO is enable for row or column"]
    #[inline(always)]
    pub fn kbscn_gpio_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, KbscnP04ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,KbscnP04ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "\'1\' GPIO is row, \'0\' GPIO is column"]
    #[inline(always)]
    pub fn kbscn_row(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, KbscnP04ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,KbscnP04ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Defines the row/column index that has to be connected"]
    #[inline(always)]
    pub fn kbscn_mode(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, KbscnP04ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,KbscnP04ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for KbscnP04ModeReg {
    #[inline(always)]
    fn default() -> KbscnP04ModeReg {
        <crate::RegValueT<KbscnP04ModeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KbscnP05ModeReg_SPEC;
impl crate::sealed::RegSpec for KbscnP05ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "Defines the keyboard mode for P05"]
pub type KbscnP05ModeReg = crate::RegValueT<KbscnP05ModeReg_SPEC>;

impl KbscnP05ModeReg {
    #[doc = "\'1\' GPIO is enable for row or column"]
    #[inline(always)]
    pub fn kbscn_gpio_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, KbscnP05ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,KbscnP05ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "\'1\' GPIO is row, \'0\' GPIO is column"]
    #[inline(always)]
    pub fn kbscn_row(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, KbscnP05ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,KbscnP05ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Defines the row/column index that has to be connected"]
    #[inline(always)]
    pub fn kbscn_mode(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, KbscnP05ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,KbscnP05ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for KbscnP05ModeReg {
    #[inline(always)]
    fn default() -> KbscnP05ModeReg {
        <crate::RegValueT<KbscnP05ModeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KbscnP06ModeReg_SPEC;
impl crate::sealed::RegSpec for KbscnP06ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "Defines the keyboard mode for P06"]
pub type KbscnP06ModeReg = crate::RegValueT<KbscnP06ModeReg_SPEC>;

impl KbscnP06ModeReg {
    #[doc = "\'1\' GPIO is enable for row or column"]
    #[inline(always)]
    pub fn kbscn_gpio_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, KbscnP06ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,KbscnP06ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "\'1\' GPIO is row, \'0\' GPIO is column"]
    #[inline(always)]
    pub fn kbscn_row(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, KbscnP06ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,KbscnP06ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Defines the row/column index that has to be connected"]
    #[inline(always)]
    pub fn kbscn_mode(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, KbscnP06ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,KbscnP06ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for KbscnP06ModeReg {
    #[inline(always)]
    fn default() -> KbscnP06ModeReg {
        <crate::RegValueT<KbscnP06ModeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KbscnP07ModeReg_SPEC;
impl crate::sealed::RegSpec for KbscnP07ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "Defines the keyboard mode for P07"]
pub type KbscnP07ModeReg = crate::RegValueT<KbscnP07ModeReg_SPEC>;

impl KbscnP07ModeReg {
    #[doc = "\'1\' GPIO is enable for row or column"]
    #[inline(always)]
    pub fn kbscn_gpio_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, KbscnP07ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,KbscnP07ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "\'1\' GPIO is row, \'0\' GPIO is column"]
    #[inline(always)]
    pub fn kbscn_row(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, KbscnP07ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,KbscnP07ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Defines the row/column index that has to be connected"]
    #[inline(always)]
    pub fn kbscn_mode(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, KbscnP07ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,KbscnP07ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for KbscnP07ModeReg {
    #[inline(always)]
    fn default() -> KbscnP07ModeReg {
        <crate::RegValueT<KbscnP07ModeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KbscnP10ModeReg_SPEC;
impl crate::sealed::RegSpec for KbscnP10ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "Defines the keyboard mode for P10"]
pub type KbscnP10ModeReg = crate::RegValueT<KbscnP10ModeReg_SPEC>;

impl KbscnP10ModeReg {
    #[doc = "\'1\' GPIO is enable for row or column"]
    #[inline(always)]
    pub fn kbscn_gpio_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, KbscnP10ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,KbscnP10ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "\'1\' GPIO is row, \'0\' GPIO is column"]
    #[inline(always)]
    pub fn kbscn_row(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, KbscnP10ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,KbscnP10ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Defines the row/column index that has to be connected"]
    #[inline(always)]
    pub fn kbscn_mode(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, KbscnP10ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,KbscnP10ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for KbscnP10ModeReg {
    #[inline(always)]
    fn default() -> KbscnP10ModeReg {
        <crate::RegValueT<KbscnP10ModeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KbscnP11ModeReg_SPEC;
impl crate::sealed::RegSpec for KbscnP11ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "Defines the keyboard mode for P11"]
pub type KbscnP11ModeReg = crate::RegValueT<KbscnP11ModeReg_SPEC>;

impl KbscnP11ModeReg {
    #[doc = "\'1\' GPIO is enable for row or column"]
    #[inline(always)]
    pub fn kbscn_gpio_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, KbscnP11ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,KbscnP11ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "\'1\' GPIO is row, \'0\' GPIO is column"]
    #[inline(always)]
    pub fn kbscn_row(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, KbscnP11ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,KbscnP11ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Defines the row/column index that has to be connected"]
    #[inline(always)]
    pub fn kbscn_mode(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, KbscnP11ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,KbscnP11ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for KbscnP11ModeReg {
    #[inline(always)]
    fn default() -> KbscnP11ModeReg {
        <crate::RegValueT<KbscnP11ModeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KbscnP12ModeReg_SPEC;
impl crate::sealed::RegSpec for KbscnP12ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "Defines the keyboard mode for P12"]
pub type KbscnP12ModeReg = crate::RegValueT<KbscnP12ModeReg_SPEC>;

impl KbscnP12ModeReg {
    #[doc = "\'1\' GPIO is enable for row or column"]
    #[inline(always)]
    pub fn kbscn_gpio_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, KbscnP12ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,KbscnP12ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "\'1\' GPIO is row, \'0\' GPIO is column"]
    #[inline(always)]
    pub fn kbscn_row(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, KbscnP12ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,KbscnP12ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Defines the row/column index that has to be connected"]
    #[inline(always)]
    pub fn kbscn_mode(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, KbscnP12ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,KbscnP12ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for KbscnP12ModeReg {
    #[inline(always)]
    fn default() -> KbscnP12ModeReg {
        <crate::RegValueT<KbscnP12ModeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KbscnP13ModeReg_SPEC;
impl crate::sealed::RegSpec for KbscnP13ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "Defines the keyboard mode for P13"]
pub type KbscnP13ModeReg = crate::RegValueT<KbscnP13ModeReg_SPEC>;

impl KbscnP13ModeReg {
    #[doc = "\'1\' GPIO is enable for row or column"]
    #[inline(always)]
    pub fn kbscn_gpio_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, KbscnP13ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,KbscnP13ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "\'1\' GPIO is row, \'0\' GPIO is column"]
    #[inline(always)]
    pub fn kbscn_row(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, KbscnP13ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,KbscnP13ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Defines the row/column index that has to be connected"]
    #[inline(always)]
    pub fn kbscn_mode(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, KbscnP13ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,KbscnP13ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for KbscnP13ModeReg {
    #[inline(always)]
    fn default() -> KbscnP13ModeReg {
        <crate::RegValueT<KbscnP13ModeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KbscnP14ModeReg_SPEC;
impl crate::sealed::RegSpec for KbscnP14ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "Defines the keyboard mode for P14"]
pub type KbscnP14ModeReg = crate::RegValueT<KbscnP14ModeReg_SPEC>;

impl KbscnP14ModeReg {
    #[doc = "\'1\' GPIO is enable for row or column"]
    #[inline(always)]
    pub fn kbscn_gpio_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, KbscnP14ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,KbscnP14ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "\'1\' GPIO is row, \'0\' GPIO is column"]
    #[inline(always)]
    pub fn kbscn_row(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, KbscnP14ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,KbscnP14ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Defines the row/column index that has to be connected"]
    #[inline(always)]
    pub fn kbscn_mode(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, KbscnP14ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,KbscnP14ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for KbscnP14ModeReg {
    #[inline(always)]
    fn default() -> KbscnP14ModeReg {
        <crate::RegValueT<KbscnP14ModeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KbscnP15ModeReg_SPEC;
impl crate::sealed::RegSpec for KbscnP15ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "Defines the keyboard mode for P15"]
pub type KbscnP15ModeReg = crate::RegValueT<KbscnP15ModeReg_SPEC>;

impl KbscnP15ModeReg {
    #[doc = "\'1\' GPIO is enable for row or column"]
    #[inline(always)]
    pub fn kbscn_gpio_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, KbscnP15ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,KbscnP15ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "\'1\' GPIO is row, \'0\' GPIO is column"]
    #[inline(always)]
    pub fn kbscn_row(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, KbscnP15ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,KbscnP15ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Defines the row/column index that has to be connected"]
    #[inline(always)]
    pub fn kbscn_mode(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, KbscnP15ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,KbscnP15ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for KbscnP15ModeReg {
    #[inline(always)]
    fn default() -> KbscnP15ModeReg {
        <crate::RegValueT<KbscnP15ModeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KbscnP16ModeReg_SPEC;
impl crate::sealed::RegSpec for KbscnP16ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "Defines the keyboard mode for P16"]
pub type KbscnP16ModeReg = crate::RegValueT<KbscnP16ModeReg_SPEC>;

impl KbscnP16ModeReg {
    #[doc = "\'1\' GPIO is enable for row or column"]
    #[inline(always)]
    pub fn kbscn_gpio_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, KbscnP16ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,KbscnP16ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "\'1\' GPIO is row, \'0\' GPIO is column"]
    #[inline(always)]
    pub fn kbscn_row(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, KbscnP16ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,KbscnP16ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Defines the row/column index that has to be connected"]
    #[inline(always)]
    pub fn kbscn_mode(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, KbscnP16ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,KbscnP16ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for KbscnP16ModeReg {
    #[inline(always)]
    fn default() -> KbscnP16ModeReg {
        <crate::RegValueT<KbscnP16ModeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KbscnP17ModeReg_SPEC;
impl crate::sealed::RegSpec for KbscnP17ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "Defines the keyboard mode for P17"]
pub type KbscnP17ModeReg = crate::RegValueT<KbscnP17ModeReg_SPEC>;

impl KbscnP17ModeReg {
    #[doc = "\'1\' GPIO is enable for row or column"]
    #[inline(always)]
    pub fn kbscn_gpio_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, KbscnP17ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,KbscnP17ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "\'1\' GPIO is row, \'0\' GPIO is column"]
    #[inline(always)]
    pub fn kbscn_row(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, KbscnP17ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,KbscnP17ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Defines the row/column index that has to be connected"]
    #[inline(always)]
    pub fn kbscn_mode(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, KbscnP17ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,KbscnP17ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for KbscnP17ModeReg {
    #[inline(always)]
    fn default() -> KbscnP17ModeReg {
        <crate::RegValueT<KbscnP17ModeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KbscnP20ModeReg_SPEC;
impl crate::sealed::RegSpec for KbscnP20ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "Defines the keyboard mode for P20"]
pub type KbscnP20ModeReg = crate::RegValueT<KbscnP20ModeReg_SPEC>;

impl KbscnP20ModeReg {
    #[doc = "\'1\' GPIO is enable for row or column"]
    #[inline(always)]
    pub fn kbscn_gpio_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, KbscnP20ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,KbscnP20ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "\'1\' GPIO is row, \'0\' GPIO is column"]
    #[inline(always)]
    pub fn kbscn_row(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, KbscnP20ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,KbscnP20ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Defines the row/column index that has to be connected"]
    #[inline(always)]
    pub fn kbscn_mode(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, KbscnP20ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,KbscnP20ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for KbscnP20ModeReg {
    #[inline(always)]
    fn default() -> KbscnP20ModeReg {
        <crate::RegValueT<KbscnP20ModeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KbscnP21ModeReg_SPEC;
impl crate::sealed::RegSpec for KbscnP21ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "Defines the keyboard mode for P21"]
pub type KbscnP21ModeReg = crate::RegValueT<KbscnP21ModeReg_SPEC>;

impl KbscnP21ModeReg {
    #[doc = "\'1\' GPIO is enable for row or column"]
    #[inline(always)]
    pub fn kbscn_gpio_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, KbscnP21ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,KbscnP21ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "\'1\' GPIO is row, \'0\' GPIO is column"]
    #[inline(always)]
    pub fn kbscn_row(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, KbscnP21ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,KbscnP21ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Defines the row/column index that has to be connected"]
    #[inline(always)]
    pub fn kbscn_mode(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, KbscnP21ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,KbscnP21ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for KbscnP21ModeReg {
    #[inline(always)]
    fn default() -> KbscnP21ModeReg {
        <crate::RegValueT<KbscnP21ModeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KbscnP22ModeReg_SPEC;
impl crate::sealed::RegSpec for KbscnP22ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "Defines the keyboard mode for P22"]
pub type KbscnP22ModeReg = crate::RegValueT<KbscnP22ModeReg_SPEC>;

impl KbscnP22ModeReg {
    #[doc = "\'1\' GPIO is enable for row or column"]
    #[inline(always)]
    pub fn kbscn_gpio_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, KbscnP22ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,KbscnP22ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "\'1\' GPIO is row, \'0\' GPIO is column"]
    #[inline(always)]
    pub fn kbscn_row(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, KbscnP22ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,KbscnP22ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Defines the row/column index that has to be connected"]
    #[inline(always)]
    pub fn kbscn_mode(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, KbscnP22ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,KbscnP22ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for KbscnP22ModeReg {
    #[inline(always)]
    fn default() -> KbscnP22ModeReg {
        <crate::RegValueT<KbscnP22ModeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KbscnP23ModeReg_SPEC;
impl crate::sealed::RegSpec for KbscnP23ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "Defines the keyboard mode for P23"]
pub type KbscnP23ModeReg = crate::RegValueT<KbscnP23ModeReg_SPEC>;

impl KbscnP23ModeReg {
    #[doc = "\'1\' GPIO is enable for row or column"]
    #[inline(always)]
    pub fn kbscn_gpio_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, KbscnP23ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,KbscnP23ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "\'1\' GPIO is row, \'0\' GPIO is column"]
    #[inline(always)]
    pub fn kbscn_row(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, KbscnP23ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,KbscnP23ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Defines the row/column index that has to be connected"]
    #[inline(always)]
    pub fn kbscn_mode(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, KbscnP23ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,KbscnP23ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for KbscnP23ModeReg {
    #[inline(always)]
    fn default() -> KbscnP23ModeReg {
        <crate::RegValueT<KbscnP23ModeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KbscnP24ModeReg_SPEC;
impl crate::sealed::RegSpec for KbscnP24ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "Defines the keyboard mode for P24"]
pub type KbscnP24ModeReg = crate::RegValueT<KbscnP24ModeReg_SPEC>;

impl KbscnP24ModeReg {
    #[doc = "\'1\' GPIO is enable for row or column"]
    #[inline(always)]
    pub fn kbscn_gpio_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, KbscnP24ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,KbscnP24ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "\'1\' GPIO is row, \'0\' GPIO is column"]
    #[inline(always)]
    pub fn kbscn_row(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, KbscnP24ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,KbscnP24ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Defines the row/column index that has to be connected"]
    #[inline(always)]
    pub fn kbscn_mode(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, KbscnP24ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,KbscnP24ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for KbscnP24ModeReg {
    #[inline(always)]
    fn default() -> KbscnP24ModeReg {
        <crate::RegValueT<KbscnP24ModeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KbscnP30ModeReg_SPEC;
impl crate::sealed::RegSpec for KbscnP30ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "Defines the keyboard mode for P30"]
pub type KbscnP30ModeReg = crate::RegValueT<KbscnP30ModeReg_SPEC>;

impl KbscnP30ModeReg {
    #[doc = "\'1\' GPIO is enable for row or column"]
    #[inline(always)]
    pub fn kbscn_gpio_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, KbscnP30ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,KbscnP30ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "\'1\' GPIO is row, \'0\' GPIO is column"]
    #[inline(always)]
    pub fn kbscn_row(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, KbscnP30ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,KbscnP30ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Defines the row/column index that has to be connected"]
    #[inline(always)]
    pub fn kbscn_mode(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, KbscnP30ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,KbscnP30ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for KbscnP30ModeReg {
    #[inline(always)]
    fn default() -> KbscnP30ModeReg {
        <crate::RegValueT<KbscnP30ModeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KbscnP31ModeReg_SPEC;
impl crate::sealed::RegSpec for KbscnP31ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "Defines the keyboard mode for P31"]
pub type KbscnP31ModeReg = crate::RegValueT<KbscnP31ModeReg_SPEC>;

impl KbscnP31ModeReg {
    #[doc = "\'1\' GPIO is enable for row or column"]
    #[inline(always)]
    pub fn kbscn_gpio_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, KbscnP31ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,KbscnP31ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "\'1\' GPIO is row, \'0\' GPIO is column"]
    #[inline(always)]
    pub fn kbscn_row(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, KbscnP31ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,KbscnP31ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Defines the row/column index that has to be connected"]
    #[inline(always)]
    pub fn kbscn_mode(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, KbscnP31ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,KbscnP31ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for KbscnP31ModeReg {
    #[inline(always)]
    fn default() -> KbscnP31ModeReg {
        <crate::RegValueT<KbscnP31ModeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KbscnP32ModeReg_SPEC;
impl crate::sealed::RegSpec for KbscnP32ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "Defines the keyboard mode for P32"]
pub type KbscnP32ModeReg = crate::RegValueT<KbscnP32ModeReg_SPEC>;

impl KbscnP32ModeReg {
    #[doc = "\'1\' GPIO is enable for row or column"]
    #[inline(always)]
    pub fn kbscn_gpio_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, KbscnP32ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,KbscnP32ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "\'1\' GPIO is row, \'0\' GPIO is column"]
    #[inline(always)]
    pub fn kbscn_row(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, KbscnP32ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,KbscnP32ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Defines the row/column index that has to be connected"]
    #[inline(always)]
    pub fn kbscn_mode(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, KbscnP32ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,KbscnP32ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for KbscnP32ModeReg {
    #[inline(always)]
    fn default() -> KbscnP32ModeReg {
        <crate::RegValueT<KbscnP32ModeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KbscnP33ModeReg_SPEC;
impl crate::sealed::RegSpec for KbscnP33ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "Defines the keyboard mode for P33"]
pub type KbscnP33ModeReg = crate::RegValueT<KbscnP33ModeReg_SPEC>;

impl KbscnP33ModeReg {
    #[doc = "\'1\' GPIO is enable for row or column"]
    #[inline(always)]
    pub fn kbscn_gpio_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, KbscnP33ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,KbscnP33ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "\'1\' GPIO is row, \'0\' GPIO is column"]
    #[inline(always)]
    pub fn kbscn_row(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, KbscnP33ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,KbscnP33ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Defines the row/column index that has to be connected"]
    #[inline(always)]
    pub fn kbscn_mode(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, KbscnP33ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,KbscnP33ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for KbscnP33ModeReg {
    #[inline(always)]
    fn default() -> KbscnP33ModeReg {
        <crate::RegValueT<KbscnP33ModeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KbscnP34ModeReg_SPEC;
impl crate::sealed::RegSpec for KbscnP34ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "Defines the keyboard mode for P34"]
pub type KbscnP34ModeReg = crate::RegValueT<KbscnP34ModeReg_SPEC>;

impl KbscnP34ModeReg {
    #[doc = "\'1\' GPIO is enable for row or column"]
    #[inline(always)]
    pub fn kbscn_gpio_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, KbscnP34ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,KbscnP34ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "\'1\' GPIO is row, \'0\' GPIO is column"]
    #[inline(always)]
    pub fn kbscn_row(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, KbscnP34ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,KbscnP34ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Defines the row/column index that has to be connected"]
    #[inline(always)]
    pub fn kbscn_mode(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, KbscnP34ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,KbscnP34ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for KbscnP34ModeReg {
    #[inline(always)]
    fn default() -> KbscnP34ModeReg {
        <crate::RegValueT<KbscnP34ModeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KbscnP35ModeReg_SPEC;
impl crate::sealed::RegSpec for KbscnP35ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "Defines the keyboard mode for P35"]
pub type KbscnP35ModeReg = crate::RegValueT<KbscnP35ModeReg_SPEC>;

impl KbscnP35ModeReg {
    #[doc = "\'1\' GPIO is enable for row or column"]
    #[inline(always)]
    pub fn kbscn_gpio_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, KbscnP35ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,KbscnP35ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "\'1\' GPIO is row, \'0\' GPIO is column"]
    #[inline(always)]
    pub fn kbscn_row(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, KbscnP35ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,KbscnP35ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Defines the row/column index that has to be connected"]
    #[inline(always)]
    pub fn kbscn_mode(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, KbscnP35ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,KbscnP35ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for KbscnP35ModeReg {
    #[inline(always)]
    fn default() -> KbscnP35ModeReg {
        <crate::RegValueT<KbscnP35ModeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KbscnP36ModeReg_SPEC;
impl crate::sealed::RegSpec for KbscnP36ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "Defines the keyboard mode for P36"]
pub type KbscnP36ModeReg = crate::RegValueT<KbscnP36ModeReg_SPEC>;

impl KbscnP36ModeReg {
    #[doc = "\'1\' GPIO is enable for row or column"]
    #[inline(always)]
    pub fn kbscn_gpio_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, KbscnP36ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,KbscnP36ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "\'1\' GPIO is row, \'0\' GPIO is column"]
    #[inline(always)]
    pub fn kbscn_row(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, KbscnP36ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,KbscnP36ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Defines the row/column index that has to be connected"]
    #[inline(always)]
    pub fn kbscn_mode(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, KbscnP36ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,KbscnP36ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for KbscnP36ModeReg {
    #[inline(always)]
    fn default() -> KbscnP36ModeReg {
        <crate::RegValueT<KbscnP36ModeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KbscnP37ModeReg_SPEC;
impl crate::sealed::RegSpec for KbscnP37ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "Defines the keyboard mode for P37"]
pub type KbscnP37ModeReg = crate::RegValueT<KbscnP37ModeReg_SPEC>;

impl KbscnP37ModeReg {
    #[doc = "\'1\' GPIO is enable for row or column"]
    #[inline(always)]
    pub fn kbscn_gpio_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, KbscnP37ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,KbscnP37ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "\'1\' GPIO is row, \'0\' GPIO is column"]
    #[inline(always)]
    pub fn kbscn_row(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, KbscnP37ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,KbscnP37ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Defines the row/column index that has to be connected"]
    #[inline(always)]
    pub fn kbscn_mode(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, KbscnP37ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,KbscnP37ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for KbscnP37ModeReg {
    #[inline(always)]
    fn default() -> KbscnP37ModeReg {
        <crate::RegValueT<KbscnP37ModeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KbscnP40ModeReg_SPEC;
impl crate::sealed::RegSpec for KbscnP40ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "Defines the keyboard mode for P40"]
pub type KbscnP40ModeReg = crate::RegValueT<KbscnP40ModeReg_SPEC>;

impl KbscnP40ModeReg {
    #[doc = "\'1\' GPIO is enable for row or column"]
    #[inline(always)]
    pub fn kbscn_gpio_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, KbscnP40ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,KbscnP40ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "\'1\' GPIO is row, \'0\' GPIO is column"]
    #[inline(always)]
    pub fn kbscn_row(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, KbscnP40ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,KbscnP40ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Defines the row/column index that has to be connected"]
    #[inline(always)]
    pub fn kbscn_mode(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, KbscnP40ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,KbscnP40ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for KbscnP40ModeReg {
    #[inline(always)]
    fn default() -> KbscnP40ModeReg {
        <crate::RegValueT<KbscnP40ModeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KbscnP41ModeReg_SPEC;
impl crate::sealed::RegSpec for KbscnP41ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "Defines the keyboard mode for P41"]
pub type KbscnP41ModeReg = crate::RegValueT<KbscnP41ModeReg_SPEC>;

impl KbscnP41ModeReg {
    #[doc = "\'1\' GPIO is enable for row or column"]
    #[inline(always)]
    pub fn kbscn_gpio_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, KbscnP41ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,KbscnP41ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "\'1\' GPIO is row, \'0\' GPIO is column"]
    #[inline(always)]
    pub fn kbscn_row(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, KbscnP41ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,KbscnP41ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Defines the row/column index that has to be connected"]
    #[inline(always)]
    pub fn kbscn_mode(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, KbscnP41ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,KbscnP41ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for KbscnP41ModeReg {
    #[inline(always)]
    fn default() -> KbscnP41ModeReg {
        <crate::RegValueT<KbscnP41ModeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KbscnP42ModeReg_SPEC;
impl crate::sealed::RegSpec for KbscnP42ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "Defines the keyboard mode for P42"]
pub type KbscnP42ModeReg = crate::RegValueT<KbscnP42ModeReg_SPEC>;

impl KbscnP42ModeReg {
    #[doc = "\'1\' GPIO is enable for row or column"]
    #[inline(always)]
    pub fn kbscn_gpio_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, KbscnP42ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,KbscnP42ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "\'1\' GPIO is row, \'0\' GPIO is column"]
    #[inline(always)]
    pub fn kbscn_row(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, KbscnP42ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,KbscnP42ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Defines the row/column index that has to be connected"]
    #[inline(always)]
    pub fn kbscn_mode(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, KbscnP42ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,KbscnP42ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for KbscnP42ModeReg {
    #[inline(always)]
    fn default() -> KbscnP42ModeReg {
        <crate::RegValueT<KbscnP42ModeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KbscnP43ModeReg_SPEC;
impl crate::sealed::RegSpec for KbscnP43ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "Defines the keyboard mode for P43"]
pub type KbscnP43ModeReg = crate::RegValueT<KbscnP43ModeReg_SPEC>;

impl KbscnP43ModeReg {
    #[doc = "\'1\' GPIO is enable for row or column"]
    #[inline(always)]
    pub fn kbscn_gpio_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, KbscnP43ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,KbscnP43ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "\'1\' GPIO is row, \'0\' GPIO is column"]
    #[inline(always)]
    pub fn kbscn_row(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, KbscnP43ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,KbscnP43ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Defines the row/column index that has to be connected"]
    #[inline(always)]
    pub fn kbscn_mode(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, KbscnP43ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,KbscnP43ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for KbscnP43ModeReg {
    #[inline(always)]
    fn default() -> KbscnP43ModeReg {
        <crate::RegValueT<KbscnP43ModeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KbscnP44ModeReg_SPEC;
impl crate::sealed::RegSpec for KbscnP44ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "Defines the keyboard mode for P44"]
pub type KbscnP44ModeReg = crate::RegValueT<KbscnP44ModeReg_SPEC>;

impl KbscnP44ModeReg {
    #[doc = "\'1\' GPIO is enable for row or column"]
    #[inline(always)]
    pub fn kbscn_gpio_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, KbscnP44ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,KbscnP44ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "\'1\' GPIO is row, \'0\' GPIO is column"]
    #[inline(always)]
    pub fn kbscn_row(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, KbscnP44ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,KbscnP44ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Defines the row/column index that has to be connected"]
    #[inline(always)]
    pub fn kbscn_mode(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, KbscnP44ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,KbscnP44ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for KbscnP44ModeReg {
    #[inline(always)]
    fn default() -> KbscnP44ModeReg {
        <crate::RegValueT<KbscnP44ModeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KbscnP45ModeReg_SPEC;
impl crate::sealed::RegSpec for KbscnP45ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "Defines the keyboard mode for P45"]
pub type KbscnP45ModeReg = crate::RegValueT<KbscnP45ModeReg_SPEC>;

impl KbscnP45ModeReg {
    #[doc = "\'1\' GPIO is enable for row or column"]
    #[inline(always)]
    pub fn kbscn_gpio_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, KbscnP45ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,KbscnP45ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "\'1\' GPIO is row, \'0\' GPIO is column"]
    #[inline(always)]
    pub fn kbscn_row(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, KbscnP45ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,KbscnP45ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Defines the row/column index that has to be connected"]
    #[inline(always)]
    pub fn kbscn_mode(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, KbscnP45ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,KbscnP45ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for KbscnP45ModeReg {
    #[inline(always)]
    fn default() -> KbscnP45ModeReg {
        <crate::RegValueT<KbscnP45ModeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KbscnP46ModeReg_SPEC;
impl crate::sealed::RegSpec for KbscnP46ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "Defines the keyboard mode for P46"]
pub type KbscnP46ModeReg = crate::RegValueT<KbscnP46ModeReg_SPEC>;

impl KbscnP46ModeReg {
    #[doc = "\'1\' GPIO is enable for row or column"]
    #[inline(always)]
    pub fn kbscn_gpio_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, KbscnP46ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,KbscnP46ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "\'1\' GPIO is row, \'0\' GPIO is column"]
    #[inline(always)]
    pub fn kbscn_row(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, KbscnP46ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,KbscnP46ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Defines the row/column index that has to be connected"]
    #[inline(always)]
    pub fn kbscn_mode(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, KbscnP46ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,KbscnP46ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for KbscnP46ModeReg {
    #[inline(always)]
    fn default() -> KbscnP46ModeReg {
        <crate::RegValueT<KbscnP46ModeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KbscnP47ModeReg_SPEC;
impl crate::sealed::RegSpec for KbscnP47ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "Defines the keyboard mode for P47"]
pub type KbscnP47ModeReg = crate::RegValueT<KbscnP47ModeReg_SPEC>;

impl KbscnP47ModeReg {
    #[doc = "\'1\' GPIO is enable for row or column"]
    #[inline(always)]
    pub fn kbscn_gpio_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, KbscnP47ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,KbscnP47ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "\'1\' GPIO is row, \'0\' GPIO is column"]
    #[inline(always)]
    pub fn kbscn_row(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, KbscnP47ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,KbscnP47ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Defines the row/column index that has to be connected"]
    #[inline(always)]
    pub fn kbscn_mode(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, KbscnP47ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,KbscnP47ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for KbscnP47ModeReg {
    #[inline(always)]
    fn default() -> KbscnP47ModeReg {
        <crate::RegValueT<KbscnP47ModeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KbscnStatusReg_SPEC;
impl crate::sealed::RegSpec for KbscnStatusReg_SPEC {
    type DataType = u16;
}

#[doc = "keyboard scanner Interrupt status register"]
pub type KbscnStatusReg = crate::RegValueT<KbscnStatusReg_SPEC>;

impl KbscnStatusReg {
    #[doc = "\'1\' Fifo Underflow occurred"]
    #[inline(always)]
    pub fn kbscn_fifo_underfl(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, KbscnStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,KbscnStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "\'1\' Fifo Overflow occurred"]
    #[inline(always)]
    pub fn kbscn_fifo_overfl(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, KbscnStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,KbscnStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Defines how many messages there are in the fifo."]
    #[inline(always)]
    pub fn kbscn_num_message(
        self,
    ) -> crate::common::RegisterField<2, 0x1f, 1, 0, u8, u8, KbscnStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<2,0x1f,1,0,u8,u8,KbscnStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "There is no keyboard activity for a predefined time"]
    #[inline(always)]
    pub fn kbscn_inactive_irq_status(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, KbscnStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,KbscnStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "There is at least one last message in the fifo."]
    #[inline(always)]
    pub fn kbscn_mes_irq_status(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, KbscnStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,KbscnStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for KbscnStatusReg {
    #[inline(always)]
    fn default() -> KbscnStatusReg {
        <crate::RegValueT<KbscnStatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
