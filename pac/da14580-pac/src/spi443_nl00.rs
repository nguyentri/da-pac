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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:15:24 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"spi443_nl00 registers"]
unsafe impl ::core::marker::Send for super::Spi443Nl00 {}
unsafe impl ::core::marker::Sync for super::Spi443Nl00 {}
impl super::Spi443Nl00 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn spi_clear_int_reg(
        &self,
    ) -> &'static crate::common::Reg<self::SpiClearIntReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SpiClearIntReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(6usize),
            )
        }
    }

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

    #[inline(always)]
    pub const fn spi_ctrl_reg1(
        &self,
    ) -> &'static crate::common::Reg<self::SpiCtrlReg1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SpiCtrlReg1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn spi_rx_tx_reg0(
        &self,
    ) -> &'static crate::common::Reg<self::SpiRxTxReg0_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SpiRxTxReg0_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(2usize),
            )
        }
    }

    #[inline(always)]
    pub const fn spi_rx_tx_reg1(
        &self,
    ) -> &'static crate::common::Reg<self::SpiRxTxReg1_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::SpiRxTxReg1_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SpiClearIntReg_SPEC;
impl crate::sealed::RegSpec for SpiClearIntReg_SPEC {
    type DataType = u16;
}

pub type SpiClearIntReg = crate::RegValueT<SpiClearIntReg_SPEC>;

impl SpiClearIntReg {
    #[inline(always)]
    pub fn spi_clear_int(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        SpiClearIntReg_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            SpiClearIntReg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for SpiClearIntReg {
    #[inline(always)]
    fn default() -> SpiClearIntReg {
        <crate::RegValueT<SpiClearIntReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SpiCtrlReg_SPEC;
impl crate::sealed::RegSpec for SpiCtrlReg_SPEC {
    type DataType = u16;
}

pub type SpiCtrlReg = crate::RegValueT<SpiCtrlReg_SPEC>;

impl SpiCtrlReg {
    #[inline(always)]
    pub fn spi_en_ctrl(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, SpiCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,SpiCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn spi_mint(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, SpiCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,SpiCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn spi_int_bit(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, SpiCtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13,1,0,SpiCtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn spi_di(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, SpiCtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12,1,0,SpiCtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn spi_txh(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, SpiCtrlReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11,1,0,SpiCtrlReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn spi_force_do(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, SpiCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,SpiCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn spi_rst(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, SpiCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,SpiCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn spi_word(
        self,
    ) -> crate::common::RegisterField<7, 0x3, 1, 0, u8, u8, SpiCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x3,1,0,u8,u8,SpiCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn spi_smn(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, SpiCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,SpiCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn spi_do(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, SpiCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,SpiCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn spi_clk(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, u8, SpiCtrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x3,1,0,u8,u8,SpiCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn spi_pol(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, SpiCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,SpiCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn spi_pha(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, SpiCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,SpiCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn spi_on(
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
pub struct SpiCtrlReg1_SPEC;
impl crate::sealed::RegSpec for SpiCtrlReg1_SPEC {
    type DataType = u16;
}

pub type SpiCtrlReg1 = crate::RegValueT<SpiCtrlReg1_SPEC>;

impl SpiCtrlReg1 {
    #[inline(always)]
    pub fn spi_9bit_val(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, SpiCtrlReg1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,SpiCtrlReg1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn spi_busy(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, SpiCtrlReg1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,SpiCtrlReg1_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn spi_priority(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, SpiCtrlReg1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,SpiCtrlReg1_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn spi_fifo_mode(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, SpiCtrlReg1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,SpiCtrlReg1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for SpiCtrlReg1 {
    #[inline(always)]
    fn default() -> SpiCtrlReg1 {
        <crate::RegValueT<SpiCtrlReg1_SPEC> as RegisterValue<_>>::new(3)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SpiRxTxReg0_SPEC;
impl crate::sealed::RegSpec for SpiRxTxReg0_SPEC {
    type DataType = u16;
}

pub type SpiRxTxReg0 = crate::RegValueT<SpiRxTxReg0_SPEC>;

impl SpiRxTxReg0 {
    #[inline(always)]
    pub fn spi_data0(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, SpiRxTxReg0_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,SpiRxTxReg0_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for SpiRxTxReg0 {
    #[inline(always)]
    fn default() -> SpiRxTxReg0 {
        <crate::RegValueT<SpiRxTxReg0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SpiRxTxReg1_SPEC;
impl crate::sealed::RegSpec for SpiRxTxReg1_SPEC {
    type DataType = u16;
}

pub type SpiRxTxReg1 = crate::RegValueT<SpiRxTxReg1_SPEC>;

impl SpiRxTxReg1 {
    #[inline(always)]
    pub fn spi_data1(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, SpiRxTxReg1_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,SpiRxTxReg1_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for SpiRxTxReg1 {
    #[inline(always)]
    fn default() -> SpiRxTxReg1 {
        <crate::RegValueT<SpiRxTxReg1_SPEC> as RegisterValue<_>>::new(0)
    }
}
