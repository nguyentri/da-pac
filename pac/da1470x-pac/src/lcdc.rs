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

    #[doc = "Controls the CLock Gaters and the routing of format and pixel clock"]
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

    #[doc = "Color mode status register"]
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

    #[doc = "Supported config"]
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

    #[doc = "Data read by DBI Type-B/SPI interface"]
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

    #[doc = "DBI and JDI format control"]
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

    #[doc = "DBI and JDI format control"]
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

    #[doc = "General Purpose IO (8-bits)"]
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

    #[doc = "Layer1 Base Addr"]
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

    #[doc = "Layer1 Mode"]
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

    #[doc = "Layer1 Res XY"]
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

    #[doc = "Layer1 Size XY"]
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

    #[doc = "Layer0 Start XY"]
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

    #[doc = "Layer1 Stride"]
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

    #[doc = "Global palette/gamma correction"]
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

    #[doc = "Global palette/gamma correction"]
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

    #[doc = "Specifies the start position of the very first frame"]
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

    #[doc = "Back porch Y (pixel clocks)"]
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

#[doc = "Background Color"]
pub type LcdcBgcolorReg = crate::RegValueT<LcdcBgcolorReg_SPEC>;

impl LcdcBgcolorReg {
    #[doc = "Red color used as background."]
    #[inline(always)]
    pub fn bg_red(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, u8, LcdcBgcolorReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8,u8,LcdcBgcolorReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Green color used as background."]
    #[inline(always)]
    pub fn bg_green(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, LcdcBgcolorReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,LcdcBgcolorReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Blue color used as background."]
    #[inline(always)]
    pub fn bg_blue(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, LcdcBgcolorReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,LcdcBgcolorReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Alpha color used as background."]
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

#[doc = "Blanking X and Y"]
pub type LcdcBlankingxyReg = crate::RegValueT<LcdcBlankingxyReg_SPEC>;

impl LcdcBlankingxyReg {
    #[doc = "Blanking period X (VSYNC lines)"]
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

    #[doc = "Blanking period Y (HSYNC pulse length)"]
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

#[doc = "Controls the CLock Gaters and the routing of format and pixel clock"]
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

#[doc = "Clock Divider"]
pub type LcdcClkctrlReg = crate::RegValueT<LcdcClkctrlReg_SPEC>;

impl LcdcClkctrlReg {
    #[doc = "Secondary clock divider that generates the format pipeline clock. Source clock of this divider is the main clock of LCD controller. The period of the generated clock is defined as : (LCDC_SEC_CLK_DIV + 1) x period_of_main_clock."]
    #[inline(always)]
    pub fn sec_clk_div(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, u8, LcdcClkctrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<27,0x1f,1,0,u8,u8,LcdcClkctrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Hold time before DMA activated."]
    #[inline(always)]
    pub fn dma_hold(
        self,
    ) -> crate::common::RegisterField<8, 0x3f, 1, 0, u8, u8, LcdcClkctrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3f,1,0,u8,u8,LcdcClkctrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Clock divider that generates the pixel pipeline clock. Source clock of this divider is the format pipeline clock (see also LCDC_SEC_CLK_DIV). The period of the generated clock is defines as : LCDC_CLK_DIV x period_of_format_clk. A zero value gives division by one."]
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

#[doc = "Color mode status register"]
pub type LcdcColmod = crate::RegValueT<LcdcColmod_SPEC>;

impl LcdcColmod {
    #[doc = "Indicates that back pressure support for the DBI Type B interface is enabled"]
    #[inline(always)]
    pub fn bp_dbib(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, LcdcColmod_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31,1,0,LcdcColmod_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "16 bit: Indicates that the LUT8 color format is enabled\n15 bit: Indicates that the RGBA5551 16-bit color format is enabled\n14 bit: Indicates that the RGBA8888 32-bit color format is enabled\n13 bit: Indicates that the RGB332 8-bit color format is enabled\n12 bit: Indicates that the RGB565 16-bit color format is enabled\n11 bit: Indicates that the ARGB8888 32-bit color format is enabled\n10 bit: Indicates that the L8 color format is enabled\n9 bit: Indicates that the L1 color format is enabled\n8 bit: Indicates that the L4 color format is enabled\n7 bit: Indicates that the YUYV color format is enabled\n6 bit: Indicates that the RGB888 24-bit color format is enabled\n5 bit: Indicates that the YUY2 color format is enabled\n4 bit: Indicates that the ABGR8888 32-bit color format is enabled\n3 bit: Indicates that the BGRA8888 32-bit color format is enabled\n2 bit: Indicates that the V_YUV420 color format is enabled\n1 bit: Indicates that the TLYUV420 color format is enabled\n0 bit: Indicates that the TSc4/TSc6 propietary color format is enabled"]
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

#[doc = "Supported config"]
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

#[doc = "CRC check"]
pub type LcdcCrcReg = crate::RegValueT<LcdcCrcReg_SPEC>;

impl LcdcCrcReg {
    #[doc = "CRC check."]
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

#[doc = "MIPI Config Register"]
pub type LcdcDbibCfgReg = crate::RegValueT<LcdcDbibCfgReg_SPEC>;

impl LcdcDbibCfgReg {
    #[doc = "When set to 1, the DBI Type-B interface is activated"]
    #[inline(always)]
    pub fn dbib_interface_en(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "When set to 1, the value of the CSX signal of the DBI Type-B interface can be configured from\nthe DBIB_CFG\\[29\\] register bit"]
    #[inline(always)]
    pub fn dbib_csx_cfg_en(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Sets the value of DBIB_CSX signal:\nCSX is set to one if DBIB_CFG\\[29\\] has the value of one\nCSX is set to zero if DBIB_CFG\\[29\\] has the value of zero"]
    #[inline(always)]
    pub fn dbib_csx_cfg(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "When set to 1, the DBIB_TE signal is disabled"]
    #[inline(always)]
    pub fn dbib_te_disable(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "When set to 1, enables the usage of SPI_DC wire as SPI_SD1"]
    #[inline(always)]
    pub fn spi_dc_as_spi_sd1(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "When set to 1, force DBI Type-B interface to idle state. Swap on the fly data for the interface"]
    #[inline(always)]
    pub fn dbib_force_idle(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Drives DBIB_RESX output signal of DBI Type-B interface"]
    #[inline(always)]
    pub fn dbib_resx_out_en(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Reverse sub pixel order"]
    #[inline(always)]
    pub fn sub_pixel_reverse(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "When set to 1, SPI 3-wire interface is enabled"]
    #[inline(always)]
    pub fn spi3_en(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "When set to 1, SPI 4-wire interface is enabled"]
    #[inline(always)]
    pub fn spi4_en(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "When set to 1, Enables back-pressure for DBI Type-B interface"]
    #[inline(always)]
    pub fn dbib_back_pressure_en(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Sets SPI Clock Phase"]
    #[inline(always)]
    pub fn spi_clk_phase(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Sets SPI Clock Polarity"]
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

    #[doc = "When set to 1, command data are used as header of each line"]
    #[inline(always)]
    pub fn cmd_data_as_header(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "When set to 1, inverts the bit-order of the horizontal line address (used along with DBIB_CFG\\[17\\]\nregister bit)"]
    #[inline(always)]
    pub fn bit_order_addr_invert(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "When set to 1, two-byte address is sent with each horizontal line (SPI)"]
    #[inline(always)]
    pub fn spi_2byte_addr(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "When set to 1, expose pixel generation clock on the DBIB_CLK"]
    #[inline(always)]
    pub fn pix_clk_at_dbib_clk(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "When set to 1, enables the external control"]
    #[inline(always)]
    pub fn ext_ctrl_en(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "When set to 1, enables the horizontal blanking"]
    #[inline(always)]
    pub fn horizontal_blank_en(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "When set to 1, Enables DualSPI sub-pixel transaction"]
    #[inline(always)]
    pub fn dual_spi_subpixel_extract_en(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "When set to 1, Enables QuadSPI"]
    #[inline(always)]
    pub fn quad_spi_en(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "When set to 1, Enables DualSPI"]
    #[inline(always)]
    pub fn dual_spi_en(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, LcdcDbibCfgReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Set DBI Type-B interface width (8, 9 or 16 bits) and the serial interface:\n0x0: 8-bit interface\n0x1: 9-bit interface\n0x2: 16-bit interface\n0x3: SPI\n0x4: Dual SPI\n0x5: Quad SPI"]
    #[inline(always)]
    pub fn dbib_interface_width(
        self,
    ) -> crate::common::RegisterField<6, 0x7, 1, 0, u8, u8, LcdcDbibCfgReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x7,1,0,u8,u8,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Set the data order of the 8-bit data word:\n0x0: option 0\n0x1: option 1\n0x2: option 2\n0x3: option 3\n0x4: option 4\n0x5: Reserved\n0x6: Reserved\n0x7: Reserved"]
    #[inline(always)]
    pub fn dbib_data_order(
        self,
    ) -> crate::common::RegisterField<3, 0x7, 1, 0, u8, u8, LcdcDbibCfgReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x7,1,0,u8,u8,LcdcDbibCfgReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Defines the output format and depends of the type of the output interface. For the SPI3/SPI4 are supported the following formats:\n0x0 : Reserved\n0x1 : RGB111\n0x2 : RGB332\n0x3 : RGB444\n0x4 : Reserved\n0x5 : RGB565\n0x6 : RGB666\n0x7 : RGB888"]
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

#[doc = "MIPI DBIB Command Register"]
pub type LcdcDbibCmdReg = crate::RegValueT<LcdcDbibCmdReg_SPEC>;

impl LcdcDbibCmdReg {
    #[doc = "When set to 0, indicates that the command data are the Base address for partial update. Applied\non SPI/JDI-SPI type"]
    #[inline(always)]
    pub fn part_update(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, LcdcDbibCmdReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,LcdcDbibCmdReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Send command to the DBI interface"]
    #[inline(always)]
    pub fn dbib_cmd_send(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, LcdcDbibCmdReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30,1,0,LcdcDbibCmdReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Determine the command width. Applicable only on the QuadSPI\n0x00: 1 Byte\n0x01: 2 Bytes\n0x02: 3 Bytes\n0x03: Reserved"]
    #[inline(always)]
    pub fn cmd_width(
        self,
    ) -> crate::common::RegisterField<28, 0x3, 1, 0, u8, u8, LcdcDbibCmdReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x3,1,0,u8,u8,LcdcDbibCmdReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "This bit has meaning only when LCDC_DBIB_CFG_REG\\[LCDC_DBIB_SPI_JDI\\] = 1. When is enabled, stores the LCDC_DBIB_CMD_VAL to the register that keeps the Y position."]
    #[inline(always)]
    pub fn dbib_cmd_store(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, LcdcDbibCmdReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27,1,0,LcdcDbibCmdReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "When sets to 1, read mode is enabled"]
    #[inline(always)]
    pub fn rd_mode_en(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, LcdcDbibCmdReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26,1,0,LcdcDbibCmdReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "When sets to 1, FMTCTRL\\[15:8\\] is exposed on DBIB_CT pins and FMTCTRL\\[31\\] on DBIB_GE\nelse FMTCTRL\\[15:8\\] is exposed on DBIB_CT pins and FMTCTRL\\[30\\] on DBIB_GE"]
    #[inline(always)]
    pub fn fmtctrl_expose_setting(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, LcdcDbibCmdReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25,1,0,LcdcDbibCmdReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "When sets to 1, store internally a command type which is transmitted at the beggining of each\nscanline"]
    #[inline(always)]
    pub fn st_int_cmd_type(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, LcdcDbibCmdReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24,1,0,LcdcDbibCmdReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Data to send to the DBI interface"]
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

#[doc = "Data read by DBI Type-B/SPI interface"]
pub type LcdcDbibRdat = crate::RegValueT<LcdcDbibRdat_SPEC>;

impl LcdcDbibRdat {
    #[doc = "On Write:\n31-30 bits: Specify the number of read cycles\n0x0: 8 cycles\n0x1: 16 cycles\n0x2: 24 cycles\n0x3: Used along with FMTCTRL\\[20:16\\] register bits\n29-0 bits: Reserved\n\nOn Read:\n31-0 bits: Read data from DBI Type-B/SPI interfaces"]
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

#[doc = "DBI and JDI format control"]
pub type LcdcFmtctrl2Reg = crate::RegValueT<LcdcFmtctrl2Reg_SPEC>;

impl LcdcFmtctrl2Reg {
    #[doc = "Bits Description\nWhen DBI-Type B Interface is selected:\nBits Description\n31-16 bits: Reserved\n15-0 bits: Specify the blanking period length for the X dimension\nWhen JDI-Parallel Interface is selected:\nBits Description\n31-20 bits: Reserved\n19-10 bits: JDI ENB width\n9-0 bits: JDI ENB offset"]
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

#[doc = "DBI and JDI format control"]
pub type LcdcFmtctrlReg = crate::RegValueT<LcdcFmtctrlReg_SPEC>;

impl LcdcFmtctrlReg {
    #[doc = "When DBI-Type B Interface is selected:\nBits Description\n31 bit : Candidate for DBIB_GE, associated with \\[15:8\\] bits (DSI interface specific)\n30 bit : Candidate for DBIB_GE, associated with \\[7:0\\] bits (DSI interface specific)\n29-21 bits: Reserved\n20-16 bits: Specify the number of read cycles for SPI/DBI Type-B. Used along with DBIB_CFG\\[31:30\\]\nregister bits\n0x0: 1 cycles\n0x1: 2 cycles\n0x2: 3 cycles\n.\n.\n0x30: 31 cycles\n0x31: 32 cycles\n15-8 bits : Candidate DBIB_CT (DSI interface specific)\n7-0 bits : Candidate DBIB_CT (DSI interface specific)\nWhen JDI-Parallel Interface is selected:\nBits Description\n31 bit : Mute DPI outputs\n30 bit : Mask DPIREADY input\n29 bit : Reserved\n28-26 bits : JDI HST width\n25-23 bits : JDI HST offset\n22-13 bits : JDI VST width\n12-3 bits : JDI VST offset\n2-0 bits : Reserved"]
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

#[doc = "Front Porch X and Y"]
pub type LcdcFrontporchxyReg = crate::RegValueT<LcdcFrontporchxyReg_SPEC>;

impl LcdcFrontporchxyReg {
    #[doc = "Front porch X (lines)"]
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

    #[doc = "Front porch Y (pixel clocks)"]
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

#[doc = "General Purpose IO (8-bits)"]
pub type LcdcGpioReg = crate::RegValueT<LcdcGpioReg_SPEC>;

impl LcdcGpioReg {
    #[doc = "Assert DPI-2 Color Mode signal"]
    #[inline(always)]
    pub fn dpi_cm_assert(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, LcdcGpioReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<16,1,0,LcdcGpioReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Assert DPI-2 Shutdown signal"]
    #[inline(always)]
    pub fn dpi_sd_assert(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, LcdcGpioReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<15,1,0,LcdcGpioReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "scalar advance"]
    #[inline(always)]
    pub fn scal_advance(
        self,
    ) -> crate::common::RegisterField<13, 0x3, 1, 0, u8, u8, LcdcGpioReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x3,1,0,u8,u8,LcdcGpioReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "TBD"]
    #[inline(always)]
    pub fn lcdc_other(
        self,
    ) -> crate::common::RegisterField<6, 0x7f, 1, 0, u8, u8, LcdcGpioReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x7f,1,0,u8,u8,LcdcGpioReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enable to have the SPI SI on the SPI SD pad"]
    #[inline(always)]
    pub fn gpio_spi_si_on_sd_pad(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, LcdcGpioReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,LcdcGpioReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Select the mode that should be mapped on the GPIO pins\n0x0 = JDI\n0x1 = DPI\n0x2 = DBI\n0x3 = SPI"]
    #[inline(always)]
    pub fn gpio_output_mode(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, u8, LcdcGpioReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x3,1,0,u8,u8,LcdcGpioReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Enable the GPIO pins for LCDC control. The GPIO_OUTPUT_MODE is used to define what LCDC pins will be mapped towards the GPIO pins."]
    #[inline(always)]
    pub fn gpio_output_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, LcdcGpioReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,LcdcGpioReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Applies an inversion on the TE (tearing effect) signal.\n0 : the inversion is not applied on the TE signal\n1 : the inversion is applied on TE signal"]
    #[inline(always)]
    pub fn te_inv(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, LcdcGpioReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,LcdcGpioReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Divide clock by 2/3 clock-gating. This is required when the DSI-DPHY is used with 2 lanes active."]
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

#[doc = "Identification Register"]
pub type LcdcIdregReg = crate::RegValueT<LcdcIdregReg_SPEC>;

impl LcdcIdregReg {
    #[doc = "Identification register"]
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

#[doc = "Interrupt Register"]
pub type LcdcInterruptReg = crate::RegValueT<LcdcInterruptReg_SPEC>;

impl LcdcInterruptReg {
    #[doc = "IRQ trigger control\n0: Level triggering\n1: Edge triggering\nIn the case of the level triggering, the request remains active in the LCDC until to be cleared. The request can be cleared by performing a write access in the LCDC_INTERRUPT_REG. This is not required in the case of the edge triggering."]
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

    #[doc = "HSYNC interrupt enabled"]
    #[inline(always)]
    pub fn hsync_irq_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, LcdcInterruptReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,LcdcInterruptReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "VSYNC or TE interrupt enabled. See also the configuration bit LCDC_DBIB_CFG_REG\\[LCDC_DBIB_TE_DIS\\] for the TE signal."]
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

#[doc = "Layer0 Base Addr"]
pub type LcdcLayer0BaseaddrReg = crate::RegValueT<LcdcLayer0BaseaddrReg_SPEC>;

impl LcdcLayer0BaseaddrReg {
    #[doc = "Base Address of the frame buffer"]
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

#[doc = "Layer0 Mode"]
pub type LcdcLayer0ModeReg = crate::RegValueT<LcdcLayer0ModeReg_SPEC>;

impl LcdcLayer0ModeReg {
    #[doc = "Enable layer.\n0 : disable\n1 : enable"]
    #[inline(always)]
    pub fn l0_en(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, LcdcLayer0ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,LcdcLayer0ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "When set to 1, force alpha with global alpha"]
    #[inline(always)]
    pub fn l0_force_alpha(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, LcdcLayer0ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30,1,0,LcdcLayer0ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "When set to 0, bilinear filtering is enabled. Scaler should be enabled"]
    #[inline(always)]
    pub fn l0_dis_bil_filtering(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, LcdcLayer0ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29,1,0,LcdcLayer0ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "When set to 1, premultiply image alpha is enabled"]
    #[inline(always)]
    pub fn l0_premul_img_alpha(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, LcdcLayer0ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28,1,0,LcdcLayer0ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "When set to 1, HLOCK signal on AHB DMAs is asserted"]
    #[inline(always)]
    pub fn l0_assert_hclk_dma(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, LcdcLayer0ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27,1,0,LcdcLayer0ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "When set to 1, Gamma Look Up Table is enabled"]
    #[inline(always)]
    pub fn l0_gamma_lut_en(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, LcdcLayer0ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26,1,0,LcdcLayer0ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Alpha layer global value (0x00-0xFF range)"]
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

    #[doc = "Destinary Blending Function\n0000: BLEND ZERO\n0001: BLEND ONE\n0010: BLEND ALPHA SRC\n0011: BLEND ALPHA GBL\n0100: BLEND ALPHA SRCGBL\n0101: BLEND INV SRC\n0110: BLEND INV GBL\n0111: BLEND INV SRCGBL\n1010: BLEND ALPHA DST\n1101: BLEND INV DST"]
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

    #[doc = "Source Blending Function\n0000: BLEND ZERO\n0001: BLEND ONE\n0010: BLEND ALPHA SRC\n0011: BLEND ALPHA GBL\n0100: BLEND ALPHA SRCGBL\n0101: BLEND INV SRC\n0110: BLEND INV GBL\n0111: BLEND INV SRCGBL\n1010: BLEND ALPHA DST\n1101: BLEND INV DST"]
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

    #[doc = "Color Mode:\n00001: 16-bit RGBX5551 color format,\n00010: 32-bit RGBX8888 color format,\n00100: 8-bit RGB332 color format,\n00101: 16-bit RGB565 color format,\n00110: 32-bit XRGB8888,\n00111: L8 Grayscale/Palette format,\n01000: L1 Grayscale/Palette format,\n01001: L4 Grayscale/Palette format,\n01101: ABGR8888,\n01110: BGRA8888"]
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

#[doc = "Layer0 Res XY"]
pub type LcdcLayer0ResxyReg = crate::RegValueT<LcdcLayer0ResxyReg_SPEC>;

impl LcdcLayer0ResxyReg {
    #[doc = "Resolution X (Resolution of layer in pixels)"]
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

    #[doc = "Resolution Y (Resolution of layer in pixels)"]
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

#[doc = "Layer0 Size XY"]
pub type LcdcLayer0SizexyReg = crate::RegValueT<LcdcLayer0SizexyReg_SPEC>;

impl LcdcLayer0SizexyReg {
    #[doc = "Size X (Size of layer in pixels)"]
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

    #[doc = "Size Y (Size of layer in pixels)"]
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

#[doc = "Layer0 Start XY"]
pub type LcdcLayer0StartxyReg = crate::RegValueT<LcdcLayer0StartxyReg_SPEC>;

impl LcdcLayer0StartxyReg {
    #[doc = "Start X (offset pixels)"]
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

    #[doc = "Start Y (offset pixels)"]
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

#[doc = "Layer0 Stride"]
pub type LcdcLayer0StrideReg = crate::RegValueT<LcdcLayer0StrideReg_SPEC>;

impl LcdcLayer0StrideReg {
    #[doc = "Layer dma fifo threshold burst start\n00: half fifo (default)\n01: 2 burst size\n10: 4 burst size\n11: 8 burst size"]
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

    #[doc = "Layer burst length\n000: 16-beats (default)\n001: 2-beats\n010: 4-beats\n011: 8-beats\n100: 16-beats"]
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

    #[doc = "Layer Stride (distance from line to line in bytes)"]
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

#[doc = "Layer1 Base Addr"]
pub type LcdcLayer1BaseaddrReg = crate::RegValueT<LcdcLayer1BaseaddrReg_SPEC>;

impl LcdcLayer1BaseaddrReg {
    #[doc = "Base Address of the frame buffer"]
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

#[doc = "Layer1 Mode"]
pub type LcdcLayer1ModeReg = crate::RegValueT<LcdcLayer1ModeReg_SPEC>;

impl LcdcLayer1ModeReg {
    #[doc = "Enable layer.\n0 : disable\n1 : enable"]
    #[inline(always)]
    pub fn l1_en(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, LcdcLayer1ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,LcdcLayer1ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "When set to 1, force alpha with global alpha"]
    #[inline(always)]
    pub fn l1_force_alpha(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, LcdcLayer1ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30,1,0,LcdcLayer1ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "When set to 0, bilinear filtering is enabled. Scaler should be enabled"]
    #[inline(always)]
    pub fn l1_dis_bil_filtering(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, LcdcLayer1ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29,1,0,LcdcLayer1ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "When set to 1, premultiply image alpha is enabled"]
    #[inline(always)]
    pub fn l1_premul_img_alpha(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, LcdcLayer1ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28,1,0,LcdcLayer1ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "When set to 1, HLOCK signal on AHB DMAs is asserted"]
    #[inline(always)]
    pub fn l1_assert_hclk_dma(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, LcdcLayer1ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27,1,0,LcdcLayer1ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "When set to 1, Gamma Look Up Table is enabled"]
    #[inline(always)]
    pub fn l1_gamma_lut_en(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, LcdcLayer1ModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26,1,0,LcdcLayer1ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Alpha layer global value (0x00-0xFF range)"]
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

    #[doc = "Destinary Blending Function\n0000: BLEND ZERO\n0001: BLEND ONE\n0010: BLEND ALPHA SRC\n0011: BLEND ALPHA GBL\n0100: BLEND ALPHA SRCGBL\n0101: BLEND INV SRC\n0110: BLEND INV GBL\n0111: BLEND INV SRCGBL\n1010: BLEND ALPHA DST\n1101: BLEND INV DST"]
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

    #[doc = "Source Blending Function\n0000: BLEND ZERO\n0001: BLEND ONE\n0010: BLEND ALPHA SRC\n0011: BLEND ALPHA GBL\n0100: BLEND ALPHA SRCGBL\n0101: BLEND INV SRC\n0110: BLEND INV GBL\n0111: BLEND INV SRCGBL\n1010: BLEND ALPHA DST\n1101: BLEND INV DST"]
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

    #[doc = "Color Mode:\n00001: 16-bit RGBX5551 color format,\n00010: 32-bit RGBX8888 color format,\n00100: 8-bit RGB332 color format,\n00101: 16-bit RGB565 color format,\n00110: 32-bit XRGB8888,\n00111: L8 Grayscale/Palette format,\n01000: L1 Grayscale/Palette format,\n01001: L4 Grayscale/Palette format,\n01101: ABGR8888,\n01110: BGRA8888"]
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

#[doc = "Layer1 Res XY"]
pub type LcdcLayer1ResxyReg = crate::RegValueT<LcdcLayer1ResxyReg_SPEC>;

impl LcdcLayer1ResxyReg {
    #[doc = "Resolution X (Resolution of layer in pixels)"]
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

    #[doc = "Resolution Y (Resolution of layer in pixels)"]
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

#[doc = "Layer1 Size XY"]
pub type LcdcLayer1SizexyReg = crate::RegValueT<LcdcLayer1SizexyReg_SPEC>;

impl LcdcLayer1SizexyReg {
    #[doc = "Size X (Size of layer in pixels)"]
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

    #[doc = "Size Y (Size of layer in pixels)"]
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

#[doc = "Layer0 Start XY"]
pub type LcdcLayer1StartxyReg = crate::RegValueT<LcdcLayer1StartxyReg_SPEC>;

impl LcdcLayer1StartxyReg {
    #[doc = "Start X (offset pixels)"]
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

    #[doc = "Start Y (offset pixels)"]
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

#[doc = "Layer1 Stride"]
pub type LcdcLayer1StrideReg = crate::RegValueT<LcdcLayer1StrideReg_SPEC>;

impl LcdcLayer1StrideReg {
    #[doc = "Layer dma fifo threshold burst start\n00: half fifo (default)\n01: 2 burst size\n10: 4 burst size\n11: 8 burst size"]
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

    #[doc = "Layer burst length\n000: 16-beats (default)\n001: 2-beats\n010: 4-beats\n011: 8-beats\n100: 16-beats"]
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

    #[doc = "Layer Stride (distance from line to line in bytes)"]
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

#[doc = "Display Mode"]
pub type LcdcModeReg = crate::RegValueT<LcdcModeReg_SPEC>;

impl LcdcModeReg {
    #[doc = "Mode register.\n0 : disable\n1 : enable"]
    #[inline(always)]
    pub fn mode_en(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, LcdcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,LcdcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "VSYNC polarity.\n0: positive\n1: negative"]
    #[inline(always)]
    pub fn vsync_pol(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, LcdcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28,1,0,LcdcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "HSYNC polarity.\n0: positive\n1: negative"]
    #[inline(always)]
    pub fn hsync_pol(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, LcdcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27,1,0,LcdcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "DE polarity.\n0: positive\n1: negative"]
    #[inline(always)]
    pub fn de_pol(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, LcdcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26,1,0,LcdcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0x00: Dithering is disabled\n0x01: Dithering 18-bits mode\n0x02: Dithering 16-bits mode\n0x03: Dithering 15-bits mode"]
    #[inline(always)]
    pub fn dith_mode(
        self,
    ) -> crate::common::RegisterField<24, 0x3, 1, 0, u8, u8, LcdcModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x3,1,0,u8,u8,LcdcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Set VSYNC for a single cycle per line.\n0: disable\n1: enable"]
    #[inline(always)]
    pub fn vsync_scpl(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, LcdcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23,1,0,LcdcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Pixel clock out polarity.\n0: positive\n1: negative"]
    #[inline(always)]
    pub fn pixclkout_pol(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, LcdcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22,1,0,LcdcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "When set to 1, global gamma correction is enabled"]
    #[inline(always)]
    pub fn global_gamma_en(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, LcdcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20,1,0,LcdcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Forces output to blank.\n0: disable\n1: enable"]
    #[inline(always)]
    pub fn force_blank(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, LcdcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19,1,0,LcdcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Single frame update.\n0: disable\n1: enable"]
    #[inline(always)]
    pub fn sframe_upd(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, LcdcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17,1,0,LcdcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Defines MIPI DPI-2 Configuration\n0x00: RGB888 24-bits\n0x01: RGB666 18-bits Configuration 1\n0x02: RGB666 18-bits Configuration 2\n0x03: RGB565 16-bits Configuration 1\n0x04: RGB565 16-bits Configuration 2\n0x05: RGB565 16-bits Configuration 3\n0x06: Reserved\n0x07: Reserved"]
    #[inline(always)]
    pub fn dpi2_config(
        self,
    ) -> crate::common::RegisterField<12, 0x7, 1, 0, u8, u8, LcdcModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x7,1,0,u8,u8,LcdcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Selects the pixel out clock for the display.\n0: based on the pixel pipeline clock\n1: based on the format pipeline clock\nSee also the LCDC_CLKCTRL_REG."]
    #[inline(always)]
    pub fn pixclkout_sel(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, LcdcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,LcdcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Selection of the output mode\n0000: Parallel RGB\n1000: JDI MIP\nAll the other values are reserved."]
    #[inline(always)]
    pub fn out_mode(
        self,
    ) -> crate::common::RegisterField<5, 0xf, 1, 0, u8, u8, LcdcModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0xf,1,0,u8,u8,LcdcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "When set to 0, DBI Type-B interface is enabled"]
    #[inline(always)]
    pub fn dbib_off(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, LcdcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,LcdcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Formating off\n0: disabled\n1: enabled"]
    #[inline(always)]
    pub fn form_off(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, LcdcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,LcdcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Double horizontal scan\n0: disabled\n1: enabled"]
    #[inline(always)]
    pub fn dscan(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, LcdcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,LcdcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Test mode\n0: disabled\n1: enabled"]
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

#[doc = "Global palette/gamma correction"]
pub type LcdcPalette255 = crate::RegValueT<LcdcPalette255_SPEC>;

impl LcdcPalette255 {
    #[doc = "Gamma ramp red bits"]
    #[inline(always)]
    pub fn pallete_r(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, LcdcPalette255_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,LcdcPalette255_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Gamma ramp green bits"]
    #[inline(always)]
    pub fn pallete_g(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, LcdcPalette255_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,LcdcPalette255_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Gamma ramp blue bits"]
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

#[doc = "Global palette/gamma correction"]
pub type LcdcPaletteBase = crate::RegValueT<LcdcPaletteBase_SPEC>;

impl LcdcPaletteBase {
    #[doc = "Gamma ramp red bits"]
    #[inline(always)]
    pub fn pallete_r(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, u8, LcdcPaletteBase_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8,u8,LcdcPaletteBase_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Gamma ramp green bits"]
    #[inline(always)]
    pub fn pallete_g(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, LcdcPaletteBase_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,LcdcPaletteBase_SPEC,crate::common::W>::from_register(self,0)
    }

    #[doc = "Gamma ramp blue bits"]
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

#[doc = "Resolution X,Y"]
pub type LcdcResxyReg = crate::RegValueT<LcdcResxyReg_SPEC>;

impl LcdcResxyReg {
    #[doc = "Resolution X in pixels."]
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

    #[doc = "Resolution Y in pixels."]
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

#[doc = "Specifies the start position of the very first frame"]
pub type LcdcStartxyReg = crate::RegValueT<LcdcStartxyReg_SPEC>;

impl LcdcStartxyReg {
    #[doc = "Specify framess X dimension"]
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

    #[doc = "Specify framess Y dimension"]
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

#[doc = "Status Register"]
pub type LcdcStatusReg = crate::RegValueT<LcdcStatusReg_SPEC>;

impl LcdcStatusReg {
    #[doc = "Indicates if the command FIFO is full"]
    #[inline(always)]
    pub fn dbib_cmd_fifo_full(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, LcdcStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15,1,0,LcdcStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Indicates DBI/SPI CS status"]
    #[inline(always)]
    pub fn dbi_spi_cs(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, LcdcStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14,1,0,LcdcStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Frame end (active high)"]
    #[inline(always)]
    pub fn frame_end(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, LcdcStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13,1,0,LcdcStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Pending output transaction in DBI Type-B interface"]
    #[inline(always)]
    pub fn dbib_out_trans_pending(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, LcdcStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12,1,0,LcdcStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Transferring of command in progress.\n0: idle\n1: in progress"]
    #[inline(always)]
    pub fn dbib_cmd_pending(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, LcdcStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11,1,0,LcdcStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Pending RGB data in DBI Type-B interface"]
    #[inline(always)]
    pub fn dbib_data_pending(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, LcdcStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,LcdcStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "The DBIB tearing effect signal"]
    #[inline(always)]
    pub fn dbib_te(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, LcdcStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,LcdcStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Sticky underflow(clear with write in the LCDC_INTERRUPT_REG)\n0: There is no underflow\n1: Underflow has been detected.Remains high until to be cleared by performing a write access on the register LCDC_INTERRUPT_REG."]
    #[inline(always)]
    pub fn sticky_underflow(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, LcdcStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,LcdcStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Underflow on the current transfer.\n0: There is no underflow\n1: Underflow has been detected."]
    #[inline(always)]
    pub fn underflow(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, LcdcStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,LcdcStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Last row (Last row is currently displayed)"]
    #[inline(always)]
    pub fn last_row(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, LcdcStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,LcdcStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "CSYNC signal level"]
    #[inline(always)]
    pub fn stat_csync(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, LcdcStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,LcdcStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "VSYNC signal level"]
    #[inline(always)]
    pub fn stat_vsync(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, LcdcStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,LcdcStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "HSYNC signal level"]
    #[inline(always)]
    pub fn stat_hsync(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, LcdcStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,LcdcStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "The frame generator is busy (active high)."]
    #[inline(always)]
    pub fn framegen_busy(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, LcdcStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,LcdcStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Active (When not in vertical blanking)"]
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
