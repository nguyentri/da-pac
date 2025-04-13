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
// Generated from SVD 1.2, with svd2pac 0.4.0 on Sat, 12 Apr 2025 22:24:51 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"otpc580_gr01 registers"]
unsafe impl ::core::marker::Send for super::Otpc580Gr01 {}
unsafe impl ::core::marker::Sync for super::Otpc580Gr01 {}
impl super::Otpc580Gr01 {
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

    #[doc = "Latest read data from the OTPC_FFPRT_REG"]
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
    #[doc = "Tthe AHB address used by the AHB master interface of the controller (\nbits \\[31:2\\])."]
    #[inline(always)]
    pub fn otpc_ahbadr(
        self,
    ) -> crate::common::RegisterField<2, 0x3fffffff, 1, 0, u32, OtpcAhbadrReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            2,
            0x3fffffff,
            1,
            0,
            u32,
            OtpcAhbadrReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OtpcAhbadrReg {
    #[inline(always)]
    fn default() -> OtpcAhbadrReg {
        <crate::RegValueT<OtpcAhbadrReg_SPEC> as RegisterValue<_>>::new(0)
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
    #[doc = "Defines a word address inside the macrocell. Used in modes AREAD and APROG and is automatically updated."]
    #[inline(always)]
    pub fn otpc_celadr(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, OtpcCeladrReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1fff,1,0,u16, OtpcCeladrReg_SPEC,crate::common::RW>::from_register(self,0)
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
    #[doc = "Provides access to the fifo through an access port. Write this register with the corresponding data, when the APROG mode is selected and the DMA is disabled. Read from this register the corresponding data, when the AREAD mode is selected and the DMA is disabled.\nCheck OTPC_STAT_FWORDS register for data/space availability, before accessing the fifo."]
    #[inline(always)]
    pub fn otpc_ffprt(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, OtpcFfprtReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, OtpcFfprtReg_SPEC,crate::common::RW>::from_register(self,0)
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
#[doc = "Latest read data from the OTPC_FFPRT_REG"]
pub type OtpcFfrdReg = crate::RegValueT<OtpcFfrdReg_SPEC>;

impl OtpcFfrdReg {
    #[doc = "Contains the value read from the fifo, after a read of the OTPC_FFPRT_REG register."]
    #[inline(always)]
    pub fn otpc_ffrd(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, OtpcFfrdReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, OtpcFfrdReg_SPEC,crate::common::R>::from_register(self,0)
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
    #[doc = "Selects the source that is connected to the prg_port port of the controller.\n00 - {16\'d0, BANDGAP_REG\\[15:0\\]}\n01 - {RF_RSSI_COMP_CTRL_REG\\[15:0\\], 8\'d0, RFIO_CTRL1_REG{7:0\\]}\n10 - {3\'d0, RF_LNA_CTRL3_REG\\[4:0\\], RF_LNA_CTRL2_REG\\[11:0\\], RF_LNA_CTRL1_REG\\[11:0\\]}\n11 - {28\'d0, RF_VCO_CTRL_REG\\[3:0\\]}\nSee OTPC_MODE_PRG_PORT_SEL about the use of the prg_port"]
    #[inline(always)]
    pub fn otpc_mode_prg_port_mux(
        self,
    ) -> crate::common::RegisterField<28, 0x3, 1, 0, u8, OtpcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0x3,1,0,u8, OtpcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Defines the timing that will be used for all the programming activities (APROG, MPROG and TWR)\n0 - Selects the normal timing\n1 - Selects the fast timing"]
    #[inline(always)]
    pub fn optc_mode_prg_fast(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, OtpcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,OtpcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Selects an alternative data source for the programming of the OTP macrocells, when the controller is configured in APROG mode.\n0 - The fifo will be used as the data source. The fifo will be filled with a way defined by the register OTPC_MODE_USE_DMA. The number of words that will be programmed is defined by OTPC_NWORDS.\n1 - Only one word will programmed. The value of the word is contained in the prg_port port of the controller. The values of the registers OTPC_MODE_USE_DMA, OTPC_NWORDS and the contents of the FIFO will not be used."]
    #[inline(always)]
    pub fn otpc_mode_prg_port_sel(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, OtpcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,OtpcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Defines the duration of each read from the OTP macrocells.\n0 - Reads 16 bits of data every one clock cycle.\n1 - Reads 16 bits of data every two clock cycles."]
    #[inline(always)]
    pub fn otpc_mode_two_cc_acc(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, OtpcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,OtpcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Writing 1, removes any content from the FIFO. This bit returns automatically to 0."]
    #[inline(always)]
    pub fn otpc_mode_fifo_flush(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, OtpcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,OtpcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Selects the use of the dma, when the controller is configured in one of the modes: AREAD or APROG.\n0 - DMAis not used. The data should be transfered from/to controller through OTPC_FFPRT_REG\n1 - DMA is used. Data transfers from/to controller are performed automatically. The AHB base address should be configured in OTPC_AHBADR_REG before the selection of the mode.\nIf programming of the OTPC_MODE_REG is performed through the serial interface,the OTPC_MODE_USE_DMA will be set to 0 automatically.\nIf the controller is in APROG mode and the OTPC_MODE_PRG_PORT_SEL is enabled, the dma will stay inactive."]
    #[inline(always)]
    pub fn otpc_mode_use_dma(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, OtpcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,OtpcModeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Defines the mode of operation of the OTPC controller. The encoding of the modes is as follows:\n000 - STBY mode\n001 - MREAD mode\n010 - MPROG mode\n011 - AREAD mode\n100 - APROG mode\n101 - Test mode. Reserved\n110 - Test mode. Reserved\n111 - Test mode. Reserved\nTo manually move between modes, always return to STBY mode first."]
    #[inline(always)]
    pub fn otpc_mode_mode(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, OtpcModeReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8, OtpcModeReg_SPEC,crate::common::RW>::from_register(self,0)
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
    #[doc = "The number of words (minus one) for reading/programming during the AREAD/APROG mode.\nIf in APROG mode, and the OTPC_MODE_PRG_PORT_SEL is enabled (=1), this register will not be used and will stay unchanged.\nDuring mirroring, this register reflects the current amount of data that will be copied. It keeps its value until be written by the software with a new value. The number of the words that remaining to be processed by the controller is contained in the field OTPC_STAT_NWORDS."]
    #[inline(always)]
    pub fn otpc_nwords(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, OtpcNwordsReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1fff,1,0,u16, OtpcNwordsReg_SPEC,crate::common::RW>::from_register(self,0)
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
    #[doc = "Enables the programming in the upper bank of the OTP.\n0 - Programming sequence is not applied in the upper bank.\n1 - Programming sequence is applied in the upper bank."]
    #[inline(always)]
    pub fn otpc_pctrl_enu(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, OtpcPctrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27,1,0,OtpcPctrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Defines the value of the selected bit in the upper bank, after the programming sequence."]
    #[inline(always)]
    pub fn otpc_pctrl_bitu(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, OtpcPctrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26,1,0,OtpcPctrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enables the programming in the lower bank.\n0 - The programming sequence is not applied in the lower bank.\n1 -The programming sequence is applied in the lower bank."]
    #[inline(always)]
    pub fn otpc_pctrl_enl(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, OtpcPctrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25,1,0,OtpcPctrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Defines the value of the selected bit in the lower bank, after the programming sequence."]
    #[inline(always)]
    pub fn otpc_pctrl_bitl(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, OtpcPctrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24,1,0,OtpcPctrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Selects between the U1 and U0 byte for the programming sequence in the upper bank.\n0 - Program the U0 byte\n1 - Program the U1 byte -"]
    #[inline(always)]
    pub fn otpc_pctrl_bselu(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, OtpcPctrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23,1,0,OtpcPctrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Selects the bit inside the Ux (x=0,1) byte, which will be programmed in the upper bank."]
    #[inline(always)]
    pub fn otpc_pctrl_badru(
        self,
    ) -> crate::common::RegisterField<20, 0x7, 1, 0, u8, OtpcPctrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x7,1,0,u8, OtpcPctrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Selects between the L1 and L0 byte for the programming sequence in the lower bank.\n0 - Program the L0 byte\n1 - Program the L1 byte"]
    #[inline(always)]
    pub fn otpc_pctrl_bsell(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, OtpcPctrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19,1,0,OtpcPctrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Selects the bit inside the Lx (x=0,1) byte, which will be programmed in the lower bank."]
    #[inline(always)]
    pub fn otpc_pctrl_badrl(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, OtpcPctrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x7,1,0,u8, OtpcPctrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Defines the address of a 32 bits word {U1,L1,U0,L0} in the macrocells, where one or two bits will be programmed. There are two macrocell banks, with 8 bits each. Each bank contribute with two memory positions for each 32 bits word. The Ux, Lx represent the bytes of the upper and lower bank respectively."]
    #[inline(always)]
    pub fn otpc_pctrl_waddr(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, OtpcPctrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1fff,1,0,u16, OtpcPctrlReg_SPEC,crate::common::RW>::from_register(self,0)
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
pub struct OtpcStatReg_SPEC;
impl crate::sealed::RegSpec for OtpcStatReg_SPEC {
    type DataType = u32;
}
#[doc = "Status register"]
pub type OtpcStatReg = crate::RegValueT<OtpcStatReg_SPEC>;

impl OtpcStatReg {
    #[doc = "Contains the current value of the words to be processed."]
    #[inline(always)]
    pub fn otpc_stat_nwords(
        self,
    ) -> crate::common::RegisterField<16, 0x1fff, 1, 0, u16, OtpcStatReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0x1fff,1,0,u16, OtpcStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Indicates the upper bank as the source of a test error. This value is valid when OTPC_STAT_TERROR is valid.\n0 - There is no test error in the upper bank\n1 - A test error has occured in the upper bank"]
    #[inline(always)]
    pub fn otpc_stat_terr_u(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, OtpcStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15,1,0,OtpcStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Indicates the lower bank as the source of a test error. The value is valid when OTPC_STAT_TERROR is valid.\n0 - There is no test error in the lower bank\n1 - A test error has occured in the lower bank"]
    #[inline(always)]
    pub fn otpc_stat_terr_l(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, OtpcStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14,1,0,OtpcStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Indicates the upper bank as the source of a programming error. The value is valid when OTPC_STAT_PERROR is valid.\n0 - There is no programming error in the upper bank\n1 - A programming error has occured in the upper bank"]
    #[inline(always)]
    pub fn otpc_stat_perr_u(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, OtpcStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13,1,0,OtpcStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Indicates the lower bank as the source of a programming error. The value is valid when OTPC_STAT_PERROR is valid.\n0 - There is no programming error in the lower bank\n1 - A programming error has occured in the lower bank"]
    #[inline(always)]
    pub fn otpc_stat_perr_l(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, OtpcStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12,1,0,OtpcStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Indicates the number of words which contained in the fifo of the controller."]
    #[inline(always)]
    pub fn otpc_stat_fwords(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, OtpcStatReg_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xf,1,0,u8, OtpcStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Monitors the progress of read or programming operations while in the AREAD or APROG modes.\n0 - The controller is busy while reading or programming (AREAD or APROG modes).\n1 - The controller is not busy in AREAD or APROG mode."]
    #[inline(always)]
    pub fn otpc_stat_ardy(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, OtpcStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,OtpcStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Indicates the result of a test sequence. Should be checked after the end of a TBLANK, TDEC and TWR mode (OTPC_STAT_TRDY= 1).\n0 - The test sequence ends with no error.\n1 - The test sequence has failed."]
    #[inline(always)]
    pub fn otpc_stat_terror(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, OtpcStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,OtpcStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Indicates the state of a test mode. Should be used to monitor the progress of the TBLANK, TDEC and TWR modes.\n0 - The controller is busy. A test mode is in progress.\n1 - There is no active test mode."]
    #[inline(always)]
    pub fn otpc_stat_trdy(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, OtpcStatReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,OtpcStatReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Indicates that an error has occurred during the bit-programming process.\n0 - No error during the bit-programming process.\n1 - The process of bit-programming failed.\nWhen the controller is in MPROG mode, this bit should be checked after the end of the programming process (OTPC_STAT_PRDY= 1).\nDuring APROG mode, the value of this field is normal to change periodically. Upon finishing the operation in the APROG mode (OTPC_STAT_ARDY= 1), this field indicates if the programming has failed or ended succesfully."]
    #[inline(always)]
    pub fn otpc_stat_perror(
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
        <crate::RegValueT<OtpcStatReg_SPEC> as RegisterValue<_>>::new(21)
    }
}
