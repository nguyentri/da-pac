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
// Generated from SVD 1.2, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:45:24 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"CRG_COM registers"]
unsafe impl ::core::marker::Send for super::CrgCom {}
unsafe impl ::core::marker::Sync for super::CrgCom {}
impl super::CrgCom {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Peripheral divider register"]
    #[inline(always)]
    pub const fn clk_com_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ClkComReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ClkComReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Peripheral divider register RESET register. Reads back 0x0000"]
    #[inline(always)]
    pub const fn reset_clk_com_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ResetClkComReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ResetClkComReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Peripheral divider register SET register. Reads back 0x0000"]
    #[inline(always)]
    pub const fn set_clk_com_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SetClkComReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SetClkComReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkComReg_SPEC;
impl crate::sealed::RegSpec for ClkComReg_SPEC {
    type DataType = u32;
}

#[doc = "Peripheral divider register"]
pub type ClkComReg = crate::RegValueT<ClkComReg_SPEC>;

impl ClkComReg {
    #[doc = "Select LCD external clock speed.\n0x0: 1 Hz\n0x1: 62.5 Hz\n0x2: 125 Hz\n0x3: off"]
    #[inline(always)]
    pub fn lcd_ext_clk_sel(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, u8, ClkComReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3,1,0,u8,u8,ClkComReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Division factor for SNC, w.r.t. pclk setting\n0x0 = divide by 1\n0x1 = divide by 2\n0x2 = divide by 4\n0x3 = divide by 8"]
    #[inline(always)]
    pub fn snc_div(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, u8, ClkComReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x3,1,0,u8,u8,ClkComReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Selects the clock source\n1 = DIV1 clock\n0 = DIVN clock"]
    #[inline(always)]
    pub fn i2c2_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, ClkComReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,ClkComReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enables the clock"]
    #[inline(always)]
    pub fn i2c2_enable(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, ClkComReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,ClkComReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Selects the clock source\n1 = DIV1 clock\n0 = DIVN clock"]
    #[inline(always)]
    pub fn i2c_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, ClkComReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,ClkComReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enables the clock"]
    #[inline(always)]
    pub fn i2c_enable(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, ClkComReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,ClkComReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Selects the clock source\n1 = DIV1 clock\n0 = DIVN clock"]
    #[inline(always)]
    pub fn spi2_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, ClkComReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,ClkComReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enables the clock"]
    #[inline(always)]
    pub fn spi2_enable(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, ClkComReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,ClkComReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Selects the clock source\n1 = DIV1 clock\n0 = DIVN clock"]
    #[inline(always)]
    pub fn spi_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, ClkComReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,ClkComReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enables the clock"]
    #[inline(always)]
    pub fn spi_enable(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, ClkComReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,ClkComReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Selects the clock source\n1 = DIV1 clock\n0 = DIVN clock"]
    #[inline(always)]
    pub fn uart3_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, ClkComReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,ClkComReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enables the clock"]
    #[inline(always)]
    pub fn uart3_enable(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, ClkComReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,ClkComReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Selects the clock source\n1 = DIV1 clock\n0 = DIVN clock"]
    #[inline(always)]
    pub fn uart2_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ClkComReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,ClkComReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enables the clock"]
    #[inline(always)]
    pub fn uart2_enable(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, ClkComReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,ClkComReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enables the clock"]
    #[inline(always)]
    pub fn uart_enable(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ClkComReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,ClkComReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ClkComReg {
    #[inline(always)]
    fn default() -> ClkComReg {
        <crate::RegValueT<ClkComReg_SPEC> as RegisterValue<_>>::new(196608)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ResetClkComReg_SPEC;
impl crate::sealed::RegSpec for ResetClkComReg_SPEC {
    type DataType = u32;
}

#[doc = "Peripheral divider register RESET register. Reads back 0x0000"]
pub type ResetClkComReg = crate::RegValueT<ResetClkComReg_SPEC>;

impl ResetClkComReg {
    #[doc = "Select LCD external clock speed.\n0x0: 1 Hz\n0x1: 62.5 Hz\n0x2: 125 Hz\n0x3: off"]
    #[inline(always)]
    pub fn lcd_ext_clk_sel(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, u8, ResetClkComReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3,1,0,u8,u8,ResetClkComReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Division factor for SNC, w.r.t. pclk setting\n0x0 = divide by 1\n0x1 = divide by 2\n0x2 = divide by 4\n0x3 = divide by 8"]
    #[inline(always)]
    pub fn snc_div(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, u8, ResetClkComReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x3,1,0,u8,u8,ResetClkComReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Selects the clock source\n1 = DIV1 clock\n0 = DIVN clock"]
    #[inline(always)]
    pub fn sdadc_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, ResetClkComReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,ResetClkComReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Selects the clock source\n1 = DIV1 clock\n0 = DIVN clock"]
    #[inline(always)]
    pub fn i2c2_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, ResetClkComReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,ResetClkComReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enables the clock"]
    #[inline(always)]
    pub fn i2c2_enable(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, ResetClkComReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,ResetClkComReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Selects the clock source\n1 = DIV1 clock\n0 = DIVN clock"]
    #[inline(always)]
    pub fn i2c_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, ResetClkComReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,ResetClkComReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enables the clock"]
    #[inline(always)]
    pub fn i2c_enable(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, ResetClkComReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,ResetClkComReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Selects the clock source\n1 = DIV1 clock\n0 = DIVN clock"]
    #[inline(always)]
    pub fn spi2_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, ResetClkComReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,ResetClkComReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enables the clock"]
    #[inline(always)]
    pub fn spi2_enable(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, ResetClkComReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,ResetClkComReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Selects the clock source\n1 = DIV1 clock\n0 = DIVN clock"]
    #[inline(always)]
    pub fn spi_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, ResetClkComReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,ResetClkComReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enables the clock"]
    #[inline(always)]
    pub fn spi_enable(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, ResetClkComReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,ResetClkComReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Selects the clock source\n1 = DIV1 clock\n0 = DIVN clock"]
    #[inline(always)]
    pub fn uart3_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, ResetClkComReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,ResetClkComReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enables the clock"]
    #[inline(always)]
    pub fn uart3_enable(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, ResetClkComReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,ResetClkComReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Selects the clock source\n1 = DIV1 clock\n0 = DIVN clock"]
    #[inline(always)]
    pub fn uart2_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ResetClkComReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,ResetClkComReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enables the clock"]
    #[inline(always)]
    pub fn uart2_enable(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, ResetClkComReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,ResetClkComReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enables the clock"]
    #[inline(always)]
    pub fn uart_enable(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ResetClkComReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,ResetClkComReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ResetClkComReg {
    #[inline(always)]
    fn default() -> ResetClkComReg {
        <crate::RegValueT<ResetClkComReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SetClkComReg_SPEC;
impl crate::sealed::RegSpec for SetClkComReg_SPEC {
    type DataType = u32;
}

#[doc = "Peripheral divider register SET register. Reads back 0x0000"]
pub type SetClkComReg = crate::RegValueT<SetClkComReg_SPEC>;

impl SetClkComReg {
    #[doc = "Select LCD external clock speed.\n0x0: 1 Hz\n0x1: 62.5 Hz\n0x2: 125 Hz\n0x3: off"]
    #[inline(always)]
    pub fn lcd_ext_clk_sel(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, u8, SetClkComReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3,1,0,u8,u8,SetClkComReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Division factor for SNC, w.r.t. pclk setting\n0x0 = divide by 1\n0x1 = divide by 2\n0x2 = divide by 4\n0x3 = divide by 8"]
    #[inline(always)]
    pub fn snc_div(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, u8, SetClkComReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x3,1,0,u8,u8,SetClkComReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Selects the clock source\n1 = DIV1 clock\n0 = DIVN clock"]
    #[inline(always)]
    pub fn sdadc_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, SetClkComReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,SetClkComReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Selects the clock source\n1 = DIV1 clock\n0 = DIVN clock"]
    #[inline(always)]
    pub fn i2c2_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, SetClkComReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,SetClkComReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enables the clock"]
    #[inline(always)]
    pub fn i2c2_enable(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, SetClkComReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,SetClkComReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Selects the clock source\n1 = DIV1 clock\n0 = DIVN clock"]
    #[inline(always)]
    pub fn i2c_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, SetClkComReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,SetClkComReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enables the clock"]
    #[inline(always)]
    pub fn i2c_enable(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, SetClkComReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,SetClkComReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Selects the clock source\n1 = DIV1 clock\n0 = DIVN clock"]
    #[inline(always)]
    pub fn spi2_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, SetClkComReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,SetClkComReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enables the clock"]
    #[inline(always)]
    pub fn spi2_enable(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, SetClkComReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,SetClkComReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Selects the clock source\n1 = DIV1 clock\n0 = DIVN clock"]
    #[inline(always)]
    pub fn spi_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, SetClkComReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,SetClkComReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enables the clock"]
    #[inline(always)]
    pub fn spi_enable(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, SetClkComReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,SetClkComReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Selects the clock source\n1 = DIV1 clock\n0 = DIVN clock"]
    #[inline(always)]
    pub fn uart3_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, SetClkComReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,SetClkComReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enables the clock"]
    #[inline(always)]
    pub fn uart3_enable(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, SetClkComReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,SetClkComReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Selects the clock source\n1 = DIV1 clock\n0 = DIVN clock"]
    #[inline(always)]
    pub fn uart2_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, SetClkComReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,SetClkComReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enables the clock"]
    #[inline(always)]
    pub fn uart2_enable(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, SetClkComReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,SetClkComReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enables the clock"]
    #[inline(always)]
    pub fn uart_enable(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, SetClkComReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,SetClkComReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for SetClkComReg {
    #[inline(always)]
    fn default() -> SetClkComReg {
        <crate::RegValueT<SetClkComReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
