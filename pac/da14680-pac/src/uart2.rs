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
// Generated from SVD 1.2, with svd2pac 0.4.0 on Sat, 12 Apr 2025 22:14:08 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"UART2 registers"]
unsafe impl ::core::marker::Send for super::Uart2 {}
unsafe impl ::core::marker::Sync for super::Uart2 {}
impl super::Uart2 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    #[doc = "Component Parameter Register"]
    #[inline(always)]
    pub const fn uart2_cpr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart2CprReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart2CprReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(244usize),
            )
        }
    }

    #[doc = "Component Type Register"]
    #[inline(always)]
    pub const fn uart2_ctr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart2CtrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart2CtrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(252usize),
            )
        }
    }

    #[doc = "Divisor Latch Fraction Register"]
    #[inline(always)]
    pub const fn uart2_dlf_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart2DlfReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart2DlfReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(192usize),
            )
        }
    }

    #[doc = "DMA Software Acknowledge"]
    #[inline(always)]
    pub const fn uart2_dmasa_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart2DmasaReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart2DmasaReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(168usize),
            )
        }
    }

    #[doc = "FIFO Access Register"]
    #[inline(always)]
    pub const fn uart2_far_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart2FarReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart2FarReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(112usize),
            )
        }
    }

    #[doc = "Halt TX"]
    #[inline(always)]
    pub const fn uart2_htx_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart2HtxReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart2HtxReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(164usize),
            )
        }
    }

    #[doc = "Interrupt Enable Register/Divisor Latch High"]
    #[inline(always)]
    pub const fn uart2_ier_dlh_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart2IerDlhReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart2IerDlhReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Interrupt Identification Register/FIFO Control Register"]
    #[inline(always)]
    pub const fn uart2_iir_fcr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart2IirFcrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart2IirFcrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Line Control Register"]
    #[inline(always)]
    pub const fn uart2_lcr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart2LcrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart2LcrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Line Status Register"]
    #[inline(always)]
    pub const fn uart2_lsr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart2LsrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart2LsrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "Modem Control Register"]
    #[inline(always)]
    pub const fn uart2_mcr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart2McrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart2McrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Modem Status Register"]
    #[inline(always)]
    pub const fn uart2_msr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart2MsrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart2MsrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "Receive Buffer Register/Transmit Holding Register/Divisor Latch Low"]
    #[inline(always)]
    pub const fn uart2_rbr_thr_dll_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart2RbrThrDllReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart2RbrThrDllReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Receive FIFO Level"]
    #[inline(always)]
    pub const fn uart2_rfl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart2RflReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart2RflReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(132usize),
            )
        }
    }

    #[doc = "Shadow Break Control Register"]
    #[inline(always)]
    pub const fn uart2_sbcr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart2SbcrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart2SbcrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(144usize),
            )
        }
    }

    #[doc = "Scratchpad Register"]
    #[inline(always)]
    pub const fn uart2_scr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart2ScrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart2ScrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "Shadow DMA Mode"]
    #[inline(always)]
    pub const fn uart2_sdmam_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart2SdmamReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart2SdmamReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(148usize),
            )
        }
    }

    #[doc = "Shadow FIFO Enable"]
    #[inline(always)]
    pub const fn uart2_sfe_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart2SfeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart2SfeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(152usize),
            )
        }
    }

    #[doc = "Shadow Receive/Transmit Buffer Register"]
    #[inline(always)]
    pub const fn uart2_srbr_sthr0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart2SrbrSthr0Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart2SrbrSthr0Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[doc = "Shadow Receive/Transmit Buffer Register"]
    #[inline(always)]
    pub const fn uart2_srbr_sthr10_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart2SrbrSthr10Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart2SrbrSthr10Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(88usize),
            )
        }
    }

    #[doc = "Shadow Receive/Transmit Buffer Register"]
    #[inline(always)]
    pub const fn uart2_srbr_sthr11_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart2SrbrSthr11Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart2SrbrSthr11Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(92usize),
            )
        }
    }

    #[doc = "Shadow Receive/Transmit Buffer Register"]
    #[inline(always)]
    pub const fn uart2_srbr_sthr12_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart2SrbrSthr12Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart2SrbrSthr12Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(96usize),
            )
        }
    }

    #[doc = "Shadow Receive/Transmit Buffer Register"]
    #[inline(always)]
    pub const fn uart2_srbr_sthr13_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart2SrbrSthr13Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart2SrbrSthr13Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(100usize),
            )
        }
    }

    #[doc = "Shadow Receive/Transmit Buffer Register"]
    #[inline(always)]
    pub const fn uart2_srbr_sthr14_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart2SrbrSthr14Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart2SrbrSthr14Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(104usize),
            )
        }
    }

    #[doc = "Shadow Receive/Transmit Buffer Register"]
    #[inline(always)]
    pub const fn uart2_srbr_sthr15_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart2SrbrSthr15Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart2SrbrSthr15Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(108usize),
            )
        }
    }

    #[doc = "Shadow Receive/Transmit Buffer Register"]
    #[inline(always)]
    pub const fn uart2_srbr_sthr1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart2SrbrSthr1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart2SrbrSthr1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[doc = "Shadow Receive/Transmit Buffer Register"]
    #[inline(always)]
    pub const fn uart2_srbr_sthr2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart2SrbrSthr2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart2SrbrSthr2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[doc = "Shadow Receive/Transmit Buffer Register"]
    #[inline(always)]
    pub const fn uart2_srbr_sthr3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart2SrbrSthr3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart2SrbrSthr3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }

    #[doc = "Shadow Receive/Transmit Buffer Register"]
    #[inline(always)]
    pub const fn uart2_srbr_sthr4_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart2SrbrSthr4Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart2SrbrSthr4Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[doc = "Shadow Receive/Transmit Buffer Register"]
    #[inline(always)]
    pub const fn uart2_srbr_sthr5_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart2SrbrSthr5Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart2SrbrSthr5Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(68usize),
            )
        }
    }

    #[doc = "Shadow Receive/Transmit Buffer Register"]
    #[inline(always)]
    pub const fn uart2_srbr_sthr6_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart2SrbrSthr6Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart2SrbrSthr6Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(72usize),
            )
        }
    }

    #[doc = "Shadow Receive/Transmit Buffer Register"]
    #[inline(always)]
    pub const fn uart2_srbr_sthr7_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart2SrbrSthr7Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart2SrbrSthr7Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(76usize),
            )
        }
    }

    #[doc = "Shadow Receive/Transmit Buffer Register"]
    #[inline(always)]
    pub const fn uart2_srbr_sthr8_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart2SrbrSthr8Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart2SrbrSthr8Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }

    #[doc = "Shadow Receive/Transmit Buffer Register"]
    #[inline(always)]
    pub const fn uart2_srbr_sthr9_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart2SrbrSthr9Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart2SrbrSthr9Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(84usize),
            )
        }
    }

    #[doc = "Software Reset Register."]
    #[inline(always)]
    pub const fn uart2_srr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart2SrrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart2SrrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(136usize),
            )
        }
    }

    #[doc = "Shadow Request to Send"]
    #[inline(always)]
    pub const fn uart2_srts_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart2SrtsReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart2SrtsReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(140usize),
            )
        }
    }

    #[doc = "Shadow RCVR Trigger"]
    #[inline(always)]
    pub const fn uart2_srt_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart2SrtReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart2SrtReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(156usize),
            )
        }
    }

    #[doc = "Shadow TX Empty Trigger"]
    #[inline(always)]
    pub const fn uart2_stet_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart2StetReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart2StetReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(160usize),
            )
        }
    }

    #[doc = "Transmit FIFO Level"]
    #[inline(always)]
    pub const fn uart2_tfl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart2TflReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart2TflReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(128usize),
            )
        }
    }

    #[doc = "Component Version"]
    #[inline(always)]
    pub const fn uart2_ucv_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart2UcvReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart2UcvReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(248usize),
            )
        }
    }

    #[doc = "UART Status Register"]
    #[inline(always)]
    pub const fn uart2_usr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart2UsrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart2UsrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(124usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart2CprReg_SPEC;
impl crate::sealed::RegSpec for Uart2CprReg_SPEC {
    type DataType = u16;
}
#[doc = "Component Parameter Register"]
pub type Uart2CprReg = crate::RegValueT<Uart2CprReg_SPEC>;

impl Uart2CprReg {
    #[doc = "Component Parameter Register"]
    #[inline(always)]
    pub fn cpr(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Uart2CprReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Uart2CprReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart2CprReg {
    #[inline(always)]
    fn default() -> Uart2CprReg {
        <crate::RegValueT<Uart2CprReg_SPEC> as RegisterValue<_>>::new(15729)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart2CtrReg_SPEC;
impl crate::sealed::RegSpec for Uart2CtrReg_SPEC {
    type DataType = u32;
}
#[doc = "Component Type Register"]
pub type Uart2CtrReg = crate::RegValueT<Uart2CtrReg_SPEC>;

impl Uart2CtrReg {
    #[doc = "Component Type Register"]
    #[inline(always)]
    pub fn ctr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Uart2CtrReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Uart2CtrReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart2CtrReg {
    #[inline(always)]
    fn default() -> Uart2CtrReg {
        <crate::RegValueT<Uart2CtrReg_SPEC> as RegisterValue<_>>::new(1146552592)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart2DlfReg_SPEC;
impl crate::sealed::RegSpec for Uart2DlfReg_SPEC {
    type DataType = u16;
}
#[doc = "Divisor Latch Fraction Register"]
pub type Uart2DlfReg = crate::RegValueT<Uart2DlfReg_SPEC>;

impl Uart2DlfReg {
    #[doc = "The fractional value is added to integer value set by DLH, DLL. Fractional value is equal UART_DLF/16"]
    #[inline(always)]
    pub fn uart_dlf(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Uart2DlfReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Uart2DlfReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart2DlfReg {
    #[inline(always)]
    fn default() -> Uart2DlfReg {
        <crate::RegValueT<Uart2DlfReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart2DmasaReg_SPEC;
impl crate::sealed::RegSpec for Uart2DmasaReg_SPEC {
    type DataType = u16;
}
#[doc = "DMA Software Acknowledge"]
pub type Uart2DmasaReg = crate::RegValueT<Uart2DmasaReg_SPEC>;

impl Uart2DmasaReg {
    #[doc = "This register is use to perform DMA software acknowledge if a transfer needs to be terminated due to an error condition. For example, if the DMA disables the channel, then the DW_apb_uart should clear its request. This will cause the TX request, TX single, RX request and RX single signals to de-assert. Note that this bit is \'self-clearing\' and it is not necessary to clear this bit."]
    #[inline(always)]
    pub fn dmasa(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Uart2DmasaReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0,1,0,Uart2DmasaReg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart2DmasaReg {
    #[inline(always)]
    fn default() -> Uart2DmasaReg {
        <crate::RegValueT<Uart2DmasaReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart2FarReg_SPEC;
impl crate::sealed::RegSpec for Uart2FarReg_SPEC {
    type DataType = u16;
}
#[doc = "FIFO Access Register"]
pub type Uart2FarReg = crate::RegValueT<Uart2FarReg_SPEC>;

impl Uart2FarReg {
    #[doc = "Description: Writes will have no effect when FIFO_ACCESS == No, always readable. This register is use to enable a FIFO access mode for testing, so that the receive FIFO can be written by the master and the transmit FIFO can be read by the master when FIFO\'s are implemented and enabled. When FIFO\'s are not implemented or not enabled it allows the RBR to be written by the master and the THR to be read by the master. 0 = FIFO access mode disabled 1 = FIFO access mode enabled Note, that when the FIFO access mode is enabled/disabled, the control portion of the receive FIFO and transmit FIFO is reset and the FIFO\'s are treated as empty."]
    #[inline(always)]
    pub fn uart_far(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Uart2FarReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,Uart2FarReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart2FarReg {
    #[inline(always)]
    fn default() -> Uart2FarReg {
        <crate::RegValueT<Uart2FarReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart2HtxReg_SPEC;
impl crate::sealed::RegSpec for Uart2HtxReg_SPEC {
    type DataType = u16;
}
#[doc = "Halt TX"]
pub type Uart2HtxReg = crate::RegValueT<Uart2HtxReg_SPEC>;

impl Uart2HtxReg {
    #[doc = "This register is use to halt transmissions for testing, so that the transmit FIFO can be filled by the master when FIFOs are implemented and enabled.\n0 = Halt TX disabled\n1 = Halt TX enabled\nNote, if FIFOs are implemented and not enabled, the setting of the halt TX register has no effect on operation."]
    #[inline(always)]
    pub fn uart_halt_tx(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Uart2HtxReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Uart2HtxReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart2HtxReg {
    #[inline(always)]
    fn default() -> Uart2HtxReg {
        <crate::RegValueT<Uart2HtxReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart2IerDlhReg_SPEC;
impl crate::sealed::RegSpec for Uart2IerDlhReg_SPEC {
    type DataType = u16;
}
#[doc = "Interrupt Enable Register/Divisor Latch High"]
pub type Uart2IerDlhReg = crate::RegValueT<Uart2IerDlhReg_SPEC>;

impl Uart2IerDlhReg {
    #[doc = "Interrupt Enable Register: PTIME, Programmable THRE Interrupt Mode Enable. This is used to enable/disable the generation of THRE Interrupt. 0 = disabled 1 = enabled. \nDivisor Latch (High): DLH7, Bit 7 of the upper part of a 16-bit, read/write, Divisor Latch register that contains the baud rate divisor for the UART. This register may be accessed only when the DLAB bit (LCR\\[7\\]) is set. See register UART_RBR_THR_DLL_REG."]
    #[inline(always)]
    pub fn ptime_dlh7(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Uart2IerDlhReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,Uart2IerDlhReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Divisor Latch (High): DLH6 to DLH4, Bits 6 to 4 of the upper part of a 16-bit, read/write, Divisor Latch register that contains the baud rate divisor for the UART. This register may be accessed only when the DLAB bit (LCR\\[7\\]) is set, otherwise, this field is reserved. See register UART_RBR_THR_DLL_REG."]
    #[inline(always)]
    pub fn dlh6_4(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, Uart2IerDlhReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x7,1,0,u8, Uart2IerDlhReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Interrupt Enable Register: EDSSI, Enable Modem Status Interrupt. This is used to enable/disable the generation of Modem Status Interrupt. This is the fourth highest priority interrupt. 0 = disabled 1 = enabled\nDivisor Latch (High): DLH3, Bit 3 of the upper part of a 16-bit, read/write, Divisor Latch register that contains the baud rate divisor for the UART. This register may be accessed only when the DLAB bit (LCR\\[7\\]) is set. See register UART_RBR_THR_DLL_REG."]
    #[inline(always)]
    pub fn edssi_dlh3(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Uart2IerDlhReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,Uart2IerDlhReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Interrupt Enable Register: ELSI, Enable Receiver Line Status Interrupt. This is used to enable/disable the generation of Receiver Line Status Interrupt. This is the highest priority interrupt. 0 = disabled 1 = enabled\nDivisor Latch (High): DLH2, Bit 2 of the upper part of a 16-bit, read/write, Divisor Latch register that contains the baud rate divisor for the UART. This register may be accessed only when the DLAB bit (LCR\\[7\\]) is set. See register UART_RBR_THR_DLL_REG."]
    #[inline(always)]
    pub fn elsi_dhl2(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Uart2IerDlhReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,Uart2IerDlhReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Interrupt Enable Register: ETBEI, Enable Transmit Holding Register Empty Interrupt. This is used to enable/disable the generation of Transmitter Holding Register Empty Interrupt. This is the third highest priority interrupt. 0 = disabled 1 = enabled \nDivisor Latch (High): DLH1, Bit 1 of the upper part of a 16-bit, read/write, Divisor Latch register that contains the baud rate divisor for the UART. This register may be accessed only when the DLAB bit (LCR\\[7\\]) is set. See register UART_RBR_THR_DLL_REG."]
    #[inline(always)]
    pub fn etbei_dlh1(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Uart2IerDlhReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,Uart2IerDlhReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Interrupt Enable Register: ERBFI, Enable Received Data Available Interrupt. This is used to enable/disable the generation of Received Data Available Interrupt and the Character Timeout Interrupt (if in FIFO mode and FIFO\'s enabled). These are the second highest priority interrupts. 0 = disabled 1 = enabled\nDivisor Latch (High): DLH0, Bit 0 of the upper part of a 16-bit, read/write, Divisor Latch register that contains the baud rate divisor for the UART. This register may be accessed only when the DLAB bit (LCR\\[7\\]) is set. See register UART_RBR_THR_DLL_REG."]
    #[inline(always)]
    pub fn erbfi_dlh0(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Uart2IerDlhReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Uart2IerDlhReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart2IerDlhReg {
    #[inline(always)]
    fn default() -> Uart2IerDlhReg {
        <crate::RegValueT<Uart2IerDlhReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart2IirFcrReg_SPEC;
impl crate::sealed::RegSpec for Uart2IirFcrReg_SPEC {
    type DataType = u16;
}
#[doc = "Interrupt Identification Register/FIFO Control Register"]
pub type Uart2IirFcrReg = crate::RegValueT<Uart2IirFcrReg_SPEC>;

impl Uart2IirFcrReg {
    #[doc = "Interrupt Identification Register, reading this register; FIFO Control Register, writing to this register.\nInterrupt Identification Register: Bits\\[7:6\\], FIFO\'s Enabled (or FIFOSE): This is used to indicate whether the FIFO\'s are enabled or disabled. 00 = disabled. 11 = enabled. Bits\\[3:0\\], Interrupt ID (or IID): This indicates the highest priority pending interrupt which can be one of the following types:0001 = no interrupt pending. 0010 = THR empty. 0100 = received data available. 0110 = receiver line status. 0111 = busy detect. 1100 = character timeout.\nFIFO Control Register Bits\\[7:6\\], RCVR Trigger (or RT):. This is used to select the trigger level in the receiver FIFO at which the Received Data Available Interrupt will be generated. In auto flow control mode it is used to determine when the rts_n signal will be de-asserted. It also determines when the dma_rx_req_n signal will be asserted when in certain modes of operation. The following trigger levels are supported: 00 = 1 character in the FIFO 01 = FIFO 1/4 full 10 = FIFO 1/2 full 11 = FIFO 2 less than full Bits\\[5:4\\], TX Empty Trigger (or TET): This is used to select the empty threshold level at which the THRE Interrupts will be generated when the mode is active. It also determines when the dma_tx_req_n signal will be asserted when in certain modes of operation. The following trigger levels are supported: 00 = FIFO empty 01 = 2 characters in the FIFO 10 = FIFO 1/4 full 11 = FIFO 1/2 full Bit\\[3\\], DMA Mode (or DMAM): This determines the DMA signalling mode used for the dma_tx_req_n and dma_rx_req_n output signals. 0 = mode 0 1 = mode 1 Bit\\[2\\], XMIT FIFO Reset (or XFIFOR): This resets the control portion of the transmit FIFO and treats the FIFO as empty. Note that this bit is \'self-clearing\' and it is not necessary to clear this bit. Bit\\[1\\], RCVR FIFO Reset (or RFIFOR): This resets the control portion of the receive FIFO and treats the FIFO as empty. Note that this bit is \'self-clearing\' and it is not necessary to clear this bit. Bit\\[0\\], FIFO Enable (or FIFOE): This enables/disables the transmit (XMIT) and receive (RCVR) FIFO\'s. Whenever the value of this bit is changed both the XMIT and RCVR controller portion of FIFO\'s will be reset."]
    #[inline(always)]
    pub fn iir_fcr(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Uart2IirFcrReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Uart2IirFcrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart2IirFcrReg {
    #[inline(always)]
    fn default() -> Uart2IirFcrReg {
        <crate::RegValueT<Uart2IirFcrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart2LcrReg_SPEC;
impl crate::sealed::RegSpec for Uart2LcrReg_SPEC {
    type DataType = u16;
}
#[doc = "Line Control Register"]
pub type Uart2LcrReg = crate::RegValueT<Uart2LcrReg_SPEC>;

impl Uart2LcrReg {
    #[doc = "Divisor Latch Access Bit.\nThis bit is used to enable reading and writing of the Divisor Latch register (DLL and DLH) to set the baud rate of the UART.\nThis bit must be cleared after initial baud rate setup in order to access other registers."]
    #[inline(always)]
    pub fn uart_dlab(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Uart2LcrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,Uart2LcrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Break Control Bit.\nThis is used to cause a break condition to be transmitted to the receiving device. If set to one the serial output is forced to the spacing (logic 0) state. When not in Loopback Mode, as determined by MCR\\[4\\], the sout line is forced low until the Break bit is cleared. If active (MCR\\[6\\] set to one) the sir_out_n line is continuously pulsed. When in Loopback Mode, the break condition is internally looped back to the receiver and the sir_out_n line is forced low."]
    #[inline(always)]
    pub fn uart_bc(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Uart2LcrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,Uart2LcrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Even Parity Select. Writeable only when UART is not busy (USR\\[0\\] is zero).\nThis is used to select between even and odd parity, when parity is enabled (PEN set to one). If set to one, an even number of logic 1s is transmitted or checked. If set to zero, an odd number of logic 1s is transmitted or checked."]
    #[inline(always)]
    pub fn uart_eps(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Uart2LcrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,Uart2LcrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Parity Enable. Writeable only when UART is not busy (USR\\[0\\] is zero)\nThis bit is used to enable and disable parity generation and detection in transmitted and received serial character respectively.\n0 = parity disabled\n1 = parity enabled"]
    #[inline(always)]
    pub fn uart_pen(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Uart2LcrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,Uart2LcrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Number of stop bits.\nThis is used to select the number of stop bits per character that the peripheral transmits and receives. If set to zero, one stop bit is transmitted in the serial data.\nIf set to one and the data bits are set to 5 (LCR\\[1:0\\] set to zero) one and a half stop bits is transmitted. Otherwise, two stop bits are transmitted. Note that regardless of the number of stop bits selected, the receiver checks only the first stop bit.\n0 = 1 stop bit\n1 = 1.5 stop bits when DLS (LCR\\[1:0\\]) is zero, else 2 stop bit"]
    #[inline(always)]
    pub fn uart_stop(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Uart2LcrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,Uart2LcrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Data Length Select.\nThis is used to select the number of data bits per character that the peripheral transmits and receives. The number of bit that may be selected areas follows:\n00 = 5 bits\n01 = 6 bits\n10 = 7 bits\n11 = 8 bits"]
    #[inline(always)]
    pub fn uart_dls(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Uart2LcrReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, Uart2LcrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart2LcrReg {
    #[inline(always)]
    fn default() -> Uart2LcrReg {
        <crate::RegValueT<Uart2LcrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart2LsrReg_SPEC;
impl crate::sealed::RegSpec for Uart2LsrReg_SPEC {
    type DataType = u16;
}
#[doc = "Line Status Register"]
pub type Uart2LsrReg = crate::RegValueT<Uart2LsrReg_SPEC>;

impl Uart2LsrReg {
    #[doc = "Receiver FIFO Error bit.\nThis bit is only relevant when FIFOs are enabled (FCR\\[0\\] set to one). This is used to indicate if there is at least one parity error, framing error, or break indication in the FIFO.\n0 = no error in RX FIFO\n1 = error in RX FIFO\nThis bit is cleared when the LSR is read and the character with the error is at the top of the receiver FIFO and there are no subsequent errors in the FIFO."]
    #[inline(always)]
    pub fn uart_rfe(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Uart2LsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,Uart2LsrReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Transmitter Empty bit.\nIf FIFOs enabled (FCR\\[0\\] set to one), this bit is set whenever the Transmitter Shift Register and the FIFO are both empty. If FIFOs are disabled, this bit is set whenever the Transmitter Holding Register and the Transmitter Shift Register are both empty."]
    #[inline(always)]
    pub fn uart_temt(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Uart2LsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,Uart2LsrReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Transmit Holding Register Empty bit.\nIf THRE mode is disabled (IER\\[7\\] set to zero) and regardless of FIFO\'s being implemented/enabled or not, this bit indicates that the THR or TX FIFO is empty.\nThis bit is set whenever data is transferred from the THR or TX FIFO to the transmitter shift register and no new data has been written to the THR or TX FIFO. This also causes a THRE Interrupt to occur, if the THRE Interrupt is enabled. If both modes are active (IER\\[7\\] set to one and FCR\\[0\\] set to one respectively), the functionality is switched to indicate the transmitter FIFO is full, and no longer controls THRE interrupts, which are then controlled by the FCR\\[5:4\\] threshold setting."]
    #[inline(always)]
    pub fn uart_thre(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Uart2LsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,Uart2LsrReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Break Interrupt bit.\nThis is used to indicate the detection of a break sequence on the serial input data.\nIf in UART mode (SIR_MODE == Disabled), it is set whenever the serial input, sin, is held in a logic \'0\' state for longer than the sum of start time + data bits + parity + stop bits.\nIf in infrared mode (SIR_MODE == Enabled), it is set whenever the serial input, sir_in, is continuously pulsed to logic \'0\' for longer than the sum of start time + data bits + parity + stop bits. A break condition on serial input causes one and only one character, consisting of all zeros, to be received by the UART.\nIn the FIFO mode, the character associated with the break condition is carried through the FIFO and is revealed when the character is at the top of the FIFO.\nReading the LSR clears the BI bit. In the non-FIFO mode, the BI indication occurs immediately and persists until the LSR is read."]
    #[inline(always)]
    pub fn uart_bi(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Uart2LsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,Uart2LsrReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Framing Error bit.\nThis is used to indicate the occurrence of a framing error in the receiver. A framing error occurs when the receiver does not detect a valid STOP bit in the received data.\nIn the FIFO mode, since the framing error is associated with a character received, it is revealed when the character with the framing error is at the top of the FIFO.\nWhen a framing error occurs, the UART tries to resynchronize. It does this by assuming that the error was due to the start bit of the next character and then continues receiving the other bit i.e. data, and/or parity and stop. It should be noted that the Framing Error (FE) bit (LSR\\[3\\]) is set if a break interrupt has occurred, as indicated by Break Interrupt (BI) bit (LSR\\[4\\]).\n0 = no framing error\n1 = framing error\nReading the LSR clears the FE bit."]
    #[inline(always)]
    pub fn uart_fe(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Uart2LsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,Uart2LsrReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Parity Error bit.\nThis is used to indicate the occurrence of a parity error in the receiver if the Parity Enable (PEN) bit (LCR\\[3\\]) is set.\nIn the FIFO mode, since the parity error is associated with a character received, it is revealed when the character with the parity error arrives at the top of the FIFO.\nIt should be noted that the Parity Error (PE) bit (LSR\\[2\\]) is set if a break interrupt has occurred, as indicated by Break Interrupt (BI) bit (LSR\\[4\\]).\n0 = no parity error\n1 = parity error\nReading the LSR clears the PE bit."]
    #[inline(always)]
    pub fn uart_pe(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Uart2LsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,Uart2LsrReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Overrun error bit.\nThis is used to indicate the occurrence of an overrun error.\nThis occurs if a new data character was received before the previous data was read.\nIn the non-FIFO mode, the OE bit is set when a new character arrives in the receiver before the previous character was read from the RBR. When this happens, the data in the RBR is overwritten. In the FIFO mode, an overrun error occurs when the FIFO is full and a new character arrives at the receiver. The data in the FIFO is retained and the data in the receive shift register is lost.\n0 = no overrun error\n1 = overrun error\nReading the LSR clears the OE bit."]
    #[inline(always)]
    pub fn uart_oe(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Uart2LsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,Uart2LsrReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Data Ready bit.\nThis is used to indicate that the receiver contains at least one character in the RBR or the receiver FIFO.\n0 = no data ready\n1 = data ready\nThis bit is cleared when the RBR is read in non-FIFO mode, or when the receiver FIFO is empty, in FIFO mode."]
    #[inline(always)]
    pub fn uart_dr(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Uart2LsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,Uart2LsrReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart2LsrReg {
    #[inline(always)]
    fn default() -> Uart2LsrReg {
        <crate::RegValueT<Uart2LsrReg_SPEC> as RegisterValue<_>>::new(96)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart2McrReg_SPEC;
impl crate::sealed::RegSpec for Uart2McrReg_SPEC {
    type DataType = u16;
}
#[doc = "Modem Control Register"]
pub type Uart2McrReg = crate::RegValueT<Uart2McrReg_SPEC>;

impl Uart2McrReg {
    #[doc = "SIR Mode Enable.\nThis is used to enable/disable the IrDA SIR Mode features as described in \"IrDA 1.0 SIR Protocol\".\n0 = IrDA SIR Mode disabled\n1 = IrDA SIR Mode enabled"]
    #[inline(always)]
    pub fn uart_sire(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Uart2McrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,Uart2McrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Auto Flow Control Enable.\nWriteable only when AFCE_MODE == Enabled, always readable. When FIFOs are enabled and the Auto Flow Control Enable (AFCE) bit is set, Auto Flow Control features are enabled as described in \"Auto Flow Control\".\n0 = Auto Flow Control Mode disabled\n1 = Auto Flow Control Mode enabled"]
    #[inline(always)]
    pub fn uart_afce(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Uart2McrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,Uart2McrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LoopBack Bit.\nThis is used to put the UART into a diagnostic mode for test purposes.\nIf operating in UART mode (SIR_MODE not active, MCR\\[6\\] set to zero), data on the sout line is held high, while serial data output is looped back to the sin line, internally. In this mode all the interrupts are fully functional. Also, in loopback mode, the modem control inputs (dsr_n, cts_n, ri_n, dcd_n) are disconnected and the modem control outputs (dtr_n, rts_n, out1_n, out2_n) are looped back to the inputs, internally.\nIf operating in infrared mode (SIR_MODE active, MCR\\[6\\] set to one), data on the sir_out_n line is held low, while serial data output is inverted and looped back to the sir_in line."]
    #[inline(always)]
    pub fn uart_lb(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Uart2McrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,Uart2McrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "OUT2.\nThis is used to directly control the user-designated Output2 (out2_n) output. The value written to this location is inverted and driven out on out2_n, that is:\n0 = out2_n de-asserted (logic 1)\n1 = out2_n asserted (logic 0)\nNote that in Loopback mode (MCR\\[4\\] set to one), the out2_n output is held inactive high while the value of this location is internally looped back to an input."]
    #[inline(always)]
    pub fn uart_out2(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Uart2McrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,Uart2McrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "OUT1.\nThis is used to directly control the user-designated Output1 (out1_n) output. The value written to this location is inverted and driven out on out1_n, that is:\n0 = out1_n de-asserted (logic 1)\n1 = out1_n asserted (logic 0)\nNote that in Loopback mode (MCR\\[4\\] set to one), the out1_n output is held inactive high while the value of this location is internally looped back to an input."]
    #[inline(always)]
    pub fn uart_out1(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Uart2McrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,Uart2McrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Request to Send.\nThis is used to directly control the Request to Send (rts_n) output. The Request To Send (rts_n) output is used to inform the modem or data set that the UART is ready to exchange data.\nWhen Auto RTS Flow Control is not enabled (MCR\\[5\\] set to zero), the rts_n signal is set low by programming MCR\\[1\\] (RTS) to a high.In Auto Flow Control, AFCE_MODE == Enabled and active (MCR\\[5\\] set to one) and FIFOs enable (FCR\\[0\\] set to one), the rts_n output is controlled in the same way, but is also gated with the receiver FIFO threshold trigger (rts_n is inactive high when above the threshold). The rts_n signal is de-asserted when MCR\\[1\\] is set low.\nNote that in Loopback mode (MCR\\[4\\] set to one), the rts_n output is held inactive high while the value of this location is internally looped back to an input."]
    #[inline(always)]
    pub fn uart_rts(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Uart2McrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,Uart2McrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart2McrReg {
    #[inline(always)]
    fn default() -> Uart2McrReg {
        <crate::RegValueT<Uart2McrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart2MsrReg_SPEC;
impl crate::sealed::RegSpec for Uart2MsrReg_SPEC {
    type DataType = u16;
}
#[doc = "Modem Status Register"]
pub type Uart2MsrReg = crate::RegValueT<Uart2MsrReg_SPEC>;

impl Uart2MsrReg {
    #[doc = "Clear to Send.\nThis is used to indicate the current state of the modem control line cts_n. This bit is the complement of cts_n. When the Clear to Send input (cts_n) is asserted it is an indication that the modem or data set is ready to exchange data with the UART Ctrl.\n0 = cts_n input is de-asserted (logic 1)\n1 = cts_n input is asserted (logic 0)\nIn Loopback Mode (MCR\\[4\\] = 1), CTS is the same as MCR\\[1\\] (RTS)."]
    #[inline(always)]
    pub fn uart_cts(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Uart2MsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,Uart2MsrReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Delta Clear to Send.\nThis is used to indicate that the modem control line cts_n has changed since the last time the MSR was read.\n0 = no change on cts_n since last read of MSR\n1 = change on cts_n since last read of MSR\nReading the MSR clears the DCTS bit. In Loopback Mode (MCR\\[4\\] = 1), DCTS reflects changes on MCR\\[1\\] (RTS).\nNote, if the DCTS bit is not set and the cts_n signal is asserted (low) and a reset occurs (software or otherwise), then the DCTS bit is set when the reset is removed if the cts_n signal remains asserted."]
    #[inline(always)]
    pub fn uart_dcts(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Uart2MsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,Uart2MsrReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart2MsrReg {
    #[inline(always)]
    fn default() -> Uart2MsrReg {
        <crate::RegValueT<Uart2MsrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart2RbrThrDllReg_SPEC;
impl crate::sealed::RegSpec for Uart2RbrThrDllReg_SPEC {
    type DataType = u16;
}
#[doc = "Receive Buffer Register/Transmit Holding Register/Divisor Latch Low"]
pub type Uart2RbrThrDllReg = crate::RegValueT<Uart2RbrThrDllReg_SPEC>;

impl Uart2RbrThrDllReg {
    #[doc = "Receive Buffer Register: (RBR).\nThis register contains the data byte received on the serial input port (sin) in UART mode or the serial infrared input (sir_in) in infrared mode. The data in this register is valid only if the Data Ready (DR) bit in the Line status Register (LSR) is set. If FIFOs are disabled (FCR\\[0\\] set to zero), the data in the RBR must be read before the next data arrives, otherwise it will be overwritten, resulting in an overrun error. If FIFOs are enabled (FCR\\[0\\] set to one), this register accesses the head of the receive FIFO. If the receive FIFO is full and this register is not read before the next data character arrives, then the data already in the FIFO will be preserved but any incoming data will be lost. An overrun error will also occur.\nTransmit Holding Register: (THR)\nThis register contains data to be transmitted on the serial output port (sout) in UART mode or the serial infrared output (sir_out_n) in infrared mode. Data should only be written to the THR when the THR Empty (THRE) bit (LSR\\[5\\]) is set. If FIFO\'s are disabled (FCR\\[0\\] set to zero) and THRE is set, writing a single character to the THR clears the THRE. Any additional writes to the THR before the THRE is set again causes the THR data to be overwritten. If FIFO\'s are enabled (FCR\\[0\\] set to one) and THRE is set, x number of characters of data may be written to the THR before the FIFO is full. The number x (default=16) is determined by the value of FIFO Depth that you set during configuration. Any attempt to write data when the FIFO is full results in the write data being lost.\nDivisor Latch (Low): (DLL)\nThis register makes up the lower 8-bits of a 16-bit, read/write, Divisor Latch register that contains the baud rate divisor for the UART. This register may only be accessed when the DLAB bit (LCR\\[7\\]) is set. The output baud rate is equal to the serial clock (sclk) frequency divided by sixteen times the value of the baud rate divisor, as follows:\nbaud rate = (serial clock freq) / (16 * divisor)\nNote that with the Divisor Latch Registers (DLL and DLH) set to zero, the baud clock is disabled and no serial communications will occur. Also, once the Divisor Latch is set, at least 8 clock cycles of the slowest UART clock should be allowed to pass before transmitting or receiving data.\nFor the Divisor Latch (High) bits, see register UART_IER_DLH_REG."]
    #[inline(always)]
    pub fn rbr_thr_dll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Uart2RbrThrDllReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, Uart2RbrThrDllReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart2RbrThrDllReg {
    #[inline(always)]
    fn default() -> Uart2RbrThrDllReg {
        <crate::RegValueT<Uart2RbrThrDllReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart2RflReg_SPEC;
impl crate::sealed::RegSpec for Uart2RflReg_SPEC {
    type DataType = u16;
}
#[doc = "Receive FIFO Level"]
pub type Uart2RflReg = crate::RegValueT<Uart2RflReg_SPEC>;

impl Uart2RflReg {
    #[doc = "Receive FIFO Level.\nThis is indicates the number of data entries in the receive FIFO."]
    #[inline(always)]
    pub fn uart_receive_fifo_level(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Uart2RflReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Uart2RflReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart2RflReg {
    #[inline(always)]
    fn default() -> Uart2RflReg {
        <crate::RegValueT<Uart2RflReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart2SbcrReg_SPEC;
impl crate::sealed::RegSpec for Uart2SbcrReg_SPEC {
    type DataType = u16;
}
#[doc = "Shadow Break Control Register"]
pub type Uart2SbcrReg = crate::RegValueT<Uart2SbcrReg_SPEC>;

impl Uart2SbcrReg {
    #[doc = "Shadow Break Control Bit.\nThis is a shadow register for the Break bit (LCR\\[6\\]), this can be used to remove the burden of having to performing a read modify write on the LCR. This is used to cause a break condition to be transmitted to the receiving device.\nIf set to one the serial output is forced to the spacing (logic 0) state. When not in Loopback Mode, as determined by MCR\\[4\\], the sout line is forced low until the Break bit is cleared.\nIf SIR_MODE active (MCR\\[6\\] = 1) the sir_out_n line is continuously pulsed. When in Loopback Mode, the break condition is internally looped back to the receiver."]
    #[inline(always)]
    pub fn uart_shadow_break_control(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Uart2SbcrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Uart2SbcrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart2SbcrReg {
    #[inline(always)]
    fn default() -> Uart2SbcrReg {
        <crate::RegValueT<Uart2SbcrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart2ScrReg_SPEC;
impl crate::sealed::RegSpec for Uart2ScrReg_SPEC {
    type DataType = u16;
}
#[doc = "Scratchpad Register"]
pub type Uart2ScrReg = crate::RegValueT<Uart2ScrReg_SPEC>;

impl Uart2ScrReg {
    #[doc = "This register is for programmers to use as a temporary storage space. It has no defined purpose in the UART Ctrl."]
    #[inline(always)]
    pub fn uart_scratch_pad(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Uart2ScrReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Uart2ScrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart2ScrReg {
    #[inline(always)]
    fn default() -> Uart2ScrReg {
        <crate::RegValueT<Uart2ScrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart2SdmamReg_SPEC;
impl crate::sealed::RegSpec for Uart2SdmamReg_SPEC {
    type DataType = u16;
}
#[doc = "Shadow DMA Mode"]
pub type Uart2SdmamReg = crate::RegValueT<Uart2SdmamReg_SPEC>;

impl Uart2SdmamReg {
    #[doc = "Shadow DMA Mode.\nThis is a shadow register for the DMA mode bit (FCR\\[3\\]). This can be used to remove the burden of having to store the previously written value to the FCR in memory and having to mask this value so that only the DMA Mode bit gets updated. This determines the DMA signalling mode used for the dma_tx_req_n and dma_rx_req_n output signals.\n0 = mode 0\n1 = mode 1"]
    #[inline(always)]
    pub fn uart_shadow_dma_mode(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Uart2SdmamReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Uart2SdmamReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart2SdmamReg {
    #[inline(always)]
    fn default() -> Uart2SdmamReg {
        <crate::RegValueT<Uart2SdmamReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart2SfeReg_SPEC;
impl crate::sealed::RegSpec for Uart2SfeReg_SPEC {
    type DataType = u16;
}
#[doc = "Shadow FIFO Enable"]
pub type Uart2SfeReg = crate::RegValueT<Uart2SfeReg_SPEC>;

impl Uart2SfeReg {
    #[doc = "Shadow FIFO Enable.\nThis is a shadow register for the FIFO enable bit (FCR\\[0\\]). This can be used to remove the burden of having to store the previously written value to the FCR in memory and having to mask this value so that only the FIFO enable bit gets updated.This enables/disables the transmit (XMIT) and receive (RCVR) FIFOs. If this bit is set to zero (disabled) after being enabled then both the XMIT and RCVR controller portion of FIFOs are reset."]
    #[inline(always)]
    pub fn uart_shadow_fifo_enable(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Uart2SfeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Uart2SfeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart2SfeReg {
    #[inline(always)]
    fn default() -> Uart2SfeReg {
        <crate::RegValueT<Uart2SfeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart2SrbrSthr0Reg_SPEC;
impl crate::sealed::RegSpec for Uart2SrbrSthr0Reg_SPEC {
    type DataType = u16;
}
#[doc = "Shadow Receive/Transmit Buffer Register"]
pub type Uart2SrbrSthr0Reg = crate::RegValueT<Uart2SrbrSthr0Reg_SPEC>;

impl Uart2SrbrSthr0Reg {
    #[doc = "Shadow Receive Buffer Register x: This is a shadow register for the RBR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains the data byte received on the serial input port (sin) in UART mode or the serial infrared input (sir_in) in infrared mode. The data in this register is valid only if the Data Ready (DR) bit in the Line status Register (LSR) is set. If FIFOs are disabled (FCR\\[0\\] set to zero), the data in the RBR must be read before the next data arrives, otherwise it will be overwritten, resulting in an overrun error. If FIFOs are enabled (FCR\\[0\\] set to one), this register accesses the head of the receive FIFO. If the receive FIFO is full and this register is not read before the next data character arrives, then the data already in the FIFO will be preserved but any incoming data will be lost. An overrun error will also occur. Shadow Transmit Holding Register 0: This is a shadow register for the THR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains data to be transmitted on the serial output port (sout) in UART mode or the serial infrared output (sir_out_n) in infrared mode. Data should only be written to the THR when the THR Empty (THRE) bit (LSR\\[5\\]) is set. If FIFO\'s are disabled (FCR\\[0\\] set to zero) and THRE is set, writing a single character to the THR clears the THRE. Any additional writes to the THR before the THRE is set again causes the THR data to be overwritten. If FIFO\'s are enabled (FCR\\[0\\] set to one) and THRE is set, x number of characters of data may be written to the THR before the FIFO is full. The number x (default=16) is determined by the value of FIFO Depth that you set during configuration. Any attempt to write data when the FIFO is full results in the write data being lost."]
    #[inline(always)]
    pub fn srbr_sthrx(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Uart2SrbrSthr0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, Uart2SrbrSthr0Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart2SrbrSthr0Reg {
    #[inline(always)]
    fn default() -> Uart2SrbrSthr0Reg {
        <crate::RegValueT<Uart2SrbrSthr0Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart2SrbrSthr10Reg_SPEC;
impl crate::sealed::RegSpec for Uart2SrbrSthr10Reg_SPEC {
    type DataType = u16;
}
#[doc = "Shadow Receive/Transmit Buffer Register"]
pub type Uart2SrbrSthr10Reg = crate::RegValueT<Uart2SrbrSthr10Reg_SPEC>;

impl Uart2SrbrSthr10Reg {
    #[doc = "Shadow Receive Buffer Register x: This is a shadow register for the RBR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains the data byte received on the serial input port (sin) in UART mode or the serial infrared input (sir_in) in infrared mode. The data in this register is valid only if the Data Ready (DR) bit in the Line status Register (LSR) is set. If FIFOs are disabled (FCR\\[0\\] set to zero), the data in the RBR must be read before the next data arrives, otherwise it will be overwritten, resulting in an overrun error. If FIFOs are enabled (FCR\\[0\\] set to one), this register accesses the head of the receive FIFO. If the receive FIFO is full and this register is not read before the next data character arrives, then the data already in the FIFO will be preserved but any incoming data will be lost. An overrun error will also occur. Shadow Transmit Holding Register 0: This is a shadow register for the THR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains data to be transmitted on the serial output port (sout) in UART mode or the serial infrared output (sir_out_n) in infrared mode. Data should only be written to the THR when the THR Empty (THRE) bit (LSR\\[5\\]) is set. If FIFO\'s are disabled (FCR\\[0\\] set to zero) and THRE is set, writing a single character to the THR clears the THRE. Any additional writes to the THR before the THRE is set again causes the THR data to be overwritten. If FIFO\'s are enabled (FCR\\[0\\] set to one) and THRE is set, x number of characters of data may be written to the THR before the FIFO is full. The number x (default=16) is determined by the value of FIFO Depth that you set during configuration. Any attempt to write data when the FIFO is full results in the write data being lost."]
    #[inline(always)]
    pub fn srbr_sthrx(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Uart2SrbrSthr10Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, Uart2SrbrSthr10Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart2SrbrSthr10Reg {
    #[inline(always)]
    fn default() -> Uart2SrbrSthr10Reg {
        <crate::RegValueT<Uart2SrbrSthr10Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart2SrbrSthr11Reg_SPEC;
impl crate::sealed::RegSpec for Uart2SrbrSthr11Reg_SPEC {
    type DataType = u16;
}
#[doc = "Shadow Receive/Transmit Buffer Register"]
pub type Uart2SrbrSthr11Reg = crate::RegValueT<Uart2SrbrSthr11Reg_SPEC>;

impl Uart2SrbrSthr11Reg {
    #[doc = "Shadow Receive Buffer Register x: This is a shadow register for the RBR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains the data byte received on the serial input port (sin) in UART mode or the serial infrared input (sir_in) in infrared mode. The data in this register is valid only if the Data Ready (DR) bit in the Line status Register (LSR) is set. If FIFOs are disabled (FCR\\[0\\] set to zero), the data in the RBR must be read before the next data arrives, otherwise it will be overwritten, resulting in an overrun error. If FIFOs are enabled (FCR\\[0\\] set to one), this register accesses the head of the receive FIFO. If the receive FIFO is full and this register is not read before the next data character arrives, then the data already in the FIFO will be preserved but any incoming data will be lost. An overrun error will also occur. Shadow Transmit Holding Register 0: This is a shadow register for the THR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains data to be transmitted on the serial output port (sout) in UART mode or the serial infrared output (sir_out_n) in infrared mode. Data should only be written to the THR when the THR Empty (THRE) bit (LSR\\[5\\]) is set. If FIFO\'s are disabled (FCR\\[0\\] set to zero) and THRE is set, writing a single character to the THR clears the THRE. Any additional writes to the THR before the THRE is set again causes the THR data to be overwritten. If FIFO\'s are enabled (FCR\\[0\\] set to one) and THRE is set, x number of characters of data may be written to the THR before the FIFO is full. The number x (default=16) is determined by the value of FIFO Depth that you set during configuration. Any attempt to write data when the FIFO is full results in the write data being lost."]
    #[inline(always)]
    pub fn srbr_sthrx(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Uart2SrbrSthr11Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, Uart2SrbrSthr11Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart2SrbrSthr11Reg {
    #[inline(always)]
    fn default() -> Uart2SrbrSthr11Reg {
        <crate::RegValueT<Uart2SrbrSthr11Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart2SrbrSthr12Reg_SPEC;
impl crate::sealed::RegSpec for Uart2SrbrSthr12Reg_SPEC {
    type DataType = u16;
}
#[doc = "Shadow Receive/Transmit Buffer Register"]
pub type Uart2SrbrSthr12Reg = crate::RegValueT<Uart2SrbrSthr12Reg_SPEC>;

impl Uart2SrbrSthr12Reg {
    #[doc = "Shadow Receive Buffer Register x: This is a shadow register for the RBR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains the data byte received on the serial input port (sin) in UART mode or the serial infrared input (sir_in) in infrared mode. The data in this register is valid only if the Data Ready (DR) bit in the Line status Register (LSR) is set. If FIFOs are disabled (FCR\\[0\\] set to zero), the data in the RBR must be read before the next data arrives, otherwise it will be overwritten, resulting in an overrun error. If FIFOs are enabled (FCR\\[0\\] set to one), this register accesses the head of the receive FIFO. If the receive FIFO is full and this register is not read before the next data character arrives, then the data already in the FIFO will be preserved but any incoming data will be lost. An overrun error will also occur. Shadow Transmit Holding Register 0: This is a shadow register for the THR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains data to be transmitted on the serial output port (sout) in UART mode or the serial infrared output (sir_out_n) in infrared mode. Data should only be written to the THR when the THR Empty (THRE) bit (LSR\\[5\\]) is set. If FIFO\'s are disabled (FCR\\[0\\] set to zero) and THRE is set, writing a single character to the THR clears the THRE. Any additional writes to the THR before the THRE is set again causes the THR data to be overwritten. If FIFO\'s are enabled (FCR\\[0\\] set to one) and THRE is set, x number of characters of data may be written to the THR before the FIFO is full. The number x (default=16) is determined by the value of FIFO Depth that you set during configuration. Any attempt to write data when the FIFO is full results in the write data being lost."]
    #[inline(always)]
    pub fn srbr_sthrx(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Uart2SrbrSthr12Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, Uart2SrbrSthr12Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart2SrbrSthr12Reg {
    #[inline(always)]
    fn default() -> Uart2SrbrSthr12Reg {
        <crate::RegValueT<Uart2SrbrSthr12Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart2SrbrSthr13Reg_SPEC;
impl crate::sealed::RegSpec for Uart2SrbrSthr13Reg_SPEC {
    type DataType = u16;
}
#[doc = "Shadow Receive/Transmit Buffer Register"]
pub type Uart2SrbrSthr13Reg = crate::RegValueT<Uart2SrbrSthr13Reg_SPEC>;

impl Uart2SrbrSthr13Reg {
    #[doc = "Shadow Receive Buffer Register x: This is a shadow register for the RBR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains the data byte received on the serial input port (sin) in UART mode or the serial infrared input (sir_in) in infrared mode. The data in this register is valid only if the Data Ready (DR) bit in the Line status Register (LSR) is set. If FIFOs are disabled (FCR\\[0\\] set to zero), the data in the RBR must be read before the next data arrives, otherwise it will be overwritten, resulting in an overrun error. If FIFOs are enabled (FCR\\[0\\] set to one), this register accesses the head of the receive FIFO. If the receive FIFO is full and this register is not read before the next data character arrives, then the data already in the FIFO will be preserved but any incoming data will be lost. An overrun error will also occur. Shadow Transmit Holding Register 0: This is a shadow register for the THR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains data to be transmitted on the serial output port (sout) in UART mode or the serial infrared output (sir_out_n) in infrared mode. Data should only be written to the THR when the THR Empty (THRE) bit (LSR\\[5\\]) is set. If FIFO\'s are disabled (FCR\\[0\\] set to zero) and THRE is set, writing a single character to the THR clears the THRE. Any additional writes to the THR before the THRE is set again causes the THR data to be overwritten. If FIFO\'s are enabled (FCR\\[0\\] set to one) and THRE is set, x number of characters of data may be written to the THR before the FIFO is full. The number x (default=16) is determined by the value of FIFO Depth that you set during configuration. Any attempt to write data when the FIFO is full results in the write data being lost."]
    #[inline(always)]
    pub fn srbr_sthrx(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Uart2SrbrSthr13Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, Uart2SrbrSthr13Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart2SrbrSthr13Reg {
    #[inline(always)]
    fn default() -> Uart2SrbrSthr13Reg {
        <crate::RegValueT<Uart2SrbrSthr13Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart2SrbrSthr14Reg_SPEC;
impl crate::sealed::RegSpec for Uart2SrbrSthr14Reg_SPEC {
    type DataType = u16;
}
#[doc = "Shadow Receive/Transmit Buffer Register"]
pub type Uart2SrbrSthr14Reg = crate::RegValueT<Uart2SrbrSthr14Reg_SPEC>;

impl Uart2SrbrSthr14Reg {
    #[doc = "Shadow Receive Buffer Register x: This is a shadow register for the RBR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains the data byte received on the serial input port (sin) in UART mode or the serial infrared input (sir_in) in infrared mode. The data in this register is valid only if the Data Ready (DR) bit in the Line status Register (LSR) is set. If FIFOs are disabled (FCR\\[0\\] set to zero), the data in the RBR must be read before the next data arrives, otherwise it will be overwritten, resulting in an overrun error. If FIFOs are enabled (FCR\\[0\\] set to one), this register accesses the head of the receive FIFO. If the receive FIFO is full and this register is not read before the next data character arrives, then the data already in the FIFO will be preserved but any incoming data will be lost. An overrun error will also occur. Shadow Transmit Holding Register 0: This is a shadow register for the THR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains data to be transmitted on the serial output port (sout) in UART mode or the serial infrared output (sir_out_n) in infrared mode. Data should only be written to the THR when the THR Empty (THRE) bit (LSR\\[5\\]) is set. If FIFO\'s are disabled (FCR\\[0\\] set to zero) and THRE is set, writing a single character to the THR clears the THRE. Any additional writes to the THR before the THRE is set again causes the THR data to be overwritten. If FIFO\'s are enabled (FCR\\[0\\] set to one) and THRE is set, x number of characters of data may be written to the THR before the FIFO is full. The number x (default=16) is determined by the value of FIFO Depth that you set during configuration. Any attempt to write data when the FIFO is full results in the write data being lost."]
    #[inline(always)]
    pub fn srbr_sthrx(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Uart2SrbrSthr14Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, Uart2SrbrSthr14Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart2SrbrSthr14Reg {
    #[inline(always)]
    fn default() -> Uart2SrbrSthr14Reg {
        <crate::RegValueT<Uart2SrbrSthr14Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart2SrbrSthr15Reg_SPEC;
impl crate::sealed::RegSpec for Uart2SrbrSthr15Reg_SPEC {
    type DataType = u16;
}
#[doc = "Shadow Receive/Transmit Buffer Register"]
pub type Uart2SrbrSthr15Reg = crate::RegValueT<Uart2SrbrSthr15Reg_SPEC>;

impl Uart2SrbrSthr15Reg {
    #[doc = "Shadow Receive Buffer Register x: This is a shadow register for the RBR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains the data byte received on the serial input port (sin) in UART mode or the serial infrared input (sir_in) in infrared mode. The data in this register is valid only if the Data Ready (DR) bit in the Line status Register (LSR) is set. If FIFOs are disabled (FCR\\[0\\] set to zero), the data in the RBR must be read before the next data arrives, otherwise it will be overwritten, resulting in an overrun error. If FIFOs are enabled (FCR\\[0\\] set to one), this register accesses the head of the receive FIFO. If the receive FIFO is full and this register is not read before the next data character arrives, then the data already in the FIFO will be preserved but any incoming data will be lost. An overrun error will also occur. Shadow Transmit Holding Register 0: This is a shadow register for the THR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains data to be transmitted on the serial output port (sout) in UART mode or the serial infrared output (sir_out_n) in infrared mode. Data should only be written to the THR when the THR Empty (THRE) bit (LSR\\[5\\]) is set. If FIFO\'s are disabled (FCR\\[0\\] set to zero) and THRE is set, writing a single character to the THR clears the THRE. Any additional writes to the THR before the THRE is set again causes the THR data to be overwritten. If FIFO\'s are enabled (FCR\\[0\\] set to one) and THRE is set, x number of characters of data may be written to the THR before the FIFO is full. The number x (default=16) is determined by the value of FIFO Depth that you set during configuration. Any attempt to write data when the FIFO is full results in the write data being lost."]
    #[inline(always)]
    pub fn srbr_sthrx(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Uart2SrbrSthr15Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, Uart2SrbrSthr15Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart2SrbrSthr15Reg {
    #[inline(always)]
    fn default() -> Uart2SrbrSthr15Reg {
        <crate::RegValueT<Uart2SrbrSthr15Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart2SrbrSthr1Reg_SPEC;
impl crate::sealed::RegSpec for Uart2SrbrSthr1Reg_SPEC {
    type DataType = u16;
}
#[doc = "Shadow Receive/Transmit Buffer Register"]
pub type Uart2SrbrSthr1Reg = crate::RegValueT<Uart2SrbrSthr1Reg_SPEC>;

impl Uart2SrbrSthr1Reg {
    #[doc = "Shadow Receive Buffer Register x: This is a shadow register for the RBR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains the data byte received on the serial input port (sin) in UART mode or the serial infrared input (sir_in) in infrared mode. The data in this register is valid only if the Data Ready (DR) bit in the Line status Register (LSR) is set. If FIFOs are disabled (FCR\\[0\\] set to zero), the data in the RBR must be read before the next data arrives, otherwise it will be overwritten, resulting in an overrun error. If FIFOs are enabled (FCR\\[0\\] set to one), this register accesses the head of the receive FIFO. If the receive FIFO is full and this register is not read before the next data character arrives, then the data already in the FIFO will be preserved but any incoming data will be lost. An overrun error will also occur. Shadow Transmit Holding Register 0: This is a shadow register for the THR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains data to be transmitted on the serial output port (sout) in UART mode or the serial infrared output (sir_out_n) in infrared mode. Data should only be written to the THR when the THR Empty (THRE) bit (LSR\\[5\\]) is set. If FIFO\'s are disabled (FCR\\[0\\] set to zero) and THRE is set, writing a single character to the THR clears the THRE. Any additional writes to the THR before the THRE is set again causes the THR data to be overwritten. If FIFO\'s are enabled (FCR\\[0\\] set to one) and THRE is set, x number of characters of data may be written to the THR before the FIFO is full. The number x (default=16) is determined by the value of FIFO Depth that you set during configuration. Any attempt to write data when the FIFO is full results in the write data being lost."]
    #[inline(always)]
    pub fn srbr_sthrx(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Uart2SrbrSthr1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, Uart2SrbrSthr1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart2SrbrSthr1Reg {
    #[inline(always)]
    fn default() -> Uart2SrbrSthr1Reg {
        <crate::RegValueT<Uart2SrbrSthr1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart2SrbrSthr2Reg_SPEC;
impl crate::sealed::RegSpec for Uart2SrbrSthr2Reg_SPEC {
    type DataType = u16;
}
#[doc = "Shadow Receive/Transmit Buffer Register"]
pub type Uart2SrbrSthr2Reg = crate::RegValueT<Uart2SrbrSthr2Reg_SPEC>;

impl Uart2SrbrSthr2Reg {
    #[doc = "Shadow Receive Buffer Register x: This is a shadow register for the RBR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains the data byte received on the serial input port (sin) in UART mode or the serial infrared input (sir_in) in infrared mode. The data in this register is valid only if the Data Ready (DR) bit in the Line status Register (LSR) is set. If FIFOs are disabled (FCR\\[0\\] set to zero), the data in the RBR must be read before the next data arrives, otherwise it will be overwritten, resulting in an overrun error. If FIFOs are enabled (FCR\\[0\\] set to one), this register accesses the head of the receive FIFO. If the receive FIFO is full and this register is not read before the next data character arrives, then the data already in the FIFO will be preserved but any incoming data will be lost. An overrun error will also occur. Shadow Transmit Holding Register 0: This is a shadow register for the THR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains data to be transmitted on the serial output port (sout) in UART mode or the serial infrared output (sir_out_n) in infrared mode. Data should only be written to the THR when the THR Empty (THRE) bit (LSR\\[5\\]) is set. If FIFO\'s are disabled (FCR\\[0\\] set to zero) and THRE is set, writing a single character to the THR clears the THRE. Any additional writes to the THR before the THRE is set again causes the THR data to be overwritten. If FIFO\'s are enabled (FCR\\[0\\] set to one) and THRE is set, x number of characters of data may be written to the THR before the FIFO is full. The number x (default=16) is determined by the value of FIFO Depth that you set during configuration. Any attempt to write data when the FIFO is full results in the write data being lost."]
    #[inline(always)]
    pub fn srbr_sthrx(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Uart2SrbrSthr2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, Uart2SrbrSthr2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart2SrbrSthr2Reg {
    #[inline(always)]
    fn default() -> Uart2SrbrSthr2Reg {
        <crate::RegValueT<Uart2SrbrSthr2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart2SrbrSthr3Reg_SPEC;
impl crate::sealed::RegSpec for Uart2SrbrSthr3Reg_SPEC {
    type DataType = u16;
}
#[doc = "Shadow Receive/Transmit Buffer Register"]
pub type Uart2SrbrSthr3Reg = crate::RegValueT<Uart2SrbrSthr3Reg_SPEC>;

impl Uart2SrbrSthr3Reg {
    #[doc = "Shadow Receive Buffer Register x: This is a shadow register for the RBR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains the data byte received on the serial input port (sin) in UART mode or the serial infrared input (sir_in) in infrared mode. The data in this register is valid only if the Data Ready (DR) bit in the Line status Register (LSR) is set. If FIFOs are disabled (FCR\\[0\\] set to zero), the data in the RBR must be read before the next data arrives, otherwise it will be overwritten, resulting in an overrun error. If FIFOs are enabled (FCR\\[0\\] set to one), this register accesses the head of the receive FIFO. If the receive FIFO is full and this register is not read before the next data character arrives, then the data already in the FIFO will be preserved but any incoming data will be lost. An overrun error will also occur. Shadow Transmit Holding Register 0: This is a shadow register for the THR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains data to be transmitted on the serial output port (sout) in UART mode or the serial infrared output (sir_out_n) in infrared mode. Data should only be written to the THR when the THR Empty (THRE) bit (LSR\\[5\\]) is set. If FIFO\'s are disabled (FCR\\[0\\] set to zero) and THRE is set, writing a single character to the THR clears the THRE. Any additional writes to the THR before the THRE is set again causes the THR data to be overwritten. If FIFO\'s are enabled (FCR\\[0\\] set to one) and THRE is set, x number of characters of data may be written to the THR before the FIFO is full. The number x (default=16) is determined by the value of FIFO Depth that you set during configuration. Any attempt to write data when the FIFO is full results in the write data being lost."]
    #[inline(always)]
    pub fn srbr_sthrx(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Uart2SrbrSthr3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, Uart2SrbrSthr3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart2SrbrSthr3Reg {
    #[inline(always)]
    fn default() -> Uart2SrbrSthr3Reg {
        <crate::RegValueT<Uart2SrbrSthr3Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart2SrbrSthr4Reg_SPEC;
impl crate::sealed::RegSpec for Uart2SrbrSthr4Reg_SPEC {
    type DataType = u16;
}
#[doc = "Shadow Receive/Transmit Buffer Register"]
pub type Uart2SrbrSthr4Reg = crate::RegValueT<Uart2SrbrSthr4Reg_SPEC>;

impl Uart2SrbrSthr4Reg {
    #[doc = "Shadow Receive Buffer Register x: This is a shadow register for the RBR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains the data byte received on the serial input port (sin) in UART mode or the serial infrared input (sir_in) in infrared mode. The data in this register is valid only if the Data Ready (DR) bit in the Line status Register (LSR) is set. If FIFOs are disabled (FCR\\[0\\] set to zero), the data in the RBR must be read before the next data arrives, otherwise it will be overwritten, resulting in an overrun error. If FIFOs are enabled (FCR\\[0\\] set to one), this register accesses the head of the receive FIFO. If the receive FIFO is full and this register is not read before the next data character arrives, then the data already in the FIFO will be preserved but any incoming data will be lost. An overrun error will also occur. Shadow Transmit Holding Register 0: This is a shadow register for the THR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains data to be transmitted on the serial output port (sout) in UART mode or the serial infrared output (sir_out_n) in infrared mode. Data should only be written to the THR when the THR Empty (THRE) bit (LSR\\[5\\]) is set. If FIFO\'s are disabled (FCR\\[0\\] set to zero) and THRE is set, writing a single character to the THR clears the THRE. Any additional writes to the THR before the THRE is set again causes the THR data to be overwritten. If FIFO\'s are enabled (FCR\\[0\\] set to one) and THRE is set, x number of characters of data may be written to the THR before the FIFO is full. The number x (default=16) is determined by the value of FIFO Depth that you set during configuration. Any attempt to write data when the FIFO is full results in the write data being lost."]
    #[inline(always)]
    pub fn srbr_sthrx(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Uart2SrbrSthr4Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, Uart2SrbrSthr4Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart2SrbrSthr4Reg {
    #[inline(always)]
    fn default() -> Uart2SrbrSthr4Reg {
        <crate::RegValueT<Uart2SrbrSthr4Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart2SrbrSthr5Reg_SPEC;
impl crate::sealed::RegSpec for Uart2SrbrSthr5Reg_SPEC {
    type DataType = u16;
}
#[doc = "Shadow Receive/Transmit Buffer Register"]
pub type Uart2SrbrSthr5Reg = crate::RegValueT<Uart2SrbrSthr5Reg_SPEC>;

impl Uart2SrbrSthr5Reg {
    #[doc = "Shadow Receive Buffer Register x: This is a shadow register for the RBR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains the data byte received on the serial input port (sin) in UART mode or the serial infrared input (sir_in) in infrared mode. The data in this register is valid only if the Data Ready (DR) bit in the Line status Register (LSR) is set. If FIFOs are disabled (FCR\\[0\\] set to zero), the data in the RBR must be read before the next data arrives, otherwise it will be overwritten, resulting in an overrun error. If FIFOs are enabled (FCR\\[0\\] set to one), this register accesses the head of the receive FIFO. If the receive FIFO is full and this register is not read before the next data character arrives, then the data already in the FIFO will be preserved but any incoming data will be lost. An overrun error will also occur. Shadow Transmit Holding Register 0: This is a shadow register for the THR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains data to be transmitted on the serial output port (sout) in UART mode or the serial infrared output (sir_out_n) in infrared mode. Data should only be written to the THR when the THR Empty (THRE) bit (LSR\\[5\\]) is set. If FIFO\'s are disabled (FCR\\[0\\] set to zero) and THRE is set, writing a single character to the THR clears the THRE. Any additional writes to the THR before the THRE is set again causes the THR data to be overwritten. If FIFO\'s are enabled (FCR\\[0\\] set to one) and THRE is set, x number of characters of data may be written to the THR before the FIFO is full. The number x (default=16) is determined by the value of FIFO Depth that you set during configuration. Any attempt to write data when the FIFO is full results in the write data being lost."]
    #[inline(always)]
    pub fn srbr_sthrx(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Uart2SrbrSthr5Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, Uart2SrbrSthr5Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart2SrbrSthr5Reg {
    #[inline(always)]
    fn default() -> Uart2SrbrSthr5Reg {
        <crate::RegValueT<Uart2SrbrSthr5Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart2SrbrSthr6Reg_SPEC;
impl crate::sealed::RegSpec for Uart2SrbrSthr6Reg_SPEC {
    type DataType = u16;
}
#[doc = "Shadow Receive/Transmit Buffer Register"]
pub type Uart2SrbrSthr6Reg = crate::RegValueT<Uart2SrbrSthr6Reg_SPEC>;

impl Uart2SrbrSthr6Reg {
    #[doc = "Shadow Receive Buffer Register x: This is a shadow register for the RBR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains the data byte received on the serial input port (sin) in UART mode or the serial infrared input (sir_in) in infrared mode. The data in this register is valid only if the Data Ready (DR) bit in the Line status Register (LSR) is set. If FIFOs are disabled (FCR\\[0\\] set to zero), the data in the RBR must be read before the next data arrives, otherwise it will be overwritten, resulting in an overrun error. If FIFOs are enabled (FCR\\[0\\] set to one), this register accesses the head of the receive FIFO. If the receive FIFO is full and this register is not read before the next data character arrives, then the data already in the FIFO will be preserved but any incoming data will be lost. An overrun error will also occur. Shadow Transmit Holding Register 0: This is a shadow register for the THR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains data to be transmitted on the serial output port (sout) in UART mode or the serial infrared output (sir_out_n) in infrared mode. Data should only be written to the THR when the THR Empty (THRE) bit (LSR\\[5\\]) is set. If FIFO\'s are disabled (FCR\\[0\\] set to zero) and THRE is set, writing a single character to the THR clears the THRE. Any additional writes to the THR before the THRE is set again causes the THR data to be overwritten. If FIFO\'s are enabled (FCR\\[0\\] set to one) and THRE is set, x number of characters of data may be written to the THR before the FIFO is full. The number x (default=16) is determined by the value of FIFO Depth that you set during configuration. Any attempt to write data when the FIFO is full results in the write data being lost."]
    #[inline(always)]
    pub fn srbr_sthrx(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Uart2SrbrSthr6Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, Uart2SrbrSthr6Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart2SrbrSthr6Reg {
    #[inline(always)]
    fn default() -> Uart2SrbrSthr6Reg {
        <crate::RegValueT<Uart2SrbrSthr6Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart2SrbrSthr7Reg_SPEC;
impl crate::sealed::RegSpec for Uart2SrbrSthr7Reg_SPEC {
    type DataType = u16;
}
#[doc = "Shadow Receive/Transmit Buffer Register"]
pub type Uart2SrbrSthr7Reg = crate::RegValueT<Uart2SrbrSthr7Reg_SPEC>;

impl Uart2SrbrSthr7Reg {
    #[doc = "Shadow Receive Buffer Register x: This is a shadow register for the RBR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains the data byte received on the serial input port (sin) in UART mode or the serial infrared input (sir_in) in infrared mode. The data in this register is valid only if the Data Ready (DR) bit in the Line status Register (LSR) is set. If FIFOs are disabled (FCR\\[0\\] set to zero), the data in the RBR must be read before the next data arrives, otherwise it will be overwritten, resulting in an overrun error. If FIFOs are enabled (FCR\\[0\\] set to one), this register accesses the head of the receive FIFO. If the receive FIFO is full and this register is not read before the next data character arrives, then the data already in the FIFO will be preserved but any incoming data will be lost. An overrun error will also occur. Shadow Transmit Holding Register 0: This is a shadow register for the THR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains data to be transmitted on the serial output port (sout) in UART mode or the serial infrared output (sir_out_n) in infrared mode. Data should only be written to the THR when the THR Empty (THRE) bit (LSR\\[5\\]) is set. If FIFO\'s are disabled (FCR\\[0\\] set to zero) and THRE is set, writing a single character to the THR clears the THRE. Any additional writes to the THR before the THRE is set again causes the THR data to be overwritten. If FIFO\'s are enabled (FCR\\[0\\] set to one) and THRE is set, x number of characters of data may be written to the THR before the FIFO is full. The number x (default=16) is determined by the value of FIFO Depth that you set during configuration. Any attempt to write data when the FIFO is full results in the write data being lost."]
    #[inline(always)]
    pub fn srbr_sthrx(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Uart2SrbrSthr7Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, Uart2SrbrSthr7Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart2SrbrSthr7Reg {
    #[inline(always)]
    fn default() -> Uart2SrbrSthr7Reg {
        <crate::RegValueT<Uart2SrbrSthr7Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart2SrbrSthr8Reg_SPEC;
impl crate::sealed::RegSpec for Uart2SrbrSthr8Reg_SPEC {
    type DataType = u16;
}
#[doc = "Shadow Receive/Transmit Buffer Register"]
pub type Uart2SrbrSthr8Reg = crate::RegValueT<Uart2SrbrSthr8Reg_SPEC>;

impl Uart2SrbrSthr8Reg {
    #[doc = "Shadow Receive Buffer Register x: This is a shadow register for the RBR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains the data byte received on the serial input port (sin) in UART mode or the serial infrared input (sir_in) in infrared mode. The data in this register is valid only if the Data Ready (DR) bit in the Line status Register (LSR) is set. If FIFOs are disabled (FCR\\[0\\] set to zero), the data in the RBR must be read before the next data arrives, otherwise it will be overwritten, resulting in an overrun error. If FIFOs are enabled (FCR\\[0\\] set to one), this register accesses the head of the receive FIFO. If the receive FIFO is full and this register is not read before the next data character arrives, then the data already in the FIFO will be preserved but any incoming data will be lost. An overrun error will also occur. Shadow Transmit Holding Register 0: This is a shadow register for the THR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains data to be transmitted on the serial output port (sout) in UART mode or the serial infrared output (sir_out_n) in infrared mode. Data should only be written to the THR when the THR Empty (THRE) bit (LSR\\[5\\]) is set. If FIFO\'s are disabled (FCR\\[0\\] set to zero) and THRE is set, writing a single character to the THR clears the THRE. Any additional writes to the THR before the THRE is set again causes the THR data to be overwritten. If FIFO\'s are enabled (FCR\\[0\\] set to one) and THRE is set, x number of characters of data may be written to the THR before the FIFO is full. The number x (default=16) is determined by the value of FIFO Depth that you set during configuration. Any attempt to write data when the FIFO is full results in the write data being lost."]
    #[inline(always)]
    pub fn srbr_sthrx(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Uart2SrbrSthr8Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, Uart2SrbrSthr8Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart2SrbrSthr8Reg {
    #[inline(always)]
    fn default() -> Uart2SrbrSthr8Reg {
        <crate::RegValueT<Uart2SrbrSthr8Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart2SrbrSthr9Reg_SPEC;
impl crate::sealed::RegSpec for Uart2SrbrSthr9Reg_SPEC {
    type DataType = u16;
}
#[doc = "Shadow Receive/Transmit Buffer Register"]
pub type Uart2SrbrSthr9Reg = crate::RegValueT<Uart2SrbrSthr9Reg_SPEC>;

impl Uart2SrbrSthr9Reg {
    #[doc = "Shadow Receive Buffer Register x: This is a shadow register for the RBR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains the data byte received on the serial input port (sin) in UART mode or the serial infrared input (sir_in) in infrared mode. The data in this register is valid only if the Data Ready (DR) bit in the Line status Register (LSR) is set. If FIFOs are disabled (FCR\\[0\\] set to zero), the data in the RBR must be read before the next data arrives, otherwise it will be overwritten, resulting in an overrun error. If FIFOs are enabled (FCR\\[0\\] set to one), this register accesses the head of the receive FIFO. If the receive FIFO is full and this register is not read before the next data character arrives, then the data already in the FIFO will be preserved but any incoming data will be lost. An overrun error will also occur. Shadow Transmit Holding Register 0: This is a shadow register for the THR and has been allocated sixteen 32-bit locations so as to accommodate burst accesses from the master. This register contains data to be transmitted on the serial output port (sout) in UART mode or the serial infrared output (sir_out_n) in infrared mode. Data should only be written to the THR when the THR Empty (THRE) bit (LSR\\[5\\]) is set. If FIFO\'s are disabled (FCR\\[0\\] set to zero) and THRE is set, writing a single character to the THR clears the THRE. Any additional writes to the THR before the THRE is set again causes the THR data to be overwritten. If FIFO\'s are enabled (FCR\\[0\\] set to one) and THRE is set, x number of characters of data may be written to the THR before the FIFO is full. The number x (default=16) is determined by the value of FIFO Depth that you set during configuration. Any attempt to write data when the FIFO is full results in the write data being lost."]
    #[inline(always)]
    pub fn srbr_sthrx(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Uart2SrbrSthr9Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, Uart2SrbrSthr9Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart2SrbrSthr9Reg {
    #[inline(always)]
    fn default() -> Uart2SrbrSthr9Reg {
        <crate::RegValueT<Uart2SrbrSthr9Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart2SrrReg_SPEC;
impl crate::sealed::RegSpec for Uart2SrrReg_SPEC {
    type DataType = u16;
}
#[doc = "Software Reset Register."]
pub type Uart2SrrReg = crate::RegValueT<Uart2SrrReg_SPEC>;

impl Uart2SrrReg {
    #[doc = "XMIT FIFO Reset.\nThis is a shadow register for the XMIT FIFO Reset bit (FCR\\[2\\]). This can be used to remove the burden on software having to store previously written FCR values (which are pretty static) just to reset the transmit FIFO. This resets the control portion of the transmit FIFO and treats the FIFO as empty. Note that this bit is \'self-clearing\'. It is not necessary to clear this bit."]
    #[inline(always)]
    pub fn uart_xfr(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Uart2SrrReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2,1,0,Uart2SrrReg_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "RCVR FIFO Reset.\nThis is a shadow register for the RCVR FIFO Reset bit (FCR\\[1\\]). This can be used to remove the burden on software having to store previously written FCR values (which are pretty static) just to reset the receive FIFO This resets the control portion of the receive FIFO and treats the FIFO as empty.\nNote that this bit is \'self-clearing\'. It is not necessary to clear this bit."]
    #[inline(always)]
    pub fn uart_rfr(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Uart2SrrReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1,1,0,Uart2SrrReg_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "UART Reset. This asynchronously resets the UART Ctrl and synchronously removes the reset assertion. For a two clock implementation both pclk and sclk domains are reset."]
    #[inline(always)]
    pub fn uart_ur(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Uart2SrrReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0,1,0,Uart2SrrReg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart2SrrReg {
    #[inline(always)]
    fn default() -> Uart2SrrReg {
        <crate::RegValueT<Uart2SrrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart2SrtsReg_SPEC;
impl crate::sealed::RegSpec for Uart2SrtsReg_SPEC {
    type DataType = u16;
}
#[doc = "Shadow Request to Send"]
pub type Uart2SrtsReg = crate::RegValueT<Uart2SrtsReg_SPEC>;

impl Uart2SrtsReg {
    #[doc = "Shadow Request to Send.\nThis is a shadow register for the RTS bit (MCR\\[1\\]), this can be used to remove the burden of having to\nperforming a read-modify-write on the MCR. This is used to directly control the Request to Send (rts_n) output. The Request To Send (rts_n) output is used to inform the modem or data set that the UART Ctrl is ready to exchange data.\nWhen Auto RTS Flow Control is not enabled (MCR\\[5\\] = 0), the rts_n signal is set low by programming MCR\\[1\\] (RTS) to a high.\nIn Auto Flow Control, AFCE_MODE == Enabled and active (MCR\\[5\\] = 1) and FIFOs enable (FCR\\[0\\] = 1), the rts_n output is controlled in the same way, but is also gated with the receiver FIFO threshold trigger (rts_n is inactive high when above the threshold).\nNote that in Loopback mode (MCR\\[4\\] = 1), the rts_n output is held inactive-high while the value of this location is internally looped back to an input."]
    #[inline(always)]
    pub fn uart_shadow_request_to_send(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Uart2SrtsReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Uart2SrtsReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart2SrtsReg {
    #[inline(always)]
    fn default() -> Uart2SrtsReg {
        <crate::RegValueT<Uart2SrtsReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart2SrtReg_SPEC;
impl crate::sealed::RegSpec for Uart2SrtReg_SPEC {
    type DataType = u16;
}
#[doc = "Shadow RCVR Trigger"]
pub type Uart2SrtReg = crate::RegValueT<Uart2SrtReg_SPEC>;

impl Uart2SrtReg {
    #[doc = "Shadow RCVR Trigger.\nThis is a shadow register for the RCVR trigger bits (FCR\\[7:6\\]). This can be used to remove the burden of having to store the previously written value to the FCR in memory and having to mask this value so that only the RCVR trigger bit gets updated.\nThis is used to select the trigger level in the receiver FIFO at which the Received Data Available Interrupt is generated. It also determines when the dma_rx_req_n signal is asserted when DMA Mode (FCR\\[3\\]) = 1. The following trigger levels are supported:\n00 = 1 character in the FIFO\n01 = FIFO ¼ full\n10 = FIFO ½ full\n11 = FIFO 2 less than full"]
    #[inline(always)]
    pub fn uart_shadow_rcvr_trigger(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Uart2SrtReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, Uart2SrtReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart2SrtReg {
    #[inline(always)]
    fn default() -> Uart2SrtReg {
        <crate::RegValueT<Uart2SrtReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart2StetReg_SPEC;
impl crate::sealed::RegSpec for Uart2StetReg_SPEC {
    type DataType = u16;
}
#[doc = "Shadow TX Empty Trigger"]
pub type Uart2StetReg = crate::RegValueT<Uart2StetReg_SPEC>;

impl Uart2StetReg {
    #[doc = "Shadow TX Empty Trigger.\nThis is a shadow register for the TX empty trigger bits (FCR\\[5:4\\]). This can be used to remove the burden of having to store the previously written value to the FCR in memory and having to mask this value so that only the TX empty trigger bit gets updated.\nThis is used to select the empty threshold level at which the THRE Interrupts are generated when the mode is active. The following trigger levels are supported:\n00 = FIFO empty\n01 = 2 characters in the FIFO\n10 = FIFO ¼ full\n11 = FIFO ½ full"]
    #[inline(always)]
    pub fn uart_shadow_tx_empty_trigger(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Uart2StetReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, Uart2StetReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart2StetReg {
    #[inline(always)]
    fn default() -> Uart2StetReg {
        <crate::RegValueT<Uart2StetReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart2TflReg_SPEC;
impl crate::sealed::RegSpec for Uart2TflReg_SPEC {
    type DataType = u16;
}
#[doc = "Transmit FIFO Level"]
pub type Uart2TflReg = crate::RegValueT<Uart2TflReg_SPEC>;

impl Uart2TflReg {
    #[doc = "Transmit FIFO Level.\nThis is indicates the number of data entries in the transmit FIFO."]
    #[inline(always)]
    pub fn uart_transmit_fifo_level(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Uart2TflReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Uart2TflReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart2TflReg {
    #[inline(always)]
    fn default() -> Uart2TflReg {
        <crate::RegValueT<Uart2TflReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart2UcvReg_SPEC;
impl crate::sealed::RegSpec for Uart2UcvReg_SPEC {
    type DataType = u32;
}
#[doc = "Component Version"]
pub type Uart2UcvReg = crate::RegValueT<Uart2UcvReg_SPEC>;

impl Uart2UcvReg {
    #[doc = "Component Version"]
    #[inline(always)]
    pub fn ucv(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Uart2UcvReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Uart2UcvReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart2UcvReg {
    #[inline(always)]
    fn default() -> Uart2UcvReg {
        <crate::RegValueT<Uart2UcvReg_SPEC> as RegisterValue<_>>::new(858862890)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart2UsrReg_SPEC;
impl crate::sealed::RegSpec for Uart2UsrReg_SPEC {
    type DataType = u16;
}
#[doc = "UART Status Register"]
pub type Uart2UsrReg = crate::RegValueT<Uart2UsrReg_SPEC>;

impl Uart2UsrReg {
    #[doc = "Receive FIFO Full.\nThis is used to indicate that the receive FIFO is completely full.\n0 = Receive FIFO not full\n1 = Receive FIFO Full\nThis bit is cleared when the RX FIFO is no longer full."]
    #[inline(always)]
    pub fn uart_rff(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Uart2UsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,Uart2UsrReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Receive FIFO Not Empty.\nThis is used to indicate that the receive FIFO contains one or more entries.\n0 = Receive FIFO is empty\n1 = Receive FIFO is not empty\nThis bit is cleared when the RX FIFO is empty."]
    #[inline(always)]
    pub fn uart_rfne(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Uart2UsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,Uart2UsrReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Transmit FIFO Empty.\nThis is used to indicate that the transmit FIFO is completely empty.\n0 = Transmit FIFO is not empty\n1 = Transmit FIFO is empty\nThis bit is cleared when the TX FIFO is no longer empty."]
    #[inline(always)]
    pub fn uart_tfe(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Uart2UsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,Uart2UsrReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Transmit FIFO Not Full.\nThis is used to indicate that the transmit FIFO in not full.\n0 = Transmit FIFO is full\n1 = Transmit FIFO is not full\nThis bit is cleared when the TX FIFO is full."]
    #[inline(always)]
    pub fn uart_tfnf(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Uart2UsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,Uart2UsrReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "UART Busy. This indicates that a serial transfer is in progress, when cleared indicates that the DW_apb_uart is idle or inactive. 0 - DW_apb_uart is idle or inactive 1 - DW_apb_uart is busy (actively transferring data) Note that it is possible for the UART Busy bit to be cleared even though a new character may have been sent from another device. That is, if the DW_apb_uart has no data in the THR and RBR and there is no transmission in progress and a start bit of a new character has just reached the DW_apb_uart. This is due to the fact that a valid start is not seen until the middle of the bit period and this duration is dependent on the baud divisor that has been programmed. If a second system clock has been implemented (CLOCK_MODE == Enabled) the assertion of this bit will also be delayed by several cycles of the slower clock."]
    #[inline(always)]
    pub fn uart_busy(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Uart2UsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,Uart2UsrReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart2UsrReg {
    #[inline(always)]
    fn default() -> Uart2UsrReg {
        <crate::RegValueT<Uart2UsrReg_SPEC> as RegisterValue<_>>::new(6)
    }
}
