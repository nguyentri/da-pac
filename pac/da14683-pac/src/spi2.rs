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
// Generated from SVD 1.2, with svd2pac 0.4.0 on Sat, 12 Apr 2025 22:14:22 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"SPI2 registers"]
unsafe impl ::core::marker::Send for super::Spi2 {}
unsafe impl ::core::marker::Sync for super::Spi2 {}
impl super::Spi2 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    #[doc = "SPI clear interrupt register"]
    #[inline(always)]
    pub const fn spi2_clear_int_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Spi2ClearIntReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Spi2ClearIntReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

    #[doc = "SPI control register 0"]
    #[inline(always)]
    pub const fn spi2_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Spi2CtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Spi2CtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "SPI control register 1"]
    #[inline(always)]
    pub const fn spi2_ctrl_reg1(
        &self,
    ) -> &'static crate::common::Reg<self::Spi2CtrlReg1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Spi2CtrlReg1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "SPI RX/TX register0"]
    #[inline(always)]
    pub const fn spi2_rx_tx_reg0(
        &self,
    ) -> &'static crate::common::Reg<self::Spi2RxTxReg0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Spi2RxTxReg0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[doc = "SPI RX/TX register1"]
    #[inline(always)]
    pub const fn spi2_rx_tx_reg1(
        &self,
    ) -> &'static crate::common::Reg<self::Spi2RxTxReg1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Spi2RxTxReg1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spi2ClearIntReg_SPEC;
impl crate::sealed::RegSpec for Spi2ClearIntReg_SPEC {
    type DataType = u16;
}
#[doc = "SPI clear interrupt register"]
pub type Spi2ClearIntReg = crate::RegValueT<Spi2ClearIntReg_SPEC>;

impl Spi2ClearIntReg {
    #[doc = "Writing any value to this register will clear the SPI_CTRL_REG\\[SPI_INT_BIT\\]\nReading returns 0."]
    #[inline(always)]
    pub fn spi_clear_int(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Spi2ClearIntReg_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Spi2ClearIntReg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Spi2ClearIntReg {
    #[inline(always)]
    fn default() -> Spi2ClearIntReg {
        <crate::RegValueT<Spi2ClearIntReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spi2CtrlReg_SPEC;
impl crate::sealed::RegSpec for Spi2CtrlReg_SPEC {
    type DataType = u16;
}
#[doc = "SPI control register 0"]
pub type Spi2CtrlReg = crate::RegValueT<Spi2CtrlReg_SPEC>;

impl Spi2CtrlReg {
    #[doc = "0 = SPI_EN pin disabled in slave mode. Pin SPI_EN is don\'t care.\n1 = SPI_EN pin enabled in slave mode."]
    #[inline(always)]
    pub fn spi_en_ctrl(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Spi2CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,Spi2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0 = Disable SPI_INT_BIT to ICU\n1 = Enable SPI_INT_BIT to ICU.\nNote that the SPI_INT interrupt is shared with AD_INT interrupt"]
    #[inline(always)]
    pub fn spi_mint(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Spi2CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,Spi2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0 = RX Register or FIFO is empty.\n1 = SPI interrupt. Data has been transmitted and receivedMust be reset by SW by writing to SPI_CLEAR_INT_REG."]
    #[inline(always)]
    pub fn spi_int_bit(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Spi2CtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13,1,0,Spi2CtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Returns the actual value of pin SPI_DIN (delayed with two internal SPI clock cycles)"]
    #[inline(always)]
    pub fn spi_di(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Spi2CtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12,1,0,Spi2CtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "0 = TX-FIFO is not full, data can be written.\n1 = TX-FIFO is full, data can not be written."]
    #[inline(always)]
    pub fn spi_txh(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Spi2CtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11,1,0,Spi2CtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "0 = normal operation\n1 = Force SPIDO output level to value of SPI_DO."]
    #[inline(always)]
    pub fn spi_force_do(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Spi2CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,Spi2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0 = normal operation\n1 = Reset SPI. Same function as SPI_ON except that internal clock remain active."]
    #[inline(always)]
    pub fn spi_rst(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Spi2CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,Spi2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "00 = 8 bits mode, only SPI_RX_TX_REG0 used\n01 = 16 bit mode, only SPI_RX_TX_REG0 used\n10 = 32 bits mode, SPI_RX_TX_REG0 & SPI_RX_TX_REG1 used\n11 = 9 bits mode. Only valid in master mode."]
    #[inline(always)]
    pub fn spi_word(
        self,
    ) -> crate::common::RegisterField<7, 0x3, 1, 0, u8, Spi2CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<7,0x3,1,0,u8, Spi2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Master/slave mode\n0 = Master,\n1 = Slave(SPI1 only)"]
    #[inline(always)]
    pub fn spi_smn(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Spi2CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,Spi2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pin SPI_DO output level when SPI is idle or when SPI_FORCE_DO=1"]
    #[inline(always)]
    pub fn spi_do(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Spi2CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,Spi2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select SPI_CLK clock output frequency in master mode:\n00 = SPI_CLK / 8\n01 = SPI_CLK / 4\n10 = SPI_CLK / 2\n11 = SPI_CLK / 14"]
    #[inline(always)]
    pub fn spi_clk(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, Spi2CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x3,1,0,u8, Spi2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select SPI_CLK polarity.\n0 = SPI_CLK is initially low.\n1 = SPI_CLK is initially high."]
    #[inline(always)]
    pub fn spi_pol(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Spi2CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,Spi2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select SPI_CLK phase. See functional timing diagrams in SPI chapter"]
    #[inline(always)]
    pub fn spi_pha(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Spi2CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,Spi2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0 = SPI Module switched off (power saving). Everything is reset except SPI_CTRL_REG0 and SPI_CTRL_REG1. When this bit is cleared the SPI will remain active in master mode until the shift register and holding register are both empty.\n1 = SPI Module switched on. Should only be set after all control bits have their desired values. So two writes are needed!"]
    #[inline(always)]
    pub fn spi_on(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Spi2CtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Spi2CtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Spi2CtrlReg {
    #[inline(always)]
    fn default() -> Spi2CtrlReg {
        <crate::RegValueT<Spi2CtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spi2CtrlReg1_SPEC;
impl crate::sealed::RegSpec for Spi2CtrlReg1_SPEC {
    type DataType = u16;
}
#[doc = "SPI control register 1"]
pub type Spi2CtrlReg1 = crate::RegValueT<Spi2CtrlReg1_SPEC>;

impl Spi2CtrlReg1 {
    #[doc = "Determines the value of the first bit in 9 bits SPI mode."]
    #[inline(always)]
    pub fn spi_9bit_val(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Spi2CtrlReg1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,Spi2CtrlReg1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0 = The SPI is not busy with a transfer. This means that either no TX-data is available or that the transfers have been suspended due to a full RX-FIFO. The SPIx_CTRL_REG0\\[SPI_INT_BIT\\] can be used to distinguish between these situations.\n1 = The SPI is busy with a transfer."]
    #[inline(always)]
    pub fn spi_busy(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Spi2CtrlReg1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,Spi2CtrlReg1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "0 = The SPI has low priority, the DMA request signals are reset after the corresponding acknowledge.\n1 = The SPI has high priority, DMA request signals remain\nactive until the FIFOS are filled/emptied, so the DMA holds the AHB bus."]
    #[inline(always)]
    pub fn spi_priority(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Spi2CtrlReg1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,Spi2CtrlReg1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "0: TX-FIFO and RX-FIFO used (Bidirectional mode).\n1: RX-FIFO used (Read Only Mode) TX-FIFO single depth, no flow control\n2: TX-FIFO used (Write Only Mode), RX-FIFO single depth, no flow control\n3: No FIFOs used (backwards compatible mode)"]
    #[inline(always)]
    pub fn spi_fifo_mode(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Spi2CtrlReg1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, Spi2CtrlReg1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Spi2CtrlReg1 {
    #[inline(always)]
    fn default() -> Spi2CtrlReg1 {
        <crate::RegValueT<Spi2CtrlReg1_SPEC> as RegisterValue<_>>::new(3)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spi2RxTxReg0_SPEC;
impl crate::sealed::RegSpec for Spi2RxTxReg0_SPEC {
    type DataType = u16;
}
#[doc = "SPI RX/TX register0"]
pub type Spi2RxTxReg0 = crate::RegValueT<Spi2RxTxReg0_SPEC>;

impl Spi2RxTxReg0 {
    #[doc = "Write: SPI_TX_REG0 output register 0 (TX-FIFO)\nRead: SPI_RX_REG0 input register 0 (RX-FIFO)\nIn 8 or 9 bits mode bits 15 to 8 are not used, they contain old data."]
    #[inline(always)]
    pub fn spi_data0(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Spi2RxTxReg0_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Spi2RxTxReg0_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Spi2RxTxReg0 {
    #[inline(always)]
    fn default() -> Spi2RxTxReg0 {
        <crate::RegValueT<Spi2RxTxReg0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spi2RxTxReg1_SPEC;
impl crate::sealed::RegSpec for Spi2RxTxReg1_SPEC {
    type DataType = u16;
}
#[doc = "SPI RX/TX register1"]
pub type Spi2RxTxReg1 = crate::RegValueT<Spi2RxTxReg1_SPEC>;

impl Spi2RxTxReg1 {
    #[doc = "Write: SPI_TX_REG1 output register 1 (MSB\'s of TX-FIFO)\nRead: SPI_RX_REG1 input register 1 (MSB\'s of RX-FIFO)\nIn 8 or 9 or 16 bits mode bits this register is not used."]
    #[inline(always)]
    pub fn spi_data1(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Spi2RxTxReg1_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Spi2RxTxReg1_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Spi2RxTxReg1 {
    #[inline(always)]
    fn default() -> Spi2RxTxReg1 {
        <crate::RegValueT<Spi2RxTxReg1_SPEC> as RegisterValue<_>>::new(0)
    }
}
