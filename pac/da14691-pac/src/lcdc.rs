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
// Generated from SVD 1.2, with svd2pac 0.4.0 on Sat, 12 Apr 2025 22:53:38 +0000

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
    #[doc = "Back Porch X and Y"]
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

    #[doc = "Background Color"]
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

    #[doc = "Blanking X and Y"]
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

    #[doc = "Clock Divider"]
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

    #[doc = "CRC check"]
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

    #[doc = "MIPI Config Register"]
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

    #[doc = "MIPI DBIB Command Register"]
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

    #[doc = "Front Porch X and Y"]
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

    #[doc = "General Purpose IO (2-bits)"]
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

    #[doc = "Identification Register"]
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

    #[doc = "Interrupt Register"]
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

    #[doc = "ENB end horizontal line"]
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

    #[doc = "ENB start delay"]
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

    #[doc = "ENB start horizontal line"]
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

    #[doc = "ENB width"]
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

    #[doc = "Horizontal front/back blanking (hck half periods)"]
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

    #[doc = "Vertical front/back blanking (vck half periods)"]
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

    #[doc = "HCK high/low width"]
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

    #[doc = "VCK-to-HST delay"]
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

    #[doc = "HST width"]
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

    #[doc = "Resolution XY for the JDI parallel I/F"]
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

    #[doc = "XRST-to-VCK delay"]
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

    #[doc = "XRST-to-VST delay"]
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

    #[doc = "VST width"]
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

    #[doc = "XRST width"]
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

    #[doc = "Layer0 Base Addr"]
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

    #[doc = "Layer0 Mode"]
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

    #[doc = "Layer0 OffsetX and DMA prefetch"]
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

    #[doc = "Layer0 Res XY"]
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

    #[doc = "Layer0 Size XY"]
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

    #[doc = "Layer0 Start XY"]
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

    #[doc = "Layer0 Stride"]
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

    #[doc = "Display Mode"]
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

    #[doc = "Resolution X,Y"]
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

    #[doc = "Status Register"]
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
#[doc = "Back Porch X and Y"]
pub type LcdcBackporchxyReg = crate::RegValueT<LcdcBackporchxyReg_SPEC>;

impl LcdcBackporchxyReg {
    #[doc = "Back porch X (lines)"]
    #[inline(always)]
    pub fn lcdc_bporch_x(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
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
            LcdcBackporchxyReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Back porch Y (pixel clocks)"]
    #[inline(always)]
    pub fn lcdc_bporch_y(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
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
#[doc = "Background Color"]
pub type LcdcBgcolorReg = crate::RegValueT<LcdcBgcolorReg_SPEC>;

impl LcdcBgcolorReg {
    #[doc = "Red color used as background."]
    #[inline(always)]
    pub fn lcdc_bg_red(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, LcdcBgcolorReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8, LcdcBgcolorReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Green color used as background."]
    #[inline(always)]
    pub fn lcdc_bg_green(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, LcdcBgcolorReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8, LcdcBgcolorReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Blue color used as background."]
    #[inline(always)]
    pub fn lcdc_bg_blue(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, LcdcBgcolorReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8, LcdcBgcolorReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Alpha color used as background."]
    #[inline(always)]
    pub fn lcdc_bg_alpha(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, LcdcBgcolorReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, LcdcBgcolorReg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Blanking X and Y"]
pub type LcdcBlankingxyReg = crate::RegValueT<LcdcBlankingxyReg_SPEC>;

impl LcdcBlankingxyReg {
    #[doc = "Blanking period X (VSYNC lines)"]
    #[inline(always)]
    pub fn lcdc_blanking_x(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
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
            LcdcBlankingxyReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Blanking period Y (HSYNC pulse length)"]
    #[inline(always)]
    pub fn lcdc_blanking_y(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, LcdcBlankingxyReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
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
#[doc = "Clock Divider"]
pub type LcdcClkctrlReg = crate::RegValueT<LcdcClkctrlReg_SPEC>;

impl LcdcClkctrlReg {
    #[doc = "Secondary clock divider that generates the format pipeline clock. Source clock of this divider is the main clock of LCD controller. The period of the generated clock is defined as : (LCDC_SEC_CLK_DIV + 1) x period_of_main_clock."]
    #[inline(always)]
    pub fn lcdc_sec_clk_div(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, LcdcClkctrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<27,0x1f,1,0,u8, LcdcClkctrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Hold time before DMA activated."]
    #[inline(always)]
    pub fn lcdc_dma_hold(
        self,
    ) -> crate::common::RegisterField<8, 0x3f, 1, 0, u8, LcdcClkctrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3f,1,0,u8, LcdcClkctrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clock divider that generates the pixel pipeline clock. Source clock of this divider is the format pipeline clock (see also LCDC_SEC_CLK_DIV). The period of the generated clock is defines as : LCDC_CLK_DIV x period_of_format_clk. A zero value gives division by one."]
    #[inline(always)]
    pub fn lcdc_clk_div(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, LcdcClkctrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8, LcdcClkctrlReg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "CRC check"]
pub type LcdcCrcReg = crate::RegValueT<LcdcCrcReg_SPEC>;

impl LcdcCrcReg {
    #[doc = "CRC check."]
    #[inline(always)]
    pub fn lcdc_crc(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, LcdcCrcReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, LcdcCrcReg_SPEC,crate::common::R>::from_register(self,0)
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
#[doc = "MIPI Config Register"]
pub type LcdcDbibCfgReg = crate::RegValueT<LcdcDbibCfgReg_SPEC>;

impl LcdcDbibCfgReg {
    #[doc = "Disable the sampling of the tearing effect signal, which is provided by the LCD device.\n0: the tearing effect signal is sampled\n1: the tearing effect signal is not sampled."]
    #[inline(always)]
    pub fn lcdc_dbib_te_dis(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Forces the DBIB CSX value. When is enabled the DBIB CSX takes the value of the LCDC_DBIB_CSX_FORCE_VAL.\n0 : disable\n1 : enable"]
    #[inline(always)]
    pub fn lcdc_dbib_csx_force(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Value of DBIB CSX to be forced, if bit 30 is set. Defines also the active level of the DBIB CSX even if the bit 30 is not set."]
    #[inline(always)]
    pub fn lcdc_dbib_csx_force_val(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Data padding :\n0 : disable\n1 : enable"]
    #[inline(always)]
    pub fn lcdc_dbib_spi_pad(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "DBIB RESX, reset signal for MIPI DBIB display."]
    #[inline(always)]
    pub fn lcdc_dbib_resx(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Send pixels from DMA to DBIB display.\n0 : disable\n1 : enable"]
    #[inline(always)]
    pub fn lcdc_dbib_dma_en(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enable SPI3 interface.\n0 : disable\n1 : enable"]
    #[inline(always)]
    pub fn lcdc_dbib_spi3_en(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enable SPI4 interface.\n0 : disable\n1 : enable"]
    #[inline(always)]
    pub fn lcdc_dbib_spi4_en(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Sets the data phase for the SPI interface"]
    #[inline(always)]
    pub fn lcdc_dbib_spi_cpha(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Sets the polarity of the clock (SCL)"]
    #[inline(always)]
    pub fn lcdc_dbib_spi_cpol(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enables the line addressing between the horizontal lines (JDI SPI output format).\n0 : disable\n1 : enable"]
    #[inline(always)]
    pub fn lcdc_dbib_spi_jdi(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enables the command HOLD mode of operation. Commands and data transmissions binding.\n0 : disable\n1 : enable"]
    #[inline(always)]
    pub fn lcdc_dbib_spi_hold(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enables horizontal line address inversion.\n0 : disable\n1 : enable"]
    #[inline(always)]
    pub fn lcdc_dbib_spi_inv_addr(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Data inversion\n0 : disable\n1 : enable"]
    #[inline(always)]
    pub fn lcdc_dbib_inv_data(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "MSB-LSB bit selection for JDI parallel interface\n0 : disable (MSB - LSB)\n1 : enable (LSB -MSB)"]
    #[inline(always)]
    pub fn lcdc_dbib_jdi_inv_pix(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "JDI timing generation soft reset.\n0 : disable\n1 : enable"]
    #[inline(always)]
    pub fn lcdc_dbib_jdi_soft_rst(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Defines the output format and depends of the type of the output interface. For the SPI3/SPI4 are supported the following formats:\n0x06 : RGB111-1 {2b00, R(n), G(n), B(n), R(n+1), G(n+1), B(n+1)}\n0x07 : RGB111-2 {R(n), G(n), B(n), 1b0, R(n+1), G(n+1), B(n+1), 1b0}\n0x08 : RGB111-3 {R(n), G(n), B(n), R(n+1), G(n+1), B(n+1), R(n+2), G(n+2), B(n+2),... }\n0x09 : RGB111-4 {D(n), D(n+1), D(n+2),...}\n0x10 : RGB332\n0x11 : RGB444\n0x12 : RGB565\n0x13 : RGB666\n0x14 : RGB888\n\nFor the JDI parallel interface should be used the format :\n0x0A : RGB222"]
    #[inline(always)]
    pub fn lcdc_dbib_fmt(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, LcdcDbibCfgReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8, LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "MIPI DBIB Command Register"]
pub type LcdcDbibCmdReg = crate::RegValueT<LcdcDbibCmdReg_SPEC>;

impl LcdcDbibCmdReg {
    #[doc = "Send command to the DBI interface"]
    #[inline(always)]
    pub fn lcdc_dbib_cmd_send(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, LcdcDbibCmdReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30,1,0,LcdcDbibCmdReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "This bit has meaning only when LCDC_DBIB_CFG_REG\\[LCDC_DBIB_SPI_JDI\\] = 1. When is enabled, stores the LCDC_DBIB_CMD_VAL to the register that keeps the Y position."]
    #[inline(always)]
    pub fn lcdc_dbib_cmd_store(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, LcdcDbibCmdReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27,1,0,LcdcDbibCmdReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Data to send to the DBI interface"]
    #[inline(always)]
    pub fn lcdc_dbib_cmd_val(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, LcdcDbibCmdReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16, LcdcDbibCmdReg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Front Porch X and Y"]
pub type LcdcFrontporchxyReg = crate::RegValueT<LcdcFrontporchxyReg_SPEC>;

impl LcdcFrontporchxyReg {
    #[doc = "Front porch X (lines)"]
    #[inline(always)]
    pub fn lcdc_fporch_x(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
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
            LcdcFrontporchxyReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Front porch Y (pixel clocks)"]
    #[inline(always)]
    pub fn lcdc_fporch_y(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
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
#[doc = "General Purpose IO (2-bits)"]
pub type LcdcGpioReg = crate::RegValueT<LcdcGpioReg_SPEC>;

impl LcdcGpioReg {
    #[doc = "Applies an inversion on the TE (tearing effect) signal.\n0 : the inversion is not applied on the TE signal\n1 : the inversion is applied on TE signal"]
    #[inline(always)]
    pub fn lcdc_te_inv(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, LcdcGpioReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,LcdcGpioReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Selection of the parallel interface type that is forwarded to the gpio pins.\n0 : JDI interface signals\n1 : Clasic parallel interface"]
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
#[doc = "Identification Register"]
pub type LcdcIdregReg = crate::RegValueT<LcdcIdregReg_SPEC>;

impl LcdcIdregReg {
    #[doc = "Identification register"]
    #[inline(always)]
    pub fn lcdc_id(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, LcdcIdregReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, LcdcIdregReg_SPEC,crate::common::R>::from_register(self,0)
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
#[doc = "Interrupt Register"]
pub type LcdcInterruptReg = crate::RegValueT<LcdcInterruptReg_SPEC>;

impl LcdcInterruptReg {
    #[doc = "IRQ trigger control\n0: Level triggering\n1: Edge triggering\nIn the case of the level triggering, the request remains active in the LCDC until to be cleared. The request can be cleared by performing a write access in the LCDC_INTERRUPT_REG. This is not required in the case of the edge triggering."]
    #[inline(always)]
    pub fn lcdc_irq_trigger_sel(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, LcdcInterruptReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,LcdcInterruptReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Continuous mode: frame end. Single mode: frame end or idle."]
    #[inline(always)]
    pub fn lcdc_frame_end_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, LcdcInterruptReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,LcdcInterruptReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TE interrupt enable. See also the configuration bit LCDC_DBIB_CFG_REG\\[LCDC_DBIB_TE_DIS\\]"]
    #[inline(always)]
    pub fn lcdc_te_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, LcdcInterruptReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,LcdcInterruptReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "HSYNC interrupt enabled"]
    #[inline(always)]
    pub fn lcdc_hsync_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, LcdcInterruptReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,LcdcInterruptReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VSYNC or TE interrupt enabled. See also the configuration bit LCDC_DBIB_CFG_REG\\[LCDC_DBIB_TE_DIS\\] for the TE signal."]
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
#[doc = "ENB end horizontal line"]
pub type LcdcJdiEnbEndHlineReg = crate::RegValueT<LcdcJdiEnbEndHlineReg_SPEC>;

impl LcdcJdiEnbEndHlineReg {
    #[doc = "The number of the last horizontal line where the ENB signal is asserted"]
    #[inline(always)]
    pub fn lcdc_jdi_enb_end_hline(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "ENB start delay"]
pub type LcdcJdiEnbStartClkReg = crate::RegValueT<LcdcJdiEnbStartClkReg_SPEC>;

impl LcdcJdiEnbStartClkReg {
    #[doc = "Defines the number of the HCK half periods that should take place after a transtion in the VCK and before to be enabled of the ENB."]
    #[inline(always)]
    pub fn lcdc_jdi_enb_start_clk(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "ENB start horizontal line"]
pub type LcdcJdiEnbStartHlineReg = crate::RegValueT<LcdcJdiEnbStartHlineReg_SPEC>;

impl LcdcJdiEnbStartHlineReg {
    #[doc = "The number of the first horizontal line where the ENB signal is asserted"]
    #[inline(always)]
    pub fn lcdc_jdi_enb_start_hline(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "ENB width"]
pub type LcdcJdiEnbWidthClkReg = crate::RegValueT<LcdcJdiEnbWidthClkReg_SPEC>;

impl LcdcJdiEnbWidthClkReg {
    #[doc = "ENB (high) width in HCK half periods"]
    #[inline(always)]
    pub fn lcdc_jdi_enb_width_clk(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "Horizontal front/back blanking (hck half periods)"]
pub type LcdcJdiFbxBlankingReg = crate::RegValueT<LcdcJdiFbxBlankingReg_SPEC>;

impl LcdcJdiFbxBlankingReg {
    #[doc = "Horizontal front blanking as a number of hck half periods"]
    #[inline(always)]
    pub fn lcdc_jdi_fxblanking(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
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
            LcdcJdiFbxBlankingReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Horizontal back blanking as a number of hck half periods"]
    #[inline(always)]
    pub fn lcdc_jdi_bxblanking(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
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
#[doc = "Vertical front/back blanking (vck half periods)"]
pub type LcdcJdiFbyBlankingReg = crate::RegValueT<LcdcJdiFbyBlankingReg_SPEC>;

impl LcdcJdiFbyBlankingReg {
    #[doc = "Vertical front blanking as a number of vck half periods"]
    #[inline(always)]
    pub fn lcdc_jdi_fyblanking(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
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
            LcdcJdiFbyBlankingReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Vertical back blanking as a number of vck half periods"]
    #[inline(always)]
    pub fn lcdc_jdi_byblanking(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
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
#[doc = "HCK high/low width"]
pub type LcdcJdiHckWidthReg = crate::RegValueT<LcdcJdiHckWidthReg_SPEC>;

impl LcdcJdiHckWidthReg {
    #[doc = "Number of format pipeline clock cycles that define the half period of the of the HCK (high and low width). The minimum allowed value is 2."]
    #[inline(always)]
    pub fn lcdc_jdi_hck_width(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "VCK-to-HST delay"]
pub type LcdcJdiHstDelayReg = crate::RegValueT<LcdcJdiHstDelayReg_SPEC>;

impl LcdcJdiHstDelayReg {
    #[doc = "VCK-to-HST delay in format pipeline clock cycles"]
    #[inline(always)]
    pub fn lcdc_jdi_hst_delay(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "HST width"]
pub type LcdcJdiHstWidthReg = crate::RegValueT<LcdcJdiHstWidthReg_SPEC>;

impl LcdcJdiHstWidthReg {
    #[doc = "HST width in format pipeline clock cycles"]
    #[inline(always)]
    pub fn lcdc_jdi_hst_width(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "Resolution XY for the JDI parallel I/F"]
pub type LcdcJdiResxyReg = crate::RegValueT<LcdcJdiResxyReg_SPEC>;

impl LcdcJdiResxyReg {
    #[doc = "Number of horizontal transfers. Should be equal to the half of the horizontal resolution (in pixels)."]
    #[inline(always)]
    pub fn lcdc_jdi_res_x(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, LcdcJdiResxyReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16, LcdcJdiResxyReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Number of vertical transfers. Should be equal to the double of the vertical resolution (in lines)."]
    #[inline(always)]
    pub fn lcdc_jdi_res_y(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, LcdcJdiResxyReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16, LcdcJdiResxyReg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "XRST-to-VCK delay"]
pub type LcdcJdiVckDelayReg = crate::RegValueT<LcdcJdiVckDelayReg_SPEC>;

impl LcdcJdiVckDelayReg {
    #[doc = "XRST-to-VCK delay in format pipeline clock cycles"]
    #[inline(always)]
    pub fn lcdc_jdi_vck_delay(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "XRST-to-VST delay"]
pub type LcdcJdiVstDelayReg = crate::RegValueT<LcdcJdiVstDelayReg_SPEC>;

impl LcdcJdiVstDelayReg {
    #[doc = "XRST-to-VST delay in format pipeline clock cycles"]
    #[inline(always)]
    pub fn lcdc_jdi_vst_delay(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "VST width"]
pub type LcdcJdiVstWidthReg = crate::RegValueT<LcdcJdiVstWidthReg_SPEC>;

impl LcdcJdiVstWidthReg {
    #[doc = "VST width in format pipeline clock cycles"]
    #[inline(always)]
    pub fn lcdc_jdi_vst_width(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "XRST width"]
pub type LcdcJdiXrstWidthReg = crate::RegValueT<LcdcJdiXrstWidthReg_SPEC>;

impl LcdcJdiXrstWidthReg {
    #[doc = "Number of format pipeline clock cycles of XRST width"]
    #[inline(always)]
    pub fn lcdc_jdi_xrst_width(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "Layer0 Base Addr"]
pub type LcdcLayer0BaseaddrReg = crate::RegValueT<LcdcLayer0BaseaddrReg_SPEC>;

impl LcdcLayer0BaseaddrReg {
    #[doc = "Base Address of the frame buffer"]
    #[inline(always)]
    pub fn lcdc_l0_fb_addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
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
#[doc = "Layer0 Mode"]
pub type LcdcLayer0ModeReg = crate::RegValueT<LcdcLayer0ModeReg_SPEC>;

impl LcdcLayer0ModeReg {
    #[doc = "Enable layer.\n0 : disable\n1 : enable"]
    #[inline(always)]
    pub fn lcdc_l0_en(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, LcdcLayer0ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,LcdcLayer0ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Colour Mode:\n00001: 16-bit RGBX5551 color format,\n00010: 32-bit RGBX8888 color format,\n00100: 8-bit RGB332 color format,\n00101: 16-bit RGB565 color format,\n00110: 32-bit XRGB8888,\n00111: L8 Grayscale/Palette format,\n01000: L1 Grayscale/Palette format,\n01001: L4 Grayscale/Palette format,\n01101: ABGR8888,\n01110: BGRA8888"]
    #[inline(always)]
    pub fn lcdc_l0_colour_mode(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, LcdcLayer0ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8, LcdcLayer0ModeReg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Layer0 OffsetX and DMA prefetch"]
pub type LcdcLayer0OffsetxReg = crate::RegValueT<LcdcLayer0OffsetxReg_SPEC>;

impl LcdcLayer0OffsetxReg {
    #[doc = "DMA fifo prefetch level (range: 0-4)\n0x0 : Prefetch mechanism is disabled\n0x1 : Prefetch at least 44 bytes\n0x2 : Prefetch at least 84 bytes\n0x3 : Prefetch at least 116 bytes\n0x4 : Prefetch at least 108 bytes\nAny other value : Reserved"]
    #[inline(always)]
    pub fn lcdc_l0_dma_prefetch(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
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
            LcdcLayer0OffsetxReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Offset X (negative) of X start pixel (range \\[n-1,0\\], n : pixels /8)"]
    #[inline(always)]
    pub fn lcdc_l0_offsetx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
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
#[doc = "Layer0 Res XY"]
pub type LcdcLayer0ResxyReg = crate::RegValueT<LcdcLayer0ResxyReg_SPEC>;

impl LcdcLayer0ResxyReg {
    #[doc = "Resolution X (Resolution of layer in pixels)"]
    #[inline(always)]
    pub fn lcdc_l0_res_x(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
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
            LcdcLayer0ResxyReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Resolution Y (Resolution of layer in pixels)"]
    #[inline(always)]
    pub fn lcdc_l0_res_y(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
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
#[doc = "Layer0 Size XY"]
pub type LcdcLayer0SizexyReg = crate::RegValueT<LcdcLayer0SizexyReg_SPEC>;

impl LcdcLayer0SizexyReg {
    #[doc = "Size X (Size of layer in pixels)"]
    #[inline(always)]
    pub fn lcdc_l0_size_x(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
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
            LcdcLayer0SizexyReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Size Y (Size of layer in pixels)"]
    #[inline(always)]
    pub fn lcdc_l0_size_y(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
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
#[doc = "Layer0 Start XY"]
pub type LcdcLayer0StartxyReg = crate::RegValueT<LcdcLayer0StartxyReg_SPEC>;

impl LcdcLayer0StartxyReg {
    #[doc = "Start X (offset pixels)"]
    #[inline(always)]
    pub fn lcdc_l0_start_x(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
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
            LcdcLayer0StartxyReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Start Y (offset pixels)"]
    #[inline(always)]
    pub fn lcdc_l0_start_y(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
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
#[doc = "Layer0 Stride"]
pub type LcdcLayer0StrideReg = crate::RegValueT<LcdcLayer0StrideReg_SPEC>;

impl LcdcLayer0StrideReg {
    #[doc = "Layer dma fifo threshold burst start\n00: half fifo (default)\n01: 2 burst size\n10: 4 burst size\n11: 8 burst size"]
    #[inline(always)]
    pub fn lcdc_l0_fifo_thr(
        self,
    ) -> crate::common::RegisterField<19, 0x3, 1, 0, u8, LcdcLayer0StrideReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<19,0x3,1,0,u8, LcdcLayer0StrideReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Layer burst length\n000: 16-beats (default)\n001: 2-beats\n010: 4-beats\n011: 8-beats\n100: 16-beats"]
    #[inline(always)]
    pub fn lcdc_l0_burst_len(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, LcdcLayer0StrideReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7,1,0,u8, LcdcLayer0StrideReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Layer Stride (distance from line to line in bytes)"]
    #[inline(always)]
    pub fn lcdc_l0_stride(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
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
#[doc = "Display Mode"]
pub type LcdcModeReg = crate::RegValueT<LcdcModeReg_SPEC>;

impl LcdcModeReg {
    #[doc = "Mode register.\n0 : disable\n1 : enable"]
    #[inline(always)]
    pub fn lcdc_mode_en(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, LcdcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,LcdcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VSYNC polarity.\n0: positive\n1: negative"]
    #[inline(always)]
    pub fn lcdc_vsync_pol(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, LcdcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28,1,0,LcdcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "HSYNC polarity.\n0: positive\n1: negative"]
    #[inline(always)]
    pub fn lcdc_hsync_pol(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, LcdcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27,1,0,LcdcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "DE polarity.\n0: positive\n1: negative"]
    #[inline(always)]
    pub fn lcdc_de_pol(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, LcdcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26,1,0,LcdcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Set VSYNC for a single cycle per line.\n0: disable\n1: enable"]
    #[inline(always)]
    pub fn lcdc_vsync_scpl(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, LcdcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23,1,0,LcdcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pixel clock out polarity.\n0: positive\n1: negative"]
    #[inline(always)]
    pub fn lcdc_pixclkout_pol(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, LcdcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22,1,0,LcdcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Forces output to blank.\n0: disable\n1: enable"]
    #[inline(always)]
    pub fn lcdc_force_blank(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, LcdcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19,1,0,LcdcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Single frame update.\n0: disable\n1: enable"]
    #[inline(always)]
    pub fn lcdc_sframe_upd(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, LcdcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17,1,0,LcdcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Selects the pixel out clock for the display.\n0: based on the pixel pipeline clock\n1: based on the format pipeline clock\nSee also the LCDC_CLKCTRL_REG."]
    #[inline(always)]
    pub fn lcdc_pixclkout_sel(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, LcdcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,LcdcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Selection of the output mode\n0000: Parallel RGB\n1000: JDI MIP\nAll the other values are reserved."]
    #[inline(always)]
    pub fn lcdc_out_mode(
        self,
    ) -> crate::common::RegisterField<5, 0xf, 1, 0, u8, LcdcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0xf,1,0,u8, LcdcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "MIPI off. (SPI mode of MIPI standard)\n0: disabled\n1: enabled"]
    #[inline(always)]
    pub fn lcdc_mipi_off(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, LcdcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,LcdcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Formating off\n0: disabled\n1: enabled"]
    #[inline(always)]
    pub fn lcdc_form_off(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, LcdcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,LcdcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Double horizontal scan\n0: disabled\n1: enabled"]
    #[inline(always)]
    pub fn lcdc_dscan(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, LcdcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,LcdcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Test mode\n0: disabled\n1: enabled"]
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
#[doc = "Resolution X,Y"]
pub type LcdcResxyReg = crate::RegValueT<LcdcResxyReg_SPEC>;

impl LcdcResxyReg {
    #[doc = "Resolution X in pixels."]
    #[inline(always)]
    pub fn lcdc_res_x(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, LcdcResxyReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16, LcdcResxyReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Resolution Y in pixels."]
    #[inline(always)]
    pub fn lcdc_res_y(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, LcdcResxyReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16, LcdcResxyReg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Status Register"]
pub type LcdcStatusReg = crate::RegValueT<LcdcStatusReg_SPEC>;

impl LcdcStatusReg {
    #[doc = "JDI timing generation soft reset (active high)"]
    #[inline(always)]
    pub fn lcdc_jdi_tim_sw_rst(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, LcdcStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15,1,0,LcdcStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Frame start (active high)"]
    #[inline(always)]
    pub fn lcdc_frame_start(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, LcdcStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14,1,0,LcdcStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Frame end (active high)"]
    #[inline(always)]
    pub fn lcdc_frame_end(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, LcdcStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13,1,0,LcdcStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Transferring of command in progress.\n0: idle\n1: in progress"]
    #[inline(always)]
    pub fn lcdc_dbib_cmd_pending(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, LcdcStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12,1,0,LcdcStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Command fifo full indication.\n0: is not full\n1: is full"]
    #[inline(always)]
    pub fn lcdc_dbib_cmd_fifo_full(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, LcdcStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11,1,0,LcdcStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Command fifo empty indication (negative)\n0: the fifo is empty\n1: the fifo is not empty"]
    #[inline(always)]
    pub fn lcdc_dbib_cmd_fifo_empty_n(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, LcdcStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,LcdcStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "The DBIB tearing effect signal"]
    #[inline(always)]
    pub fn lcdc_dbib_te(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, LcdcStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,LcdcStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Sticky underflow(clear with write in the LCDC_INTERRUPT_REG)\n0: There is no underflow\n1: Underflow has been detected.Remains high until to be cleared by performing a write access on the register LCDC_INTERRUPT_REG."]
    #[inline(always)]
    pub fn lcdc_sticky_underflow(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, LcdcStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,LcdcStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Underflow on the current transfer.\n0: There is no underflow\n1: Underflow has been detected."]
    #[inline(always)]
    pub fn lcdc_underflow(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, LcdcStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,LcdcStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Last row (Last row is currently displayed)"]
    #[inline(always)]
    pub fn lcdc_last_row(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, LcdcStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,LcdcStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "CSYNC signal level"]
    #[inline(always)]
    pub fn lcdc_stat_csync(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, LcdcStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,LcdcStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "VSYNC signal level"]
    #[inline(always)]
    pub fn lcdc_stat_vsync(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, LcdcStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,LcdcStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "HSYNC signal level"]
    #[inline(always)]
    pub fn lcdc_stat_hsync(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, LcdcStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,LcdcStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "The frame generator is busy (active high)."]
    #[inline(always)]
    pub fn lcdc_framegen_busy(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, LcdcStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,LcdcStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Active (When not in vertical blanking)"]
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
