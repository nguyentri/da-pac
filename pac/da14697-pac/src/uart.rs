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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:16:28 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"UART registers"]
unsafe impl ::core::marker::Send for super::Uart {}
unsafe impl ::core::marker::Sync for super::Uart {}
impl super::Uart {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn uart_ctr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartCtrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartCtrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(252usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart_dlf_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartDlfReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartDlfReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(192usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart_dmasa_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartDmasaReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartDmasaReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(168usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart_htx_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartHtxReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartHtxReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(164usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart_ier_dlh_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartIerDlhReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartIerDlhReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart_iir_fcr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartIirFcrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartIirFcrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart_lcr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartLcrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartLcrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart_lsr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartLsrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartLsrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart_mcr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartMcrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartMcrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart_rbr_thr_dll_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartRbrThrDllReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartRbrThrDllReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart_rfl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartRflReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartRflReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(132usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart_sbcr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartSbcrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartSbcrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(144usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart_scr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartScrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartScrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart_sdmam_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartSdmamReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartSdmamReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(148usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart_sfe_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartSfeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartSfeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(152usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart_srbr_sthr0_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartSrbrSthr0Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartSrbrSthr0Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart_srbr_sthr10_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartSrbrSthr10Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartSrbrSthr10Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(88usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart_srbr_sthr11_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartSrbrSthr11Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartSrbrSthr11Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(92usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart_srbr_sthr12_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartSrbrSthr12Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartSrbrSthr12Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(96usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart_srbr_sthr13_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartSrbrSthr13Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartSrbrSthr13Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(100usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart_srbr_sthr14_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartSrbrSthr14Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartSrbrSthr14Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(104usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart_srbr_sthr15_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartSrbrSthr15Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartSrbrSthr15Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(108usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart_srbr_sthr1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartSrbrSthr1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartSrbrSthr1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart_srbr_sthr2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartSrbrSthr2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartSrbrSthr2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart_srbr_sthr3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartSrbrSthr3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartSrbrSthr3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart_srbr_sthr4_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartSrbrSthr4Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartSrbrSthr4Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart_srbr_sthr5_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartSrbrSthr5Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartSrbrSthr5Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(68usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart_srbr_sthr6_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartSrbrSthr6Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartSrbrSthr6Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(72usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart_srbr_sthr7_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartSrbrSthr7Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartSrbrSthr7Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(76usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart_srbr_sthr8_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartSrbrSthr8Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartSrbrSthr8Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart_srbr_sthr9_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartSrbrSthr9Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartSrbrSthr9Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(84usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart_srr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartSrrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartSrrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(136usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart_srt_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartSrtReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartSrtReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(156usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart_stet_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartStetReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartStetReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(160usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart_tfl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartTflReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartTflReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(128usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart_ucv_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartUcvReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartUcvReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(248usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart_usr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::UartUsrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::UartUsrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(124usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartCtrReg_SPEC;
impl crate::sealed::RegSpec for UartCtrReg_SPEC {
    type DataType = u32;
}

pub type UartCtrReg = crate::RegValueT<UartCtrReg_SPEC>;

impl UartCtrReg {
    #[inline(always)]
    pub fn uart_ctr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        UartCtrReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            UartCtrReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for UartCtrReg {
    #[inline(always)]
    fn default() -> UartCtrReg {
        <crate::RegValueT<UartCtrReg_SPEC> as RegisterValue<_>>::new(1146552592)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartDlfReg_SPEC;
impl crate::sealed::RegSpec for UartDlfReg_SPEC {
    type DataType = u32;
}

pub type UartDlfReg = crate::RegValueT<UartDlfReg_SPEC>;

impl UartDlfReg {
    #[inline(always)]
    pub fn uart_dlf(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, UartDlfReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,UartDlfReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for UartDlfReg {
    #[inline(always)]
    fn default() -> UartDlfReg {
        <crate::RegValueT<UartDlfReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartDmasaReg_SPEC;
impl crate::sealed::RegSpec for UartDmasaReg_SPEC {
    type DataType = u32;
}

pub type UartDmasaReg = crate::RegValueT<UartDmasaReg_SPEC>;

impl UartDmasaReg {
    #[inline(always)]
    pub fn uart_dmasa(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, UartDmasaReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0,1,0,UartDmasaReg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for UartDmasaReg {
    #[inline(always)]
    fn default() -> UartDmasaReg {
        <crate::RegValueT<UartDmasaReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartHtxReg_SPEC;
impl crate::sealed::RegSpec for UartHtxReg_SPEC {
    type DataType = u32;
}

pub type UartHtxReg = crate::RegValueT<UartHtxReg_SPEC>;

impl UartHtxReg {
    #[inline(always)]
    pub fn uart_halt_tx(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, UartHtxReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,UartHtxReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for UartHtxReg {
    #[inline(always)]
    fn default() -> UartHtxReg {
        <crate::RegValueT<UartHtxReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartIerDlhReg_SPEC;
impl crate::sealed::RegSpec for UartIerDlhReg_SPEC {
    type DataType = u32;
}

pub type UartIerDlhReg = crate::RegValueT<UartIerDlhReg_SPEC>;

impl UartIerDlhReg {
    #[inline(always)]
    pub fn ptime_dlh7(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, UartIerDlhReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,UartIerDlhReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dlh6_5(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, u8, u8, UartIerDlhReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x3,1,0,u8,u8,UartIerDlhReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn elcolr_dlh4(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, UartIerDlhReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,UartIerDlhReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn edssi_dlh3(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, UartIerDlhReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,UartIerDlhReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn elsi_dlh2(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, UartIerDlhReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,UartIerDlhReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn etbei_dlh1(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, UartIerDlhReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,UartIerDlhReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn erbfi_dlh0(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, UartIerDlhReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,UartIerDlhReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for UartIerDlhReg {
    #[inline(always)]
    fn default() -> UartIerDlhReg {
        <crate::RegValueT<UartIerDlhReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartIirFcrReg_SPEC;
impl crate::sealed::RegSpec for UartIirFcrReg_SPEC {
    type DataType = u32;
}

pub type UartIirFcrReg = crate::RegValueT<UartIirFcrReg_SPEC>;

impl UartIirFcrReg {
    #[inline(always)]
    pub fn iir_fcr(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, UartIirFcrReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,UartIirFcrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for UartIirFcrReg {
    #[inline(always)]
    fn default() -> UartIirFcrReg {
        <crate::RegValueT<UartIirFcrReg_SPEC> as RegisterValue<_>>::new(1)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartLcrReg_SPEC;
impl crate::sealed::RegSpec for UartLcrReg_SPEC {
    type DataType = u32;
}

pub type UartLcrReg = crate::RegValueT<UartLcrReg_SPEC>;

impl UartLcrReg {
    #[inline(always)]
    pub fn uart_dlab(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, UartLcrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,UartLcrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_bc(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, UartLcrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,UartLcrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_eps(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, UartLcrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,UartLcrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_pen(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, UartLcrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,UartLcrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_stop(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, UartLcrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,UartLcrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_dls(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, UartLcrReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,UartLcrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for UartLcrReg {
    #[inline(always)]
    fn default() -> UartLcrReg {
        <crate::RegValueT<UartLcrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartLsrReg_SPEC;
impl crate::sealed::RegSpec for UartLsrReg_SPEC {
    type DataType = u32;
}

pub type UartLsrReg = crate::RegValueT<UartLsrReg_SPEC>;

impl UartLsrReg {
    #[inline(always)]
    pub fn uart_rfe(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, UartLsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,UartLsrReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_temt(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, UartLsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,UartLsrReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_thre(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, UartLsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,UartLsrReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_bi(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, UartLsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,UartLsrReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_fe(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, UartLsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,UartLsrReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_pe(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, UartLsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,UartLsrReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_oe(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, UartLsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,UartLsrReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_dr(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, UartLsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,UartLsrReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for UartLsrReg {
    #[inline(always)]
    fn default() -> UartLsrReg {
        <crate::RegValueT<UartLsrReg_SPEC> as RegisterValue<_>>::new(96)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartMcrReg_SPEC;
impl crate::sealed::RegSpec for UartMcrReg_SPEC {
    type DataType = u32;
}

pub type UartMcrReg = crate::RegValueT<UartMcrReg_SPEC>;

impl UartMcrReg {
    #[inline(always)]
    pub fn uart_lb(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, UartMcrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,UartMcrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for UartMcrReg {
    #[inline(always)]
    fn default() -> UartMcrReg {
        <crate::RegValueT<UartMcrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartRbrThrDllReg_SPEC;
impl crate::sealed::RegSpec for UartRbrThrDllReg_SPEC {
    type DataType = u32;
}

pub type UartRbrThrDllReg = crate::RegValueT<UartRbrThrDllReg_SPEC>;

impl UartRbrThrDllReg {
    #[inline(always)]
    pub fn rbr_thr_dll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, UartRbrThrDllReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            UartRbrThrDllReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for UartRbrThrDllReg {
    #[inline(always)]
    fn default() -> UartRbrThrDllReg {
        <crate::RegValueT<UartRbrThrDllReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartRflReg_SPEC;
impl crate::sealed::RegSpec for UartRflReg_SPEC {
    type DataType = u32;
}

pub type UartRflReg = crate::RegValueT<UartRflReg_SPEC>;

impl UartRflReg {
    #[inline(always)]
    pub fn uart_receive_fifo_level(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, UartRflReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,UartRflReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for UartRflReg {
    #[inline(always)]
    fn default() -> UartRflReg {
        <crate::RegValueT<UartRflReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartSbcrReg_SPEC;
impl crate::sealed::RegSpec for UartSbcrReg_SPEC {
    type DataType = u32;
}

pub type UartSbcrReg = crate::RegValueT<UartSbcrReg_SPEC>;

impl UartSbcrReg {
    #[inline(always)]
    pub fn uart_shadow_break_control(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, UartSbcrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,UartSbcrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for UartSbcrReg {
    #[inline(always)]
    fn default() -> UartSbcrReg {
        <crate::RegValueT<UartSbcrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartScrReg_SPEC;
impl crate::sealed::RegSpec for UartScrReg_SPEC {
    type DataType = u32;
}

pub type UartScrReg = crate::RegValueT<UartScrReg_SPEC>;

impl UartScrReg {
    #[inline(always)]
    pub fn uart_scratch_pad(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, UartScrReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,UartScrReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for UartScrReg {
    #[inline(always)]
    fn default() -> UartScrReg {
        <crate::RegValueT<UartScrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartSdmamReg_SPEC;
impl crate::sealed::RegSpec for UartSdmamReg_SPEC {
    type DataType = u32;
}

pub type UartSdmamReg = crate::RegValueT<UartSdmamReg_SPEC>;

impl UartSdmamReg {
    #[inline(always)]
    pub fn uart_shadow_dma_mode(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, UartSdmamReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,UartSdmamReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for UartSdmamReg {
    #[inline(always)]
    fn default() -> UartSdmamReg {
        <crate::RegValueT<UartSdmamReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartSfeReg_SPEC;
impl crate::sealed::RegSpec for UartSfeReg_SPEC {
    type DataType = u32;
}

pub type UartSfeReg = crate::RegValueT<UartSfeReg_SPEC>;

impl UartSfeReg {
    #[inline(always)]
    pub fn uart_shadow_fifo_enable(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, UartSfeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,UartSfeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for UartSfeReg {
    #[inline(always)]
    fn default() -> UartSfeReg {
        <crate::RegValueT<UartSfeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartSrbrSthr0Reg_SPEC;
impl crate::sealed::RegSpec for UartSrbrSthr0Reg_SPEC {
    type DataType = u32;
}

pub type UartSrbrSthr0Reg = crate::RegValueT<UartSrbrSthr0Reg_SPEC>;

impl UartSrbrSthr0Reg {
    #[inline(always)]
    pub fn srbr_sthrx(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, UartSrbrSthr0Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            UartSrbrSthr0Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for UartSrbrSthr0Reg {
    #[inline(always)]
    fn default() -> UartSrbrSthr0Reg {
        <crate::RegValueT<UartSrbrSthr0Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartSrbrSthr10Reg_SPEC;
impl crate::sealed::RegSpec for UartSrbrSthr10Reg_SPEC {
    type DataType = u32;
}

pub type UartSrbrSthr10Reg = crate::RegValueT<UartSrbrSthr10Reg_SPEC>;

impl UartSrbrSthr10Reg {
    #[inline(always)]
    pub fn srbr_sthrx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        UartSrbrSthr10Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            UartSrbrSthr10Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for UartSrbrSthr10Reg {
    #[inline(always)]
    fn default() -> UartSrbrSthr10Reg {
        <crate::RegValueT<UartSrbrSthr10Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartSrbrSthr11Reg_SPEC;
impl crate::sealed::RegSpec for UartSrbrSthr11Reg_SPEC {
    type DataType = u32;
}

pub type UartSrbrSthr11Reg = crate::RegValueT<UartSrbrSthr11Reg_SPEC>;

impl UartSrbrSthr11Reg {
    #[inline(always)]
    pub fn srbr_sthrx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        UartSrbrSthr11Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            UartSrbrSthr11Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for UartSrbrSthr11Reg {
    #[inline(always)]
    fn default() -> UartSrbrSthr11Reg {
        <crate::RegValueT<UartSrbrSthr11Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartSrbrSthr12Reg_SPEC;
impl crate::sealed::RegSpec for UartSrbrSthr12Reg_SPEC {
    type DataType = u32;
}

pub type UartSrbrSthr12Reg = crate::RegValueT<UartSrbrSthr12Reg_SPEC>;

impl UartSrbrSthr12Reg {
    #[inline(always)]
    pub fn srbr_sthrx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        UartSrbrSthr12Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            UartSrbrSthr12Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for UartSrbrSthr12Reg {
    #[inline(always)]
    fn default() -> UartSrbrSthr12Reg {
        <crate::RegValueT<UartSrbrSthr12Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartSrbrSthr13Reg_SPEC;
impl crate::sealed::RegSpec for UartSrbrSthr13Reg_SPEC {
    type DataType = u32;
}

pub type UartSrbrSthr13Reg = crate::RegValueT<UartSrbrSthr13Reg_SPEC>;

impl UartSrbrSthr13Reg {
    #[inline(always)]
    pub fn srbr_sthrx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        UartSrbrSthr13Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            UartSrbrSthr13Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for UartSrbrSthr13Reg {
    #[inline(always)]
    fn default() -> UartSrbrSthr13Reg {
        <crate::RegValueT<UartSrbrSthr13Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartSrbrSthr14Reg_SPEC;
impl crate::sealed::RegSpec for UartSrbrSthr14Reg_SPEC {
    type DataType = u32;
}

pub type UartSrbrSthr14Reg = crate::RegValueT<UartSrbrSthr14Reg_SPEC>;

impl UartSrbrSthr14Reg {
    #[inline(always)]
    pub fn srbr_sthrx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        UartSrbrSthr14Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            UartSrbrSthr14Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for UartSrbrSthr14Reg {
    #[inline(always)]
    fn default() -> UartSrbrSthr14Reg {
        <crate::RegValueT<UartSrbrSthr14Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartSrbrSthr15Reg_SPEC;
impl crate::sealed::RegSpec for UartSrbrSthr15Reg_SPEC {
    type DataType = u32;
}

pub type UartSrbrSthr15Reg = crate::RegValueT<UartSrbrSthr15Reg_SPEC>;

impl UartSrbrSthr15Reg {
    #[inline(always)]
    pub fn srbr_sthrx(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        UartSrbrSthr15Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            UartSrbrSthr15Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for UartSrbrSthr15Reg {
    #[inline(always)]
    fn default() -> UartSrbrSthr15Reg {
        <crate::RegValueT<UartSrbrSthr15Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartSrbrSthr1Reg_SPEC;
impl crate::sealed::RegSpec for UartSrbrSthr1Reg_SPEC {
    type DataType = u32;
}

pub type UartSrbrSthr1Reg = crate::RegValueT<UartSrbrSthr1Reg_SPEC>;

impl UartSrbrSthr1Reg {
    #[inline(always)]
    pub fn srbr_sthrx(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, UartSrbrSthr1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            UartSrbrSthr1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for UartSrbrSthr1Reg {
    #[inline(always)]
    fn default() -> UartSrbrSthr1Reg {
        <crate::RegValueT<UartSrbrSthr1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartSrbrSthr2Reg_SPEC;
impl crate::sealed::RegSpec for UartSrbrSthr2Reg_SPEC {
    type DataType = u32;
}

pub type UartSrbrSthr2Reg = crate::RegValueT<UartSrbrSthr2Reg_SPEC>;

impl UartSrbrSthr2Reg {
    #[inline(always)]
    pub fn srbr_sthrx(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, UartSrbrSthr2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            UartSrbrSthr2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for UartSrbrSthr2Reg {
    #[inline(always)]
    fn default() -> UartSrbrSthr2Reg {
        <crate::RegValueT<UartSrbrSthr2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartSrbrSthr3Reg_SPEC;
impl crate::sealed::RegSpec for UartSrbrSthr3Reg_SPEC {
    type DataType = u32;
}

pub type UartSrbrSthr3Reg = crate::RegValueT<UartSrbrSthr3Reg_SPEC>;

impl UartSrbrSthr3Reg {
    #[inline(always)]
    pub fn srbr_sthrx(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, UartSrbrSthr3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            UartSrbrSthr3Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for UartSrbrSthr3Reg {
    #[inline(always)]
    fn default() -> UartSrbrSthr3Reg {
        <crate::RegValueT<UartSrbrSthr3Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartSrbrSthr4Reg_SPEC;
impl crate::sealed::RegSpec for UartSrbrSthr4Reg_SPEC {
    type DataType = u32;
}

pub type UartSrbrSthr4Reg = crate::RegValueT<UartSrbrSthr4Reg_SPEC>;

impl UartSrbrSthr4Reg {
    #[inline(always)]
    pub fn srbr_sthrx(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, UartSrbrSthr4Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            UartSrbrSthr4Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for UartSrbrSthr4Reg {
    #[inline(always)]
    fn default() -> UartSrbrSthr4Reg {
        <crate::RegValueT<UartSrbrSthr4Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartSrbrSthr5Reg_SPEC;
impl crate::sealed::RegSpec for UartSrbrSthr5Reg_SPEC {
    type DataType = u32;
}

pub type UartSrbrSthr5Reg = crate::RegValueT<UartSrbrSthr5Reg_SPEC>;

impl UartSrbrSthr5Reg {
    #[inline(always)]
    pub fn srbr_sthrx(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, UartSrbrSthr5Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            UartSrbrSthr5Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for UartSrbrSthr5Reg {
    #[inline(always)]
    fn default() -> UartSrbrSthr5Reg {
        <crate::RegValueT<UartSrbrSthr5Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartSrbrSthr6Reg_SPEC;
impl crate::sealed::RegSpec for UartSrbrSthr6Reg_SPEC {
    type DataType = u32;
}

pub type UartSrbrSthr6Reg = crate::RegValueT<UartSrbrSthr6Reg_SPEC>;

impl UartSrbrSthr6Reg {
    #[inline(always)]
    pub fn srbr_sthrx(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, UartSrbrSthr6Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            UartSrbrSthr6Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for UartSrbrSthr6Reg {
    #[inline(always)]
    fn default() -> UartSrbrSthr6Reg {
        <crate::RegValueT<UartSrbrSthr6Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartSrbrSthr7Reg_SPEC;
impl crate::sealed::RegSpec for UartSrbrSthr7Reg_SPEC {
    type DataType = u32;
}

pub type UartSrbrSthr7Reg = crate::RegValueT<UartSrbrSthr7Reg_SPEC>;

impl UartSrbrSthr7Reg {
    #[inline(always)]
    pub fn srbr_sthrx(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, UartSrbrSthr7Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            UartSrbrSthr7Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for UartSrbrSthr7Reg {
    #[inline(always)]
    fn default() -> UartSrbrSthr7Reg {
        <crate::RegValueT<UartSrbrSthr7Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartSrbrSthr8Reg_SPEC;
impl crate::sealed::RegSpec for UartSrbrSthr8Reg_SPEC {
    type DataType = u32;
}

pub type UartSrbrSthr8Reg = crate::RegValueT<UartSrbrSthr8Reg_SPEC>;

impl UartSrbrSthr8Reg {
    #[inline(always)]
    pub fn srbr_sthrx(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, UartSrbrSthr8Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            UartSrbrSthr8Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for UartSrbrSthr8Reg {
    #[inline(always)]
    fn default() -> UartSrbrSthr8Reg {
        <crate::RegValueT<UartSrbrSthr8Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartSrbrSthr9Reg_SPEC;
impl crate::sealed::RegSpec for UartSrbrSthr9Reg_SPEC {
    type DataType = u32;
}

pub type UartSrbrSthr9Reg = crate::RegValueT<UartSrbrSthr9Reg_SPEC>;

impl UartSrbrSthr9Reg {
    #[inline(always)]
    pub fn srbr_sthrx(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, UartSrbrSthr9Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            UartSrbrSthr9Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for UartSrbrSthr9Reg {
    #[inline(always)]
    fn default() -> UartSrbrSthr9Reg {
        <crate::RegValueT<UartSrbrSthr9Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartSrrReg_SPEC;
impl crate::sealed::RegSpec for UartSrrReg_SPEC {
    type DataType = u32;
}

pub type UartSrrReg = crate::RegValueT<UartSrrReg_SPEC>;

impl UartSrrReg {
    #[inline(always)]
    pub fn uart_xfr(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, UartSrrReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2,1,0,UartSrrReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_rfr(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, UartSrrReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1,1,0,UartSrrReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_ur(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, UartSrrReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0,1,0,UartSrrReg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for UartSrrReg {
    #[inline(always)]
    fn default() -> UartSrrReg {
        <crate::RegValueT<UartSrrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartSrtReg_SPEC;
impl crate::sealed::RegSpec for UartSrtReg_SPEC {
    type DataType = u32;
}

pub type UartSrtReg = crate::RegValueT<UartSrtReg_SPEC>;

impl UartSrtReg {
    #[inline(always)]
    pub fn uart_shadow_rcvr_trigger(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, UartSrtReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,UartSrtReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for UartSrtReg {
    #[inline(always)]
    fn default() -> UartSrtReg {
        <crate::RegValueT<UartSrtReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartStetReg_SPEC;
impl crate::sealed::RegSpec for UartStetReg_SPEC {
    type DataType = u32;
}

pub type UartStetReg = crate::RegValueT<UartStetReg_SPEC>;

impl UartStetReg {
    #[inline(always)]
    pub fn uart_shadow_tx_empty_trigger(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, UartStetReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,UartStetReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for UartStetReg {
    #[inline(always)]
    fn default() -> UartStetReg {
        <crate::RegValueT<UartStetReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartTflReg_SPEC;
impl crate::sealed::RegSpec for UartTflReg_SPEC {
    type DataType = u32;
}

pub type UartTflReg = crate::RegValueT<UartTflReg_SPEC>;

impl UartTflReg {
    #[inline(always)]
    pub fn uart_transmit_fifo_level(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, UartTflReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,UartTflReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for UartTflReg {
    #[inline(always)]
    fn default() -> UartTflReg {
        <crate::RegValueT<UartTflReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartUcvReg_SPEC;
impl crate::sealed::RegSpec for UartUcvReg_SPEC {
    type DataType = u32;
}

pub type UartUcvReg = crate::RegValueT<UartUcvReg_SPEC>;

impl UartUcvReg {
    #[inline(always)]
    pub fn uart_ucv(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        UartUcvReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            UartUcvReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for UartUcvReg {
    #[inline(always)]
    fn default() -> UartUcvReg {
        <crate::RegValueT<UartUcvReg_SPEC> as RegisterValue<_>>::new(875573546)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartUsrReg_SPEC;
impl crate::sealed::RegSpec for UartUsrReg_SPEC {
    type DataType = u32;
}

pub type UartUsrReg = crate::RegValueT<UartUsrReg_SPEC>;

impl UartUsrReg {
    #[inline(always)]
    pub fn uart_rff(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, UartUsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,UartUsrReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_rfne(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, UartUsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,UartUsrReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_tfe(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, UartUsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,UartUsrReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_tfnf(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, UartUsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,UartUsrReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_busy(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, UartUsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,UartUsrReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for UartUsrReg {
    #[inline(always)]
    fn default() -> UartUsrReg {
        <crate::RegValueT<UartUsrReg_SPEC> as RegisterValue<_>>::new(6)
    }
}
