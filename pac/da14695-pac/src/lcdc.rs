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
    pub const fn lcdc_jdi_enb_end_hline_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LcdcJdiEnbEndHlineReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LcdcJdiEnbEndHlineReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(188usize),
            )
        }
    }

    #[inline(always)]
    pub const fn lcdc_jdi_enb_start_clk_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LcdcJdiEnbStartClkReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LcdcJdiEnbStartClkReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(192usize),
            )
        }
    }

    #[inline(always)]
    pub const fn lcdc_jdi_enb_start_hline_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LcdcJdiEnbStartHlineReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LcdcJdiEnbStartHlineReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(184usize),
            )
        }
    }

    #[inline(always)]
    pub const fn lcdc_jdi_enb_width_clk_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LcdcJdiEnbWidthClkReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LcdcJdiEnbWidthClkReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(196usize),
            )
        }
    }

    #[inline(always)]
    pub const fn lcdc_jdi_fbx_blanking_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LcdcJdiFbxBlankingReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LcdcJdiFbxBlankingReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(148usize),
            )
        }
    }

    #[inline(always)]
    pub const fn lcdc_jdi_fby_blanking_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LcdcJdiFbyBlankingReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LcdcJdiFbyBlankingReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(152usize),
            )
        }
    }

    #[inline(always)]
    pub const fn lcdc_jdi_hck_width_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LcdcJdiHckWidthReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LcdcJdiHckWidthReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(156usize),
            )
        }
    }

    #[inline(always)]
    pub const fn lcdc_jdi_hst_delay_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LcdcJdiHstDelayReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LcdcJdiHstDelayReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(176usize),
            )
        }
    }

    #[inline(always)]
    pub const fn lcdc_jdi_hst_width_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LcdcJdiHstWidthReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LcdcJdiHstWidthReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(180usize),
            )
        }
    }

    #[inline(always)]
    pub const fn lcdc_jdi_resxy_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LcdcJdiResxyReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LcdcJdiResxyReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(144usize),
            )
        }
    }

    #[inline(always)]
    pub const fn lcdc_jdi_vck_delay_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LcdcJdiVckDelayReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LcdcJdiVckDelayReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(172usize),
            )
        }
    }

    #[inline(always)]
    pub const fn lcdc_jdi_vst_delay_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LcdcJdiVstDelayReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LcdcJdiVstDelayReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(164usize),
            )
        }
    }

    #[inline(always)]
    pub const fn lcdc_jdi_vst_width_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LcdcJdiVstWidthReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LcdcJdiVstWidthReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(168usize),
            )
        }
    }

    #[inline(always)]
    pub const fn lcdc_jdi_xrst_width_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LcdcJdiXrstWidthReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LcdcJdiXrstWidthReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(160usize),
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
    pub const fn lcdc_layer0_offsetx_reg(
        &self,
    ) -> &'static crate::common::Reg<self::LcdcLayer0OffsetxReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::LcdcLayer0OffsetxReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(392usize),
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
    pub fn lcdc_bporch_x(
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
    pub fn lcdc_bporch_y(
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
    pub fn lcdc_bg_red(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, LcdcBgcolorReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,LcdcBgcolorReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lcdc_bg_green(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, LcdcBgcolorReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,LcdcBgcolorReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lcdc_bg_blue(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, LcdcBgcolorReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,LcdcBgcolorReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lcdc_bg_alpha(
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
    pub fn lcdc_blanking_x(
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
    pub fn lcdc_blanking_y(
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
pub struct LcdcClkctrlReg_SPEC;
impl crate::sealed::RegSpec for LcdcClkctrlReg_SPEC {
    type DataType = u32;
}

pub type LcdcClkctrlReg = crate::RegValueT<LcdcClkctrlReg_SPEC>;

impl LcdcClkctrlReg {
    #[inline(always)]
    pub fn lcdc_sec_clk_div(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, u8, LcdcClkctrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<27,0x1f,1,0,u8,u8,LcdcClkctrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lcdc_dma_hold(
        self,
    ) -> crate::common::RegisterField<8, 0x3f, 1, 0, u8, u8, LcdcClkctrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3f,1,0,u8,u8,LcdcClkctrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lcdc_clk_div(
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
pub struct LcdcCrcReg_SPEC;
impl crate::sealed::RegSpec for LcdcCrcReg_SPEC {
    type DataType = u32;
}

pub type LcdcCrcReg = crate::RegValueT<LcdcCrcReg_SPEC>;

impl LcdcCrcReg {
    #[inline(always)]
    pub fn lcdc_crc(
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
    pub fn lcdc_dbib_te_dis(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lcdc_dbib_csx_force(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lcdc_dbib_csx_force_val(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lcdc_dbib_spi_pad(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lcdc_dbib_resx(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lcdc_dbib_dma_en(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lcdc_dbib_spi3_en(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lcdc_dbib_spi4_en(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lcdc_dbib_spi_cpha(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lcdc_dbib_spi_cpol(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lcdc_dbib_spi_jdi(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lcdc_dbib_spi_hold(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lcdc_dbib_spi_inv_addr(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lcdc_dbib_inv_data(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lcdc_dbib_jdi_inv_pix(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lcdc_dbib_jdi_soft_rst(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lcdc_dbib_fmt(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, LcdcDbibCfgReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
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
    pub fn lcdc_dbib_cmd_send(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, LcdcDbibCmdReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30,1,0,LcdcDbibCmdReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lcdc_dbib_cmd_store(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, LcdcDbibCmdReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27,1,0,LcdcDbibCmdReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lcdc_dbib_cmd_val(
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
pub struct LcdcFrontporchxyReg_SPEC;
impl crate::sealed::RegSpec for LcdcFrontporchxyReg_SPEC {
    type DataType = u32;
}

pub type LcdcFrontporchxyReg = crate::RegValueT<LcdcFrontporchxyReg_SPEC>;

impl LcdcFrontporchxyReg {
    #[inline(always)]
    pub fn lcdc_fporch_x(
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
    pub fn lcdc_fporch_y(
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
    pub fn lcdc_te_inv(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, LcdcGpioReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,LcdcGpioReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lcdc_parif_sel(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, LcdcGpioReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,LcdcGpioReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for LcdcGpioReg {
    #[inline(always)]
    fn default() -> LcdcGpioReg {
        <crate::RegValueT<LcdcGpioReg_SPEC> as RegisterValue<_>>::new(0)
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
    pub fn lcdc_id(
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
    pub fn lcdc_irq_trigger_sel(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, LcdcInterruptReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,LcdcInterruptReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lcdc_frame_end_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, LcdcInterruptReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,LcdcInterruptReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lcdc_te_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, LcdcInterruptReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,LcdcInterruptReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lcdc_hsync_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, LcdcInterruptReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,LcdcInterruptReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lcdc_vsync_irq_en(
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
pub struct LcdcJdiEnbEndHlineReg_SPEC;
impl crate::sealed::RegSpec for LcdcJdiEnbEndHlineReg_SPEC {
    type DataType = u32;
}

pub type LcdcJdiEnbEndHlineReg = crate::RegValueT<LcdcJdiEnbEndHlineReg_SPEC>;

impl LcdcJdiEnbEndHlineReg {
    #[inline(always)]
    pub fn lcdc_jdi_enb_end_hline(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        LcdcJdiEnbEndHlineReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            LcdcJdiEnbEndHlineReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for LcdcJdiEnbEndHlineReg {
    #[inline(always)]
    fn default() -> LcdcJdiEnbEndHlineReg {
        <crate::RegValueT<LcdcJdiEnbEndHlineReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LcdcJdiEnbStartClkReg_SPEC;
impl crate::sealed::RegSpec for LcdcJdiEnbStartClkReg_SPEC {
    type DataType = u32;
}

pub type LcdcJdiEnbStartClkReg = crate::RegValueT<LcdcJdiEnbStartClkReg_SPEC>;

impl LcdcJdiEnbStartClkReg {
    #[inline(always)]
    pub fn lcdc_jdi_enb_start_clk(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        LcdcJdiEnbStartClkReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            LcdcJdiEnbStartClkReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for LcdcJdiEnbStartClkReg {
    #[inline(always)]
    fn default() -> LcdcJdiEnbStartClkReg {
        <crate::RegValueT<LcdcJdiEnbStartClkReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LcdcJdiEnbStartHlineReg_SPEC;
impl crate::sealed::RegSpec for LcdcJdiEnbStartHlineReg_SPEC {
    type DataType = u32;
}

pub type LcdcJdiEnbStartHlineReg = crate::RegValueT<LcdcJdiEnbStartHlineReg_SPEC>;

impl LcdcJdiEnbStartHlineReg {
    #[inline(always)]
    pub fn lcdc_jdi_enb_start_hline(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        LcdcJdiEnbStartHlineReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            LcdcJdiEnbStartHlineReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for LcdcJdiEnbStartHlineReg {
    #[inline(always)]
    fn default() -> LcdcJdiEnbStartHlineReg {
        <crate::RegValueT<LcdcJdiEnbStartHlineReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LcdcJdiEnbWidthClkReg_SPEC;
impl crate::sealed::RegSpec for LcdcJdiEnbWidthClkReg_SPEC {
    type DataType = u32;
}

pub type LcdcJdiEnbWidthClkReg = crate::RegValueT<LcdcJdiEnbWidthClkReg_SPEC>;

impl LcdcJdiEnbWidthClkReg {
    #[inline(always)]
    pub fn lcdc_jdi_enb_width_clk(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        LcdcJdiEnbWidthClkReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            LcdcJdiEnbWidthClkReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for LcdcJdiEnbWidthClkReg {
    #[inline(always)]
    fn default() -> LcdcJdiEnbWidthClkReg {
        <crate::RegValueT<LcdcJdiEnbWidthClkReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LcdcJdiFbxBlankingReg_SPEC;
impl crate::sealed::RegSpec for LcdcJdiFbxBlankingReg_SPEC {
    type DataType = u32;
}

pub type LcdcJdiFbxBlankingReg = crate::RegValueT<LcdcJdiFbxBlankingReg_SPEC>;

impl LcdcJdiFbxBlankingReg {
    #[inline(always)]
    pub fn lcdc_jdi_fxblanking(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        u16,
        u16,
        LcdcJdiFbxBlankingReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            LcdcJdiFbxBlankingReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn lcdc_jdi_bxblanking(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        LcdcJdiFbxBlankingReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            LcdcJdiFbxBlankingReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for LcdcJdiFbxBlankingReg {
    #[inline(always)]
    fn default() -> LcdcJdiFbxBlankingReg {
        <crate::RegValueT<LcdcJdiFbxBlankingReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LcdcJdiFbyBlankingReg_SPEC;
impl crate::sealed::RegSpec for LcdcJdiFbyBlankingReg_SPEC {
    type DataType = u32;
}

pub type LcdcJdiFbyBlankingReg = crate::RegValueT<LcdcJdiFbyBlankingReg_SPEC>;

impl LcdcJdiFbyBlankingReg {
    #[inline(always)]
    pub fn lcdc_jdi_fyblanking(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        u16,
        u16,
        LcdcJdiFbyBlankingReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            LcdcJdiFbyBlankingReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn lcdc_jdi_byblanking(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        LcdcJdiFbyBlankingReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            LcdcJdiFbyBlankingReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for LcdcJdiFbyBlankingReg {
    #[inline(always)]
    fn default() -> LcdcJdiFbyBlankingReg {
        <crate::RegValueT<LcdcJdiFbyBlankingReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LcdcJdiHckWidthReg_SPEC;
impl crate::sealed::RegSpec for LcdcJdiHckWidthReg_SPEC {
    type DataType = u32;
}

pub type LcdcJdiHckWidthReg = crate::RegValueT<LcdcJdiHckWidthReg_SPEC>;

impl LcdcJdiHckWidthReg {
    #[inline(always)]
    pub fn lcdc_jdi_hck_width(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        LcdcJdiHckWidthReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            LcdcJdiHckWidthReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for LcdcJdiHckWidthReg {
    #[inline(always)]
    fn default() -> LcdcJdiHckWidthReg {
        <crate::RegValueT<LcdcJdiHckWidthReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LcdcJdiHstDelayReg_SPEC;
impl crate::sealed::RegSpec for LcdcJdiHstDelayReg_SPEC {
    type DataType = u32;
}

pub type LcdcJdiHstDelayReg = crate::RegValueT<LcdcJdiHstDelayReg_SPEC>;

impl LcdcJdiHstDelayReg {
    #[inline(always)]
    pub fn lcdc_jdi_hst_delay(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        LcdcJdiHstDelayReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            LcdcJdiHstDelayReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for LcdcJdiHstDelayReg {
    #[inline(always)]
    fn default() -> LcdcJdiHstDelayReg {
        <crate::RegValueT<LcdcJdiHstDelayReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LcdcJdiHstWidthReg_SPEC;
impl crate::sealed::RegSpec for LcdcJdiHstWidthReg_SPEC {
    type DataType = u32;
}

pub type LcdcJdiHstWidthReg = crate::RegValueT<LcdcJdiHstWidthReg_SPEC>;

impl LcdcJdiHstWidthReg {
    #[inline(always)]
    pub fn lcdc_jdi_hst_width(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        LcdcJdiHstWidthReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            LcdcJdiHstWidthReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for LcdcJdiHstWidthReg {
    #[inline(always)]
    fn default() -> LcdcJdiHstWidthReg {
        <crate::RegValueT<LcdcJdiHstWidthReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LcdcJdiResxyReg_SPEC;
impl crate::sealed::RegSpec for LcdcJdiResxyReg_SPEC {
    type DataType = u32;
}

pub type LcdcJdiResxyReg = crate::RegValueT<LcdcJdiResxyReg_SPEC>;

impl LcdcJdiResxyReg {
    #[inline(always)]
    pub fn lcdc_jdi_res_x(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        u16,
        u16,
        LcdcJdiResxyReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            LcdcJdiResxyReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn lcdc_jdi_res_y(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        LcdcJdiResxyReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            LcdcJdiResxyReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for LcdcJdiResxyReg {
    #[inline(always)]
    fn default() -> LcdcJdiResxyReg {
        <crate::RegValueT<LcdcJdiResxyReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LcdcJdiVckDelayReg_SPEC;
impl crate::sealed::RegSpec for LcdcJdiVckDelayReg_SPEC {
    type DataType = u32;
}

pub type LcdcJdiVckDelayReg = crate::RegValueT<LcdcJdiVckDelayReg_SPEC>;

impl LcdcJdiVckDelayReg {
    #[inline(always)]
    pub fn lcdc_jdi_vck_delay(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        LcdcJdiVckDelayReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            LcdcJdiVckDelayReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for LcdcJdiVckDelayReg {
    #[inline(always)]
    fn default() -> LcdcJdiVckDelayReg {
        <crate::RegValueT<LcdcJdiVckDelayReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LcdcJdiVstDelayReg_SPEC;
impl crate::sealed::RegSpec for LcdcJdiVstDelayReg_SPEC {
    type DataType = u32;
}

pub type LcdcJdiVstDelayReg = crate::RegValueT<LcdcJdiVstDelayReg_SPEC>;

impl LcdcJdiVstDelayReg {
    #[inline(always)]
    pub fn lcdc_jdi_vst_delay(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        LcdcJdiVstDelayReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            LcdcJdiVstDelayReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for LcdcJdiVstDelayReg {
    #[inline(always)]
    fn default() -> LcdcJdiVstDelayReg {
        <crate::RegValueT<LcdcJdiVstDelayReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LcdcJdiVstWidthReg_SPEC;
impl crate::sealed::RegSpec for LcdcJdiVstWidthReg_SPEC {
    type DataType = u32;
}

pub type LcdcJdiVstWidthReg = crate::RegValueT<LcdcJdiVstWidthReg_SPEC>;

impl LcdcJdiVstWidthReg {
    #[inline(always)]
    pub fn lcdc_jdi_vst_width(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        LcdcJdiVstWidthReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            LcdcJdiVstWidthReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for LcdcJdiVstWidthReg {
    #[inline(always)]
    fn default() -> LcdcJdiVstWidthReg {
        <crate::RegValueT<LcdcJdiVstWidthReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LcdcJdiXrstWidthReg_SPEC;
impl crate::sealed::RegSpec for LcdcJdiXrstWidthReg_SPEC {
    type DataType = u32;
}

pub type LcdcJdiXrstWidthReg = crate::RegValueT<LcdcJdiXrstWidthReg_SPEC>;

impl LcdcJdiXrstWidthReg {
    #[inline(always)]
    pub fn lcdc_jdi_xrst_width(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        LcdcJdiXrstWidthReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            LcdcJdiXrstWidthReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for LcdcJdiXrstWidthReg {
    #[inline(always)]
    fn default() -> LcdcJdiXrstWidthReg {
        <crate::RegValueT<LcdcJdiXrstWidthReg_SPEC> as RegisterValue<_>>::new(0)
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
    pub fn lcdc_l0_fb_addr(
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
    pub fn lcdc_l0_en(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, LcdcLayer0ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,LcdcLayer0ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lcdc_l0_colour_mode(
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
pub struct LcdcLayer0OffsetxReg_SPEC;
impl crate::sealed::RegSpec for LcdcLayer0OffsetxReg_SPEC {
    type DataType = u32;
}

pub type LcdcLayer0OffsetxReg = crate::RegValueT<LcdcLayer0OffsetxReg_SPEC>;

impl LcdcLayer0OffsetxReg {
    #[inline(always)]
    pub fn lcdc_l0_dma_prefetch(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        u16,
        u16,
        LcdcLayer0OffsetxReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            LcdcLayer0OffsetxReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn lcdc_l0_offsetx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        LcdcLayer0OffsetxReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            LcdcLayer0OffsetxReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for LcdcLayer0OffsetxReg {
    #[inline(always)]
    fn default() -> LcdcLayer0OffsetxReg {
        <crate::RegValueT<LcdcLayer0OffsetxReg_SPEC> as RegisterValue<_>>::new(0)
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
    pub fn lcdc_l0_res_x(
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
    pub fn lcdc_l0_res_y(
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
    pub fn lcdc_l0_size_x(
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
    pub fn lcdc_l0_size_y(
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
    pub fn lcdc_l0_start_x(
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
    pub fn lcdc_l0_start_y(
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
    pub fn lcdc_l0_fifo_thr(
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
    pub fn lcdc_l0_burst_len(
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
    pub fn lcdc_l0_stride(
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
pub struct LcdcModeReg_SPEC;
impl crate::sealed::RegSpec for LcdcModeReg_SPEC {
    type DataType = u32;
}

pub type LcdcModeReg = crate::RegValueT<LcdcModeReg_SPEC>;

impl LcdcModeReg {
    #[inline(always)]
    pub fn lcdc_mode_en(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, LcdcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,LcdcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lcdc_vsync_pol(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, LcdcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28,1,0,LcdcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lcdc_hsync_pol(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, LcdcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27,1,0,LcdcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lcdc_de_pol(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, LcdcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26,1,0,LcdcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lcdc_vsync_scpl(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, LcdcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23,1,0,LcdcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lcdc_pixclkout_pol(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, LcdcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22,1,0,LcdcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lcdc_force_blank(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, LcdcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19,1,0,LcdcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lcdc_sframe_upd(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, LcdcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17,1,0,LcdcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lcdc_pixclkout_sel(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, LcdcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,LcdcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lcdc_out_mode(
        self,
    ) -> crate::common::RegisterField<5, 0xf, 1, 0, u8, u8, LcdcModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0xf,1,0,u8,u8,LcdcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lcdc_mipi_off(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, LcdcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,LcdcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lcdc_form_off(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, LcdcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,LcdcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lcdc_dscan(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, LcdcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,LcdcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lcdc_tmode(
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
pub struct LcdcResxyReg_SPEC;
impl crate::sealed::RegSpec for LcdcResxyReg_SPEC {
    type DataType = u32;
}

pub type LcdcResxyReg = crate::RegValueT<LcdcResxyReg_SPEC>;

impl LcdcResxyReg {
    #[inline(always)]
    pub fn lcdc_res_x(
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
    pub fn lcdc_res_y(
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
pub struct LcdcStatusReg_SPEC;
impl crate::sealed::RegSpec for LcdcStatusReg_SPEC {
    type DataType = u32;
}

pub type LcdcStatusReg = crate::RegValueT<LcdcStatusReg_SPEC>;

impl LcdcStatusReg {
    #[inline(always)]
    pub fn lcdc_jdi_tim_sw_rst(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, LcdcStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15,1,0,LcdcStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lcdc_frame_start(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, LcdcStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14,1,0,LcdcStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lcdc_frame_end(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, LcdcStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13,1,0,LcdcStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lcdc_dbib_cmd_pending(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, LcdcStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12,1,0,LcdcStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lcdc_dbib_cmd_fifo_full(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, LcdcStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11,1,0,LcdcStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lcdc_dbib_cmd_fifo_empty_n(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, LcdcStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,LcdcStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lcdc_dbib_te(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, LcdcStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,LcdcStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lcdc_sticky_underflow(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, LcdcStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,LcdcStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lcdc_underflow(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, LcdcStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,LcdcStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lcdc_last_row(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, LcdcStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,LcdcStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lcdc_stat_csync(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, LcdcStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,LcdcStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lcdc_stat_vsync(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, LcdcStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,LcdcStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lcdc_stat_hsync(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, LcdcStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,LcdcStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lcdc_framegen_busy(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, LcdcStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,LcdcStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn lcdc_stat_active(
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
