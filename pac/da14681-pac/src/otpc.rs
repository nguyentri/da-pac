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
// Generated from SVD 1.2, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:45:04 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"OTPC registers"]
unsafe impl ::core::marker::Send for super::Otpc {}
unsafe impl ::core::marker::Sync for super::Otpc {}
impl super::Otpc {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "AHB master start address"]
    #[inline(always)]
    pub const fn otpc_ahbadr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OtpcAhbadrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OtpcAhbadrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Macrocell start address"]
    #[inline(always)]
    pub const fn otpc_celadr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OtpcCeladrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OtpcCeladrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Ports access to fifo logic"]
    #[inline(always)]
    pub const fn otpc_ffprt_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OtpcFfprtReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OtpcFfprtReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[doc = "The data which have taken with the latest read from the OTPC_FFPRT_REG"]
    #[inline(always)]
    pub const fn otpc_ffrd_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OtpcFfrdReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OtpcFfrdReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "Mode register"]
    #[inline(always)]
    pub const fn otpc_mode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OtpcModeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OtpcModeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Number of words"]
    #[inline(always)]
    pub const fn otpc_nwords_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OtpcNwordsReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OtpcNwordsReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "Bit-programming control register"]
    #[inline(always)]
    pub const fn otpc_pctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OtpcPctrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OtpcPctrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "The 32 higher bits of the 64-bit word that will be programmed, when the MPROG mode is used."]
    #[inline(always)]
    pub const fn otpc_pwordh_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OtpcPwordhReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OtpcPwordhReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[doc = "The 32 lower bits of the 64-bit word that will be programmed, when the MPROG mode is used."]
    #[inline(always)]
    pub const fn otpc_pwordl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OtpcPwordlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OtpcPwordlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "Status register"]
    #[inline(always)]
    pub const fn otpc_stat_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OtpcStatReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OtpcStatReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Various timing parameters of the OTP cell."]
    #[inline(always)]
    pub const fn otpc_tim1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OtpcTim1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OtpcTim1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[doc = "Various timing parameters of the OTP cell."]
    #[inline(always)]
    pub const fn otpc_tim2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OtpcTim2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OtpcTim2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OtpcAhbadrReg_SPEC;
impl crate::sealed::RegSpec for OtpcAhbadrReg_SPEC {
    type DataType = u32;
}

#[doc = "AHB master start address"]
pub type OtpcAhbadrReg = crate::RegValueT<OtpcAhbadrReg_SPEC>;

impl OtpcAhbadrReg {
    #[doc = "It is the AHB address used by the AHB master interface of the controller (the bits \\[31:2\\]). The bits \\[1:0\\] of the address are considered always as equal to zero."]
    #[inline(always)]
    pub fn otpc_ahbadr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x3fffffff,
        1,
        0,
        u32,
        u32,
        OtpcAhbadrReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x3fffffff,
            1,
            0,
            u32,
            u32,
            OtpcAhbadrReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OtpcAhbadrReg {
    #[inline(always)]
    fn default() -> OtpcAhbadrReg {
        <crate::RegValueT<OtpcAhbadrReg_SPEC> as RegisterValue<_>>::new(133955584)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OtpcCeladrReg_SPEC;
impl crate::sealed::RegSpec for OtpcCeladrReg_SPEC {
    type DataType = u32;
}

#[doc = "Macrocell start address"]
pub type OtpcCeladrReg = crate::RegValueT<OtpcCeladrReg_SPEC>;

impl OtpcCeladrReg {
    #[doc = "It represents an OTP address, where the OTP word width should be considered equal to 32-bits.\nThe physical word width of the OTP memory is 72 bits. The 8-bits of them are used for the implementation of an error correcting code and are not available for the application. The remaining 64 bits of the physical word are available for the application. \nThe OTPC_CELADDR can distinguish the upper 32 bits from the lower 32 bits of the available for the application bits of the OTP word.\nWhen OTPC_CELADDR\\[0\\] = 1 the address refers to the upper 32 bits of the physical OTP address OTPC_CELADDR\\[14:1\\].\nThe register is used during the modes: AREAD and APROG."]
    #[inline(always)]
    pub fn otpc_celadr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3fff,
        1,
        0,
        u16,
        u16,
        OtpcCeladrReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3fff,
            1,
            0,
            u16,
            u16,
            OtpcCeladrReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OtpcCeladrReg {
    #[inline(always)]
    fn default() -> OtpcCeladrReg {
        <crate::RegValueT<OtpcCeladrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OtpcFfprtReg_SPEC;
impl crate::sealed::RegSpec for OtpcFfprtReg_SPEC {
    type DataType = u32;
}

#[doc = "Ports access to fifo logic"]
pub type OtpcFfprtReg = crate::RegValueT<OtpcFfprtReg_SPEC>;

impl OtpcFfprtReg {
    #[doc = "Provides access to the fifo through an access port.\nWrite to this register with the corresponding data, when the APROG mode is selected and the dma is disabled.\nRead from this register the corresponding data, when the AREAD mode is selected and the dma is disabled.\nThe software should check the OTPCC_STAT_FWORDS register for the availability of data/space, before accessing the fifo."]
    #[inline(always)]
    pub fn otpc_ffprt(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        OtpcFfprtReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            OtpcFfprtReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OtpcFfprtReg {
    #[inline(always)]
    fn default() -> OtpcFfprtReg {
        <crate::RegValueT<OtpcFfprtReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OtpcFfrdReg_SPEC;
impl crate::sealed::RegSpec for OtpcFfrdReg_SPEC {
    type DataType = u32;
}

#[doc = "The data which have taken with the latest read from the OTPC_FFPRT_REG"]
pub type OtpcFfrdReg = crate::RegValueT<OtpcFfrdReg_SPEC>;

impl OtpcFfrdReg {
    #[doc = "Contains the value which taken from the fifo, after a read of the OTPC_FFPRT_REG register."]
    #[inline(always)]
    pub fn otpc_ffrd(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        OtpcFfrdReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            OtpcFfrdReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OtpcFfrdReg {
    #[inline(always)]
    fn default() -> OtpcFfrdReg {
        <crate::RegValueT<OtpcFfrdReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OtpcModeReg_SPEC;
impl crate::sealed::RegSpec for OtpcModeReg_SPEC {
    type DataType = u32;
}

#[doc = "Mode register"]
pub type OtpcModeReg = crate::RegValueT<OtpcModeReg_SPEC>;

impl OtpcModeReg {
    #[doc = "Write with 1 in order to be requested the reloading of the repair records. The reloading of the repair records will be performed at the next enabling of the OTP cell. That means that first the controller should be configured to the STBY mode and after should be activated any other mode. The hardware will clear this register, when the reloading will be performed.\nThe reloading has meaning only if the repair records have been updated manually (MPROG mode)."]
    #[inline(always)]
    pub fn otpc_mode_rld_rr_req(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, OtpcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,OtpcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Selects the memory area of the OTP cell that will be used.\n0 - Uses the normal memory area of the OTP cell\n1 - Uses the spare rows of the OTP cell\nThis selection has meaning only if the mode of the controller is not TDEC and TWR. The controller should be in STBY mode, in order to takes into account this bit. The selection will take effect at the next mode that will be enabled."]
    #[inline(always)]
    pub fn otpc_mode_use_sp_rows(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, OtpcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,OtpcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "When is performed a read from the OTP memory in the MREAD mode, a double error is likely be detected during the retrieving of the data from the OTP. This error condition is always indicated in the status bit OTPC_STAT_REG\\[OTPC_STAT_RERROR\\]. However, the OTP controller has also the ability to indicates this error condition, by generating an ERROR response in the AHB bus.\nThe generation of the ERROR response can be avoided with the help of this configuration bit.\n0 - The OTP controller generates an ERROR response in the AHB bus, when a double error is detected during a reading in MREAD mode. The OTPC_STAT_REG\\[OTPC_STAT_RERROR\\] is also updated. The receiving of an ERROR response by the CPU causes a Hard Fault exception in the CPU.\n1 - Only the OTPC_STAT_REG\\[OTPC_STAT_RERROR\\] is updated in a case of such error. The OTP controller will not generate an ERROR response in the AHB bus."]
    #[inline(always)]
    pub fn otpc_mode_err_resp_dis(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, OtpcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,OtpcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "By writing with 1, removes any content from the fifo. This bit returns automatically to value 0."]
    #[inline(always)]
    pub fn otpc_mode_fifo_flush(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, OtpcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,OtpcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Selects the use of the dma, when the controller is configured in one of the modes: AREAD or APROG.\n0 - The dma is not used. The data should be transferred from/to controller through the register OTPC_FFPRT_REG.\n1 - The dma is used. The data transfers from/to controller are performed automatically, with the help of the internal DMA of the OTP controller. The AHB base address should be configured in register OTPC_AHBADR_REG, before the selection of one of the two modes: AREAD or APROG."]
    #[inline(always)]
    pub fn otpc_mode_use_dma(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, OtpcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,OtpcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Defines the mode of operation of the OTPC controller. The encoding of the modes is as follows:\n000 - STBY mode\n001 - MREAD mode\n010 - MPROG mode\n011 - AREAD mode\n100 - APROG mode\n101 - TBLANK mode\n110 - TDEC mode\n111 - TWR mode"]
    #[inline(always)]
    pub fn otpc_mode_mode(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, OtpcModeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8,u8,OtpcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for OtpcModeReg {
    #[inline(always)]
    fn default() -> OtpcModeReg {
        <crate::RegValueT<OtpcModeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OtpcNwordsReg_SPEC;
impl crate::sealed::RegSpec for OtpcNwordsReg_SPEC {
    type DataType = u32;
}

#[doc = "Number of words"]
pub type OtpcNwordsReg = crate::RegValueT<OtpcNwordsReg_SPEC>;

impl OtpcNwordsReg {
    #[doc = "The number of words (minus one) for reading /programming during the AREAD/APROG mode.\nThe width of the word should be considered equal to 32-bits.\nThe value of the register remains unchanged, by the internal logic of the controller.\nDuring mirroring, this register reflects the current ammount of copied data."]
    #[inline(always)]
    pub fn otpc_nwords(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3fff,
        1,
        0,
        u16,
        u16,
        OtpcNwordsReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3fff,
            1,
            0,
            u16,
            u16,
            OtpcNwordsReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OtpcNwordsReg {
    #[inline(always)]
    fn default() -> OtpcNwordsReg {
        <crate::RegValueT<OtpcNwordsReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OtpcPctrlReg_SPEC;
impl crate::sealed::RegSpec for OtpcPctrlReg_SPEC {
    type DataType = u32;
}

#[doc = "Bit-programming control register"]
pub type OtpcPctrlReg = crate::RegValueT<OtpcPctrlReg_SPEC>;

impl OtpcPctrlReg {
    #[doc = "Write with \'1\' to trigger the programming of one OTP word, in the case where the MPROG mode is selected. The bit is cleared automatically. The 64-bits that will be programmed into the OTP memory are contained into the two registers OTPC_PWORDx_REG.\nThis bit should be used when a new programming is initiated, but also when the programming must be retried.\nThe OTPC_PCTRL_WADDR defines the OTP position where will be performed the programming."]
    #[inline(always)]
    pub fn otpc_pctrl_pstart(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, OtpcPctrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,OtpcPctrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "It distinguishes the first attempt of a programming of an OTP position, from a retry of programming.\n0 - A new value will be programmed in a blank OTP position. The hardware will try to write all the bits that are equal to \'1\'.\n1 - The programming that is applied is not the first attempt, but is a request for reprogramming. Will be processed only the bits that were failed to be programmed during the previous attempt. The hardware knows the bits that were failed during the previous attempt.\nThe registers OTPC_PWORDx_REG should contain the 64 bits of the value that should be programmed, independent of the value of the OTPC_PCTRL_PRETRY bit.\nAlso, the OTPC_PCTRL_WADDR should contain always the required OTP address.\nA retry of a programming should be requested only if the previous action was the first attempt of programming or a retry of programming. Should not be requested a retry if the first attempt has not been performed."]
    #[inline(always)]
    pub fn otpc_pctrl_pretry(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, OtpcPctrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,OtpcPctrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Defines the OTP position where will be programmed the 64-bits that are contained into the registers OTPC_PWORDx_REG. It points to a physical 72 bits OTP word."]
    #[inline(always)]
    pub fn otpc_pctrl_waddr(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, u16, OtpcPctrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1fff,
            1,
            0,
            u16,
            u16,
            OtpcPctrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OtpcPctrlReg {
    #[inline(always)]
    fn default() -> OtpcPctrlReg {
        <crate::RegValueT<OtpcPctrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OtpcPwordhReg_SPEC;
impl crate::sealed::RegSpec for OtpcPwordhReg_SPEC {
    type DataType = u32;
}

#[doc = "The 32 higher bits of the 64-bit word that will be programmed, when the MPROG mode is used."]
pub type OtpcPwordhReg = crate::RegValueT<OtpcPwordhReg_SPEC>;

impl OtpcPwordhReg {
    #[doc = "Contains the upper 32 bits that can be programmed with the help of the OTPC_PCTRL_REG, while the controller is in MPROG mode."]
    #[inline(always)]
    pub fn otpc_pwordh(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        OtpcPwordhReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            OtpcPwordhReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OtpcPwordhReg {
    #[inline(always)]
    fn default() -> OtpcPwordhReg {
        <crate::RegValueT<OtpcPwordhReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OtpcPwordlReg_SPEC;
impl crate::sealed::RegSpec for OtpcPwordlReg_SPEC {
    type DataType = u32;
}

#[doc = "The 32 lower bits of the 64-bit word that will be programmed, when the MPROG mode is used."]
pub type OtpcPwordlReg = crate::RegValueT<OtpcPwordlReg_SPEC>;

impl OtpcPwordlReg {
    #[doc = "Contains the lower 32 bits that can be programmed with the help of the OTPC_PCTRL_REG, while the controller is in MPROG mode."]
    #[inline(always)]
    pub fn otpc_pwordl(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        OtpcPwordlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            OtpcPwordlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OtpcPwordlReg {
    #[inline(always)]
    fn default() -> OtpcPwordlReg {
        <crate::RegValueT<OtpcPwordlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OtpcStatReg_SPEC;
impl crate::sealed::RegSpec for OtpcStatReg_SPEC {
    type DataType = u32;
}

#[doc = "Status register"]
pub type OtpcStatReg = crate::RegValueT<OtpcStatReg_SPEC>;

impl OtpcStatReg {
    #[doc = "It contains the \"live\" value of the number of (32 bits) words that remain to be processed by the controller."]
    #[inline(always)]
    pub fn otpc_stat_nwords(
        self,
    ) -> crate::common::RegisterField<16, 0x3fff, 1, 0, u16, u16, OtpcStatReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0x3fff,1,0,u16,u16,OtpcStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Indicates the number of words which contained in the fifo of the controller."]
    #[inline(always)]
    pub fn otpc_stat_fwords(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, u8, OtpcStatReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0xf,1,0,u8,u8,OtpcStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Indicates that during a normal reading (MREAD or AREAD) was reported a double error by the SECDED logic. That means that the data are corrupted.\n0 - The read data are considered as correct.\n1- The SECDED logic detects a double error.\nThis bit can be cleared only with a write with \'1\'."]
    #[inline(always)]
    pub fn otpc_stat_rerror(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, OtpcStatReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,OtpcStatReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Should be used to monitor the progress of the AREAD and APROG modes.\n0 - One of the APROG or AREAD mode is selected. The controller is busy.\n1 - The controller is not in an active AREAD or APROG mode."]
    #[inline(always)]
    pub fn otpc_stat_ardy(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, OtpcStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,OtpcStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Indicates the result of a test sequence. Should be checked after the end of a TBLANK, TDEC and TWR mode (OTPC_STAT_TRDY = 1).\n0 - The test sequence ends with no error.\n1 - The test sequence has failed."]
    #[inline(always)]
    pub fn otpc_stat_terror(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, OtpcStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,OtpcStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Indicates the state of a test mode. Should be used to monitor the progress of the TBLANK, TDEC and TWR modes.\n0 - The controller is busy. One of the test modes is in progress.\n1 - There is no active test mode."]
    #[inline(always)]
    pub fn otpc_stat_trdy(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, OtpcStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,OtpcStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Indicates that the programming sequence has been avoided during a programming request, due to that the word that should be programmed is equal to zero.\n0 - At least one bit has been programmed into the OTP.\n1 - The programming has not been performed. All the bits of the word that should be programmed are equal to zero.\nWhen the controller is in MPROG mode, this bit can be checked after the end of the programming process (OTPC_STAT_PRDY = 1).\nDuring APROG mode, the value of this field it is normal to changing periodically. After the end of the APROG mode (OTPC_STAT_ARDY = 1), this field indicates that one or more of words that have been processed are equal to zero."]
    #[inline(always)]
    pub fn otpc_stat_pzero(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, OtpcStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,OtpcStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Indicates that a correctable error has been occurred during the word programming process.\n0 - There is no correctable error in the word-programming process.\n1 - The process of word - programming reported a correctable error.\nThe correctable error occurs when exactly one bit in an OTP position cannot take the required value. This is not a critical failure in the programming process. The data can still be retrieved correctly by the OTP memory, due to that the error correcting algorithm can repair the corrupted bit.\nWhen the controller is in MPROG mode, this bit can be checked after the end of the programming process (OTPC_STAT_PRDY = 1).\nDuring APROG mode, the value of this field it is normal to changing periodically. After the end of the APROG mode (OTPC_STAT_ARDY = 1), this field indicates that one or more words had a correctable error."]
    #[inline(always)]
    pub fn otpc_stat_perr_cor(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, OtpcStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,OtpcStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Indicates that an uncorrectable error has been occurred during the word programming process.\n0 - There is no uncorrectable error in the word-programming process.\n1 - The process of word-programming failed due to an uncorrectable error.\nAn uncorrectable error is considered when two or more of the bits in an OTP position cannot take the required values. This is a critical failure in the programming process, which means that the data cannot corrected by the single error correcting algorithm.\nWhen the controller is in MPROG mode, this bit should be checked after the end of the programming process (OTPC_STAT_PRDY = 1).\nDuring APROG mode, the value of this field it is normal to changing periodically. After the end of the APROG mode (OTPC_STAT_ARDY = 1), this field indicates if the programming was failed or ended successfully."]
    #[inline(always)]
    pub fn otpc_stat_perr_unc(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, OtpcStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,OtpcStatReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Indicates the state of a bit-programming process.\n0 - The controller is busy. A bit-programming is in progress\n1 - The logic which performs bit-programming is idle.\nWhen the controller is in MPROG mode, this bit should be used to monitor the progress of a programming request.\nDuring APROG mode, the value of this field it is normal to changing periodically."]
    #[inline(always)]
    pub fn otpc_stat_prdy(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, OtpcStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,OtpcStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for OtpcStatReg {
    #[inline(always)]
    fn default() -> OtpcStatReg {
        <crate::RegValueT<OtpcStatReg_SPEC> as RegisterValue<_>>::new(81)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OtpcTim1Reg_SPEC;
impl crate::sealed::RegSpec for OtpcTim1Reg_SPEC {
    type DataType = u32;
}

#[doc = "Various timing parameters of the OTP cell."]
pub type OtpcTim1Reg = crate::RegValueT<OtpcTim1Reg_SPEC>;

impl OtpcTim1Reg {
    #[doc = "The number of hclk_c clock periods (minus one) that give a time interval at least higher than 25ns."]
    #[inline(always)]
    pub fn otpc_tim1_cc_t_25ns(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, OtpcTim1Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,OtpcTim1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "The number of hclk_c clock periods (minus one) that give a time interval at least higher than 200ns."]
    #[inline(always)]
    pub fn otpc_tim1_cc_t_200ns(
        self,
    ) -> crate::common::RegisterField<27, 0xf, 1, 0, u8, u8, OtpcTim1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<27,0xf,1,0,u8,u8,OtpcTim1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "The number of hclk_c clock periods (minus one) that give a time interval at least higher than 500ns"]
    #[inline(always)]
    pub fn otpc_tim1_cc_t_500ns(
        self,
    ) -> crate::common::RegisterField<22, 0x1f, 1, 0, u8, u8, OtpcTim1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<22,0x1f,1,0,u8,u8,OtpcTim1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "The number of hclk_c clock periods (minus one) that give a time interval at least higher than 1us."]
    #[inline(always)]
    pub fn otpc_tim1_cc_t_1us(
        self,
    ) -> crate::common::RegisterField<16, 0x3f, 1, 0, u8, u8, OtpcTim1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3f,1,0,u8,u8,OtpcTim1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "The number of hclk_c clock periods (minus one) that give a time interval that is\n- at least higher than 4.8us\n- and lower than 5.2 us\nIt is preferred the programmed value to give a time interval equal to 5us.\nIt defines the duration of the programming pulse for every bit that written in the OTP cell."]
    #[inline(always)]
    pub fn otpc_tim1_cc_t_pw(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, OtpcTim1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,OtpcTim1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "The number of hclk_c clock periods (minus one) that give a time interval at least higher than 2us.It is used as a wait time each time where the OTP cell is enabled."]
    #[inline(always)]
    pub fn otpc_tim1_cc_t_cadx(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, OtpcTim1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,OtpcTim1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for OtpcTim1Reg {
    #[inline(always)]
    fn default() -> OtpcTim1Reg {
        <crate::RegValueT<OtpcTim1Reg_SPEC> as RegisterValue<_>>::new(437276448)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OtpcTim2Reg_SPEC;
impl crate::sealed::RegSpec for OtpcTim2Reg_SPEC {
    type DataType = u32;
}

#[doc = "Various timing parameters of the OTP cell."]
pub type OtpcTim2Reg = crate::RegValueT<OtpcTim2Reg_SPEC>;

impl OtpcTim2Reg {
    #[doc = "This bit has meaning only when the OTPC_TIM1_CC_T_25NS = 1, otherwise has no functionality. \n0 - The minimum number of clock cycles for which the signal read_enable of the OTP memory stays inactive is one clock cycle. This is also applicable if OTPC_TIM1_CC_T_25NS = 0.\n1 - The minimum number of clock cycles for which the signal read_enable of the OTP memory stays inactive is two clock cycles. The controller adds one extra wait state in the AHB access , if it is required, in order to achieves this constraint. This setting is applicable only if OTPC_TIM1_CC_T_25NS = 1."]
    #[inline(always)]
    pub fn otpc_tim2_rdenl_prot(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, OtpcTim2Reg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23,1,0,OtpcTim2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "The number of hclk_c clock periods (minus one) that give a time interval between 100ns and 200ns. This time interval is used for the reading of the contents of the OTP cell during the TBLANK mode."]
    #[inline(always)]
    pub fn otpc_tim2_cc_t_bchk(
        self,
    ) -> crate::common::RegisterField<16, 0x7f, 1, 0, u8, u8, OtpcTim2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7f,1,0,u8,u8,OtpcTim2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "This register controls a power saving feature, which is applicable only in MREAD mode. The controller monitors the accesses in the OTP cell. If there is no access for more than OTPC_TIM2_CC_STBY_THR hclk_c clock cycles, the OTP cell goes to the standby while the controller itself remains in the MREAD mode. The OTP cell will be enabled again when will be applied a new read request. The enabling of the OTP cell has a cost of 2us (OTPC_TIM1_CC_T_CADX hclk_c clock cycles).\nWhen OTPC_TIM2_CC_STBY_THR = 0 the power saving feature is disabled and the OTP cell remains active while the controller is in MREAD mode."]
    #[inline(always)]
    pub fn otpc_tim2_cc_stby_thr(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, u16, OtpcTim2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16,u16,OtpcTim2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for OtpcTim2Reg {
    #[inline(always)]
    fn default() -> OtpcTim2Reg {
        <crate::RegValueT<OtpcTim2Reg_SPEC> as RegisterValue<_>>::new(65536)
    }
}
