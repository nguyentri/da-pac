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
#[doc = r"SPI registers"]
unsafe impl ::core::marker::Send for super::Spi {}
unsafe impl ::core::marker::Sync for super::Spi {}
impl super::Spi {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Spi clock register"]
    #[inline(always)]
    pub const fn spi_clock_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SpiClockReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SpiClockReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Spi control register"]
    #[inline(always)]
    pub const fn spi_config_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SpiConfigReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SpiConfigReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Spi cs configuration register"]
    #[inline(always)]
    pub const fn spi_cs_config_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SpiCsConfigReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SpiCsConfigReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[doc = "Spi control register"]
    #[inline(always)]
    pub const fn spi_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SpiCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SpiCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Spi fifo configuration register"]
    #[inline(always)]
    pub const fn spi_fifo_config_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SpiFifoConfigReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SpiFifoConfigReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Spi TX/RX High 16bit word"]
    #[inline(always)]
    pub const fn spi_fifo_high_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SpiFifoHighReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SpiFifoHighReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[doc = "Spi RX fifo read register"]
    #[inline(always)]
    pub const fn spi_fifo_read_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SpiFifoReadReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SpiFifoReadReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "SPI RX/TX fifo status register"]
    #[inline(always)]
    pub const fn spi_fifo_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SpiFifoStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SpiFifoStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "Spi TX fifo wtite register"]
    #[inline(always)]
    pub const fn spi_fifo_write_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SpiFifoWriteReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SpiFifoWriteReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "Spi interrupt mask register"]
    #[inline(always)]
    pub const fn spi_irq_mask_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SpiIrqMaskReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SpiIrqMaskReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Spi status register"]
    #[inline(always)]
    pub const fn spi_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SpiStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SpiStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "SPI TX buffer force high value"]
    #[inline(always)]
    pub const fn spi_txbuffer_force_h_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SpiTxbufferForceHReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SpiTxbufferForceHReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[doc = "SPI TX buffer force low value"]
    #[inline(always)]
    pub const fn spi_txbuffer_force_l_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SpiTxbufferForceLReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SpiTxbufferForceLReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SpiClockReg_SPEC;
impl crate::sealed::RegSpec for SpiClockReg_SPEC {
    type DataType = u16;
}

#[doc = "Spi clock register"]
pub type SpiClockReg = crate::RegValueT<SpiClockReg_SPEC>;

impl SpiClockReg {
    #[doc = "Should be always 1"]
    #[inline(always)]
    pub fn spi_master_clk_mode(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, SpiClockReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,SpiClockReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Applicable only in master mode\nDefines the spi clock frequency in master only mode\nSPI_CLK = module_clk / 2*(SPI_CLK_DIV+1) when SPI_CLK_DIV not 0x7F\nif SPI_CLK_DIV=0x7F then SPI_CLK=module_clk"]
    #[inline(always)]
    pub fn spi_clk_div(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, u8, SpiClockReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7f,1,0,u8,u8,SpiClockReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for SpiClockReg {
    #[inline(always)]
    fn default() -> SpiClockReg {
        <crate::RegValueT<SpiClockReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SpiConfigReg_SPEC;
impl crate::sealed::RegSpec for SpiConfigReg_SPEC {
    type DataType = u16;
}

#[doc = "Spi control register"]
pub type SpiConfigReg = crate::RegValueT<SpiConfigReg_SPEC>;

impl SpiConfigReg {
    #[doc = "0 = SPI module master mode\n1 = SPI module slave mode"]
    #[inline(always)]
    pub fn spi_slave_en(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, SpiConfigReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,SpiConfigReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Define the spi word length = 1+ SPI_WORD_LENGTH (range 4 to 32)"]
    #[inline(always)]
    pub fn spi_word_length(
        self,
    ) -> crate::common::RegisterField<2, 0x1f, 1, 0, u8, u8, SpiConfigReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1f,1,0,u8,u8,SpiConfigReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Define the spi mode (CPOL, CPHA)\n0 = new data on falling, capture on rising, clk low in idle state\n1 = new data on rising, capture on falling, Clk low in idle state\n2 = new data on rising, capture on falling, Clk high in idle state\n3 = new data on falling, capture on rising Clk high in idle state"]
    #[inline(always)]
    pub fn spi_mode(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, SpiConfigReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,SpiConfigReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for SpiConfigReg {
    #[inline(always)]
    fn default() -> SpiConfigReg {
        <crate::RegValueT<SpiConfigReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SpiCsConfigReg_SPEC;
impl crate::sealed::RegSpec for SpiCsConfigReg_SPEC {
    type DataType = u16;
}

#[doc = "Spi cs configuration register"]
pub type SpiCsConfigReg = crate::RegValueT<SpiCsConfigReg_SPEC>;

impl SpiCsConfigReg {
    #[doc = "Control the cs output in master mode\n0 = none slave device selected\n1 = selected slave device connected to GPIO with FUNC_MODE=SPI_CS0\n2 = selected slave device connected to GPIO with FUNC_MODE=SPI_CS1\n4 = selected slave device connected to GPIO with FUNC_MODE=GPIO"]
    #[inline(always)]
    pub fn spi_cs_select(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, SpiCsConfigReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,SpiCsConfigReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for SpiCsConfigReg {
    #[inline(always)]
    fn default() -> SpiCsConfigReg {
        <crate::RegValueT<SpiCsConfigReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SpiCtrlReg_SPEC;
impl crate::sealed::RegSpec for SpiCtrlReg_SPEC {
    type DataType = u16;
}

#[doc = "Spi control register"]
pub type SpiCtrlReg = crate::RegValueT<SpiCtrlReg_SPEC>;

impl SpiCtrlReg {
    #[doc = "0 = normal operation\n1 = LSB and MSB are swaped in APB interface\nIn case of 8bit spi interface, DMA/SPI can be configured in 16bit mode to off load the bus. Enabling SPI_SWAP_BYTES bytes will read/wrte correctly"]
    #[inline(always)]
    pub fn spi_swap_bytes(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, SpiCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,SpiCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = SPI captures data at correct clock edge\n1 = SPI captures data at next clock edge. (only for Master mode and high clock)"]
    #[inline(always)]
    pub fn spi_capture_at_next_edge(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, SpiCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,SpiCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = Fifo normal operation\n1 = Fifo in reset state"]
    #[inline(always)]
    pub fn spi_fifo_reset(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, SpiCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,SpiCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "applicable only when SPI_RX_EN=1\n0 = No DMA request for RX\n1 = DMA request when SPI_STATUS_RX_FULL=\'1\'"]
    #[inline(always)]
    pub fn spi_dma_rx_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, SpiCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,SpiCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "applicable only when SPI_TX_EN=1\n0 = No DMA request for TX\n1 = DMA request when SPI_STATUS_TX_EMPTY=\'1\'"]
    #[inline(always)]
    pub fn spi_dma_tx_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, SpiCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,SpiCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = RX path is disabled\n1 = RX path is enabled\nNote: if master clk async or spi mode=1 or spi mode=3 readonly is not supported"]
    #[inline(always)]
    pub fn spi_rx_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, SpiCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,SpiCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = TX path is disabled\n1 = TX path is enabled"]
    #[inline(always)]
    pub fn spi_tx_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, SpiCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,SpiCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = SPI module is disable\n1 = SPI module is enable"]
    #[inline(always)]
    pub fn spi_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, SpiCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,SpiCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for SpiCtrlReg {
    #[inline(always)]
    fn default() -> SpiCtrlReg {
        <crate::RegValueT<SpiCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SpiFifoConfigReg_SPEC;
impl crate::sealed::RegSpec for SpiFifoConfigReg_SPEC {
    type DataType = u16;
}

#[doc = "Spi fifo configuration register"]
pub type SpiFifoConfigReg = crate::RegValueT<SpiFifoConfigReg_SPEC>;

impl SpiFifoConfigReg {
    #[doc = "Receive FIFO threshold level in bytes. Control the level of bytes in fifo that triggers the RX_FULL interrupt. IRQ is occurred when fifo level is more or equal to SPI_RX_TL+1. Fifo level is from 0 to 4"]
    #[inline(always)]
    pub fn spi_rx_tl(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, u8, SpiFifoConfigReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0xf,1,0,u8,u8,SpiFifoConfigReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Transmit FIFO threshold level in bytes. Control the level of bytes in fifo that triggers the TX_EMPTY interrupt. IRQ is occurred when fifo level is less or equal to SPI_TX_TL. Fifo level is from 0 to 4"]
    #[inline(always)]
    pub fn spi_tx_tl(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, SpiFifoConfigReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,SpiFifoConfigReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for SpiFifoConfigReg {
    #[inline(always)]
    fn default() -> SpiFifoConfigReg {
        <crate::RegValueT<SpiFifoConfigReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SpiFifoHighReg_SPEC;
impl crate::sealed::RegSpec for SpiFifoHighReg_SPEC {
    type DataType = u16;
}

#[doc = "Spi TX/RX High 16bit word"]
pub type SpiFifoHighReg = crate::RegValueT<SpiFifoHighReg_SPEC>;

impl SpiFifoHighReg {
    #[doc = "RX/TX fifo data. 16 MSb when spi word is larger than 16bits\nThis register has to be written before the SPI_FIFO_WRITE_REG\nThis register has to be read after the SPI_FIFO_READ_REG"]
    #[inline(always)]
    pub fn spi_fifo_high(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        SpiFifoHighReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            SpiFifoHighReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SpiFifoHighReg {
    #[inline(always)]
    fn default() -> SpiFifoHighReg {
        <crate::RegValueT<SpiFifoHighReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SpiFifoReadReg_SPEC;
impl crate::sealed::RegSpec for SpiFifoReadReg_SPEC {
    type DataType = u16;
}

#[doc = "Spi RX fifo read register"]
pub type SpiFifoReadReg = crate::RegValueT<SpiFifoReadReg_SPEC>;

impl SpiFifoReadReg {
    #[doc = "Read from RX fifo. Read access is permit only if SPI_STATUS_RX_EMPTY=0. Returns the 16 LSb"]
    #[inline(always)]
    pub fn spi_fifo_read(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        SpiFifoReadReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            SpiFifoReadReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SpiFifoReadReg {
    #[inline(always)]
    fn default() -> SpiFifoReadReg {
        <crate::RegValueT<SpiFifoReadReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SpiFifoStatusReg_SPEC;
impl crate::sealed::RegSpec for SpiFifoStatusReg_SPEC {
    type DataType = u16;
}

#[doc = "SPI RX/TX fifo status register"]
pub type SpiFifoStatusReg = crate::RegValueT<SpiFifoStatusReg_SPEC>;

impl SpiFifoStatusReg {
    #[doc = "In master mode\n0 = spi transaction is inactive\n1 = spi transaction is active"]
    #[inline(always)]
    pub fn spi_transaction_active(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, SpiFifoStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15,1,0,SpiFifoStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "When 1, receive data is not written to fifo because fifo was full and interrupt is generated. It clears with SPI_CTRL_REG.SPI_FIFO_RESET"]
    #[inline(always)]
    pub fn spi_rx_fifo_ovfl(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, SpiFifoStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14,1,0,SpiFifoStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "0 = TX fifo is not full\n1 = TX fifo is full"]
    #[inline(always)]
    pub fn spi_status_tx_full(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, SpiFifoStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13,1,0,SpiFifoStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "0 = RX fifo is not empty\n1 = RX fifo is empty"]
    #[inline(always)]
    pub fn spi_status_rx_empty(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, SpiFifoStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12,1,0,SpiFifoStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Gives the number of bytes in TX fifo"]
    #[inline(always)]
    pub fn spi_tx_fifo_level(
        self,
    ) -> crate::common::RegisterField<6, 0x3f, 1, 0, u8, u8, SpiFifoStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<6,0x3f,1,0,u8,u8,SpiFifoStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Gives the number of bytes in RX fifo"]
    #[inline(always)]
    pub fn spi_rx_fifo_level(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, u8, SpiFifoStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8,u8,SpiFifoStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for SpiFifoStatusReg {
    #[inline(always)]
    fn default() -> SpiFifoStatusReg {
        <crate::RegValueT<SpiFifoStatusReg_SPEC> as RegisterValue<_>>::new(4096)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SpiFifoWriteReg_SPEC;
impl crate::sealed::RegSpec for SpiFifoWriteReg_SPEC {
    type DataType = u16;
}

#[doc = "Spi TX fifo wtite register"]
pub type SpiFifoWriteReg = crate::RegValueT<SpiFifoWriteReg_SPEC>;

impl SpiFifoWriteReg {
    #[doc = "Write to TX fifo. Write access is permit only if SPI_STATUS_TX_FULL is 0"]
    #[inline(always)]
    pub fn spi_fifo_write(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        SpiFifoWriteReg_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            SpiFifoWriteReg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SpiFifoWriteReg {
    #[inline(always)]
    fn default() -> SpiFifoWriteReg {
        <crate::RegValueT<SpiFifoWriteReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SpiIrqMaskReg_SPEC;
impl crate::sealed::RegSpec for SpiIrqMaskReg_SPEC {
    type DataType = u16;
}

#[doc = "Spi interrupt mask register"]
pub type SpiIrqMaskReg = crate::RegValueT<SpiIrqMaskReg_SPEC>;

impl SpiIrqMaskReg {
    #[doc = "0 = FIFO RX full irq is masked\n1 = FIFO RX full irq is enabled"]
    #[inline(always)]
    pub fn spi_irq_mask_rx_full(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, SpiIrqMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,SpiIrqMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "0 = FIFO TX empty irq is masked\n1 = FIFO TX empy irq is enabled"]
    #[inline(always)]
    pub fn spi_irq_mask_tx_empty(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, SpiIrqMaskReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,SpiIrqMaskReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for SpiIrqMaskReg {
    #[inline(always)]
    fn default() -> SpiIrqMaskReg {
        <crate::RegValueT<SpiIrqMaskReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SpiStatusReg_SPEC;
impl crate::sealed::RegSpec for SpiStatusReg_SPEC {
    type DataType = u16;
}

#[doc = "Spi status register"]
pub type SpiStatusReg = crate::RegValueT<SpiStatusReg_SPEC>;

impl SpiStatusReg {
    #[doc = "Auto clear\n0 = RX fifo level is less than SPI_RX_TL+1\n1 = RX fifo level is more or equal to SPI_RX_TL+1"]
    #[inline(always)]
    pub fn spi_status_rx_full(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, SpiStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,SpiStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Auto clear\n0 = TX fifo level is larger than SPI_TX_TL\n1 = TX fifo level is less or equal to SPI_TX_TL"]
    #[inline(always)]
    pub fn spi_status_tx_empty(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, SpiStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,SpiStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for SpiStatusReg {
    #[inline(always)]
    fn default() -> SpiStatusReg {
        <crate::RegValueT<SpiStatusReg_SPEC> as RegisterValue<_>>::new(1)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SpiTxbufferForceHReg_SPEC;
impl crate::sealed::RegSpec for SpiTxbufferForceHReg_SPEC {
    type DataType = u16;
}

#[doc = "SPI TX buffer force high value"]
pub type SpiTxbufferForceHReg = crate::RegValueT<SpiTxbufferForceHReg_SPEC>;

impl SpiTxbufferForceHReg {
    #[doc = "Write directly the tx buffer (2 MSB). It must to be used only in slave mode.\nThis register has to be written before the SPI_FIFO_WRITE_REG"]
    #[inline(always)]
    pub fn spi_txbuffer_force_h(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        SpiTxbufferForceHReg_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            SpiTxbufferForceHReg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SpiTxbufferForceHReg {
    #[inline(always)]
    fn default() -> SpiTxbufferForceHReg {
        <crate::RegValueT<SpiTxbufferForceHReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SpiTxbufferForceLReg_SPEC;
impl crate::sealed::RegSpec for SpiTxbufferForceLReg_SPEC {
    type DataType = u16;
}

#[doc = "SPI TX buffer force low value"]
pub type SpiTxbufferForceLReg = crate::RegValueT<SpiTxbufferForceLReg_SPEC>;

impl SpiTxbufferForceLReg {
    #[doc = "Write directly the tx buffer (2 LSB). It must to be used only in slave mode"]
    #[inline(always)]
    pub fn spi_txbuffer_force_l(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        SpiTxbufferForceLReg_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            SpiTxbufferForceLReg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SpiTxbufferForceLReg {
    #[inline(always)]
    fn default() -> SpiTxbufferForceLReg {
        <crate::RegValueT<SpiTxbufferForceLReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
