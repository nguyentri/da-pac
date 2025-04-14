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
#[doc = r"LCDC registers"]
unsafe impl ::core::marker::Send for super::Lcdc {}
unsafe impl ::core::marker::Sync for super::Lcdc {}
impl super::Lcdc {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn lcdc_backporchxy_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LcdcBackporchxyReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LcdcBackporchxyReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[inline(always)]
    pub const fn lcdc_bgcolor_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LcdcBgcolorReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LcdcBgcolorReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn lcdc_blankingxy_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LcdcBlankingxyReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LcdcBlankingxyReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[inline(always)]
    pub const fn lcdc_clkctrl_cg_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LcdcClkctrlCgReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LcdcClkctrlCgReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(424usize),
            )
        }
    }

    #[inline(always)]
    pub const fn lcdc_clkctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LcdcClkctrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LcdcClkctrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn lcdc_colmod(
        &self,
    ) -> &'static crate::common::Reg<self::LcdcColmod_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LcdcColmod_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(256usize),
            )
        }
    }

    #[inline(always)]
    pub const fn lcdc_conf_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LcdcConfReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LcdcConfReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(240usize),
            )
        }
    }

    #[inline(always)]
    pub const fn lcdc_crc_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LcdcCrcReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LcdcCrcReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(388usize),
            )
        }
    }

    #[inline(always)]
    pub const fn lcdc_dbib_cfg_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LcdcDbibCfgReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LcdcDbibCfgReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[inline(always)]
    pub const fn lcdc_dbib_cmd_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LcdcDbibCmdReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LcdcDbibCmdReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(232usize),
            )
        }
    }

    #[inline(always)]
    pub const fn lcdc_dbib_rdat(
        &self,
    ) -> &'static crate::common::Reg<self::LcdcDbibRdat_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LcdcDbibRdat_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(236usize),
            )
        }
    }

    #[inline(always)]
    pub const fn lcdc_fmtctrl_2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LcdcFmtctrl2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LcdcFmtctrl2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(420usize),
            )
        }
    }

    #[inline(always)]
    pub const fn lcdc_fmtctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LcdcFmtctrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LcdcFmtctrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(416usize),
            )
        }
    }

    #[inline(always)]
    pub const fn lcdc_frontporchxy_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LcdcFrontporchxyReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LcdcFrontporchxyReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[inline(always)]
    pub const fn lcdc_gpio_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LcdcGpioReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LcdcGpioReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[inline(always)]
    pub const fn lcdc_idreg_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LcdcIdregReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LcdcIdregReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(244usize),
            )
        }
    }

    #[inline(always)]
    pub const fn lcdc_interrupt_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LcdcInterruptReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LcdcInterruptReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(248usize),
            )
        }
    }

    #[inline(always)]
    pub const fn lcdc_layer0_baseaddr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LcdcLayer0BaseaddrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LcdcLayer0BaseaddrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }

    #[inline(always)]
    pub const fn lcdc_layer0_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LcdcLayer0ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LcdcLayer0ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[inline(always)]
    pub const fn lcdc_layer0_resxy_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LcdcLayer0ResxyReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LcdcLayer0ResxyReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(68usize),
            )
        }
    }

    #[inline(always)]
    pub const fn lcdc_layer0_sizexy_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LcdcLayer0SizexyReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LcdcLayer0SizexyReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[inline(always)]
    pub const fn lcdc_layer0_startxy_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LcdcLayer0StartxyReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LcdcLayer0StartxyReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[inline(always)]
    pub const fn lcdc_layer0_stride_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LcdcLayer0StrideReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LcdcLayer0StrideReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[inline(always)]
    pub const fn lcdc_layer1_baseaddr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LcdcLayer1BaseaddrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LcdcLayer1BaseaddrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(92usize),
            )
        }
    }

    #[inline(always)]
    pub const fn lcdc_layer1_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LcdcLayer1ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LcdcLayer1ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }

    #[inline(always)]
    pub const fn lcdc_layer1_resxy_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LcdcLayer1ResxyReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LcdcLayer1ResxyReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(100usize),
            )
        }
    }

    #[inline(always)]
    pub const fn lcdc_layer1_sizexy_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LcdcLayer1SizexyReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LcdcLayer1SizexyReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(88usize),
            )
        }
    }

    #[inline(always)]
    pub const fn lcdc_layer1_startxy_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LcdcLayer1StartxyReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LcdcLayer1StartxyReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(84usize),
            )
        }
    }

    #[inline(always)]
    pub const fn lcdc_layer1_stride_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LcdcLayer1StrideReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LcdcLayer1StrideReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(96usize),
            )
        }
    }

    #[inline(always)]
    pub const fn lcdc_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LcdcModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LcdcModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn lcdc_palette_255(
        &self,
    ) -> &'static crate::common::Reg<self::LcdcPalette255_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LcdcPalette255_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2044usize),
            )
        }
    }

    #[inline(always)]
    pub const fn lcdc_palette_base(
        &self,
    ) -> &'static crate::common::Reg<self::LcdcPaletteBase_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LcdcPaletteBase_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(1024usize),
            )
        }
    }

    #[inline(always)]
    pub const fn lcdc_resxy_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LcdcResxyReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LcdcResxyReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn lcdc_startxy_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LcdcStartxyReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LcdcStartxyReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[inline(always)]
    pub const fn lcdc_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LcdcStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LcdcStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(252usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LcdcBackporchxyReg_SPEC;
impl crate::sealed::RegSpec for LcdcBackporchxyReg_SPEC {
    type DataType = u32;
}

pub type LcdcBackporchxyReg = crate::RegValueT<LcdcBackporchxyReg_SPEC>;

impl LcdcBackporchxyReg {
    #[inline(always)]
    pub fn bporch_x(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        u16,
        u16,
        LcdcBackporchxyReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            LcdcBackporchxyReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn bporch_y(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        LcdcBackporchxyReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            LcdcBackporchxyReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for LcdcBackporchxyReg {
    #[inline(always)]
    fn default() -> LcdcBackporchxyReg {
        <crate::RegValueT<LcdcBackporchxyReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LcdcBgcolorReg_SPEC;
impl crate::sealed::RegSpec for LcdcBgcolorReg_SPEC {
    type DataType = u32;
}

pub type LcdcBgcolorReg = crate::RegValueT<LcdcBgcolorReg_SPEC>;

impl LcdcBgcolorReg {
    #[inline(always)]
    pub fn bg_red(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, LcdcBgcolorReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,LcdcBgcolorReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bg_green(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, LcdcBgcolorReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,LcdcBgcolorReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bg_blue(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, LcdcBgcolorReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,LcdcBgcolorReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bg_alpha(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, LcdcBgcolorReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,LcdcBgcolorReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for LcdcBgcolorReg {
    #[inline(always)]
    fn default() -> LcdcBgcolorReg {
        <crate::RegValueT<LcdcBgcolorReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LcdcBlankingxyReg_SPEC;
impl crate::sealed::RegSpec for LcdcBlankingxyReg_SPEC {
    type DataType = u32;
}

pub type LcdcBlankingxyReg = crate::RegValueT<LcdcBlankingxyReg_SPEC>;

impl LcdcBlankingxyReg {
    #[inline(always)]
    pub fn blanking_x(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        u16,
        u16,
        LcdcBlankingxyReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            LcdcBlankingxyReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn blanking_y(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        LcdcBlankingxyReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            LcdcBlankingxyReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for LcdcBlankingxyReg {
    #[inline(always)]
    fn default() -> LcdcBlankingxyReg {
        <crate::RegValueT<LcdcBlankingxyReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LcdcClkctrlCgReg_SPEC;
impl crate::sealed::RegSpec for LcdcClkctrlCgReg_SPEC {
    type DataType = u32;
}

pub type LcdcClkctrlCgReg = crate::RegValueT<LcdcClkctrlCgReg_SPEC>;

impl LcdcClkctrlCgReg {
    #[inline(always)]
    pub fn lcdc_swap_pix_format_clk(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, LcdcClkctrlCgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,LcdcClkctrlCgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lcdc_inv_clk_polarity(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, LcdcClkctrlCgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,LcdcClkctrlCgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lcdc_clk_div_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, LcdcClkctrlCgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,LcdcClkctrlCgReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for LcdcClkctrlCgReg {
    #[inline(always)]
    fn default() -> LcdcClkctrlCgReg {
        <crate::RegValueT<LcdcClkctrlCgReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LcdcClkctrlReg_SPEC;
impl crate::sealed::RegSpec for LcdcClkctrlReg_SPEC {
    type DataType = u32;
}

pub type LcdcClkctrlReg = crate::RegValueT<LcdcClkctrlReg_SPEC>;

impl LcdcClkctrlReg {
    #[inline(always)]
    pub fn sec_clk_div(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, u8, LcdcClkctrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<27,0x1f,1,0,u8,u8,LcdcClkctrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_hold(
        self,
    ) -> crate::common::RegisterField<8, 0x3f, 1, 0, u8, u8, LcdcClkctrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3f,1,0,u8,u8,LcdcClkctrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn clk_div(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, LcdcClkctrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,LcdcClkctrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for LcdcClkctrlReg {
    #[inline(always)]
    fn default() -> LcdcClkctrlReg {
        <crate::RegValueT<LcdcClkctrlReg_SPEC> as RegisterValue<_>>::new(1025)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LcdcColmod_SPEC;
impl crate::sealed::RegSpec for LcdcColmod_SPEC {
    type DataType = u32;
}

pub type LcdcColmod = crate::RegValueT<LcdcColmod_SPEC>;

impl LcdcColmod {
    #[inline(always)]
    pub fn bp_dbib(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, LcdcColmod_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31,1,0,LcdcColmod_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn colmodes(
        self,
    ) -> crate::common::RegisterField<0, 0x1ffff, 1, 0, u32, u32, LcdcColmod_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1ffff,1,0,u32,u32,LcdcColmod_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for LcdcColmod {
    #[inline(always)]
    fn default() -> LcdcColmod {
        <crate::RegValueT<LcdcColmod_SPEC> as RegisterValue<_>>::new(544274520)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LcdcConfReg_SPEC;
impl crate::sealed::RegSpec for LcdcConfReg_SPEC {
    type DataType = u32;
}

pub type LcdcConfReg = crate::RegValueT<LcdcConfReg_SPEC>;

impl LcdcConfReg {
    #[inline(always)]
    pub fn conf(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, LcdcConfReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,LcdcConfReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for LcdcConfReg {
    #[inline(always)]
    fn default() -> LcdcConfReg {
        <crate::RegValueT<LcdcConfReg_SPEC> as RegisterValue<_>>::new(13145)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LcdcCrcReg_SPEC;
impl crate::sealed::RegSpec for LcdcCrcReg_SPEC {
    type DataType = u32;
}

pub type LcdcCrcReg = crate::RegValueT<LcdcCrcReg_SPEC>;

impl LcdcCrcReg {
    #[inline(always)]
    pub fn crc(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        LcdcCrcReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            LcdcCrcReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for LcdcCrcReg {
    #[inline(always)]
    fn default() -> LcdcCrcReg {
        <crate::RegValueT<LcdcCrcReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LcdcDbibCfgReg_SPEC;
impl crate::sealed::RegSpec for LcdcDbibCfgReg_SPEC {
    type DataType = u32;
}

pub type LcdcDbibCfgReg = crate::RegValueT<LcdcDbibCfgReg_SPEC>;

impl LcdcDbibCfgReg {
    #[inline(always)]
    pub fn dbib_interface_en(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dbib_csx_cfg_en(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dbib_csx_cfg(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dbib_te_disable(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn spi_dc_as_spi_sd1(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dbib_force_idle(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dbib_resx_out_en(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sub_pixel_reverse(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn spi3_en(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn spi4_en(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dbib_back_pressure_en(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn spi_clk_phase(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn spi_clk_polarity(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn spid_jdi(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cmd_data_as_header(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn bit_order_addr_invert(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn spi_2byte_addr(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pix_clk_at_dbib_clk(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ext_ctrl_en(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn horizontal_blank_en(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dual_spi_subpixel_extract_en(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn quad_spi_en(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dual_spi_en(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dbib_interface_width(
        self,
    ) -> crate::common::RegisterField<6, 0x7, 1, 0, u8, u8, LcdcDbibCfgReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x7,1,0,u8,u8,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dbib_data_order(
        self,
    ) -> crate::common::RegisterField<3, 0x7, 1, 0, u8, u8, LcdcDbibCfgReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x7,1,0,u8,u8,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dbib_color_fmt(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, LcdcDbibCfgReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for LcdcDbibCfgReg {
    #[inline(always)]
    fn default() -> LcdcDbibCfgReg {
        <crate::RegValueT<LcdcDbibCfgReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LcdcDbibCmdReg_SPEC;
impl crate::sealed::RegSpec for LcdcDbibCmdReg_SPEC {
    type DataType = u32;
}

pub type LcdcDbibCmdReg = crate::RegValueT<LcdcDbibCmdReg_SPEC>;

impl LcdcDbibCmdReg {
    #[inline(always)]
    pub fn part_update(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, LcdcDbibCmdReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,LcdcDbibCmdReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dbib_cmd_send(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, LcdcDbibCmdReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30,1,0,LcdcDbibCmdReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cmd_width(
        self,
    ) -> crate::common::RegisterField<28, 0x3, 1, 0, u8, u8, LcdcDbibCmdReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x3,1,0,u8,u8,LcdcDbibCmdReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dbib_cmd_store(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, LcdcDbibCmdReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27,1,0,LcdcDbibCmdReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rd_mode_en(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, LcdcDbibCmdReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26,1,0,LcdcDbibCmdReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn fmtctrl_expose_setting(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, LcdcDbibCmdReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25,1,0,LcdcDbibCmdReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn st_int_cmd_type(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, LcdcDbibCmdReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24,1,0,LcdcDbibCmdReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dbib_cmd_val(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        LcdcDbibCmdReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            LcdcDbibCmdReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for LcdcDbibCmdReg {
    #[inline(always)]
    fn default() -> LcdcDbibCmdReg {
        <crate::RegValueT<LcdcDbibCmdReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LcdcDbibRdat_SPEC;
impl crate::sealed::RegSpec for LcdcDbibRdat_SPEC {
    type DataType = u32;
}

pub type LcdcDbibRdat = crate::RegValueT<LcdcDbibRdat_SPEC>;

impl LcdcDbibRdat {
    #[inline(always)]
    pub fn dbib_rdat(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        LcdcDbibRdat_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            LcdcDbibRdat_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for LcdcDbibRdat {
    #[inline(always)]
    fn default() -> LcdcDbibRdat {
        <crate::RegValueT<LcdcDbibRdat_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LcdcFmtctrl2Reg_SPEC;
impl crate::sealed::RegSpec for LcdcFmtctrl2Reg_SPEC {
    type DataType = u32;
}

pub type LcdcFmtctrl2Reg = crate::RegValueT<LcdcFmtctrl2Reg_SPEC>;

impl LcdcFmtctrl2Reg {
    #[inline(always)]
    pub fn fmtctrl(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        LcdcFmtctrl2Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            LcdcFmtctrl2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for LcdcFmtctrl2Reg {
    #[inline(always)]
    fn default() -> LcdcFmtctrl2Reg {
        <crate::RegValueT<LcdcFmtctrl2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LcdcFmtctrlReg_SPEC;
impl crate::sealed::RegSpec for LcdcFmtctrlReg_SPEC {
    type DataType = u32;
}

pub type LcdcFmtctrlReg = crate::RegValueT<LcdcFmtctrlReg_SPEC>;

impl LcdcFmtctrlReg {
    #[inline(always)]
    pub fn fmtctrl(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        LcdcFmtctrlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            LcdcFmtctrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for LcdcFmtctrlReg {
    #[inline(always)]
    fn default() -> LcdcFmtctrlReg {
        <crate::RegValueT<LcdcFmtctrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LcdcFrontporchxyReg_SPEC;
impl crate::sealed::RegSpec for LcdcFrontporchxyReg_SPEC {
    type DataType = u32;
}

pub type LcdcFrontporchxyReg = crate::RegValueT<LcdcFrontporchxyReg_SPEC>;

impl LcdcFrontporchxyReg {
    #[inline(always)]
    pub fn fporch_x(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        u16,
        u16,
        LcdcFrontporchxyReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            LcdcFrontporchxyReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn fporch_y(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        LcdcFrontporchxyReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            LcdcFrontporchxyReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for LcdcFrontporchxyReg {
    #[inline(always)]
    fn default() -> LcdcFrontporchxyReg {
        <crate::RegValueT<LcdcFrontporchxyReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LcdcGpioReg_SPEC;
impl crate::sealed::RegSpec for LcdcGpioReg_SPEC {
    type DataType = u32;
}

pub type LcdcGpioReg = crate::RegValueT<LcdcGpioReg_SPEC>;

impl LcdcGpioReg {
    #[inline(always)]
    pub fn dpi_cm_assert(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, LcdcGpioReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<16,1,0,LcdcGpioReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dpi_sd_assert(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, LcdcGpioReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<15,1,0,LcdcGpioReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn scal_advance(
        self,
    ) -> crate::common::RegisterField<13, 0x3, 1, 0, u8, u8, LcdcGpioReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x3,1,0,u8,u8,LcdcGpioReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lcdc_other(
        self,
    ) -> crate::common::RegisterField<6, 0x7f, 1, 0, u8, u8, LcdcGpioReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x7f,1,0,u8,u8,LcdcGpioReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn gpio_spi_si_on_sd_pad(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, LcdcGpioReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,LcdcGpioReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn gpio_output_mode(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, u8, LcdcGpioReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x3,1,0,u8,u8,LcdcGpioReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn gpio_output_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, LcdcGpioReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,LcdcGpioReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn te_inv(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, LcdcGpioReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,LcdcGpioReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn clk_div_2div3(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, LcdcGpioReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,LcdcGpioReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for LcdcGpioReg {
    #[inline(always)]
    fn default() -> LcdcGpioReg {
        <crate::RegValueT<LcdcGpioReg_SPEC> as RegisterValue<_>>::new(16384)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LcdcIdregReg_SPEC;
impl crate::sealed::RegSpec for LcdcIdregReg_SPEC {
    type DataType = u32;
}

pub type LcdcIdregReg = crate::RegValueT<LcdcIdregReg_SPEC>;

impl LcdcIdregReg {
    #[inline(always)]
    pub fn nemadc_id(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        LcdcIdregReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            LcdcIdregReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for LcdcIdregReg {
    #[inline(always)]
    fn default() -> LcdcIdregReg {
        <crate::RegValueT<LcdcIdregReg_SPEC> as RegisterValue<_>>::new(2147483647)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LcdcInterruptReg_SPEC;
impl crate::sealed::RegSpec for LcdcInterruptReg_SPEC {
    type DataType = u32;
}

pub type LcdcInterruptReg = crate::RegValueT<LcdcInterruptReg_SPEC>;

impl LcdcInterruptReg {
    #[inline(always)]
    pub fn irq_trigger_sel(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, LcdcInterruptReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,LcdcInterruptReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn irq_out(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, LcdcInterruptReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30,1,0,LcdcInterruptReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn fe_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, LcdcInterruptReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,LcdcInterruptReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn te_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, LcdcInterruptReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,LcdcInterruptReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn mmu_error_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, LcdcInterruptReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,LcdcInterruptReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn hsync_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, LcdcInterruptReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,LcdcInterruptReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vsync_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, LcdcInterruptReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,LcdcInterruptReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for LcdcInterruptReg {
    #[inline(always)]
    fn default() -> LcdcInterruptReg {
        <crate::RegValueT<LcdcInterruptReg_SPEC> as RegisterValue<_>>::new(1)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LcdcLayer0BaseaddrReg_SPEC;
impl crate::sealed::RegSpec for LcdcLayer0BaseaddrReg_SPEC {
    type DataType = u32;
}

pub type LcdcLayer0BaseaddrReg = crate::RegValueT<LcdcLayer0BaseaddrReg_SPEC>;

impl LcdcLayer0BaseaddrReg {
    #[inline(always)]
    pub fn l0_base_addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        LcdcLayer0BaseaddrReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            LcdcLayer0BaseaddrReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for LcdcLayer0BaseaddrReg {
    #[inline(always)]
    fn default() -> LcdcLayer0BaseaddrReg {
        <crate::RegValueT<LcdcLayer0BaseaddrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LcdcLayer0ModeReg_SPEC;
impl crate::sealed::RegSpec for LcdcLayer0ModeReg_SPEC {
    type DataType = u32;
}

pub type LcdcLayer0ModeReg = crate::RegValueT<LcdcLayer0ModeReg_SPEC>;

impl LcdcLayer0ModeReg {
    #[inline(always)]
    pub fn l0_en(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, LcdcLayer0ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,LcdcLayer0ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn l0_force_alpha(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, LcdcLayer0ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30,1,0,LcdcLayer0ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn l0_dis_bil_filtering(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, LcdcLayer0ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29,1,0,LcdcLayer0ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn l0_premul_img_alpha(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, LcdcLayer0ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28,1,0,LcdcLayer0ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn l0_assert_hclk_dma(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, LcdcLayer0ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27,1,0,LcdcLayer0ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn l0_gamma_lut_en(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, LcdcLayer0ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26,1,0,LcdcLayer0ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn l0_alpha(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        u8,
        u8,
        LcdcLayer0ModeReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            u8,
            u8,
            LcdcLayer0ModeReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn l0_dst_blend(
        self,
    ) -> crate::common::RegisterField<
        12,
        0xf,
        1,
        0,
        u8,
        u8,
        LcdcLayer0ModeReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0xf,
            1,
            0,
            u8,
            u8,
            LcdcLayer0ModeReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn l0_src_blend(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, LcdcLayer0ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            8,
            0xf,
            1,
            0,
            u8,
            u8,
            LcdcLayer0ModeReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn l0_color_mode(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        u8,
        u8,
        LcdcLayer0ModeReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            u8,
            u8,
            LcdcLayer0ModeReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for LcdcLayer0ModeReg {
    #[inline(always)]
    fn default() -> LcdcLayer0ModeReg {
        <crate::RegValueT<LcdcLayer0ModeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LcdcLayer0ResxyReg_SPEC;
impl crate::sealed::RegSpec for LcdcLayer0ResxyReg_SPEC {
    type DataType = u32;
}

pub type LcdcLayer0ResxyReg = crate::RegValueT<LcdcLayer0ResxyReg_SPEC>;

impl LcdcLayer0ResxyReg {
    #[inline(always)]
    pub fn l0_res_x(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        u16,
        u16,
        LcdcLayer0ResxyReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            LcdcLayer0ResxyReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn l0_res_y(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        LcdcLayer0ResxyReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            LcdcLayer0ResxyReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for LcdcLayer0ResxyReg {
    #[inline(always)]
    fn default() -> LcdcLayer0ResxyReg {
        <crate::RegValueT<LcdcLayer0ResxyReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LcdcLayer0SizexyReg_SPEC;
impl crate::sealed::RegSpec for LcdcLayer0SizexyReg_SPEC {
    type DataType = u32;
}

pub type LcdcLayer0SizexyReg = crate::RegValueT<LcdcLayer0SizexyReg_SPEC>;

impl LcdcLayer0SizexyReg {
    #[inline(always)]
    pub fn l0_size_x(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        u16,
        u16,
        LcdcLayer0SizexyReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            LcdcLayer0SizexyReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn l0_size_y(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        LcdcLayer0SizexyReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            LcdcLayer0SizexyReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for LcdcLayer0SizexyReg {
    #[inline(always)]
    fn default() -> LcdcLayer0SizexyReg {
        <crate::RegValueT<LcdcLayer0SizexyReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LcdcLayer0StartxyReg_SPEC;
impl crate::sealed::RegSpec for LcdcLayer0StartxyReg_SPEC {
    type DataType = u32;
}

pub type LcdcLayer0StartxyReg = crate::RegValueT<LcdcLayer0StartxyReg_SPEC>;

impl LcdcLayer0StartxyReg {
    #[inline(always)]
    pub fn l0_start_x(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        u16,
        u16,
        LcdcLayer0StartxyReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            LcdcLayer0StartxyReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn l0_start_y(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        LcdcLayer0StartxyReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            LcdcLayer0StartxyReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for LcdcLayer0StartxyReg {
    #[inline(always)]
    fn default() -> LcdcLayer0StartxyReg {
        <crate::RegValueT<LcdcLayer0StartxyReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LcdcLayer0StrideReg_SPEC;
impl crate::sealed::RegSpec for LcdcLayer0StrideReg_SPEC {
    type DataType = u32;
}

pub type LcdcLayer0StrideReg = crate::RegValueT<LcdcLayer0StrideReg_SPEC>;

impl LcdcLayer0StrideReg {
    #[inline(always)]
    pub fn l0_fifo_thr(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x3,
        1,
        0,
        u8,
        u8,
        LcdcLayer0StrideReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x3,
            1,
            0,
            u8,
            u8,
            LcdcLayer0StrideReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn l0_burst_len(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7,
        1,
        0,
        u8,
        u8,
        LcdcLayer0StrideReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7,
            1,
            0,
            u8,
            u8,
            LcdcLayer0StrideReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn l0_stride(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        LcdcLayer0StrideReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            LcdcLayer0StrideReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for LcdcLayer0StrideReg {
    #[inline(always)]
    fn default() -> LcdcLayer0StrideReg {
        <crate::RegValueT<LcdcLayer0StrideReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LcdcLayer1BaseaddrReg_SPEC;
impl crate::sealed::RegSpec for LcdcLayer1BaseaddrReg_SPEC {
    type DataType = u32;
}

pub type LcdcLayer1BaseaddrReg = crate::RegValueT<LcdcLayer1BaseaddrReg_SPEC>;

impl LcdcLayer1BaseaddrReg {
    #[inline(always)]
    pub fn l1_base_addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        LcdcLayer1BaseaddrReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            LcdcLayer1BaseaddrReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for LcdcLayer1BaseaddrReg {
    #[inline(always)]
    fn default() -> LcdcLayer1BaseaddrReg {
        <crate::RegValueT<LcdcLayer1BaseaddrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LcdcLayer1ModeReg_SPEC;
impl crate::sealed::RegSpec for LcdcLayer1ModeReg_SPEC {
    type DataType = u32;
}

pub type LcdcLayer1ModeReg = crate::RegValueT<LcdcLayer1ModeReg_SPEC>;

impl LcdcLayer1ModeReg {
    #[inline(always)]
    pub fn l1_en(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, LcdcLayer1ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,LcdcLayer1ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn l1_force_alpha(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, LcdcLayer1ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30,1,0,LcdcLayer1ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn l1_dis_bil_filtering(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, LcdcLayer1ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29,1,0,LcdcLayer1ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn l1_premul_img_alpha(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, LcdcLayer1ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28,1,0,LcdcLayer1ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn l1_assert_hclk_dma(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, LcdcLayer1ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27,1,0,LcdcLayer1ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn l1_gamma_lut_en(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, LcdcLayer1ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26,1,0,LcdcLayer1ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn l1_alpha(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        u8,
        u8,
        LcdcLayer1ModeReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            u8,
            u8,
            LcdcLayer1ModeReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn l1_dst_blend(
        self,
    ) -> crate::common::RegisterField<
        12,
        0xf,
        1,
        0,
        u8,
        u8,
        LcdcLayer1ModeReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0xf,
            1,
            0,
            u8,
            u8,
            LcdcLayer1ModeReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn l1_src_blend(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, LcdcLayer1ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            8,
            0xf,
            1,
            0,
            u8,
            u8,
            LcdcLayer1ModeReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn l1_color_mode(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1f,
        1,
        0,
        u8,
        u8,
        LcdcLayer1ModeReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1f,
            1,
            0,
            u8,
            u8,
            LcdcLayer1ModeReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for LcdcLayer1ModeReg {
    #[inline(always)]
    fn default() -> LcdcLayer1ModeReg {
        <crate::RegValueT<LcdcLayer1ModeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LcdcLayer1ResxyReg_SPEC;
impl crate::sealed::RegSpec for LcdcLayer1ResxyReg_SPEC {
    type DataType = u32;
}

pub type LcdcLayer1ResxyReg = crate::RegValueT<LcdcLayer1ResxyReg_SPEC>;

impl LcdcLayer1ResxyReg {
    #[inline(always)]
    pub fn l1_res_x(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        u16,
        u16,
        LcdcLayer1ResxyReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            LcdcLayer1ResxyReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn l1_res_y(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        LcdcLayer1ResxyReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            LcdcLayer1ResxyReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for LcdcLayer1ResxyReg {
    #[inline(always)]
    fn default() -> LcdcLayer1ResxyReg {
        <crate::RegValueT<LcdcLayer1ResxyReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LcdcLayer1SizexyReg_SPEC;
impl crate::sealed::RegSpec for LcdcLayer1SizexyReg_SPEC {
    type DataType = u32;
}

pub type LcdcLayer1SizexyReg = crate::RegValueT<LcdcLayer1SizexyReg_SPEC>;

impl LcdcLayer1SizexyReg {
    #[inline(always)]
    pub fn l1_size_x(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        u16,
        u16,
        LcdcLayer1SizexyReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            LcdcLayer1SizexyReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn l1_size_y(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        LcdcLayer1SizexyReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            LcdcLayer1SizexyReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for LcdcLayer1SizexyReg {
    #[inline(always)]
    fn default() -> LcdcLayer1SizexyReg {
        <crate::RegValueT<LcdcLayer1SizexyReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LcdcLayer1StartxyReg_SPEC;
impl crate::sealed::RegSpec for LcdcLayer1StartxyReg_SPEC {
    type DataType = u32;
}

pub type LcdcLayer1StartxyReg = crate::RegValueT<LcdcLayer1StartxyReg_SPEC>;

impl LcdcLayer1StartxyReg {
    #[inline(always)]
    pub fn l1_start_x(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        u16,
        u16,
        LcdcLayer1StartxyReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            LcdcLayer1StartxyReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn l1_start_y(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        LcdcLayer1StartxyReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            LcdcLayer1StartxyReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for LcdcLayer1StartxyReg {
    #[inline(always)]
    fn default() -> LcdcLayer1StartxyReg {
        <crate::RegValueT<LcdcLayer1StartxyReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LcdcLayer1StrideReg_SPEC;
impl crate::sealed::RegSpec for LcdcLayer1StrideReg_SPEC {
    type DataType = u32;
}

pub type LcdcLayer1StrideReg = crate::RegValueT<LcdcLayer1StrideReg_SPEC>;

impl LcdcLayer1StrideReg {
    #[inline(always)]
    pub fn l1_fifo_thr(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x3,
        1,
        0,
        u8,
        u8,
        LcdcLayer1StrideReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x3,
            1,
            0,
            u8,
            u8,
            LcdcLayer1StrideReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn l1_burst_len(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7,
        1,
        0,
        u8,
        u8,
        LcdcLayer1StrideReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7,
            1,
            0,
            u8,
            u8,
            LcdcLayer1StrideReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn l1_stride(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        LcdcLayer1StrideReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            LcdcLayer1StrideReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for LcdcLayer1StrideReg {
    #[inline(always)]
    fn default() -> LcdcLayer1StrideReg {
        <crate::RegValueT<LcdcLayer1StrideReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LcdcModeReg_SPEC;
impl crate::sealed::RegSpec for LcdcModeReg_SPEC {
    type DataType = u32;
}

pub type LcdcModeReg = crate::RegValueT<LcdcModeReg_SPEC>;

impl LcdcModeReg {
    #[inline(always)]
    pub fn mode_en(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, LcdcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,LcdcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vsync_pol(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, LcdcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28,1,0,LcdcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn hsync_pol(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, LcdcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27,1,0,LcdcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn de_pol(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, LcdcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26,1,0,LcdcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dith_mode(
        self,
    ) -> crate::common::RegisterField<24, 0x3, 1, 0, u8, u8, LcdcModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x3,1,0,u8,u8,LcdcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn vsync_scpl(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, LcdcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23,1,0,LcdcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pixclkout_pol(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, LcdcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22,1,0,LcdcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn global_gamma_en(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, LcdcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20,1,0,LcdcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn force_blank(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, LcdcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19,1,0,LcdcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sframe_upd(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, LcdcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17,1,0,LcdcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dpi2_config(
        self,
    ) -> crate::common::RegisterField<12, 0x7, 1, 0, u8, u8, LcdcModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x7,1,0,u8,u8,LcdcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pixclkout_sel(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, LcdcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,LcdcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn out_mode(
        self,
    ) -> crate::common::RegisterField<5, 0xf, 1, 0, u8, u8, LcdcModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0xf,1,0,u8,u8,LcdcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dbib_off(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, LcdcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,LcdcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn form_off(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, LcdcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,LcdcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dscan(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, LcdcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,LcdcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tmode(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, LcdcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,LcdcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for LcdcModeReg {
    #[inline(always)]
    fn default() -> LcdcModeReg {
        <crate::RegValueT<LcdcModeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LcdcPalette255_SPEC;
impl crate::sealed::RegSpec for LcdcPalette255_SPEC {
    type DataType = u32;
}

pub type LcdcPalette255 = crate::RegValueT<LcdcPalette255_SPEC>;

impl LcdcPalette255 {
    #[inline(always)]
    pub fn pallete_r(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, LcdcPalette255_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,LcdcPalette255_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pallete_g(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, LcdcPalette255_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,LcdcPalette255_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pallete_b(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, LcdcPalette255_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,LcdcPalette255_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for LcdcPalette255 {
    #[inline(always)]
    fn default() -> LcdcPalette255 {
        <crate::RegValueT<LcdcPalette255_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LcdcPaletteBase_SPEC;
impl crate::sealed::RegSpec for LcdcPaletteBase_SPEC {
    type DataType = u32;
}

pub type LcdcPaletteBase = crate::RegValueT<LcdcPaletteBase_SPEC>;

impl LcdcPaletteBase {
    #[inline(always)]
    pub fn pallete_r(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, LcdcPaletteBase_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,LcdcPaletteBase_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pallete_g(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, LcdcPaletteBase_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,LcdcPaletteBase_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn pallete_b(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, LcdcPaletteBase_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,LcdcPaletteBase_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for LcdcPaletteBase {
    #[inline(always)]
    fn default() -> LcdcPaletteBase {
        <crate::RegValueT<LcdcPaletteBase_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LcdcResxyReg_SPEC;
impl crate::sealed::RegSpec for LcdcResxyReg_SPEC {
    type DataType = u32;
}

pub type LcdcResxyReg = crate::RegValueT<LcdcResxyReg_SPEC>;

impl LcdcResxyReg {
    #[inline(always)]
    pub fn res_x(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        u16,
        u16,
        LcdcResxyReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            LcdcResxyReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn res_y(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, LcdcResxyReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            LcdcResxyReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for LcdcResxyReg {
    #[inline(always)]
    fn default() -> LcdcResxyReg {
        <crate::RegValueT<LcdcResxyReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LcdcStartxyReg_SPEC;
impl crate::sealed::RegSpec for LcdcStartxyReg_SPEC {
    type DataType = u32;
}

pub type LcdcStartxyReg = crate::RegValueT<LcdcStartxyReg_SPEC>;

impl LcdcStartxyReg {
    #[inline(always)]
    pub fn start_x(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        u16,
        u16,
        LcdcStartxyReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            LcdcStartxyReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn start_y(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        LcdcStartxyReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            LcdcStartxyReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for LcdcStartxyReg {
    #[inline(always)]
    fn default() -> LcdcStartxyReg {
        <crate::RegValueT<LcdcStartxyReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LcdcStatusReg_SPEC;
impl crate::sealed::RegSpec for LcdcStatusReg_SPEC {
    type DataType = u32;
}

pub type LcdcStatusReg = crate::RegValueT<LcdcStatusReg_SPEC>;

impl LcdcStatusReg {
    #[inline(always)]
    pub fn dbib_cmd_fifo_full(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, LcdcStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15,1,0,LcdcStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dbi_spi_cs(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, LcdcStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14,1,0,LcdcStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn frame_end(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, LcdcStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13,1,0,LcdcStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dbib_out_trans_pending(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, LcdcStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12,1,0,LcdcStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dbib_cmd_pending(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, LcdcStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11,1,0,LcdcStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dbib_data_pending(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, LcdcStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,LcdcStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dbib_te(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, LcdcStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,LcdcStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sticky_underflow(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, LcdcStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,LcdcStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn underflow(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, LcdcStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,LcdcStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn last_row(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, LcdcStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,LcdcStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn stat_csync(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, LcdcStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,LcdcStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn stat_vsync(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, LcdcStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,LcdcStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn stat_hsync(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, LcdcStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,LcdcStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn framegen_busy(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, LcdcStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,LcdcStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn stat_active(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, LcdcStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,LcdcStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for LcdcStatusReg {
    #[inline(always)]
    fn default() -> LcdcStatusReg {
        <crate::RegValueT<LcdcStatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
