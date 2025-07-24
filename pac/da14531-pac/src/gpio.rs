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
#[doc = r"GPIO registers"]
unsafe impl ::core::marker::Send for super::Gpio {}
unsafe impl ::core::marker::Sync for super::Gpio {}
impl super::Gpio {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn bist_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::BistCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::BistCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }

    #[doc = "P00 Mode Register"]
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

    #[doc = "P010 Mode Register"]
    #[inline(always)]
    pub const fn p010_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P010ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P010ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(26usize),
            )
        }
    }

    #[doc = "P011 Mode Register"]
    #[inline(always)]
    pub const fn p011_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P011ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P011ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "P01 Mode Register"]
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

    #[doc = "P02 Mode Register"]
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

    #[doc = "P03 Mode Register"]
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

    #[doc = "P04 Mode Register"]
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

    #[doc = "P05 Mode Register"]
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

    #[doc = "P06 Mode Register"]
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

    #[doc = "P07 Mode Register"]
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

    #[doc = "P08 Mode Register"]
    #[inline(always)]
    pub const fn p08_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P08ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P08ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(22usize),
            )
        }
    }

    #[doc = "P09 Mode Register"]
    #[inline(always)]
    pub const fn p09_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P09ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P09ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "P0 Data input/output Register"]
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

    #[doc = "P0 Reset port pins Register"]
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

    #[doc = "P0 Set port pins Register"]
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

    #[doc = "Pad driving strength control Register"]
    #[inline(always)]
    pub const fn pad_weak_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::PadWeakCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::PadWeakCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(30usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rombist_resulth_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RombistResulthReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RombistResulthReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[inline(always)]
    pub const fn rombist_resultl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::RombistResultlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::RombistResultlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(62usize),
            )
        }
    }

    #[inline(always)]
    pub const fn scan_observe_reg(
        &self,
    ) -> &'static crate::common::Reg<self::ScanObserveReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::ScanObserveReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[inline(always)]
    pub const fn test_ctrl2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::TestCtrl2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TestCtrl2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(50usize),
            )
        }
    }

    #[inline(always)]
    pub const fn test_ctrl3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::TestCtrl3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TestCtrl3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[inline(always)]
    pub const fn test_ctrl4_reg(
        &self,
    ) -> &'static crate::common::Reg<self::TestCtrl4Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TestCtrl4Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(54usize),
            )
        }
    }

    #[inline(always)]
    pub const fn test_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::TestCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::TestCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[inline(always)]
    pub const fn xtal32m_testctrl0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Xtal32MTestctrl0Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Xtal32MTestctrl0Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[inline(always)]
    pub const fn xtal32m_testctrl1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Xtal32MTestctrl1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Xtal32MTestctrl1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(58usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BistCtrlReg_SPEC;
impl crate::sealed::RegSpec for BistCtrlReg_SPEC {
    type DataType = u16;
}

pub type BistCtrlReg = crate::RegValueT<BistCtrlReg_SPEC>;

impl BistCtrlReg {
    #[inline(always)]
    pub fn sysram3_bist_enable(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, BistCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,BistCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ram_bist_pattern(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, u8, BistCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,u8,u8,BistCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sysram12_bist_busy(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, BistCtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11,1,0,BistCtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sysram12_bist_fail(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, BistCtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,BistCtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sysram3_bist_busy(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, BistCtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,BistCtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sysram3_bist_fail(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, BistCtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,BistCtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rom_bist_busy(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, BistCtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,BistCtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sysram12_bist_enable(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, BistCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,BistCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rombist_enable(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, BistCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,BistCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ram_bist_config(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, BistCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,BistCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BistCtrlReg {
    #[inline(always)]
    fn default() -> BistCtrlReg {
        <crate::RegValueT<BistCtrlReg_SPEC> as RegisterValue<_>>::new(1152)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P00ModeReg_SPEC;
impl crate::sealed::RegSpec for P00ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "P00 Mode Register"]
pub type P00ModeReg = crate::RegValueT<P00ModeReg_SPEC>;

impl P00ModeReg {
    #[doc = "00 = Input, no resistors selected\n01 = Input, pull-up selected\n10 = Input, pull-down selected\n11 = Output, no resistors selected\nIn ADC mode, these bits are don\'t care"]
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P00ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P00ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Function of port\n0  =  GPIO (pin direction determined by \"PUPD\" field)\n1  =  UART1_RX\n2  =  UART1_TX\n3  =  UART2_RX\n4  =  UART2_TX\n5  =  SYS_CLK\n6  =  LP_CLK\n7  =  Reserved\n8  =  Reserved\n9  =  I2C_SCL\n10 = I2C_SDA\n11 = PWM5\n12 = PWM6\n13 = PWM7\n14 = Reserved\n15 = ADC (only for P0_1, P0_2, P0_6 and P0_7)\n16 = PWM0\n17 = PWM1\n18 = BLE_DIAG (signals mapped to P0\\[3:0\\] are also mapped to P0\\[11:8\\])\n19 = UART1_CTSN\n20 = UART1_RTSN\n21 = Reserved\n22 = Reserved\n23 = PWM2\n24 = PWM3\n25 = PWM4\n26 = SPI_DI\n27 = SPI_DO\n28 = SPI_CLK\n29 = SPI_CSN0\n30 = SPI_CSN1\n31 = Reserved\nNote: When a certain input function (like SPI_DI) is selected on more than 1 pins, the pin of the lowest index has the highest priority."]
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
pub struct P010ModeReg_SPEC;
impl crate::sealed::RegSpec for P010ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "P010 Mode Register"]
pub type P010ModeReg = crate::RegValueT<P010ModeReg_SPEC>;

impl P010ModeReg {
    #[doc = "00 = Input, no resistors selected\n01 = Input, pull-up selected\n10 = Input, pull-down selected\n11 = Output, no resistors selected"]
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P010ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P010ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "See P00_MODE_REG\\[PID\\]"]
    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, P010ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,P010ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P010ModeReg {
    #[inline(always)]
    fn default() -> P010ModeReg {
        <crate::RegValueT<P010ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P011ModeReg_SPEC;
impl crate::sealed::RegSpec for P011ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "P011 Mode Register"]
pub type P011ModeReg = crate::RegValueT<P011ModeReg_SPEC>;

impl P011ModeReg {
    #[doc = "00 = Input, no resistors selected\n01 = Input, pull-up selected\n10 = Input, pull-down selected\n11 = Output, no resistors selected"]
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P011ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P011ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "See P00_MODE_REG\\[PID\\]"]
    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, P011ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,P011ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P011ModeReg {
    #[inline(always)]
    fn default() -> P011ModeReg {
        <crate::RegValueT<P011ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P01ModeReg_SPEC;
impl crate::sealed::RegSpec for P01ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "P01 Mode Register"]
pub type P01ModeReg = crate::RegValueT<P01ModeReg_SPEC>;

impl P01ModeReg {
    #[doc = "00 = Input, no resistors selected\n01 = Input, pull-up selected\n10 = Input, pull-down selected\n11 = Output, no resistors selected\nIn ADC mode, these bits are don\'t care"]
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P01ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P01ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "See P00_MODE_REG\\[PID\\]"]
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
pub struct P02ModeReg_SPEC;
impl crate::sealed::RegSpec for P02ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "P02 Mode Register"]
pub type P02ModeReg = crate::RegValueT<P02ModeReg_SPEC>;

impl P02ModeReg {
    #[doc = "00 = Input, no resistors selected\n01 = Input, pull-up selected\n10 = Input, pull-down selected\n11 = Output, no resistors selected\nIn ADC mode, these bits are don\'t care"]
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P02ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P02ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "See P00_MODE_REG\\[PID\\]"]
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

#[doc = "P03 Mode Register"]
pub type P03ModeReg = crate::RegValueT<P03ModeReg_SPEC>;

impl P03ModeReg {
    #[doc = "00 = Input, no resistors selected\n01 = Input, pull-up selected\n10 = Input, pull-down selected\n11 = Output, no resistors selected\nIn ADC mode, these bits are don\'t care"]
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P03ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P03ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "See P00_MODE_REG\\[PID\\]"]
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

#[doc = "P04 Mode Register"]
pub type P04ModeReg = crate::RegValueT<P04ModeReg_SPEC>;

impl P04ModeReg {
    #[doc = "00 = Input, no resistors selected\n01 = Input, pull-up selected\n10 = Input, pull-down selected\n11 = Output, no resistors selected"]
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P04ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P04ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "See P00_MODE_REG\\[PID\\]"]
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

#[doc = "P05 Mode Register"]
pub type P05ModeReg = crate::RegValueT<P05ModeReg_SPEC>;

impl P05ModeReg {
    #[doc = "00 = Input, no resistors selected\n01 = Input, pull-up selected\n10 = Input, pull-down selected\n11 = Output, no resistors selected"]
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P05ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P05ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "See P00_MODE_REG\\[PID\\]"]
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

#[doc = "P06 Mode Register"]
pub type P06ModeReg = crate::RegValueT<P06ModeReg_SPEC>;

impl P06ModeReg {
    #[doc = "00 = Input, no resistors selected\n01 = Input, pull-up selected\n10 = Input, pull-down selected\n11 = Output, no resistors selected"]
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P06ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P06ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "See P00_MODE_REG\\[PID\\]"]
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

#[doc = "P07 Mode Register"]
pub type P07ModeReg = crate::RegValueT<P07ModeReg_SPEC>;

impl P07ModeReg {
    #[doc = "00 = Input, no resistors selected\n01 = Input, pull-up selected\n10 = Input, pull-down selected\n11 = Output, no resistors selected"]
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P07ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P07ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "See P00_MODE_REG\\[PID\\]"]
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
pub struct P08ModeReg_SPEC;
impl crate::sealed::RegSpec for P08ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "P08 Mode Register"]
pub type P08ModeReg = crate::RegValueT<P08ModeReg_SPEC>;

impl P08ModeReg {
    #[doc = "00 = Input, no resistors selected\n01 = Input, pull-up selected\n10 = Input, pull-down selected\n11 = Output, no resistors selected"]
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P08ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P08ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "See P00_MODE_REG\\[PID\\]"]
    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, P08ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,P08ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P08ModeReg {
    #[inline(always)]
    fn default() -> P08ModeReg {
        <crate::RegValueT<P08ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P09ModeReg_SPEC;
impl crate::sealed::RegSpec for P09ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "P09 Mode Register"]
pub type P09ModeReg = crate::RegValueT<P09ModeReg_SPEC>;

impl P09ModeReg {
    #[doc = "00 = Input, no resistors selected\n01 = Input, pull-up selected\n10 = Input, pull-down selected\n11 = Output, no resistors selected"]
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P09ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P09ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "See P00_MODE_REG\\[PID\\]"]
    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, P09ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,P09ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P09ModeReg {
    #[inline(always)]
    fn default() -> P09ModeReg {
        <crate::RegValueT<P09ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P0DataReg_SPEC;
impl crate::sealed::RegSpec for P0DataReg_SPEC {
    type DataType = u16;
}

#[doc = "P0 Data input/output Register"]
pub type P0DataReg = crate::RegValueT<P0DataReg_SPEC>;

impl P0DataReg {
    #[doc = "Sets P0 output register when written ; Returns the value of P0 port when read"]
    #[inline(always)]
    pub fn p0_data(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, u16, P0DataReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xfff,1,0,u16,u16,P0DataReg_SPEC,crate::common::RW>::from_register(self,0)
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

#[doc = "P0 Reset port pins Register"]
pub type P0ResetDataReg = crate::RegValueT<P0ResetDataReg_SPEC>;

impl P0ResetDataReg {
    #[doc = "Writing a 1 to P0\\[x\\] sets P0\\[x\\] to 0.\nWriting 0 is discarded, reading returns 0."]
    #[inline(always)]
    pub fn p0_reset(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, u16, P0ResetDataReg_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<
            0,
            0xfff,
            1,
            0,
            u16,
            u16,
            P0ResetDataReg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
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

#[doc = "P0 Set port pins Register"]
pub type P0SetDataReg = crate::RegValueT<P0SetDataReg_SPEC>;

impl P0SetDataReg {
    #[doc = "Writing a 1 to P0\\[x\\] sets P0\\[x\\] to 1.\nWriting 0 is discarded, reading returns 0"]
    #[inline(always)]
    pub fn p0_set(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, u16, P0SetDataReg_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xfff,1,0,u16,u16,P0SetDataReg_SPEC,crate::common::W>::from_register(self,0)
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
pub struct PadWeakCtrlReg_SPEC;
impl crate::sealed::RegSpec for PadWeakCtrlReg_SPEC {
    type DataType = u16;
}

#[doc = "Pad driving strength control Register"]
pub type PadWeakCtrlReg = crate::RegValueT<PadWeakCtrlReg_SPEC>;

impl PadWeakCtrlReg {
    #[doc = "0 = Normal operation\n1 = Reduces the driving strength of P0_x pad.\nBit x controls the driving strength of P0_x, x=0, 1,..., 11."]
    #[inline(always)]
    pub fn pad_low_drv(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xfff,
        1,
        0,
        u16,
        u16,
        PadWeakCtrlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xfff,
            1,
            0,
            u16,
            u16,
            PadWeakCtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for PadWeakCtrlReg {
    #[inline(always)]
    fn default() -> PadWeakCtrlReg {
        <crate::RegValueT<PadWeakCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RombistResulthReg_SPEC;
impl crate::sealed::RegSpec for RombistResulthReg_SPEC {
    type DataType = u16;
}

pub type RombistResulthReg = crate::RegValueT<RombistResulthReg_SPEC>;

impl RombistResulthReg {
    #[inline(always)]
    pub fn rombist_resulth(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        RombistResulthReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            RombistResulthReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RombistResulthReg {
    #[inline(always)]
    fn default() -> RombistResulthReg {
        <crate::RegValueT<RombistResulthReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RombistResultlReg_SPEC;
impl crate::sealed::RegSpec for RombistResultlReg_SPEC {
    type DataType = u16;
}

pub type RombistResultlReg = crate::RegValueT<RombistResultlReg_SPEC>;

impl RombistResultlReg {
    #[inline(always)]
    pub fn rombist_resultl(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        RombistResultlReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            RombistResultlReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for RombistResultlReg {
    #[inline(always)]
    fn default() -> RombistResultlReg {
        <crate::RegValueT<RombistResultlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ScanObserveReg_SPEC;
impl crate::sealed::RegSpec for ScanObserveReg_SPEC {
    type DataType = u16;
}

pub type ScanObserveReg = crate::RegValueT<ScanObserveReg_SPEC>;

impl ScanObserveReg {
    #[inline(always)]
    pub fn scan_feedback_mux(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        ScanObserveReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            ScanObserveReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for ScanObserveReg {
    #[inline(always)]
    fn default() -> ScanObserveReg {
        <crate::RegValueT<ScanObserveReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TestCtrl2Reg_SPEC;
impl crate::sealed::RegSpec for TestCtrl2Reg_SPEC {
    type DataType = u16;
}

pub type TestCtrl2Reg = crate::RegValueT<TestCtrl2Reg_SPEC>;

impl TestCtrl2Reg {
    #[inline(always)]
    pub fn ana_test_out_param(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, u8, TestCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0xf,1,0,u8,u8,TestCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ana_test_out_to_pin(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, TestCtrl2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,TestCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ana_test_out_sel(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, TestCtrl2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,TestCtrl2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for TestCtrl2Reg {
    #[inline(always)]
    fn default() -> TestCtrl2Reg {
        <crate::RegValueT<TestCtrl2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TestCtrl3Reg_SPEC;
impl crate::sealed::RegSpec for TestCtrl3Reg_SPEC {
    type DataType = u16;
}

pub type TestCtrl3Reg = crate::RegValueT<TestCtrl3Reg_SPEC>;

impl TestCtrl3Reg {
    #[inline(always)]
    pub fn rf_test_out_to_pin(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, TestCtrl3Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,TestCtrl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rf_test_out_param(
        self,
    ) -> crate::common::RegisterField<7, 0x3f, 1, 0, u8, u8, TestCtrl3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x3f,1,0,u8,u8,TestCtrl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn enable_rfpt(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, TestCtrl3Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,TestCtrl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rf_test_out_sel(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, TestCtrl3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,TestCtrl3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for TestCtrl3Reg {
    #[inline(always)]
    fn default() -> TestCtrl3Reg {
        <crate::RegValueT<TestCtrl3Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TestCtrl4Reg_SPEC;
impl crate::sealed::RegSpec for TestCtrl4Reg_SPEC {
    type DataType = u16;
}

pub type TestCtrl4Reg = crate::RegValueT<TestCtrl4Reg_SPEC>;

impl TestCtrl4Reg {
    #[inline(always)]
    pub fn rf_test_in_to_pin(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, TestCtrl4Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,TestCtrl4Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rf_test_in_param(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, u8, TestCtrl4Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1f,1,0,u8,u8,TestCtrl4Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rf_test_in_sel(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, TestCtrl4Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,TestCtrl4Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for TestCtrl4Reg {
    #[inline(always)]
    fn default() -> TestCtrl4Reg {
        <crate::RegValueT<TestCtrl4Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TestCtrlReg_SPEC;
impl crate::sealed::RegSpec for TestCtrlReg_SPEC {
    type DataType = u16;
}

pub type TestCtrlReg = crate::RegValueT<TestCtrlReg_SPEC>;

impl TestCtrlReg {
    #[inline(always)]
    pub fn adpll_scan_comp_en(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, TestCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,TestCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn adpll_scan_test_en(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, TestCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,TestCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cp_cap_bias_trim(
        self,
    ) -> crate::common::RegisterField<9, 0x3, 1, 0, u8, u8, TestCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x3,1,0,u8,u8,TestCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_core_dummy_load_enable(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, TestCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,TestCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_core_cap_bypass(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, TestCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,TestCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn xtal32m_cap_test_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, TestCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,TestCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn show_dcdc(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, TestCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,TestCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn show_power(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, TestCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,TestCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn show_clocks(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, TestCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,TestCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for TestCtrlReg {
    #[inline(always)]
    fn default() -> TestCtrlReg {
        <crate::RegValueT<TestCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xtal32MTestctrl0Reg_SPEC;
impl crate::sealed::RegSpec for Xtal32MTestctrl0Reg_SPEC {
    type DataType = u16;
}

pub type Xtal32MTestctrl0Reg = crate::RegValueT<Xtal32MTestctrl0Reg_SPEC>;

impl Xtal32MTestctrl0Reg {
    #[inline(always)]
    pub fn bias_sah_hold_override(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x3,
        1,
        0,
        u8,
        u8,
        Xtal32MTestctrl0Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x3,
            1,
            0,
            u8,
            u8,
            Xtal32MTestctrl0Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn core_freq_trim_sw2_amp(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x7,
        1,
        0,
        u8,
        u8,
        Xtal32MTestctrl0Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x7,
            1,
            0,
            u8,
            u8,
            Xtal32MTestctrl0Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn core_gm_current(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Xtal32MTestctrl0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<10,1,0,Xtal32MTestctrl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn core_hold_amp_reg_override(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3,
        1,
        0,
        u8,
        u8,
        Xtal32MTestctrl0Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x3,
            1,
            0,
            u8,
            u8,
            Xtal32MTestctrl0Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn core_i2v_to_testbus(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Xtal32MTestctrl0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<7,1,0,Xtal32MTestctrl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn core_i2v_to_testbus_10x(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Xtal32MTestctrl0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<6,1,0,Xtal32MTestctrl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn core_max_current(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Xtal32MTestctrl0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<5,1,0,Xtal32MTestctrl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn core_xtal_discharge(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Xtal32MTestctrl0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<4,1,0,Xtal32MTestctrl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dcblock_lv_mode(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Xtal32MTestctrl0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<3,1,0,Xtal32MTestctrl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn diffbuf_bypass(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Xtal32MTestctrl0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<2,1,0,Xtal32MTestctrl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn osc_trim_open_disable(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Xtal32MTestctrl0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<1,1,0,Xtal32MTestctrl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn spike_flt_disable(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Xtal32MTestctrl0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<0,1,0,Xtal32MTestctrl0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Xtal32MTestctrl0Reg {
    #[inline(always)]
    fn default() -> Xtal32MTestctrl0Reg {
        <crate::RegValueT<Xtal32MTestctrl0Reg_SPEC> as RegisterValue<_>>::new(13312)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xtal32MTestctrl1Reg_SPEC;
impl crate::sealed::RegSpec for Xtal32MTestctrl1Reg_SPEC {
    type DataType = u16;
}

pub type Xtal32MTestctrl1Reg = crate::RegValueT<Xtal32MTestctrl1Reg_SPEC>;

impl Xtal32MTestctrl1Reg {
    #[inline(always)]
    pub fn osc_trim_cap_bias(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Xtal32MTestctrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<8,1,0,Xtal32MTestctrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfclk_sel_adpll_adc_to_gpio(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Xtal32MTestctrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<7,1,0,Xtal32MTestctrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfclk_adc_to_gpio(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Xtal32MTestctrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<6,1,0,Xtal32MTestctrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rfclk_adpll_to_gpio(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Xtal32MTestctrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<5,1,0,Xtal32MTestctrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn prog_vref_sel(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Xtal32MTestctrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<4,1,0,Xtal32MTestctrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn varicap_test_sel_xtal(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Xtal32MTestctrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<3,1,0,Xtal32MTestctrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn varicap_test_enable(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Xtal32MTestctrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<2,1,0,Xtal32MTestctrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ldo_vref_hold_override(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Xtal32MTestctrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<1,1,0,Xtal32MTestctrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn disable_tm_clk(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Xtal32MTestctrl1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<0,1,0,Xtal32MTestctrl1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Xtal32MTestctrl1Reg {
    #[inline(always)]
    fn default() -> Xtal32MTestctrl1Reg {
        <crate::RegValueT<Xtal32MTestctrl1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}
