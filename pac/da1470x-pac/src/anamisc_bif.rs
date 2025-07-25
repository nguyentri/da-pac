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
// Generated from SVD 1.2, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:45:52 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"ANAMISC_BIF registers"]
unsafe impl ::core::marker::Send for super::AnamiscBif {}
unsafe impl ::core::marker::Sync for super::AnamiscBif {}
impl super::AnamiscBif {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Select clock for oscillator calibration"]
    #[inline(always)]
    pub const fn clk_cal_irq_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ClkCalIrqReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ClkCalIrqReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "Count value for oscillator calibration"]
    #[inline(always)]
    pub const fn clk_ref_cnt_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ClkRefCntReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ClkRefCntReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
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
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "DIVN reference cycles, lower 16 bits"]
    #[inline(always)]
    pub const fn clk_ref_val_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ClkRefValReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ClkRefValReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkCalIrqReg_SPEC;
impl crate::sealed::RegSpec for ClkCalIrqReg_SPEC {
    type DataType = u32;
}

#[doc = "Select clock for oscillator calibration"]
pub type ClkCalIrqReg = crate::RegValueT<ClkCalIrqReg_SPEC>;

impl ClkCalIrqReg {
    #[doc = "Clear the IRQ.\n1: Clear the IRQ\n0: No effect.\nRead out 0 always."]
    #[inline(always)]
    pub fn clk_cal_irq_clr(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ClkCalIrqReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,ClkCalIrqReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Shows the IRQ bit status."]
    #[inline(always)]
    pub fn clk_cal_irq_status(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, ClkCalIrqReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,ClkCalIrqReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Enable clk calibration IRQ.\n0: Disabled.\n1*: Enabled.\n\n*Note: If IRQ feature is enabled, every calibration should be started by setting CLK_REF_SEL_REG\\[REF_CAL_START\\] to 1."]
    #[inline(always)]
    pub fn clk_cal_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ClkCalIrqReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,ClkCalIrqReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ClkCalIrqReg {
    #[inline(always)]
    fn default() -> ClkCalIrqReg {
        <crate::RegValueT<ClkCalIrqReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkRefCntReg_SPEC;
impl crate::sealed::RegSpec for ClkRefCntReg_SPEC {
    type DataType = u32;
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
    type DataType = u32;
}

#[doc = "Select clock for oscillator calibration"]
pub type ClkRefSelReg = crate::RegValueT<ClkRefSelReg_SPEC>;

impl ClkRefSelReg {
    #[doc = "Select calibration clock input to be used in calibration:\n0x0 : DIVN clock\n0x1 : RCLP\n0x2 : RCHS\n0x3 : XTAL32K\n0x4 : RCOSC\n0x5, 0x6 and 0x7: Reserved"]
    #[inline(always)]
    pub fn cal_clk_sel(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, u8, ClkRefSelReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x7,1,0,u8,u8,ClkRefSelReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 : Enable XTAL_CNT counter by the REF_CLK selected by REF_CLK_SEL.\n1 : Enable XTAL_CNT counter from an external input."]
    #[inline(always)]
    pub fn ext_cnt_en_sel(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, ClkRefSelReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,ClkRefSelReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Writing a \'1\' starts a calibration. This bit is cleared when calibration is finished, and CLK_REF_VAL is ready."]
    #[inline(always)]
    pub fn ref_cal_start(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, ClkRefSelReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,ClkRefSelReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Select reference clock input for calibration:\n0x0 : RCLP\n0x1 : RCHS\n0x2 : XTAL32K\n0x3 : RCX\n0x4 : RCOSC\n0x5 : DIVN clock"]
    #[inline(always)]
    pub fn ref_clk_sel(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, ClkRefSelReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,ClkRefSelReg_SPEC,crate::common::RW>::from_register(self,0)
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
pub struct ClkRefValReg_SPEC;
impl crate::sealed::RegSpec for ClkRefValReg_SPEC {
    type DataType = u32;
}

#[doc = "DIVN reference cycles, lower 16 bits"]
pub type ClkRefValReg = crate::RegValueT<ClkRefValReg_SPEC>;

impl ClkRefValReg {
    #[doc = "Returns the number of DIVN clock cycles counted during the calibration time, defined with REF_CNT_VAL"]
    #[inline(always)]
    pub fn xtal_cnt_val(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        ClkRefValReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            ClkRefValReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for ClkRefValReg {
    #[inline(always)]
    fn default() -> ClkRefValReg {
        <crate::RegValueT<ClkRefValReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
