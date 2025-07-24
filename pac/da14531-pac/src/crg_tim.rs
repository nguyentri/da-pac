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
#[doc = r"CRG_TIM registers"]
unsafe impl ::core::marker::Send for super::CrgTim {}
unsafe impl ::core::marker::Sync for super::CrgTim {}
impl super::CrgTim {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Divisor for RTC 100Hz clock"]
    #[inline(always)]
    pub const fn clk_rtcdiv_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ClkRtcdivReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ClkRtcdivReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(76usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkRtcdivReg_SPEC;
impl crate::sealed::RegSpec for ClkRtcdivReg_SPEC {
    type DataType = u32;
}

#[doc = "Divisor for RTC 100Hz clock"]
pub type ClkRtcdivReg = crate::RegValueT<ClkRtcdivReg_SPEC>;

impl ClkRtcdivReg {
    #[doc = "Reset request for the RTC module"]
    #[inline(always)]
    pub fn rtc_reset_req(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, ClkRtcdivReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21,1,0,ClkRtcdivReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enable for the 100 Hz generation for the RTC block"]
    #[inline(always)]
    pub fn rtc_div_enable(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, ClkRtcdivReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20,1,0,ClkRtcdivReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Selects the denominator for the fractional division:\n0b0: 1000\n0b1: 1024"]
    #[inline(always)]
    pub fn rtc_div_denom(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, ClkRtcdivReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19,1,0,ClkRtcdivReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Integer divisor part for RTC 100Hz generation"]
    #[inline(always)]
    pub fn rtc_div_int(
        self,
    ) -> crate::common::RegisterField<10, 0x1ff, 1, 0, u16, u16, ClkRtcdivReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            10,
            0x1ff,
            1,
            0,
            u16,
            u16,
            ClkRtcdivReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "Fractional divisor part for RTC 100Hz generation.\nif RTC_DIV_DENOM=1, &lt;RTC_DIV_FRAC&gt; out of 1024 cycles will divide by &lt;RTC_DIV_INT+1&gt;, the rest is &lt;RTC_DIV_INT&gt;\nIf RTC_DIV_DENOM=0, &lt;RTC_DIV_FRAC&gt; out of 1000 cycles will divide by &lt;RTC_DIV_INT+1&gt;, the rest is &lt;RTC_DIV_INT&gt;"]
    #[inline(always)]
    pub fn rtc_div_frac(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, ClkRtcdivReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,ClkRtcdivReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ClkRtcdivReg {
    #[inline(always)]
    fn default() -> ClkRtcdivReg {
        <crate::RegValueT<ClkRtcdivReg_SPEC> as RegisterValue<_>>::new(335528)
    }
}
