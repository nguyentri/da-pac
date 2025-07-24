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
// Generated from SVD 1.2, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:44:20 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"gpio580_ports_nl01 registers"]
unsafe impl ::core::marker::Send for super::Gpio580PortsNl01 {}
unsafe impl ::core::marker::Sync for super::Gpio580PortsNl01 {}
impl super::Gpio580PortsNl01 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
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

    #[doc = "Ports 0 and 1 Output Power Control Register"]
    #[inline(always)]
    pub const fn p01_padpwr_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P01PadpwrCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P01PadpwrCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(112usize),
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

    #[doc = "P0 Data input / output register"]
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

    #[doc = "P0 Reset port pins register"]
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

    #[doc = "P0 Set port pins register"]
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

    #[doc = "P10 Mode Register"]
    #[inline(always)]
    pub const fn p10_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P10ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P10ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(38usize),
            )
        }
    }

    #[doc = "P11 Mode Register"]
    #[inline(always)]
    pub const fn p11_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P11ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P11ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[doc = "P12 Mode Register"]
    #[inline(always)]
    pub const fn p12_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P12ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P12ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(42usize),
            )
        }
    }

    #[doc = "P13 Mode Register"]
    #[inline(always)]
    pub const fn p13_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P13ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P13ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[doc = "P14 Mode Register"]
    #[inline(always)]
    pub const fn p14_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P14ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P14ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(46usize),
            )
        }
    }

    #[doc = "P15 Mode Register"]
    #[inline(always)]
    pub const fn p15_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P15ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P15ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[doc = "P1 Data input / output register"]
    #[inline(always)]
    pub const fn p1_data_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P1DataReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P1DataReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "P1 Reset port pins register"]
    #[inline(always)]
    pub const fn p1_reset_data_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P1ResetDataReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P1ResetDataReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[doc = "P1 Set port pins register"]
    #[inline(always)]
    pub const fn p1_set_data_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P1SetDataReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P1SetDataReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(34usize),
            )
        }
    }

    #[doc = "P20 Mode Register"]
    #[inline(always)]
    pub const fn p20_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P20ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P20ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(70usize),
            )
        }
    }

    #[doc = "P21 Mode Register"]
    #[inline(always)]
    pub const fn p21_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P21ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P21ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(72usize),
            )
        }
    }

    #[doc = "P22 Mode Register"]
    #[inline(always)]
    pub const fn p22_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P22ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P22ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(74usize),
            )
        }
    }

    #[doc = "P23 Mode Register"]
    #[inline(always)]
    pub const fn p23_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P23ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P23ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(76usize),
            )
        }
    }

    #[doc = "P24 Mode Register"]
    #[inline(always)]
    pub const fn p24_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P24ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P24ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(78usize),
            )
        }
    }

    #[doc = "P25 Mode Register"]
    #[inline(always)]
    pub const fn p25_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P25ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P25ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }

    #[doc = "P26 Mode Register"]
    #[inline(always)]
    pub const fn p26_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P26ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P26ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(82usize),
            )
        }
    }

    #[doc = "P27 Mode Register"]
    #[inline(always)]
    pub const fn p27_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P27ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P27ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(84usize),
            )
        }
    }

    #[doc = "P28 Mode Register"]
    #[inline(always)]
    pub const fn p28_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P28ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P28ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(86usize),
            )
        }
    }

    #[doc = "P29 Mode Register"]
    #[inline(always)]
    pub const fn p29_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P29ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P29ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(88usize),
            )
        }
    }

    #[doc = "P2 Data input / output register"]
    #[inline(always)]
    pub const fn p2_data_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P2DataReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P2DataReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[doc = "Port 2 Output Power Control Register"]
    #[inline(always)]
    pub const fn p2_padpwr_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P2PadpwrCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P2PadpwrCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(114usize),
            )
        }
    }

    #[doc = "P2 Reset port pins register"]
    #[inline(always)]
    pub const fn p2_reset_data_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P2ResetDataReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P2ResetDataReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(68usize),
            )
        }
    }

    #[doc = "P2 Set port pins register"]
    #[inline(always)]
    pub const fn p2_set_data_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P2SetDataReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P2SetDataReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(66usize),
            )
        }
    }

    #[doc = "P30 Mode Register"]
    #[inline(always)]
    pub const fn p30_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P30ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P30ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(134usize),
            )
        }
    }

    #[doc = "P31 Mode Register"]
    #[inline(always)]
    pub const fn p31_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P31ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P31ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(136usize),
            )
        }
    }

    #[doc = "P32 Mode Register"]
    #[inline(always)]
    pub const fn p32_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P32ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P32ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(138usize),
            )
        }
    }

    #[doc = "P33 Mode Register"]
    #[inline(always)]
    pub const fn p33_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P33ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P33ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(140usize),
            )
        }
    }

    #[doc = "P34 Mode Register"]
    #[inline(always)]
    pub const fn p34_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P34ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P34ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(142usize),
            )
        }
    }

    #[doc = "P35 Mode Register"]
    #[inline(always)]
    pub const fn p35_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P35ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P35ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(144usize),
            )
        }
    }

    #[doc = "P36 Mode Register"]
    #[inline(always)]
    pub const fn p36_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P36ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P36ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(146usize),
            )
        }
    }

    #[doc = "P37 Mode Register"]
    #[inline(always)]
    pub const fn p37_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P37ModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P37ModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(148usize),
            )
        }
    }

    #[doc = "P3 Data input / output register"]
    #[inline(always)]
    pub const fn p3_data_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P3DataReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P3DataReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(128usize),
            )
        }
    }

    #[doc = "Port 3 Output Power Control Register"]
    #[inline(always)]
    pub const fn p3_padpwr_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P3PadpwrCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P3PadpwrCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(116usize),
            )
        }
    }

    #[doc = "P3 Reset port pins register"]
    #[inline(always)]
    pub const fn p3_reset_data_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P3ResetDataReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P3ResetDataReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(132usize),
            )
        }
    }

    #[doc = "P3 Set port pins register"]
    #[inline(always)]
    pub const fn p3_set_data_reg(
        &self,
    ) -> &'static crate::common::Reg<self::P3SetDataReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::P3SetDataReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(130usize),
            )
        }
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
    #[doc = "00 = Input, no resistors selected\n01 = Input, pull-up selected\n10 = Input, Pull-down selected\n11 = Output, no resistors selected\nIn ADC mode, these bits are don\'t care"]
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P00ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P00ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Function of port\n0 = Port function, PUPD as set above\n1 = UART1_RX\n2 = UART1_TX\n3 = UART2_RX\n4 = UART2_TX\n5 = SPI_DI\n6 = SPI_DO\n7 = SPI_CLK\n8 = SPI_EN\n9 = I2C_SCL\n10 = I2C_SDA\n11 = UART1_IRDA_RX\n12 = UART1_IRDA_TX\n13 = UART2_IRDA_RX\n14 = UART2_IRDA_TX\n15 = ADC (only for P0\\[3:0\\])\n16 = PWM0\n17 = PWM1\n18 = BLE_DIAG (only for P0\\[7:0\\])\n19 = UART1_CTSN\n20 = UART1_RTSN\n21 = UART2_CTSN\n22 = UART2_RTSN\n23 = PWM2\n24 = PWM3\n25 = PWM4\nNote: when a certain input function (like SPI_DI) is selected on more than 1 port pin, the port with the lowest index has the highest priority and P0 has higher priority than P1."]
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
pub struct P01ModeReg_SPEC;
impl crate::sealed::RegSpec for P01ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "P01 Mode Register"]
pub type P01ModeReg = crate::RegValueT<P01ModeReg_SPEC>;

impl P01ModeReg {
    #[doc = "00 = Input, no resistors selected\n01 = Input, pull-up selected\n10 = Input, Pull-down selected\n11 = Output, no resistors selected\nIn ADC mode, these bits are don\'t care"]
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P01ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P01ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Function of port\n0 = Port function, PUPD as set above\n1 = UART1_RX\n2 = UART1_TX\n3 = UART2_RX\n4 = UART2_TX\n5 = SPI_DI\n6 = SPI_DO\n7 = SPI_CLK\n8 = SPI_EN\n9 = I2C_SCL\n10 = I2C_SDA\n11 = UART1_IRDA_RX\n12 = UART1_IRDA_TX\n13 = UART2_IRDA_RX\n14 = UART2_IRDA_TX\n15 = ADC (only for P0\\[3:0\\])\n16 = PWM0\n17 = PWM1\n18 = BLE_DIAG (only for P0\\[7:0\\])\n19 = UART1_CTSN\n20 = UART1_RTSN\n21 = UART2_CTSN\n22 = UART2_RTSN\n23 = PWM2\n24 = PWM3\n25 = PWM4\nNote: when a certain input function (like SPI_DI) is selected on more than 1 port pin, the port with the lowest index has the highest priority and P0 has higher priority than P1."]
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
pub struct P01PadpwrCtrlReg_SPEC;
impl crate::sealed::RegSpec for P01PadpwrCtrlReg_SPEC {
    type DataType = u16;
}

#[doc = "Ports 0 and 1 Output Power Control Register"]
pub type P01PadpwrCtrlReg = crate::RegValueT<P01PadpwrCtrlReg_SPEC>;

impl P01PadpwrCtrlReg {
    #[doc = "1 = P1_x port output is powered by the 1 V rail\n0 = P1_x port output is powered by the 3 V rail\nbit 8 controls the power of P1\\[0\\],\nbit 13 controls the power of P1\\[5\\]"]
    #[inline(always)]
    pub fn p1_out_ctrl(
        self,
    ) -> crate::common::RegisterField<8, 0x3f, 1, 0, u8, u8, P01PadpwrCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            8,
            0x3f,
            1,
            0,
            u8,
            u8,
            P01PadpwrCtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[doc = "1 = P0_x port output is powered by the 1 V rail\n0 = P0_x port output is powered by the 3 V rail\nbit 0 controls the power of P0\\[0\\],\nbit 7 controls the power of P0\\[7\\]"]
    #[inline(always)]
    pub fn p0_out_ctrl(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, P01PadpwrCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            P01PadpwrCtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P01PadpwrCtrlReg {
    #[inline(always)]
    fn default() -> P01PadpwrCtrlReg {
        <crate::RegValueT<P01PadpwrCtrlReg_SPEC> as RegisterValue<_>>::new(0)
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
    #[doc = "00 = Input, no resistors selected\n01 = Input, pull-up selected\n10 = Input, Pull-down selected\n11 = Output, no resistors selected\nIn ADC mode, these bits are don\'t care"]
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P02ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P02ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Function of port\n0 = Port function, PUPD as set above\n1 = UART1_RX\n2 = UART1_TX\n3 = UART2_RX\n4 = UART2_TX\n5 = SPI_DI\n6 = SPI_DO\n7 = SPI_CLK\n8 = SPI_EN\n9 = I2C_SCL\n10 = I2C_SDA\n11 = UART1_IRDA_RX\n12 = UART1_IRDA_TX\n13 = UART2_IRDA_RX\n14 = UART2_IRDA_TX\n15 = ADC (only for P0\\[3:0\\])\n16 = PWM0\n17 = PWM1\n18 = BLE_DIAG (only for P0\\[7:0\\])\n19 = UART1_CTSN\n20 = UART1_RTSN\n21 = UART2_CTSN\n22 = UART2_RTSN\n23 = PWM2\n24 = PWM3\n25 = PWM4\nNote: when a certain input function (like SPI_DI) is selected on more than 1 port pin, the port with the lowest index has the highest priority and P0 has higher priority than P1."]
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
    #[doc = "00 = Input, no resistors selected\n01 = Input, pull-up selected\n10 = Input, Pull-down selected\n11 = Output, no resistors selected\nIn ADC mode, these bits are don\'t care"]
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P03ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P03ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Function of port\n0 = Port function, PUPD as set above\n1 = UART1_RX\n2 = UART1_TX\n3 = UART2_RX\n4 = UART2_TX\n5 = SPI_DI\n6 = SPI_DO\n7 = SPI_CLK\n8 = SPI_EN\n9 = I2C_SCL\n10 = I2C_SDA\n11 = UART1_IRDA_RX\n12 = UART1_IRDA_TX\n13 = UART2_IRDA_RX\n14 = UART2_IRDA_TX\n15 = ADC (only for P0\\[3:0\\])\n16 = PWM0\n17 = PWM1\n18 = BLE_DIAG (only for P0\\[7:0\\])\n19 = UART1_CTSN\n20 = UART1_RTSN\n21 = UART2_CTSN\n22 = UART2_RTSN\n23 = PWM2\n24 = PWM3\n25 = PWM4\nNote: when a certain input function (like SPI_DI) is selected on more than 1 port pin, the port with the lowest index has the highest priority and P0 has higher priority than P1."]
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
    #[doc = "00 = Input, no resistors selected\n01 = Input, pull-up selected\n10 = Input, Pull-down selected\n11 = Output, no resistors selected\nIn ADC mode, these bits are don\'t care"]
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P04ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P04ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Function of port\n0 = Port function, PUPD as set above\n1 = UART1_RX\n2 = UART1_TX\n3 = UART2_RX\n4 = UART2_TX\n5 = SPI_DI\n6 = SPI_DO\n7 = SPI_CLK\n8 = SPI_EN\n9 = I2C_SCL\n10 = I2C_SDA\n11 = UART1_IRDA_RX\n12 = UART1_IRDA_TX\n13 = UART2_IRDA_RX\n14 = UART2_IRDA_TX\n15 = ADC (only for P0\\[3:0\\])\n16 = PWM0\n17 = PWM1\n18 = BLE_DIAG (only for P0\\[7:0\\])\n19 = UART1_CTSN\n20 = UART1_RTSN\n21 = UART2_CTSN\n22 = UART2_RTSN\n23 = PWM2\n24 = PWM3\n25 = PWM4\nNote: when a certain input function (like SPI_DI) is selected on more than 1 port pin, the port with the lowest index has the highest priority and P0 has higher priority than P1."]
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
    #[doc = "00 = Input, no resistors selected\n01 = Input, pull-up selected\n10 = Input, Pull-down selected\n11 = Output, no resistors selected\nIn ADC mode, these bits are don\'t care"]
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P05ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P05ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Function of port\n0 = Port function, PUPD as set above\n1 = UART1_RX\n2 = UART1_TX\n3 = UART2_RX\n4 = UART2_TX\n5 = SPI_DI\n6 = SPI_DO\n7 = SPI_CLK\n8 = SPI_EN\n9 = I2C_SCL\n10 = I2C_SDA\n11 = UART1_IRDA_RX\n12 = UART1_IRDA_TX\n13 = UART2_IRDA_RX\n14 = UART2_IRDA_TX\n15 = ADC (only for P0\\[3:0\\])\n16 = PWM0\n17 = PWM1\n18 = BLE_DIAG (only for P0\\[7:0\\])\n19 = UART1_CTSN\n20 = UART1_RTSN\n21 = UART2_CTSN\n22 = UART2_RTSN\n23 = PWM2\n24 = PWM3\n25 = PWM4\nNote: when a certain input function (like SPI_DI) is selected on more than 1 port pin, the port with the lowest index has the highest priority and P0 has higher priority than P1."]
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
    #[doc = "00 = Input, no resistors selected\n01 = Input, pull-up selected\n10 = Input, Pull-down selected\n11 = Output, no resistors selected\nIn ADC mode, these bits are don\'t care"]
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P06ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P06ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Function of port\n0 = Port function, PUPD as set above\n1 = UART1_RX\n2 = UART1_TX\n3 = UART2_RX\n4 = UART2_TX\n5 = SPI_DI\n6 = SPI_DO\n7 = SPI_CLK\n8 = SPI_EN\n9 = I2C_SCL\n10 = I2C_SDA\n11 = UART1_IRDA_RX\n12 = UART1_IRDA_TX\n13 = UART2_IRDA_RX\n14 = UART2_IRDA_TX\n15 = ADC (only for P0\\[3:0\\])\n16 = PWM0\n17 = PWM1\n18 = BLE_DIAG (only for P0\\[7:0\\])\n19 = UART1_CTSN\n20 = UART1_RTSN\n21 = UART2_CTSN\n22 = UART2_RTSN\n23 = PWM2\n24 = PWM3\n25 = PWM4\nNote: when a certain input function (like SPI_DI) is selected on more than 1 port pin, the port with the lowest index has the highest priority and P0 has higher priority than P1."]
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
    #[doc = "00 = Input, no resistors selected\n01 = Input, pull-up selected\n10 = Input, Pull-down selected\n11 = Output, no resistors selected\nIn ADC mode, these bits are don\'t care"]
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P07ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P07ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Function of port\n0 = Port function, PUPD as set above\n1 = UART1_RX\n2 = UART1_TX\n3 = UART2_RX\n4 = UART2_TX\n5 = SPI_DI\n6 = SPI_DO\n7 = SPI_CLK\n8 = SPI_EN\n9 = I2C_SCL\n10 = I2C_SDA\n11 = UART1_IRDA_RX\n12 = UART1_IRDA_TX\n13 = UART2_IRDA_RX\n14 = UART2_IRDA_TX\n15 = ADC (only for P0\\[3:0\\])\n16 = PWM0\n17 = PWM1\n18 = BLE_DIAG (only for P0\\[7:0\\])\n19 = UART1_CTSN\n20 = UART1_RTSN\n21 = UART2_CTSN\n22 = UART2_RTSN\n23 = PWM2\n24 = PWM3\n25 = PWM4\nNote: when a certain input function (like SPI_DI) is selected on more than 1 port pin, the port with the lowest index has the highest priority and P0 has higher priority than P1."]
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
pub struct P0DataReg_SPEC;
impl crate::sealed::RegSpec for P0DataReg_SPEC {
    type DataType = u16;
}

#[doc = "P0 Data input / output register"]
pub type P0DataReg = crate::RegValueT<P0DataReg_SPEC>;

impl P0DataReg {
    #[doc = "Set P0 output register when written; Returns the value of P0 port when read"]
    #[inline(always)]
    pub fn p0_data(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, P0DataReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,P0DataReg_SPEC,crate::common::RW>::from_register(self,0)
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

#[doc = "P0 Reset port pins register"]
pub type P0ResetDataReg = crate::RegValueT<P0ResetDataReg_SPEC>;

impl P0ResetDataReg {
    #[doc = "Writing a 1 to P0\\[y\\] sets P0\\[y\\] to 0. Writing 0 is discarded;\nReading returns 0"]
    #[inline(always)]
    pub fn p0_reset(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, P0ResetDataReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,P0ResetDataReg_SPEC,crate::common::RW>::from_register(self,0)
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

#[doc = "P0 Set port pins register"]
pub type P0SetDataReg = crate::RegValueT<P0SetDataReg_SPEC>;

impl P0SetDataReg {
    #[doc = "Writing a 1 to P0\\[y\\] sets P0\\[y\\] to 1. Writing 0 is discarded;\nReading returns 0"]
    #[inline(always)]
    pub fn p0_set(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, P0SetDataReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,P0SetDataReg_SPEC,crate::common::RW>::from_register(self,0)
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
pub struct P10ModeReg_SPEC;
impl crate::sealed::RegSpec for P10ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "P10 Mode Register"]
pub type P10ModeReg = crate::RegValueT<P10ModeReg_SPEC>;

impl P10ModeReg {
    #[doc = "00 = Input, no resistors selected\n01 = Input, pull-up selected\n10 = Input, Pull-down selected\n11 = Output, no resistors selected\nIn analog mode, these bits are don\'t care\nP14_MODE_REG and P15_MODE_REG reset value is 1 (i.e. pulled up)"]
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P10ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P10ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "See P0x_MODE_REG\\[PID\\]"]
    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, P10ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,P10ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P10ModeReg {
    #[inline(always)]
    fn default() -> P10ModeReg {
        <crate::RegValueT<P10ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P11ModeReg_SPEC;
impl crate::sealed::RegSpec for P11ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "P11 Mode Register"]
pub type P11ModeReg = crate::RegValueT<P11ModeReg_SPEC>;

impl P11ModeReg {
    #[doc = "00 = Input, no resistors selected\n01 = Input, pull-up selected\n10 = Input, Pull-down selected\n11 = Output, no resistors selected\nIn analog mode, these bits are don\'t care\nP14_MODE_REG and P15_MODE_REG reset value is 1 (i.e. pulled up)"]
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P11ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P11ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "See P0x_MODE_REG\\[PID\\]"]
    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, P11ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,P11ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P11ModeReg {
    #[inline(always)]
    fn default() -> P11ModeReg {
        <crate::RegValueT<P11ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P12ModeReg_SPEC;
impl crate::sealed::RegSpec for P12ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "P12 Mode Register"]
pub type P12ModeReg = crate::RegValueT<P12ModeReg_SPEC>;

impl P12ModeReg {
    #[doc = "00 = Input, no resistors selected\n01 = Input, pull-up selected\n10 = Input, Pull-down selected\n11 = Output, no resistors selected\nIn analog mode, these bits are don\'t care\nP14_MODE_REG and P15_MODE_REG reset value is 1 (i.e. pulled up)"]
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P12ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P12ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "See P0x_MODE_REG\\[PID\\]"]
    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, P12ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,P12ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P12ModeReg {
    #[inline(always)]
    fn default() -> P12ModeReg {
        <crate::RegValueT<P12ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P13ModeReg_SPEC;
impl crate::sealed::RegSpec for P13ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "P13 Mode Register"]
pub type P13ModeReg = crate::RegValueT<P13ModeReg_SPEC>;

impl P13ModeReg {
    #[doc = "00 = Input, no resistors selected\n01 = Input, pull-up selected\n10 = Input, Pull-down selected\n11 = Output, no resistors selected\nIn analog mode, these bits are don\'t care\nP14_MODE_REG and P15_MODE_REG reset value is 1 (i.e. pulled up)"]
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P13ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P13ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "See P0x_MODE_REG\\[PID\\]"]
    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, P13ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,P13ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P13ModeReg {
    #[inline(always)]
    fn default() -> P13ModeReg {
        <crate::RegValueT<P13ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P14ModeReg_SPEC;
impl crate::sealed::RegSpec for P14ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "P14 Mode Register"]
pub type P14ModeReg = crate::RegValueT<P14ModeReg_SPEC>;

impl P14ModeReg {
    #[doc = "00 = Input, no resistors selected\n01 = Input, pull-up selected\n10 = Input, Pull-down selected\n11 = Output, no resistors selected\nIn analog mode, these bits are don\'t care\nP14_MODE_REG and P15_MODE_REG reset value is 1 (i.e. pulled up)"]
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P14ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P14ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "See P0x_MODE_REG\\[PID\\]"]
    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, P14ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,P14ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P14ModeReg {
    #[inline(always)]
    fn default() -> P14ModeReg {
        <crate::RegValueT<P14ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P15ModeReg_SPEC;
impl crate::sealed::RegSpec for P15ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "P15 Mode Register"]
pub type P15ModeReg = crate::RegValueT<P15ModeReg_SPEC>;

impl P15ModeReg {
    #[doc = "00 = Input, no resistors selected\n01 = Input, pull-up selected\n10 = Input, Pull-down selected\n11 = Output, no resistors selected\nIn analog mode, these bits are don\'t care\nP14_MODE_REG and P15_MODE_REG reset value is 1 (i.e. pulled up)"]
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P15ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P15ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "See P0x_MODE_REG\\[PID\\]"]
    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, P15ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,P15ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P15ModeReg {
    #[inline(always)]
    fn default() -> P15ModeReg {
        <crate::RegValueT<P15ModeReg_SPEC> as RegisterValue<_>>::new(256)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P1DataReg_SPEC;
impl crate::sealed::RegSpec for P1DataReg_SPEC {
    type DataType = u16;
}

#[doc = "P1 Data input / output register"]
pub type P1DataReg = crate::RegValueT<P1DataReg_SPEC>;

impl P1DataReg {
    #[doc = "Set P1 output register when written; Returns the value of P1 port when read"]
    #[inline(always)]
    pub fn p1_data(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, P1DataReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,P1DataReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P1DataReg {
    #[inline(always)]
    fn default() -> P1DataReg {
        <crate::RegValueT<P1DataReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P1ResetDataReg_SPEC;
impl crate::sealed::RegSpec for P1ResetDataReg_SPEC {
    type DataType = u16;
}

#[doc = "P1 Reset port pins register"]
pub type P1ResetDataReg = crate::RegValueT<P1ResetDataReg_SPEC>;

impl P1ResetDataReg {
    #[doc = "Writing a 1 to P1\\[y\\] sets P1\\[y\\] to 0. Writing 0 is discarded;\nReading returns 0"]
    #[inline(always)]
    pub fn p1_reset(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, P1ResetDataReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,P1ResetDataReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P1ResetDataReg {
    #[inline(always)]
    fn default() -> P1ResetDataReg {
        <crate::RegValueT<P1ResetDataReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P1SetDataReg_SPEC;
impl crate::sealed::RegSpec for P1SetDataReg_SPEC {
    type DataType = u16;
}

#[doc = "P1 Set port pins register"]
pub type P1SetDataReg = crate::RegValueT<P1SetDataReg_SPEC>;

impl P1SetDataReg {
    #[doc = "Writing a 1 to P1\\[y\\] sets P1\\[y\\] to 1. Writing 0 is discarded;\nReading returns 0"]
    #[inline(always)]
    pub fn p1_set(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, P1SetDataReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,P1SetDataReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P1SetDataReg {
    #[inline(always)]
    fn default() -> P1SetDataReg {
        <crate::RegValueT<P1SetDataReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P20ModeReg_SPEC;
impl crate::sealed::RegSpec for P20ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "P20 Mode Register"]
pub type P20ModeReg = crate::RegValueT<P20ModeReg_SPEC>;

impl P20ModeReg {
    #[doc = "00 = Input, no resistors selected\n01 = Input, pull-up selected\n10 = Input, Pull-down selected\n11 = Output, no resistors selected\nIn analog mode, these bits are don\'t care"]
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P20ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P20ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "See P0x_MODE_REG\\[PID\\]"]
    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, P20ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,P20ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P20ModeReg {
    #[inline(always)]
    fn default() -> P20ModeReg {
        <crate::RegValueT<P20ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P21ModeReg_SPEC;
impl crate::sealed::RegSpec for P21ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "P21 Mode Register"]
pub type P21ModeReg = crate::RegValueT<P21ModeReg_SPEC>;

impl P21ModeReg {
    #[doc = "00 = Input, no resistors selected\n01 = Input, pull-up selected\n10 = Input, Pull-down selected\n11 = Output, no resistors selected\nIn analog mode, these bits are don\'t care"]
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P21ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P21ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "See P0x_MODE_REG\\[PID\\]"]
    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, P21ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,P21ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P21ModeReg {
    #[inline(always)]
    fn default() -> P21ModeReg {
        <crate::RegValueT<P21ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P22ModeReg_SPEC;
impl crate::sealed::RegSpec for P22ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "P22 Mode Register"]
pub type P22ModeReg = crate::RegValueT<P22ModeReg_SPEC>;

impl P22ModeReg {
    #[doc = "00 = Input, no resistors selected\n01 = Input, pull-up selected\n10 = Input, Pull-down selected\n11 = Output, no resistors selected\nIn analog mode, these bits are don\'t care"]
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P22ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P22ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "See P0x_MODE_REG\\[PID\\]"]
    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, P22ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,P22ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P22ModeReg {
    #[inline(always)]
    fn default() -> P22ModeReg {
        <crate::RegValueT<P22ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P23ModeReg_SPEC;
impl crate::sealed::RegSpec for P23ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "P23 Mode Register"]
pub type P23ModeReg = crate::RegValueT<P23ModeReg_SPEC>;

impl P23ModeReg {
    #[doc = "00 = Input, no resistors selected\n01 = Input, pull-up selected\n10 = Input, Pull-down selected\n11 = Output, no resistors selected\nIn analog mode, these bits are don\'t care"]
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P23ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P23ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "See P0x_MODE_REG\\[PID\\]"]
    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, P23ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,P23ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P23ModeReg {
    #[inline(always)]
    fn default() -> P23ModeReg {
        <crate::RegValueT<P23ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P24ModeReg_SPEC;
impl crate::sealed::RegSpec for P24ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "P24 Mode Register"]
pub type P24ModeReg = crate::RegValueT<P24ModeReg_SPEC>;

impl P24ModeReg {
    #[doc = "00 = Input, no resistors selected\n01 = Input, pull-up selected\n10 = Input, Pull-down selected\n11 = Output, no resistors selected\nIn analog mode, these bits are don\'t care"]
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P24ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P24ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "See P0x_MODE_REG\\[PID\\]"]
    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, P24ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,P24ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P24ModeReg {
    #[inline(always)]
    fn default() -> P24ModeReg {
        <crate::RegValueT<P24ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P25ModeReg_SPEC;
impl crate::sealed::RegSpec for P25ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "P25 Mode Register"]
pub type P25ModeReg = crate::RegValueT<P25ModeReg_SPEC>;

impl P25ModeReg {
    #[doc = "00 = Input, no resistors selected\n01 = Input, pull-up selected\n10 = Input, Pull-down selected\n11 = Output, no resistors selected\nIn analog mode, these bits are don\'t care"]
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P25ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P25ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "See P0x_MODE_REG\\[PID\\]"]
    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, P25ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,P25ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P25ModeReg {
    #[inline(always)]
    fn default() -> P25ModeReg {
        <crate::RegValueT<P25ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P26ModeReg_SPEC;
impl crate::sealed::RegSpec for P26ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "P26 Mode Register"]
pub type P26ModeReg = crate::RegValueT<P26ModeReg_SPEC>;

impl P26ModeReg {
    #[doc = "00 = Input, no resistors selected\n01 = Input, pull-up selected\n10 = Input, Pull-down selected\n11 = Output, no resistors selected\nIn analog mode, these bits are don\'t care"]
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P26ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P26ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "See P0x_MODE_REG\\[PID\\]"]
    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, P26ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,P26ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P26ModeReg {
    #[inline(always)]
    fn default() -> P26ModeReg {
        <crate::RegValueT<P26ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P27ModeReg_SPEC;
impl crate::sealed::RegSpec for P27ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "P27 Mode Register"]
pub type P27ModeReg = crate::RegValueT<P27ModeReg_SPEC>;

impl P27ModeReg {
    #[doc = "00 = Input, no resistors selected\n01 = Input, pull-up selected\n10 = Input, Pull-down selected\n11 = Output, no resistors selected\nIn analog mode, these bits are don\'t care"]
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P27ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P27ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "See P0x_MODE_REG\\[PID\\]"]
    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, P27ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,P27ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P27ModeReg {
    #[inline(always)]
    fn default() -> P27ModeReg {
        <crate::RegValueT<P27ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P28ModeReg_SPEC;
impl crate::sealed::RegSpec for P28ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "P28 Mode Register"]
pub type P28ModeReg = crate::RegValueT<P28ModeReg_SPEC>;

impl P28ModeReg {
    #[doc = "00 = Input, no resistors selected\n01 = Input, pull-up selected\n10 = Input, Pull-down selected\n11 = Output, no resistors selected\nIn analog mode, these bits are don\'t care"]
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P28ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P28ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "See P0x_MODE_REG\\[PID\\]"]
    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, P28ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,P28ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P28ModeReg {
    #[inline(always)]
    fn default() -> P28ModeReg {
        <crate::RegValueT<P28ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P29ModeReg_SPEC;
impl crate::sealed::RegSpec for P29ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "P29 Mode Register"]
pub type P29ModeReg = crate::RegValueT<P29ModeReg_SPEC>;

impl P29ModeReg {
    #[doc = "00 = Input, no resistors selected\n01 = Input, pull-up selected\n10 = Input, Pull-down selected\n11 = Output, no resistors selected\nIn analog mode, these bits are don\'t care"]
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P29ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P29ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "See P0x_MODE_REG\\[PID\\]"]
    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, P29ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,P29ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P29ModeReg {
    #[inline(always)]
    fn default() -> P29ModeReg {
        <crate::RegValueT<P29ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P2DataReg_SPEC;
impl crate::sealed::RegSpec for P2DataReg_SPEC {
    type DataType = u16;
}

#[doc = "P2 Data input / output register"]
pub type P2DataReg = crate::RegValueT<P2DataReg_SPEC>;

impl P2DataReg {
    #[doc = "Set P2 output register when written; Returns the value of P2 port when read"]
    #[inline(always)]
    pub fn p2_data(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, P2DataReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,P2DataReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P2DataReg {
    #[inline(always)]
    fn default() -> P2DataReg {
        <crate::RegValueT<P2DataReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P2PadpwrCtrlReg_SPEC;
impl crate::sealed::RegSpec for P2PadpwrCtrlReg_SPEC {
    type DataType = u16;
}

#[doc = "Port 2 Output Power Control Register"]
pub type P2PadpwrCtrlReg = crate::RegValueT<P2PadpwrCtrlReg_SPEC>;

impl P2PadpwrCtrlReg {
    #[doc = "1 = P2_x port output is powered by the 1 V rail\n0 = P2_x port output is powered by the 3 V rail\nbit 0 controls the power of P2\\[0\\],\nbit 9 controls the power of P2\\[9\\],"]
    #[inline(always)]
    pub fn p2_out_ctrl(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3ff,
        1,
        0,
        u16,
        u16,
        P2PadpwrCtrlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3ff,
            1,
            0,
            u16,
            u16,
            P2PadpwrCtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P2PadpwrCtrlReg {
    #[inline(always)]
    fn default() -> P2PadpwrCtrlReg {
        <crate::RegValueT<P2PadpwrCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P2ResetDataReg_SPEC;
impl crate::sealed::RegSpec for P2ResetDataReg_SPEC {
    type DataType = u16;
}

#[doc = "P2 Reset port pins register"]
pub type P2ResetDataReg = crate::RegValueT<P2ResetDataReg_SPEC>;

impl P2ResetDataReg {
    #[doc = "Writing a 1 to P2\\[y\\] sets P2\\[y\\] to 0. Writing 0 is discarded;\nReading returns 0"]
    #[inline(always)]
    pub fn p2_reset(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3ff,
        1,
        0,
        u16,
        u16,
        P2ResetDataReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3ff,
            1,
            0,
            u16,
            u16,
            P2ResetDataReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for P2ResetDataReg {
    #[inline(always)]
    fn default() -> P2ResetDataReg {
        <crate::RegValueT<P2ResetDataReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P2SetDataReg_SPEC;
impl crate::sealed::RegSpec for P2SetDataReg_SPEC {
    type DataType = u16;
}

#[doc = "P2 Set port pins register"]
pub type P2SetDataReg = crate::RegValueT<P2SetDataReg_SPEC>;

impl P2SetDataReg {
    #[doc = "Writing a 1 to P2\\[y\\] sets P2\\[y\\] to 1. Writing 0 is discarded;\nReading returns 0"]
    #[inline(always)]
    pub fn p2_set(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, P2SetDataReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,P2SetDataReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P2SetDataReg {
    #[inline(always)]
    fn default() -> P2SetDataReg {
        <crate::RegValueT<P2SetDataReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P30ModeReg_SPEC;
impl crate::sealed::RegSpec for P30ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "P30 Mode Register"]
pub type P30ModeReg = crate::RegValueT<P30ModeReg_SPEC>;

impl P30ModeReg {
    #[doc = "00 = Input, no resistors selected\n01 = Input, pull-up selected\n10 = Input, Pull-down selected\n11 = Output, no resistors selected\nIn ADC mode, these bits are don\'t care"]
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P30ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P30ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "See P0x_MODE_REG\\[PID\\]"]
    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, P30ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,P30ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P30ModeReg {
    #[inline(always)]
    fn default() -> P30ModeReg {
        <crate::RegValueT<P30ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P31ModeReg_SPEC;
impl crate::sealed::RegSpec for P31ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "P31 Mode Register"]
pub type P31ModeReg = crate::RegValueT<P31ModeReg_SPEC>;

impl P31ModeReg {
    #[doc = "00 = Input, no resistors selected\n01 = Input, pull-up selected\n10 = Input, Pull-down selected\n11 = Output, no resistors selected\nIn ADC mode, these bits are don\'t care"]
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P31ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P31ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "See P0x_MODE_REG\\[PID\\]"]
    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, P31ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,P31ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P31ModeReg {
    #[inline(always)]
    fn default() -> P31ModeReg {
        <crate::RegValueT<P31ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P32ModeReg_SPEC;
impl crate::sealed::RegSpec for P32ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "P32 Mode Register"]
pub type P32ModeReg = crate::RegValueT<P32ModeReg_SPEC>;

impl P32ModeReg {
    #[doc = "00 = Input, no resistors selected\n01 = Input, pull-up selected\n10 = Input, Pull-down selected\n11 = Output, no resistors selected\nIn ADC mode, these bits are don\'t care"]
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P32ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P32ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "See P0x_MODE_REG\\[PID\\]"]
    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, P32ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,P32ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P32ModeReg {
    #[inline(always)]
    fn default() -> P32ModeReg {
        <crate::RegValueT<P32ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P33ModeReg_SPEC;
impl crate::sealed::RegSpec for P33ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "P33 Mode Register"]
pub type P33ModeReg = crate::RegValueT<P33ModeReg_SPEC>;

impl P33ModeReg {
    #[doc = "00 = Input, no resistors selected\n01 = Input, pull-up selected\n10 = Input, Pull-down selected\n11 = Output, no resistors selected\nIn ADC mode, these bits are don\'t care"]
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P33ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P33ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "See P0x_MODE_REG\\[PID\\]"]
    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, P33ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,P33ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P33ModeReg {
    #[inline(always)]
    fn default() -> P33ModeReg {
        <crate::RegValueT<P33ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P34ModeReg_SPEC;
impl crate::sealed::RegSpec for P34ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "P34 Mode Register"]
pub type P34ModeReg = crate::RegValueT<P34ModeReg_SPEC>;

impl P34ModeReg {
    #[doc = "00 = Input, no resistors selected\n01 = Input, pull-up selected\n10 = Input, Pull-down selected\n11 = Output, no resistors selected\nIn ADC mode, these bits are don\'t care"]
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P34ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P34ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "See P0x_MODE_REG\\[PID\\]"]
    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, P34ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,P34ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P34ModeReg {
    #[inline(always)]
    fn default() -> P34ModeReg {
        <crate::RegValueT<P34ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P35ModeReg_SPEC;
impl crate::sealed::RegSpec for P35ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "P35 Mode Register"]
pub type P35ModeReg = crate::RegValueT<P35ModeReg_SPEC>;

impl P35ModeReg {
    #[doc = "00 = Input, no resistors selected\n01 = Input, pull-up selected\n10 = Input, Pull-down selected\n11 = Output, no resistors selected\nIn ADC mode, these bits are don\'t care"]
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P35ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P35ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "See P0x_MODE_REG\\[PID\\]"]
    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, P35ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,P35ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P35ModeReg {
    #[inline(always)]
    fn default() -> P35ModeReg {
        <crate::RegValueT<P35ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P36ModeReg_SPEC;
impl crate::sealed::RegSpec for P36ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "P36 Mode Register"]
pub type P36ModeReg = crate::RegValueT<P36ModeReg_SPEC>;

impl P36ModeReg {
    #[doc = "00 = Input, no resistors selected\n01 = Input, pull-up selected\n10 = Input, Pull-down selected\n11 = Output, no resistors selected\nIn ADC mode, these bits are don\'t care"]
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P36ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P36ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "See P0x_MODE_REG\\[PID\\]"]
    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, P36ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,P36ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P36ModeReg {
    #[inline(always)]
    fn default() -> P36ModeReg {
        <crate::RegValueT<P36ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P37ModeReg_SPEC;
impl crate::sealed::RegSpec for P37ModeReg_SPEC {
    type DataType = u16;
}

#[doc = "P37 Mode Register"]
pub type P37ModeReg = crate::RegValueT<P37ModeReg_SPEC>;

impl P37ModeReg {
    #[doc = "00 = Input, no resistors selected\n01 = Input, pull-up selected\n10 = Input, Pull-down selected\n11 = Output, no resistors selected\nIn ADC mode, these bits are don\'t care"]
    #[inline(always)]
    pub fn pupd(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, u8, P37ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8,u8,P37ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "See P0x_MODE_REG\\[PID\\]"]
    #[inline(always)]
    pub fn pid(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, P37ModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,P37ModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P37ModeReg {
    #[inline(always)]
    fn default() -> P37ModeReg {
        <crate::RegValueT<P37ModeReg_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P3DataReg_SPEC;
impl crate::sealed::RegSpec for P3DataReg_SPEC {
    type DataType = u16;
}

#[doc = "P3 Data input / output register"]
pub type P3DataReg = crate::RegValueT<P3DataReg_SPEC>;

impl P3DataReg {
    #[doc = "Set P3 output register when written; Returns the value of P3 port when read"]
    #[inline(always)]
    pub fn p3_data(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, P3DataReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,P3DataReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P3DataReg {
    #[inline(always)]
    fn default() -> P3DataReg {
        <crate::RegValueT<P3DataReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P3PadpwrCtrlReg_SPEC;
impl crate::sealed::RegSpec for P3PadpwrCtrlReg_SPEC {
    type DataType = u16;
}

#[doc = "Port 3 Output Power Control Register"]
pub type P3PadpwrCtrlReg = crate::RegValueT<P3PadpwrCtrlReg_SPEC>;

impl P3PadpwrCtrlReg {
    #[doc = "1 = P3_x port output is powered by the 1 V rail\n0 = P3_x port output is powered by the 3 V rail\nbit 0 controls the power of P3\\[0\\],\nbit 7 controls the power of P3\\[7\\],"]
    #[inline(always)]
    pub fn p3_out_ctrl(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, P3PadpwrCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,P3PadpwrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for P3PadpwrCtrlReg {
    #[inline(always)]
    fn default() -> P3PadpwrCtrlReg {
        <crate::RegValueT<P3PadpwrCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P3ResetDataReg_SPEC;
impl crate::sealed::RegSpec for P3ResetDataReg_SPEC {
    type DataType = u16;
}

#[doc = "P3 Reset port pins register"]
pub type P3ResetDataReg = crate::RegValueT<P3ResetDataReg_SPEC>;

impl P3ResetDataReg {
    #[doc = "Writing a 1 to P0\\[y\\] sets P0\\[y\\] to 0. Writing 0 is discarded;\nReading returns 0"]
    #[inline(always)]
    pub fn p3_reset(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, P3ResetDataReg_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,P3ResetDataReg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for P3ResetDataReg {
    #[inline(always)]
    fn default() -> P3ResetDataReg {
        <crate::RegValueT<P3ResetDataReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P3SetDataReg_SPEC;
impl crate::sealed::RegSpec for P3SetDataReg_SPEC {
    type DataType = u16;
}

#[doc = "P3 Set port pins register"]
pub type P3SetDataReg = crate::RegValueT<P3SetDataReg_SPEC>;

impl P3SetDataReg {
    #[doc = "Writing a 1 to P3\\[y\\] sets P3\\[y\\] to 1. Writing 0 is discarded; Reading returns 0"]
    #[inline(always)]
    pub fn p3_set(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, P3SetDataReg_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,P3SetDataReg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for P3SetDataReg {
    #[inline(always)]
    fn default() -> P3SetDataReg {
        <crate::RegValueT<P3SetDataReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
