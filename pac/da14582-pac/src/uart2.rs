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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:15:32 +0000

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

    #[inline(always)]
    pub const fn uart2_lpdlh_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart2LpdlhReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart2LpdlhReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[inline(always)]
    pub const fn uart2_lpdll_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Uart2LpdllReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Uart2LpdllReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

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

pub type Uart2CprReg = crate::RegValueT<Uart2CprReg_SPEC>;

impl Uart2CprReg {
    #[inline(always)]
    pub fn cpr(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Uart2CprReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Uart2CprReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart2CprReg {
    #[inline(always)]
    fn default() -> Uart2CprReg {
        <crate::RegValueT<Uart2CprReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart2CtrReg_SPEC;
impl crate::sealed::RegSpec for Uart2CtrReg_SPEC {
    type DataType = u16;
}

pub type Uart2CtrReg = crate::RegValueT<Uart2CtrReg_SPEC>;

impl Uart2CtrReg {
    #[inline(always)]
    pub fn ctr(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Uart2CtrReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Uart2CtrReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart2CtrReg {
    #[inline(always)]
    fn default() -> Uart2CtrReg {
        <crate::RegValueT<Uart2CtrReg_SPEC> as RegisterValue<_>>::new(272)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart2HtxReg_SPEC;
impl crate::sealed::RegSpec for Uart2HtxReg_SPEC {
    type DataType = u16;
}

pub type Uart2HtxReg = crate::RegValueT<Uart2HtxReg_SPEC>;

impl Uart2HtxReg {
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

pub type Uart2IerDlhReg = crate::RegValueT<Uart2IerDlhReg_SPEC>;

impl Uart2IerDlhReg {
    #[inline(always)]
    pub fn ptime_dlh7(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Uart2IerDlhReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,Uart2IerDlhReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn edssi_dlh3(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Uart2IerDlhReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,Uart2IerDlhReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn elsi_dhl2(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Uart2IerDlhReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,Uart2IerDlhReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn etbei_dlh1(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Uart2IerDlhReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,Uart2IerDlhReg_SPEC,crate::common::RW>::from_register(self,0)
    }

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

pub type Uart2IirFcrReg = crate::RegValueT<Uart2IirFcrReg_SPEC>;

impl Uart2IirFcrReg {
    #[inline(always)]
    pub fn iir_fcr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        Uart2IirFcrReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            Uart2IirFcrReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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

pub type Uart2LcrReg = crate::RegValueT<Uart2LcrReg_SPEC>;

impl Uart2LcrReg {
    #[inline(always)]
    pub fn uart_dlab(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Uart2LcrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,Uart2LcrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_bc(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Uart2LcrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,Uart2LcrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_eps(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Uart2LcrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,Uart2LcrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_pen(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Uart2LcrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,Uart2LcrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_stop(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Uart2LcrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,Uart2LcrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_dls(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, Uart2LcrReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,Uart2LcrReg_SPEC,crate::common::RW>::from_register(self,0)
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
pub struct Uart2LpdlhReg_SPEC;
impl crate::sealed::RegSpec for Uart2LpdlhReg_SPEC {
    type DataType = u16;
}

pub type Uart2LpdlhReg = crate::RegValueT<Uart2LpdlhReg_SPEC>;

impl Uart2LpdlhReg {
    #[inline(always)]
    pub fn uart_lpdlh(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Uart2LpdlhReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Uart2LpdlhReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart2LpdlhReg {
    #[inline(always)]
    fn default() -> Uart2LpdlhReg {
        <crate::RegValueT<Uart2LpdlhReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart2LpdllReg_SPEC;
impl crate::sealed::RegSpec for Uart2LpdllReg_SPEC {
    type DataType = u16;
}

pub type Uart2LpdllReg = crate::RegValueT<Uart2LpdllReg_SPEC>;

impl Uart2LpdllReg {
    #[inline(always)]
    pub fn uart_lpdll(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Uart2LpdllReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Uart2LpdllReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart2LpdllReg {
    #[inline(always)]
    fn default() -> Uart2LpdllReg {
        <crate::RegValueT<Uart2LpdllReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart2LsrReg_SPEC;
impl crate::sealed::RegSpec for Uart2LsrReg_SPEC {
    type DataType = u16;
}

pub type Uart2LsrReg = crate::RegValueT<Uart2LsrReg_SPEC>;

impl Uart2LsrReg {
    #[inline(always)]
    pub fn uart_rfe(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Uart2LsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,Uart2LsrReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_temt(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Uart2LsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,Uart2LsrReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_thre(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Uart2LsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,Uart2LsrReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_b1(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Uart2LsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,Uart2LsrReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_fe(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Uart2LsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,Uart2LsrReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_pe(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Uart2LsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,Uart2LsrReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_oe(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Uart2LsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,Uart2LsrReg_SPEC,crate::common::R>::from_register(self,0)
    }

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

pub type Uart2McrReg = crate::RegValueT<Uart2McrReg_SPEC>;

impl Uart2McrReg {
    #[inline(always)]
    pub fn uart_sire(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Uart2McrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,Uart2McrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_afce(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Uart2McrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,Uart2McrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_lb(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Uart2McrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,Uart2McrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_out2(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Uart2McrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,Uart2McrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_out1(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Uart2McrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,Uart2McrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

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

pub type Uart2MsrReg = crate::RegValueT<Uart2MsrReg_SPEC>;

impl Uart2MsrReg {
    #[inline(always)]
    pub fn uart_dcd(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Uart2MsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,Uart2MsrReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_r1(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Uart2MsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,Uart2MsrReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_cts(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Uart2MsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,Uart2MsrReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_ddcd(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Uart2MsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,Uart2MsrReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_teri(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Uart2MsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,Uart2MsrReg_SPEC,crate::common::R>::from_register(self,0)
    }

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

pub type Uart2RbrThrDllReg = crate::RegValueT<Uart2RbrThrDllReg_SPEC>;

impl Uart2RbrThrDllReg {
    #[inline(always)]
    pub fn rbr_thr_dll(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        Uart2RbrThrDllReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            Uart2RbrThrDllReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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

pub type Uart2RflReg = crate::RegValueT<Uart2RflReg_SPEC>;

impl Uart2RflReg {
    #[inline(always)]
    pub fn uart_receive_fifo_level(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Uart2RflReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Uart2RflReg_SPEC,crate::common::R>::from_register(self,0)
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

pub type Uart2SbcrReg = crate::RegValueT<Uart2SbcrReg_SPEC>;

impl Uart2SbcrReg {
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

pub type Uart2ScrReg = crate::RegValueT<Uart2ScrReg_SPEC>;

impl Uart2ScrReg {
    #[inline(always)]
    pub fn uart_scratch_pad(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, Uart2ScrReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,Uart2ScrReg_SPEC,crate::common::RW>::from_register(self,0)
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

pub type Uart2SdmamReg = crate::RegValueT<Uart2SdmamReg_SPEC>;

impl Uart2SdmamReg {
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

pub type Uart2SfeReg = crate::RegValueT<Uart2SfeReg_SPEC>;

impl Uart2SfeReg {
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

pub type Uart2SrbrSthr0Reg = crate::RegValueT<Uart2SrbrSthr0Reg_SPEC>;

impl Uart2SrbrSthr0Reg {
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
        Uart2SrbrSthr0Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            Uart2SrbrSthr0Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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

pub type Uart2SrbrSthr10Reg = crate::RegValueT<Uart2SrbrSthr10Reg_SPEC>;

impl Uart2SrbrSthr10Reg {
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
        Uart2SrbrSthr10Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            Uart2SrbrSthr10Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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

pub type Uart2SrbrSthr11Reg = crate::RegValueT<Uart2SrbrSthr11Reg_SPEC>;

impl Uart2SrbrSthr11Reg {
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
        Uart2SrbrSthr11Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            Uart2SrbrSthr11Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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

pub type Uart2SrbrSthr12Reg = crate::RegValueT<Uart2SrbrSthr12Reg_SPEC>;

impl Uart2SrbrSthr12Reg {
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
        Uart2SrbrSthr12Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            Uart2SrbrSthr12Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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

pub type Uart2SrbrSthr13Reg = crate::RegValueT<Uart2SrbrSthr13Reg_SPEC>;

impl Uart2SrbrSthr13Reg {
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
        Uart2SrbrSthr13Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            Uart2SrbrSthr13Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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

pub type Uart2SrbrSthr14Reg = crate::RegValueT<Uart2SrbrSthr14Reg_SPEC>;

impl Uart2SrbrSthr14Reg {
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
        Uart2SrbrSthr14Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            Uart2SrbrSthr14Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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

pub type Uart2SrbrSthr15Reg = crate::RegValueT<Uart2SrbrSthr15Reg_SPEC>;

impl Uart2SrbrSthr15Reg {
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
        Uart2SrbrSthr15Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            Uart2SrbrSthr15Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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

pub type Uart2SrbrSthr1Reg = crate::RegValueT<Uart2SrbrSthr1Reg_SPEC>;

impl Uart2SrbrSthr1Reg {
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
        Uart2SrbrSthr1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            Uart2SrbrSthr1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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

pub type Uart2SrbrSthr2Reg = crate::RegValueT<Uart2SrbrSthr2Reg_SPEC>;

impl Uart2SrbrSthr2Reg {
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
        Uart2SrbrSthr2Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            Uart2SrbrSthr2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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

pub type Uart2SrbrSthr3Reg = crate::RegValueT<Uart2SrbrSthr3Reg_SPEC>;

impl Uart2SrbrSthr3Reg {
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
        Uart2SrbrSthr3Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            Uart2SrbrSthr3Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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

pub type Uart2SrbrSthr4Reg = crate::RegValueT<Uart2SrbrSthr4Reg_SPEC>;

impl Uart2SrbrSthr4Reg {
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
        Uart2SrbrSthr4Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            Uart2SrbrSthr4Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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

pub type Uart2SrbrSthr5Reg = crate::RegValueT<Uart2SrbrSthr5Reg_SPEC>;

impl Uart2SrbrSthr5Reg {
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
        Uart2SrbrSthr5Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            Uart2SrbrSthr5Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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

pub type Uart2SrbrSthr6Reg = crate::RegValueT<Uart2SrbrSthr6Reg_SPEC>;

impl Uart2SrbrSthr6Reg {
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
        Uart2SrbrSthr6Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            Uart2SrbrSthr6Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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

pub type Uart2SrbrSthr7Reg = crate::RegValueT<Uart2SrbrSthr7Reg_SPEC>;

impl Uart2SrbrSthr7Reg {
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
        Uart2SrbrSthr7Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            Uart2SrbrSthr7Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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

pub type Uart2SrbrSthr8Reg = crate::RegValueT<Uart2SrbrSthr8Reg_SPEC>;

impl Uart2SrbrSthr8Reg {
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
        Uart2SrbrSthr8Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            Uart2SrbrSthr8Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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

pub type Uart2SrbrSthr9Reg = crate::RegValueT<Uart2SrbrSthr9Reg_SPEC>;

impl Uart2SrbrSthr9Reg {
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
        Uart2SrbrSthr9Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            Uart2SrbrSthr9Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
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

pub type Uart2SrrReg = crate::RegValueT<Uart2SrrReg_SPEC>;

impl Uart2SrrReg {
    #[inline(always)]
    pub fn uart_xfr(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Uart2SrrReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2,1,0,Uart2SrrReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_rfr(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Uart2SrrReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1,1,0,Uart2SrrReg_SPEC,crate::common::W>::from_register(self,0)
    }

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

pub type Uart2SrtsReg = crate::RegValueT<Uart2SrtsReg_SPEC>;

impl Uart2SrtsReg {
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

pub type Uart2SrtReg = crate::RegValueT<Uart2SrtReg_SPEC>;

impl Uart2SrtReg {
    #[inline(always)]
    pub fn uart_shadow_rcvr_trigger(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, Uart2SrtReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,Uart2SrtReg_SPEC,crate::common::RW>::from_register(self,0)
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

pub type Uart2StetReg = crate::RegValueT<Uart2StetReg_SPEC>;

impl Uart2StetReg {
    #[inline(always)]
    pub fn uart_shadow_tx_empty_trigger(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, u8, Uart2StetReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8,u8,Uart2StetReg_SPEC,crate::common::RW>::from_register(self,0)
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

pub type Uart2TflReg = crate::RegValueT<Uart2TflReg_SPEC>;

impl Uart2TflReg {
    #[inline(always)]
    pub fn uart_transmit_fifo_level(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Uart2TflReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Uart2TflReg_SPEC,crate::common::R>::from_register(self,0)
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
    type DataType = u16;
}

pub type Uart2UcvReg = crate::RegValueT<Uart2UcvReg_SPEC>;

impl Uart2UcvReg {
    #[inline(always)]
    pub fn ucv(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, u16, Uart2UcvReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16,u16,Uart2UcvReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart2UcvReg {
    #[inline(always)]
    fn default() -> Uart2UcvReg {
        <crate::RegValueT<Uart2UcvReg_SPEC> as RegisterValue<_>>::new(14378)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart2UsrReg_SPEC;
impl crate::sealed::RegSpec for Uart2UsrReg_SPEC {
    type DataType = u16;
}

pub type Uart2UsrReg = crate::RegValueT<Uart2UsrReg_SPEC>;

impl Uart2UsrReg {
    #[inline(always)]
    pub fn uart_rff(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Uart2UsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,Uart2UsrReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_rfne(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Uart2UsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,Uart2UsrReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_tfe(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Uart2UsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,Uart2UsrReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn uart_tfnf(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Uart2UsrReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,Uart2UsrReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Uart2UsrReg {
    #[inline(always)]
    fn default() -> Uart2UsrReg {
        <crate::RegValueT<Uart2UsrReg_SPEC> as RegisterValue<_>>::new(6)
    }
}
