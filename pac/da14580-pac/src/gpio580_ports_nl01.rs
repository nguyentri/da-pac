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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:15:24 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"gpio580_ports_nl01 registers"]
unsafe impl ::core::marker::Send for super::Gpio580PortsNl01 {}
unsafe impl ::core::marker::Sync for super::Gpio580PortsNl01 {}
impl super::Gpio580PortsNl01 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn p00_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P00ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P00ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p01_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P01ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P01ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p01_padpwr_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P01PadpwrCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P01PadpwrCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(112usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p02_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P02ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P02ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(10usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p03_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P03ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P03ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p04_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P04ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P04ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(14usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p05_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P05ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P05ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p06_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P06ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P06ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(18usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p07_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P07ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P07ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
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
    pub const fn p0_reset_data_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P0ResetDataReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P0ResetDataReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p0_set_data_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P0SetDataReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P0SetDataReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p10_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P10ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P10ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(38usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p11_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P11ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P11ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p12_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P12ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P12ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(42usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p13_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P13ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P13ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p14_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P14ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P14ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(46usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p15_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P15ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P15ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p1_data_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P1DataReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P1DataReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p1_reset_data_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P1ResetDataReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P1ResetDataReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p1_set_data_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P1SetDataReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P1SetDataReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(34usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p20_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P20ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P20ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(70usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p21_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P21ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P21ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(72usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p22_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P22ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P22ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(74usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p23_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P23ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P23ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(76usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p24_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P24ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P24ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(78usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p25_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P25ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P25ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p26_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P26ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P26ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(82usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p27_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P27ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P27ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(84usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p28_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P28ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P28ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(86usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p29_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P29ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P29ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(88usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p2_data_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P2DataReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P2DataReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p2_padpwr_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P2PadpwrCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P2PadpwrCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(114usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p2_reset_data_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P2ResetDataReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P2ResetDataReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(68usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p2_set_data_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P2SetDataReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P2SetDataReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(66usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p30_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P30ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P30ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(134usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p31_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P31ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P31ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(136usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p32_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P32ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P32ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(138usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p33_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P33ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P33ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(140usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p34_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P34ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P34ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(142usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p35_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P35ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P35ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(144usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p36_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P36ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P36ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(146usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p37_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P37ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P37ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(148usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p3_data_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P3DataReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P3DataReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(128usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p3_padpwr_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P3PadpwrCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P3PadpwrCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(116usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p3_reset_data_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P3ResetDataReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P3ResetDataReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(132usize),
            )
        }
    }

    #[inline(always)]
    pub const fn p3_set_data_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P3SetDataReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P3SetDataReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(130usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P00ModeReg_SPEC;
impl crate::sealed::RegSpec for P00ModeReg_SPEC {
    type DataType = u16;
}

pub type P00ModeReg = crate::RegValueT<P00ModeReg_SPEC>;

impl P00ModeReg {
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P00ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P00ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, P00ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,P00ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P00ModeReg {
    #[inline(always)]
    fn default() -> P00ModeReg {
        <crate::RegValueT<P00ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P01ModeReg_SPEC;
impl crate::sealed::RegSpec for P01ModeReg_SPEC {
    type DataType = u16;
}

pub type P01ModeReg = crate::RegValueT<P01ModeReg_SPEC>;

impl P01ModeReg {
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P01ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P01ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, P01ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,P01ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P01ModeReg {
    #[inline(always)]
    fn default() -> P01ModeReg {
        <crate::RegValueT<P01ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P01PadpwrCtrlReg_SPEC;
impl crate::sealed::RegSpec for P01PadpwrCtrlReg_SPEC {
    type DataType = u16;
}

pub type P01PadpwrCtrlReg = crate::RegValueT<P01PadpwrCtrlReg_SPEC>;

impl P01PadpwrCtrlReg {
    #[inline(always)]
    pub fn p1_out_ctrl(
        self,
    ) -> crate::common::RegisterField<8, 0x3f, 1, 0, u8, u8, P01PadpwrCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            8,
            0x3f,
            1,
            0,
            u8,
            u8,
            P01PadpwrCtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn p0_out_ctrl(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, P01PadpwrCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            P01PadpwrCtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P01PadpwrCtrlReg {
    #[inline(always)]
    fn default() -> P01PadpwrCtrlReg {
        <crate::RegValueT<P01PadpwrCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P02ModeReg_SPEC;
impl crate::sealed::RegSpec for P02ModeReg_SPEC {
    type DataType = u16;
}

pub type P02ModeReg = crate::RegValueT<P02ModeReg_SPEC>;

impl P02ModeReg {
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P02ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P02ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, P02ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,P02ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P02ModeReg {
    #[inline(always)]
    fn default() -> P02ModeReg {
        <crate::RegValueT<P02ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P03ModeReg_SPEC;
impl crate::sealed::RegSpec for P03ModeReg_SPEC {
    type DataType = u16;
}

pub type P03ModeReg = crate::RegValueT<P03ModeReg_SPEC>;

impl P03ModeReg {
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P03ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P03ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, P03ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,P03ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P03ModeReg {
    #[inline(always)]
    fn default() -> P03ModeReg {
        <crate::RegValueT<P03ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P04ModeReg_SPEC;
impl crate::sealed::RegSpec for P04ModeReg_SPEC {
    type DataType = u16;
}

pub type P04ModeReg = crate::RegValueT<P04ModeReg_SPEC>;

impl P04ModeReg {
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P04ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P04ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, P04ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,P04ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P04ModeReg {
    #[inline(always)]
    fn default() -> P04ModeReg {
        <crate::RegValueT<P04ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P05ModeReg_SPEC;
impl crate::sealed::RegSpec for P05ModeReg_SPEC {
    type DataType = u16;
}

pub type P05ModeReg = crate::RegValueT<P05ModeReg_SPEC>;

impl P05ModeReg {
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P05ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P05ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, P05ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,P05ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P05ModeReg {
    #[inline(always)]
    fn default() -> P05ModeReg {
        <crate::RegValueT<P05ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P06ModeReg_SPEC;
impl crate::sealed::RegSpec for P06ModeReg_SPEC {
    type DataType = u16;
}

pub type P06ModeReg = crate::RegValueT<P06ModeReg_SPEC>;

impl P06ModeReg {
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P06ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P06ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, P06ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,P06ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P06ModeReg {
    #[inline(always)]
    fn default() -> P06ModeReg {
        <crate::RegValueT<P06ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P07ModeReg_SPEC;
impl crate::sealed::RegSpec for P07ModeReg_SPEC {
    type DataType = u16;
}

pub type P07ModeReg = crate::RegValueT<P07ModeReg_SPEC>;

impl P07ModeReg {
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P07ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P07ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, P07ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,P07ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P07ModeReg {
    #[inline(always)]
    fn default() -> P07ModeReg {
        <crate::RegValueT<P07ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P0DataReg_SPEC;
impl crate::sealed::RegSpec for P0DataReg_SPEC {
    type DataType = u16;
}

pub type P0DataReg = crate::RegValueT<P0DataReg_SPEC>;

impl P0DataReg {
    #[inline(always)]
    pub fn p0_data(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, P0DataReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,P0DataReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P0DataReg {
    #[inline(always)]
    fn default() -> P0DataReg {
        <crate::RegValueT<P0DataReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P0ResetDataReg_SPEC;
impl crate::sealed::RegSpec for P0ResetDataReg_SPEC {
    type DataType = u16;
}

pub type P0ResetDataReg = crate::RegValueT<P0ResetDataReg_SPEC>;

impl P0ResetDataReg {
    #[inline(always)]
    pub fn p0_reset(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, P0ResetDataReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,P0ResetDataReg_SPEC,crate::common::RW>::from_register(self,0)
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
    type DataType = u16;
}

pub type P0SetDataReg = crate::RegValueT<P0SetDataReg_SPEC>;

impl P0SetDataReg {
    #[inline(always)]
    pub fn p0_set(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, P0SetDataReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,P0SetDataReg_SPEC,crate::common::RW>::from_register(self,0)
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
pub struct P10ModeReg_SPEC;
impl crate::sealed::RegSpec for P10ModeReg_SPEC {
    type DataType = u16;
}

pub type P10ModeReg = crate::RegValueT<P10ModeReg_SPEC>;

impl P10ModeReg {
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P10ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P10ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, P10ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,P10ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P10ModeReg {
    #[inline(always)]
    fn default() -> P10ModeReg {
        <crate::RegValueT<P10ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P11ModeReg_SPEC;
impl crate::sealed::RegSpec for P11ModeReg_SPEC {
    type DataType = u16;
}

pub type P11ModeReg = crate::RegValueT<P11ModeReg_SPEC>;

impl P11ModeReg {
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P11ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P11ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, P11ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,P11ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P11ModeReg {
    #[inline(always)]
    fn default() -> P11ModeReg {
        <crate::RegValueT<P11ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P12ModeReg_SPEC;
impl crate::sealed::RegSpec for P12ModeReg_SPEC {
    type DataType = u16;
}

pub type P12ModeReg = crate::RegValueT<P12ModeReg_SPEC>;

impl P12ModeReg {
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P12ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P12ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, P12ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,P12ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P12ModeReg {
    #[inline(always)]
    fn default() -> P12ModeReg {
        <crate::RegValueT<P12ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P13ModeReg_SPEC;
impl crate::sealed::RegSpec for P13ModeReg_SPEC {
    type DataType = u16;
}

pub type P13ModeReg = crate::RegValueT<P13ModeReg_SPEC>;

impl P13ModeReg {
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P13ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P13ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, P13ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,P13ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P13ModeReg {
    #[inline(always)]
    fn default() -> P13ModeReg {
        <crate::RegValueT<P13ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P14ModeReg_SPEC;
impl crate::sealed::RegSpec for P14ModeReg_SPEC {
    type DataType = u16;
}

pub type P14ModeReg = crate::RegValueT<P14ModeReg_SPEC>;

impl P14ModeReg {
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P14ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P14ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, P14ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,P14ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P14ModeReg {
    #[inline(always)]
    fn default() -> P14ModeReg {
        <crate::RegValueT<P14ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P15ModeReg_SPEC;
impl crate::sealed::RegSpec for P15ModeReg_SPEC {
    type DataType = u16;
}

pub type P15ModeReg = crate::RegValueT<P15ModeReg_SPEC>;

impl P15ModeReg {
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P15ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P15ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, P15ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,P15ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P15ModeReg {
    #[inline(always)]
    fn default() -> P15ModeReg {
        <crate::RegValueT<P15ModeReg_SPEC> as RegisterValue<_>>::new(256)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P1DataReg_SPEC;
impl crate::sealed::RegSpec for P1DataReg_SPEC {
    type DataType = u16;
}

pub type P1DataReg = crate::RegValueT<P1DataReg_SPEC>;

impl P1DataReg {
    #[inline(always)]
    pub fn p1_data(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, P1DataReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,P1DataReg_SPEC,crate::common::RW>::from_register(self,0)
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
pub struct P1ResetDataReg_SPEC;
impl crate::sealed::RegSpec for P1ResetDataReg_SPEC {
    type DataType = u16;
}

pub type P1ResetDataReg = crate::RegValueT<P1ResetDataReg_SPEC>;

impl P1ResetDataReg {
    #[inline(always)]
    pub fn p1_reset(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, P1ResetDataReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,P1ResetDataReg_SPEC,crate::common::RW>::from_register(self,0)
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
    type DataType = u16;
}

pub type P1SetDataReg = crate::RegValueT<P1SetDataReg_SPEC>;

impl P1SetDataReg {
    #[inline(always)]
    pub fn p1_set(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, P1SetDataReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,P1SetDataReg_SPEC,crate::common::RW>::from_register(self,0)
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
pub struct P20ModeReg_SPEC;
impl crate::sealed::RegSpec for P20ModeReg_SPEC {
    type DataType = u16;
}

pub type P20ModeReg = crate::RegValueT<P20ModeReg_SPEC>;

impl P20ModeReg {
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P20ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P20ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, P20ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,P20ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P20ModeReg {
    #[inline(always)]
    fn default() -> P20ModeReg {
        <crate::RegValueT<P20ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P21ModeReg_SPEC;
impl crate::sealed::RegSpec for P21ModeReg_SPEC {
    type DataType = u16;
}

pub type P21ModeReg = crate::RegValueT<P21ModeReg_SPEC>;

impl P21ModeReg {
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P21ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P21ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, P21ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,P21ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P21ModeReg {
    #[inline(always)]
    fn default() -> P21ModeReg {
        <crate::RegValueT<P21ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P22ModeReg_SPEC;
impl crate::sealed::RegSpec for P22ModeReg_SPEC {
    type DataType = u16;
}

pub type P22ModeReg = crate::RegValueT<P22ModeReg_SPEC>;

impl P22ModeReg {
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P22ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P22ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, P22ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,P22ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P22ModeReg {
    #[inline(always)]
    fn default() -> P22ModeReg {
        <crate::RegValueT<P22ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P23ModeReg_SPEC;
impl crate::sealed::RegSpec for P23ModeReg_SPEC {
    type DataType = u16;
}

pub type P23ModeReg = crate::RegValueT<P23ModeReg_SPEC>;

impl P23ModeReg {
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P23ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P23ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, P23ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,P23ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P23ModeReg {
    #[inline(always)]
    fn default() -> P23ModeReg {
        <crate::RegValueT<P23ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P24ModeReg_SPEC;
impl crate::sealed::RegSpec for P24ModeReg_SPEC {
    type DataType = u16;
}

pub type P24ModeReg = crate::RegValueT<P24ModeReg_SPEC>;

impl P24ModeReg {
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P24ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P24ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, P24ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,P24ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P24ModeReg {
    #[inline(always)]
    fn default() -> P24ModeReg {
        <crate::RegValueT<P24ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P25ModeReg_SPEC;
impl crate::sealed::RegSpec for P25ModeReg_SPEC {
    type DataType = u16;
}

pub type P25ModeReg = crate::RegValueT<P25ModeReg_SPEC>;

impl P25ModeReg {
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P25ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P25ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, P25ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,P25ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P25ModeReg {
    #[inline(always)]
    fn default() -> P25ModeReg {
        <crate::RegValueT<P25ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P26ModeReg_SPEC;
impl crate::sealed::RegSpec for P26ModeReg_SPEC {
    type DataType = u16;
}

pub type P26ModeReg = crate::RegValueT<P26ModeReg_SPEC>;

impl P26ModeReg {
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P26ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P26ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, P26ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,P26ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P26ModeReg {
    #[inline(always)]
    fn default() -> P26ModeReg {
        <crate::RegValueT<P26ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P27ModeReg_SPEC;
impl crate::sealed::RegSpec for P27ModeReg_SPEC {
    type DataType = u16;
}

pub type P27ModeReg = crate::RegValueT<P27ModeReg_SPEC>;

impl P27ModeReg {
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P27ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P27ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, P27ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,P27ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P27ModeReg {
    #[inline(always)]
    fn default() -> P27ModeReg {
        <crate::RegValueT<P27ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P28ModeReg_SPEC;
impl crate::sealed::RegSpec for P28ModeReg_SPEC {
    type DataType = u16;
}

pub type P28ModeReg = crate::RegValueT<P28ModeReg_SPEC>;

impl P28ModeReg {
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P28ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P28ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, P28ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,P28ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P28ModeReg {
    #[inline(always)]
    fn default() -> P28ModeReg {
        <crate::RegValueT<P28ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P29ModeReg_SPEC;
impl crate::sealed::RegSpec for P29ModeReg_SPEC {
    type DataType = u16;
}

pub type P29ModeReg = crate::RegValueT<P29ModeReg_SPEC>;

impl P29ModeReg {
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P29ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P29ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, P29ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,P29ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P29ModeReg {
    #[inline(always)]
    fn default() -> P29ModeReg {
        <crate::RegValueT<P29ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P2DataReg_SPEC;
impl crate::sealed::RegSpec for P2DataReg_SPEC {
    type DataType = u16;
}

pub type P2DataReg = crate::RegValueT<P2DataReg_SPEC>;

impl P2DataReg {
    #[inline(always)]
    pub fn p2_data(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, P2DataReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,P2DataReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P2DataReg {
    #[inline(always)]
    fn default() -> P2DataReg {
        <crate::RegValueT<P2DataReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P2PadpwrCtrlReg_SPEC;
impl crate::sealed::RegSpec for P2PadpwrCtrlReg_SPEC {
    type DataType = u16;
}

pub type P2PadpwrCtrlReg = crate::RegValueT<P2PadpwrCtrlReg_SPEC>;

impl P2PadpwrCtrlReg {
    #[inline(always)]
    pub fn p2_out_ctrl(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3ff,
        1,
        0,
        u16,
        u16,
        P2PadpwrCtrlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3ff,
            1,
            0,
            u16,
            u16,
            P2PadpwrCtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P2PadpwrCtrlReg {
    #[inline(always)]
    fn default() -> P2PadpwrCtrlReg {
        <crate::RegValueT<P2PadpwrCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P2ResetDataReg_SPEC;
impl crate::sealed::RegSpec for P2ResetDataReg_SPEC {
    type DataType = u16;
}

pub type P2ResetDataReg = crate::RegValueT<P2ResetDataReg_SPEC>;

impl P2ResetDataReg {
    #[inline(always)]
    pub fn p2_reset(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3ff,
        1,
        0,
        u16,
        u16,
        P2ResetDataReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3ff,
            1,
            0,
            u16,
            u16,
            P2ResetDataReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P2ResetDataReg {
    #[inline(always)]
    fn default() -> P2ResetDataReg {
        <crate::RegValueT<P2ResetDataReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P2SetDataReg_SPEC;
impl crate::sealed::RegSpec for P2SetDataReg_SPEC {
    type DataType = u16;
}

pub type P2SetDataReg = crate::RegValueT<P2SetDataReg_SPEC>;

impl P2SetDataReg {
    #[inline(always)]
    pub fn p2_set(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, P2SetDataReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,P2SetDataReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P2SetDataReg {
    #[inline(always)]
    fn default() -> P2SetDataReg {
        <crate::RegValueT<P2SetDataReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P30ModeReg_SPEC;
impl crate::sealed::RegSpec for P30ModeReg_SPEC {
    type DataType = u16;
}

pub type P30ModeReg = crate::RegValueT<P30ModeReg_SPEC>;

impl P30ModeReg {
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P30ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P30ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, P30ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,P30ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P30ModeReg {
    #[inline(always)]
    fn default() -> P30ModeReg {
        <crate::RegValueT<P30ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P31ModeReg_SPEC;
impl crate::sealed::RegSpec for P31ModeReg_SPEC {
    type DataType = u16;
}

pub type P31ModeReg = crate::RegValueT<P31ModeReg_SPEC>;

impl P31ModeReg {
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P31ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P31ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, P31ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,P31ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P31ModeReg {
    #[inline(always)]
    fn default() -> P31ModeReg {
        <crate::RegValueT<P31ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P32ModeReg_SPEC;
impl crate::sealed::RegSpec for P32ModeReg_SPEC {
    type DataType = u16;
}

pub type P32ModeReg = crate::RegValueT<P32ModeReg_SPEC>;

impl P32ModeReg {
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P32ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P32ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, P32ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,P32ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P32ModeReg {
    #[inline(always)]
    fn default() -> P32ModeReg {
        <crate::RegValueT<P32ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P33ModeReg_SPEC;
impl crate::sealed::RegSpec for P33ModeReg_SPEC {
    type DataType = u16;
}

pub type P33ModeReg = crate::RegValueT<P33ModeReg_SPEC>;

impl P33ModeReg {
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P33ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P33ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, P33ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,P33ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P33ModeReg {
    #[inline(always)]
    fn default() -> P33ModeReg {
        <crate::RegValueT<P33ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P34ModeReg_SPEC;
impl crate::sealed::RegSpec for P34ModeReg_SPEC {
    type DataType = u16;
}

pub type P34ModeReg = crate::RegValueT<P34ModeReg_SPEC>;

impl P34ModeReg {
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P34ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P34ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, P34ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,P34ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P34ModeReg {
    #[inline(always)]
    fn default() -> P34ModeReg {
        <crate::RegValueT<P34ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P35ModeReg_SPEC;
impl crate::sealed::RegSpec for P35ModeReg_SPEC {
    type DataType = u16;
}

pub type P35ModeReg = crate::RegValueT<P35ModeReg_SPEC>;

impl P35ModeReg {
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P35ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P35ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, P35ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,P35ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P35ModeReg {
    #[inline(always)]
    fn default() -> P35ModeReg {
        <crate::RegValueT<P35ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P36ModeReg_SPEC;
impl crate::sealed::RegSpec for P36ModeReg_SPEC {
    type DataType = u16;
}

pub type P36ModeReg = crate::RegValueT<P36ModeReg_SPEC>;

impl P36ModeReg {
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P36ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P36ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, P36ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,P36ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P36ModeReg {
    #[inline(always)]
    fn default() -> P36ModeReg {
        <crate::RegValueT<P36ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P37ModeReg_SPEC;
impl crate::sealed::RegSpec for P37ModeReg_SPEC {
    type DataType = u16;
}

pub type P37ModeReg = crate::RegValueT<P37ModeReg_SPEC>;

impl P37ModeReg {
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P37ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P37ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, P37ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,P37ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P37ModeReg {
    #[inline(always)]
    fn default() -> P37ModeReg {
        <crate::RegValueT<P37ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P3DataReg_SPEC;
impl crate::sealed::RegSpec for P3DataReg_SPEC {
    type DataType = u16;
}

pub type P3DataReg = crate::RegValueT<P3DataReg_SPEC>;

impl P3DataReg {
    #[inline(always)]
    pub fn p3_data(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, P3DataReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,P3DataReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P3DataReg {
    #[inline(always)]
    fn default() -> P3DataReg {
        <crate::RegValueT<P3DataReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P3PadpwrCtrlReg_SPEC;
impl crate::sealed::RegSpec for P3PadpwrCtrlReg_SPEC {
    type DataType = u16;
}

pub type P3PadpwrCtrlReg = crate::RegValueT<P3PadpwrCtrlReg_SPEC>;

impl P3PadpwrCtrlReg {
    #[inline(always)]
    pub fn p3_out_ctrl(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, P3PadpwrCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,P3PadpwrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P3PadpwrCtrlReg {
    #[inline(always)]
    fn default() -> P3PadpwrCtrlReg {
        <crate::RegValueT<P3PadpwrCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P3ResetDataReg_SPEC;
impl crate::sealed::RegSpec for P3ResetDataReg_SPEC {
    type DataType = u16;
}

pub type P3ResetDataReg = crate::RegValueT<P3ResetDataReg_SPEC>;

impl P3ResetDataReg {
    #[inline(always)]
    pub fn p3_reset(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, P3ResetDataReg_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,P3ResetDataReg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for P3ResetDataReg {
    #[inline(always)]
    fn default() -> P3ResetDataReg {
        <crate::RegValueT<P3ResetDataReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P3SetDataReg_SPEC;
impl crate::sealed::RegSpec for P3SetDataReg_SPEC {
    type DataType = u16;
}

pub type P3SetDataReg = crate::RegValueT<P3SetDataReg_SPEC>;

impl P3SetDataReg {
    #[inline(always)]
    pub fn p3_set(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, P3SetDataReg_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,P3SetDataReg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for P3SetDataReg {
    #[inline(always)]
    fn default() -> P3SetDataReg {
        <crate::RegValueT<P3SetDataReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
