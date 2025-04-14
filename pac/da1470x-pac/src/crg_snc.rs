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
#[doc = r"CRG_SNC registers"]
unsafe impl ::core::marker::Send for super::CrgSnc {}
unsafe impl ::core::marker::Sync for super::CrgSnc {}
impl super::CrgSnc {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn clk_snc_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ClkSncReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ClkSncReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn reset_clk_snc_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ResetClkSncReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ResetClkSncReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn set_clk_snc_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SetClkSncReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SetClkSncReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkSncReg_SPEC;
impl crate::sealed::RegSpec for ClkSncReg_SPEC {
    type DataType = u32;
}

pub type ClkSncReg = crate::RegValueT<ClkSncReg_SPEC>;

impl ClkSncReg {
    #[inline(always)]
    pub fn i3c_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, ClkSncReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17,1,0,ClkSncReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn i3c_enable(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, ClkSncReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,ClkSncReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn i2c3_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, ClkSncReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,ClkSncReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn i2c3_enable(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, ClkSncReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,ClkSncReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn i2c2_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, ClkSncReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,ClkSncReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn i2c2_enable(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, ClkSncReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,ClkSncReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn i2c_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, ClkSncReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,ClkSncReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn i2c_enable(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, ClkSncReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,ClkSncReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn spi2_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, ClkSncReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,ClkSncReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn spi2_enable(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, ClkSncReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,ClkSncReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn spi_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, ClkSncReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,ClkSncReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn spi_enable(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, ClkSncReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,ClkSncReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart3_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, ClkSncReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,ClkSncReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart3_enable(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, ClkSncReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,ClkSncReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart2_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, ClkSncReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,ClkSncReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart2_enable(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ClkSncReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,ClkSncReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, ClkSncReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,ClkSncReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_enable(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ClkSncReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,ClkSncReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ClkSncReg {
    #[inline(always)]
    fn default() -> ClkSncReg {
        <crate::RegValueT<ClkSncReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ResetClkSncReg_SPEC;
impl crate::sealed::RegSpec for ResetClkSncReg_SPEC {
    type DataType = u32;
}

pub type ResetClkSncReg = crate::RegValueT<ResetClkSncReg_SPEC>;

impl ResetClkSncReg {
    #[inline(always)]
    pub fn i3c_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, ResetClkSncReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17,1,0,ResetClkSncReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn i3c_enable(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, ResetClkSncReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,ResetClkSncReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn i2c3_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, ResetClkSncReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,ResetClkSncReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn i2c3_enable(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, ResetClkSncReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,ResetClkSncReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn i2c2_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, ResetClkSncReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,ResetClkSncReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn i2c2_enable(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, ResetClkSncReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,ResetClkSncReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn i2c_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, ResetClkSncReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,ResetClkSncReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn i2c_enable(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, ResetClkSncReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,ResetClkSncReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn spi2_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, ResetClkSncReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,ResetClkSncReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn spi2_enable(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, ResetClkSncReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,ResetClkSncReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn spi_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, ResetClkSncReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,ResetClkSncReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn spi_enable(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, ResetClkSncReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,ResetClkSncReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart3_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, ResetClkSncReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,ResetClkSncReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart3_enable(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, ResetClkSncReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,ResetClkSncReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart2_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, ResetClkSncReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,ResetClkSncReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart2_enable(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ResetClkSncReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,ResetClkSncReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, ResetClkSncReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,ResetClkSncReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_enable(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ResetClkSncReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,ResetClkSncReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ResetClkSncReg {
    #[inline(always)]
    fn default() -> ResetClkSncReg {
        <crate::RegValueT<ResetClkSncReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SetClkSncReg_SPEC;
impl crate::sealed::RegSpec for SetClkSncReg_SPEC {
    type DataType = u32;
}

pub type SetClkSncReg = crate::RegValueT<SetClkSncReg_SPEC>;

impl SetClkSncReg {
    #[inline(always)]
    pub fn i3c_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, SetClkSncReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17,1,0,SetClkSncReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn i3c_enable(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, SetClkSncReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,SetClkSncReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn i2c3_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, SetClkSncReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,SetClkSncReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn i2c3_enable(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, SetClkSncReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,SetClkSncReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn i2c2_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, SetClkSncReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,SetClkSncReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn i2c2_enable(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, SetClkSncReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,SetClkSncReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn i2c_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, SetClkSncReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,SetClkSncReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn i2c_enable(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, SetClkSncReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,SetClkSncReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn spi2_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, SetClkSncReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,SetClkSncReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn spi2_enable(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, SetClkSncReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,SetClkSncReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn spi_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, SetClkSncReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,SetClkSncReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn spi_enable(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, SetClkSncReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,SetClkSncReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart3_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, SetClkSncReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,SetClkSncReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart3_enable(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, SetClkSncReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,SetClkSncReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart2_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, SetClkSncReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,SetClkSncReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart2_enable(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, SetClkSncReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,SetClkSncReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, SetClkSncReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,SetClkSncReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_enable(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, SetClkSncReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,SetClkSncReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for SetClkSncReg {
    #[inline(always)]
    fn default() -> SetClkSncReg {
        <crate::RegValueT<SetClkSncReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
