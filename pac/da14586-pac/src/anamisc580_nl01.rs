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
// Generated from SVD 1.2, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:44:49 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"anamisc580_nl01 registers"]
unsafe impl ::core::marker::Send for super::Anamisc580Nl01 {}
unsafe impl ::core::marker::Sync for super::Anamisc580Nl01 {}
impl super::Anamisc580Nl01 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Count value for oscillator calibration"]
    #[inline(always)]
    pub const fn clk_ref_cnt_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ClkRefCntReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ClkRefCntReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[doc = "Select clock for oscillator calibration"]
    #[inline(always)]
    pub const fn clk_ref_sel_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ClkRefSelReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ClkRefSelReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "XTAL16M reference cycles, upper 16 bits"]
    #[inline(always)]
    pub const fn clk_ref_val_h_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ClkRefValHReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ClkRefValHReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[doc = "XTAL16M reference cycles, lower 16 bits"]
    #[inline(always)]
    pub const fn clk_ref_val_l_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ClkRefValLReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ClkRefValLReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkRefCntReg_SPEC;
impl crate::sealed::RegSpec for ClkRefCntReg_SPEC {
    type DataType = u16;
}

#[doc = "Count value for oscillator calibration"]
pub type ClkRefCntReg = crate::RegValueT<ClkRefCntReg_SPEC>;

impl ClkRefCntReg {
    #[doc = "Indicates the calibration time, with a decrement counter to 1."]
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

#[doc = "Select clock for oscillator calibration"]
pub type ClkRefSelReg = crate::RegValueT<ClkRefSelReg_SPEC>;

impl ClkRefSelReg {
    #[doc = "Writing a \'1\' starts a calibration. This bit is cleared when calibration is finished, and CLK_REF_VAL is ready."]
    #[inline(always)]
    pub fn ref_cal_start(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ClkRefSelReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,ClkRefSelReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Select clock input for calibration:\n0x0 : RC32K oscillator\n0x1 : RC16M oscillator\n0x2 : XTAL32K oscillator\n0x3 : RCX oscillator"]
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

#[doc = "XTAL16M reference cycles, upper 16 bits"]
pub type ClkRefValHReg = crate::RegValueT<ClkRefValHReg_SPEC>;

impl ClkRefValHReg {
    #[doc = "Returns the upper 16 bits of XTAL16 clock cycles during the calibration time, defined with REF_CNT_VAL"]
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

#[doc = "XTAL16M reference cycles, lower 16 bits"]
pub type ClkRefValLReg = crate::RegValueT<ClkRefValLReg_SPEC>;

impl ClkRefValLReg {
    #[doc = "Returns the lower 16 bits of XTAL16 clock cycles during the calibration time, defined with REF_CNT_VAL"]
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
