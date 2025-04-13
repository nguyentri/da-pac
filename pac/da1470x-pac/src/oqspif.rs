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
// Generated from SVD 1.2, with svd2pac 0.4.0 on Sat, 12 Apr 2025 22:14:28 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"OQSPIF registers"]
unsafe impl ::core::marker::Send for super::Oqspif {}
unsafe impl ::core::marker::Sync for super::Oqspif {}
impl super::Oqspif {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    #[doc = "Read break sequence in Auto mode"]
    #[inline(always)]
    pub const fn oqspif_burstbrk_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OqspifBurstbrkReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OqspifBurstbrkReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[doc = "The way of reading in Auto mode (command register A)"]
    #[inline(always)]
    pub const fn oqspif_burstcmda_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OqspifBurstcmdaReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OqspifBurstcmdaReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "The way of reading in Auto mode (command register B)"]
    #[inline(always)]
    pub const fn oqspif_burstcmdb_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OqspifBurstcmdbReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OqspifBurstcmdbReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Check erase progress in Auto mode"]
    #[inline(always)]
    pub const fn oqspif_chckerase_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OqspifChckeraseReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OqspifChckeraseReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }

    #[doc = "SPI Bus control register for the Manual mode"]
    #[inline(always)]
    pub const fn oqspif_ctrlbus_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OqspifCtrlbusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OqspifCtrlbusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Mode Control register"]
    #[inline(always)]
    pub const fn oqspif_ctrlmode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OqspifCtrlmodeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OqspifCtrlmodeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Control register for the decryption engine of the OSPIC"]
    #[inline(always)]
    pub const fn oqspif_ctr_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OqspifCtrCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OqspifCtrCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(256usize),
            )
        }
    }

    #[doc = "End address of the encrypted content in the OSPI flash"]
    #[inline(always)]
    pub const fn oqspif_ctr_eaddr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OqspifCtrEaddrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OqspifCtrEaddrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(264usize),
            )
        }
    }

    #[doc = "Key bytes 0 to 3 for the AES-CTR algorithm"]
    #[inline(always)]
    pub const fn oqspif_ctr_key_0_3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OqspifCtrKey03Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OqspifCtrKey03Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(276usize),
            )
        }
    }

    #[doc = "Key bytes 12 to 15 for the AES-CTR algorithm"]
    #[inline(always)]
    pub const fn oqspif_ctr_key_12_15_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OqspifCtrKey1215Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OqspifCtrKey1215Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(288usize),
            )
        }
    }

    #[doc = "Key bytes 16 to 19 for the AES-CTR algorithm"]
    #[inline(always)]
    pub const fn oqspif_ctr_key_16_19_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OqspifCtrKey1619Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OqspifCtrKey1619Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(292usize),
            )
        }
    }

    #[doc = "Key bytes 20 to 23 for the AES-CTR algorithm"]
    #[inline(always)]
    pub const fn oqspif_ctr_key_20_23_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OqspifCtrKey2023Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OqspifCtrKey2023Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(296usize),
            )
        }
    }

    #[doc = "Key bytes 24 to 27 for the AES-CTR algorithm"]
    #[inline(always)]
    pub const fn oqspif_ctr_key_24_27_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OqspifCtrKey2427Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OqspifCtrKey2427Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(300usize),
            )
        }
    }

    #[doc = "Key bytes 28 to 31 for the AES-CTR algorithm"]
    #[inline(always)]
    pub const fn oqspif_ctr_key_28_31_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OqspifCtrKey2831Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OqspifCtrKey2831Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(304usize),
            )
        }
    }

    #[doc = "Key bytes 4 to 7 for the AES-CTR algorithm"]
    #[inline(always)]
    pub const fn oqspif_ctr_key_4_7_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OqspifCtrKey47Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OqspifCtrKey47Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(280usize),
            )
        }
    }

    #[doc = "Key bytes 8 to 11 for the AES-CTR algorithm"]
    #[inline(always)]
    pub const fn oqspif_ctr_key_8_11_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OqspifCtrKey811Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OqspifCtrKey811Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(284usize),
            )
        }
    }

    #[doc = "Nonce bytes 0 to 3 for the AES-CTR algorithm"]
    #[inline(always)]
    pub const fn oqspif_ctr_nonce_0_3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OqspifCtrNonce03Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OqspifCtrNonce03Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(268usize),
            )
        }
    }

    #[doc = "Nonce bytes 4 to 7 for the AES-CTR algorithm"]
    #[inline(always)]
    pub const fn oqspif_ctr_nonce_4_7_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OqspifCtrNonce47Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OqspifCtrNonce47Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(272usize),
            )
        }
    }

    #[doc = "Start address of the encrypted content in the OSPI flash"]
    #[inline(always)]
    pub const fn oqspif_ctr_saddr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OqspifCtrSaddrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OqspifCtrSaddrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(260usize),
            )
        }
    }

    #[doc = "Send dummy clocks to SPI Bus for the Manual mode"]
    #[inline(always)]
    pub const fn oqspif_dummydata_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OqspifDummydataReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OqspifDummydataReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "The way of erasing in Auto mode (command register A)"]
    #[inline(always)]
    pub const fn oqspif_erasecmda_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OqspifErasecmdaReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OqspifErasecmdaReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[doc = "The way of erasing in Auto mode (command register B)"]
    #[inline(always)]
    pub const fn oqspif_erasecmdb_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OqspifErasecmdbReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OqspifErasecmdbReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[doc = "The way of erasing in Auto mode (command register C)"]
    #[inline(always)]
    pub const fn oqspif_erasecmdc_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OqspifErasecmdcReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OqspifErasecmdcReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[doc = "OSPI Erase control register"]
    #[inline(always)]
    pub const fn oqspif_erasectrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OqspifErasectrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OqspifErasectrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[doc = "OSPI General Purpose control register"]
    #[inline(always)]
    pub const fn oqspif_gp_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OqspifGpReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OqspifGpReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[doc = "Read data from SPI Bus for the Manual mode"]
    #[inline(always)]
    pub const fn oqspif_readdata_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OqspifReaddataReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OqspifReaddataReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "Received data for the Manual mode"]
    #[inline(always)]
    pub const fn oqspif_recvdata_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OqspifRecvdataReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OqspifRecvdataReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "The way of reading the status of external device in Auto mode"]
    #[inline(always)]
    pub const fn oqspif_statuscmd_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OqspifStatuscmdReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OqspifStatuscmdReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[doc = "The status register of the OSPI controller"]
    #[inline(always)]
    pub const fn oqspif_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OqspifStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OqspifStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "Write data to SPI Bus for the Manual mode"]
    #[inline(always)]
    pub const fn oqspif_writedata_reg(
        &self,
    ) -> &'static crate::common::Reg<self::OqspifWritedataReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::OqspifWritedataReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OqspifBurstbrkReg_SPEC;
impl crate::sealed::RegSpec for OqspifBurstbrkReg_SPEC {
    type DataType = u32;
}
#[doc = "Read break sequence in Auto mode"]
pub type OqspifBurstbrkReg = crate::RegValueT<OqspifBurstbrkReg_SPEC>;

impl OqspifBurstbrkReg {
    #[doc = "Controls the application of a special command (read burst break sequence) that is used in order to force the device to abandon the continuous read mode.\n0: The special command is not applied\n1: The special command is applied\n\nThis special command is applied by the controller to the external device under the following conditions:\n- the controller is in Auto mode\n- the OSPIC_INST_MD = 1\n- the previous command that has been applied in the external device was read\n- the controller want to apply to the external device a command different than the read."]
    #[inline(always)]
    pub fn ospic_brk_en(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, OqspifBurstbrkReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23,1,0,OqspifBurstbrkReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Disable output during the transmission of the second half (OSPIC_BRK_WRD\\[3:0\\]). Setting this bit is only useful if OSPIC_BRK_EN =1 and OSPIC_BRK_SZ >= 1. It is not applicable when the sequence is transferred in Octal mode (OSPIC_BRK_TX_MD=3).\n0: The controller drives the OSPI bus during the transmission of the OSPIC_BRK_WRD\\[3:0\\].\n1: The controller leaves the OSPI bus in Hi-Z during the transmission of the OSPIC_BRK_WORD\\[3:0\\]."]
    #[inline(always)]
    pub fn ospic_sec_hf_ds(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, OqspifBurstbrkReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22,1,0,OqspifBurstbrkReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "The mode of the OSPI Bus during the transmission of the burst break sequence.\n0x0: Single\n0x1: Dual\n0x2: Quad\n0x3: Octal"]
    #[inline(always)]
    pub fn ospic_brk_tx_md(
        self,
    ) -> crate::common::RegisterField<20, 0x3, 1, 0, u8, OqspifBurstbrkReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x3,1,0,u8, OqspifBurstbrkReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "The size of Burst Break Sequence\n0: One byte (Send OSPIC_BRK_WRD\\[15:8\\])\n1: Two bytes (Send OSPIC_BRK_WRD\\[15:0\\])\n2-15: Three up to 16 bytes will be transferred. All the bytes that will be transferred will have the value of the OSPIC_BRK_WRD\\[15:8\\], except of the last byte that will be the OSPIC_BRK_WRD\\[7:0\\]."]
    #[inline(always)]
    pub fn ospic_brk_sz(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, OqspifBurstbrkReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xf,1,0,u8, OqspifBurstbrkReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "This is the value of a special command (read burst break sequence) that is applied by the controller to the external memory device, in order to force the memory device to abandon the continuous read mode."]
    #[inline(always)]
    pub fn ospic_brk_wrd(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, OqspifBurstbrkReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            OqspifBurstbrkReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OqspifBurstbrkReg {
    #[inline(always)]
    fn default() -> OqspifBurstbrkReg {
        <crate::RegValueT<OqspifBurstbrkReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OqspifBurstcmdaReg_SPEC;
impl crate::sealed::RegSpec for OqspifBurstcmdaReg_SPEC {
    type DataType = u32;
}
#[doc = "The way of reading in Auto mode (command register A)"]
pub type OqspifBurstcmdaReg = crate::RegValueT<OqspifBurstcmdaReg_SPEC>;

impl OqspifBurstcmdaReg {
    #[doc = "It describes the mode of the SPI bus during the Dummy bytes phase.\n0x0: Single SPI\n0x1: Dual\n0x2: Quad\n0x3: Octal"]
    #[inline(always)]
    pub fn ospic_dmy_tx_md(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, u8, OqspifBurstcmdaReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<30,0x3,1,0,u8, OqspifBurstcmdaReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "It describes the mode of the SPI bus during the Extra Byte phase.\n0x0: Single SPI\n0x1: Dual\n0x2: Quad\n0x3: Octal"]
    #[inline(always)]
    pub fn ospic_ext_tx_md(
        self,
    ) -> crate::common::RegisterField<28, 0x3, 1, 0, u8, OqspifBurstcmdaReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x3,1,0,u8, OqspifBurstcmdaReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "It describes the mode of the SPI bus during the address phase.\n0x0: Single SPI\n0x1: Dual\n0x2: Quad\n0x3: Octal"]
    #[inline(always)]
    pub fn ospic_adr_tx_md(
        self,
    ) -> crate::common::RegisterField<26, 0x3, 1, 0, u8, OqspifBurstcmdaReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<26,0x3,1,0,u8, OqspifBurstcmdaReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "It describes the mode of the SPI bus during the instruction phase.\n0x0: Single SPI\n0x1: Dual\n0x2: Quad\n0x3: Octal"]
    #[inline(always)]
    pub fn ospic_inst_tx_md(
        self,
    ) -> crate::common::RegisterField<24, 0x3, 1, 0, u8, OqspifBurstcmdaReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x3,1,0,u8, OqspifBurstcmdaReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "The value of an extra byte which will be transferred after address (only if OSPIC_EXT_BYTE_EN= 1). Usually this is the Mode Bits in Dual/Quad/Octal SPI I/O instructions."]
    #[inline(always)]
    pub fn ospic_ext_byte(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, OqspifBurstcmdaReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8, OqspifBurstcmdaReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Instruction Value for Wrapping Burst. This value is the selected instruction when OSPIC_WRAP_MD is equal to 1 and the access is a wrapping burst of length and size described by the bit fields OSPIC_WRAP_LEN and OSPIC_WRAP_SIZE respectively."]
    #[inline(always)]
    pub fn ospic_inst_wb(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, OqspifBurstcmdaReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8, OqspifBurstcmdaReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Instruction Value for Incremental Burst or Single read access. This value is the selected instruction at the cases of incremental burst or single read access. Also this value is used when a wrapping burst is not supported (OSPIC_WRAP_MD)"]
    #[inline(always)]
    pub fn ospic_inst(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, OqspifBurstcmdaReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, OqspifBurstcmdaReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for OqspifBurstcmdaReg {
    #[inline(always)]
    fn default() -> OqspifBurstcmdaReg {
        <crate::RegValueT<OqspifBurstcmdaReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OqspifBurstcmdbReg_SPEC;
impl crate::sealed::RegSpec for OqspifBurstcmdbReg_SPEC {
    type DataType = u32;
}
#[doc = "The way of reading in Auto mode (command register B)"]
pub type OqspifBurstcmdbReg = crate::RegValueT<OqspifBurstcmdbReg_SPEC>;

impl OqspifBurstcmdbReg {
    #[doc = "Between the transmissions of two different instructions to the flash memory, the SPI bus stays in idle state (OSPI_CS high) for at least this number of OSPI_SCK clock cycles. See the OSPIC_ERS_CS_HI register for some exceptions."]
    #[inline(always)]
    pub fn ospic_cs_high_min(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, OqspifBurstcmdbReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7,1,0,u8, OqspifBurstcmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "It describes the selected data size of a wrapping burst (OSPIC_WRAP_MD).\n0x0: byte access (8-bits)\n0x1: half word access (16 bits)\n0x2: word access (32-bits)\n0x3: Reserved"]
    #[inline(always)]
    pub fn ospic_wrap_size(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, OqspifBurstcmdbReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x3,1,0,u8, OqspifBurstcmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "It describes the selected length of a wrapping burst (OSPIC_WRAP_MD).\n0x0: 4 beat wrapping burst\n0x1: 8 beat wrapping burst\n0x2: 16 beat wrapping burst\n0x3: Reserved"]
    #[inline(always)]
    pub fn ospic_wrap_len(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, OqspifBurstcmdbReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,u8, OqspifBurstcmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Wrap mode\n0: The OSPIC_INST is the selected instruction at any access.\n1: The OSPIC_INST_WB is the selected instruction at any wrapping burst access of length and size described by the registers OSPIC_WRAP_LEN and OSPIC_WRAP_SIZE respectively. In all other cases the OSPIC_INST is the selected instruction. Use this feature only when the serial FLASH memory supports a special instruction for wrapping burst access."]
    #[inline(always)]
    pub fn ospic_wrap_md(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, OqspifBurstcmdbReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<11,1,0,OqspifBurstcmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Instruction mode\n0: Transmit instruction at any burst access.\n1: Transmit instruction only in the first access after the selection of Auto Mode."]
    #[inline(always)]
    pub fn ospic_inst_md(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, OqspifBurstcmdbReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<10,1,0,OqspifBurstcmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Dummy bytes enable\n0: Don\'t send the dummy bytes\n1: Send the dummy bytes. The number of the dummy bytes is defined by the OSPIC_DMY_NUM."]
    #[inline(always)]
    pub fn ospic_dmy_en(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, OqspifBurstcmdbReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,OqspifBurstcmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Number of dummy bytes (minus 1). Can be set 1 up to 32 dummy bytes (values 0 up to 31). The dummy bytes are appied only when OSPIC_DMY_EN=1."]
    #[inline(always)]
    pub fn ospic_dmy_num(
        self,
    ) -> crate::common::RegisterField<4, 0x1f, 1, 0, u8, OqspifBurstcmdbReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1f,1,0,u8, OqspifBurstcmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Extra half disable output\n0: if OSPIC_EXT_BYTE_EN=1, is transmitted the complete OSPIC_EXT_BYTE\n1: if OSPIC_EXT_BYTE_EN=1, the output is disabled (hi-z) during the transmission of bits \\[3:0\\] of OSPIC_EXT_BYTE.\nThis setting has no meaning if the extra byte is transferred in Octal mode. In this case keep this bit to zero value."]
    #[inline(always)]
    pub fn ospic_ext_hf_ds(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, OqspifBurstcmdbReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,OqspifBurstcmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Extra byte enable\n0: Don\'t send the OSPIC_EXT_BYTE\n1: Send the OSPIC_EXT_BYTE"]
    #[inline(always)]
    pub fn ospic_ext_byte_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, OqspifBurstcmdbReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,OqspifBurstcmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "It describes the mode of the SPI bus during the data phase.\n0x0: Single SPI\n0x1: Dual\n0x2: Quad\n0x3: Octal"]
    #[inline(always)]
    pub fn ospic_dat_rx_md(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, OqspifBurstcmdbReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8, OqspifBurstcmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for OqspifBurstcmdbReg {
    #[inline(always)]
    fn default() -> OqspifBurstcmdbReg {
        <crate::RegValueT<OqspifBurstcmdbReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OqspifChckeraseReg_SPEC;
impl crate::sealed::RegSpec for OqspifChckeraseReg_SPEC {
    type DataType = u32;
}
#[doc = "Check erase progress in Auto mode"]
pub type OqspifChckeraseReg = crate::RegValueT<OqspifChckeraseReg_SPEC>;

impl OqspifChckeraseReg {
    #[doc = "Writing any value to this register during erasing, forces the controller to read the flash memory status register. Depending on the value of the Busy bit, it updates the OSPIC_ERASE_EN."]
    #[inline(always)]
    pub fn ospic_chckerase(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        OqspifChckeraseReg_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            OqspifChckeraseReg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OqspifChckeraseReg {
    #[inline(always)]
    fn default() -> OqspifChckeraseReg {
        <crate::RegValueT<OqspifChckeraseReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OqspifCtrlbusReg_SPEC;
impl crate::sealed::RegSpec for OqspifCtrlbusReg_SPEC {
    type DataType = u32;
}
#[doc = "SPI Bus control register for the Manual mode"]
pub type OqspifCtrlbusReg = crate::RegValueT<OqspifCtrlbusReg_SPEC>;

impl OqspifCtrlbusReg {
    #[doc = "Write 1 to disable the chip select (active low) when the controller is in Manual mode."]
    #[inline(always)]
    pub fn ospic_dis_cs(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, OqspifCtrlbusReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5,1,0,OqspifCtrlbusReg_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Write 1 to enable the chip select (active low) when the controller is in Manual mode."]
    #[inline(always)]
    pub fn ospic_en_cs(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, OqspifCtrlbusReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4,1,0,OqspifCtrlbusReg_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Write 1 to set the bus mode in Octal mode when the controller is in Manual mode."]
    #[inline(always)]
    pub fn ospic_set_octal(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, OqspifCtrlbusReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3,1,0,OqspifCtrlbusReg_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Write 1 to set the bus mode in Quad mode when the controller is in Manual mode."]
    #[inline(always)]
    pub fn ospic_set_quad(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, OqspifCtrlbusReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2,1,0,OqspifCtrlbusReg_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Write 1 to set the bus mode in Dual mode when the controller is in Manual mode."]
    #[inline(always)]
    pub fn ospic_set_dual(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, OqspifCtrlbusReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1,1,0,OqspifCtrlbusReg_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Write 1 to set the bus mode in Single SPI mode when the controller is in Manual mode."]
    #[inline(always)]
    pub fn ospic_set_single(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, OqspifCtrlbusReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0,1,0,OqspifCtrlbusReg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for OqspifCtrlbusReg {
    #[inline(always)]
    fn default() -> OqspifCtrlbusReg {
        <crate::RegValueT<OqspifCtrlbusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OqspifCtrlmodeReg_SPEC;
impl crate::sealed::RegSpec for OqspifCtrlmodeReg_SPEC {
    type DataType = u32;
}
#[doc = "Mode Control register"]
pub type OqspifCtrlmodeReg = crate::RegValueT<OqspifCtrlmodeReg_SPEC>;

impl OqspifCtrlmodeReg {
    #[doc = "The value of OSPI_IO4-7 pads if OSPI_IO_UH_OEN is 1"]
    #[inline(always)]
    pub fn ospic_io_uh_dat(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, OqspifCtrlmodeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0xf,1,0,u8, OqspifCtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Forces the output enable for the upper half of the OSPI bus (OSPI_IO4-7). Set this bit to 1 only in SPI, Dual or Quad SPI mode to control the upper half of the OSPI bus. When the Octal SPI is enabled in the flash device, set this bit to zero.\n0: The OSPI_IO4-7 pad direction is decided by the controller.\n1: The OSPI_IO4-7 pad are outputs. The output values are defined by the corresponding OSPIC_IO_UH_DAT bits."]
    #[inline(always)]
    pub fn ospic_io_uh_oen(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, OqspifCtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27,1,0,OqspifCtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "This bit has meaning only for the read in auto mode and only when the read access in the AHB bus is an incremental bust of unspecified length.\n0: The length of the burst is considered as unspecified. The access in the flash device will be implemented as is defined by the OSPIC BUF_LIM_EN bit.\n1: The length of the burst is considered as equal to 8-bytes. The access in the flash device will be implemented by the controller as one or more different bursts, until to be served the access in the AHB bus. Each burst in the flash device will have maximum length of 8 bytes.\nThe setting OSPIC_INC_LIM_EN=1 is useful if we know that the masters that make use of the incremental burst of unspecified length, require no more than 8 bytes."]
    #[inline(always)]
    pub fn ospic_inc_lim_en(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, OqspifCtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18,1,0,OqspifCtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Controls the generation of AHB bus error response when a read is performed in the address space where the flash device is mapped and the Auto mode is not enabled.\n0: The controller ignores the access. There is no error response due to the read access.\n1: The controller responds with an AHB error response."]
    #[inline(always)]
    pub fn ospic_rd_err_en(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, OqspifCtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17,1,0,OqspifCtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Selection of the direction change method in manual mode.\n0 : the bus direction goes to input after each access\n1 : the bus direction goes to input only after a dummy access"]
    #[inline(always)]
    pub fn ospic_man_dirchg_md(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, OqspifCtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,OqspifCtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Defines the clock cycle where the bus will turn in Hi-z during the transmission of dummy bytes. This is applicable in both Manual and Auto mode.\n0 : the bus will become Hi-Z on the last clock\n1 : the bus will become Hi-Z on the last two clocks"]
    #[inline(always)]
    pub fn ospic_dmy_md(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, OqspifCtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,OqspifCtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Defines the number of bytes that consist the instruction code in the command sequences that produced by the OSPIC during Auto mode.\n0 : The instruction code is one byte only.\n1 : The instruction code is two bytes. The second byte of the instruction code is the inverse of the first byte.\n\nThe command sequence that is produced by the OSPIC_BURSTBRK_REG is not affected by this setting."]
    #[inline(always)]
    pub fn ospic_cmd_x2_en(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, OqspifCtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,OqspifCtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Controls the length of the address that the external memory device uses.\n0: The external memory device uses 24 bits address.\n1: The external memory device uses 32 bits address.\nThe controller uses this bit in order to decide the number of the address bytes that has to transfer to the external device during Auto mode."]
    #[inline(always)]
    pub fn ospic_use_32ba(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, OqspifCtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,OqspifCtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "This bit has meaning only for the read in auto mode. Defines the behavior of the controller when the internal buffer is full and there are more data to be retrieved for the current burst.\n0: The access in the flash device is not terminated when the internal buffer has no empty space. In this case the OSPI_SCK clock is blocked until to free space in the internal buffer.\n1: The access in the flash device is terminated when the internal buffer has no empty space. A new access in the flash device will be initiated when will be requested addresses that are not present in the internal buffer.\nIn both cases the access in the flash device is terminated when there is no any read request."]
    #[inline(always)]
    pub fn ospic_buf_lim_en(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, OqspifCtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,OqspifCtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Read pipe clock delay relative to the falling edge of OSPI_SCK.\nRefer to OSPI Timing for timing parameters and recommended values: 0 to 7"]
    #[inline(always)]
    pub fn ospic_pclk_md(
        self,
    ) -> crate::common::RegisterField<9, 0x7, 1, 0, u8, OqspifCtrlmodeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x7,1,0,u8, OqspifCtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Controls the use of the data read pipe.\n0: The read pipe is disabled; the sampling clock is defined according to the OSPIC_RXD_NEG setting.\n1: The read pipe is enabled. The delay of the sampling clock is defined according to the OSPICI_PCLK_MD setting. (Recommended)"]
    #[inline(always)]
    pub fn ospic_rpipe_en(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, OqspifCtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,OqspifCtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Defines the clock edge that is used for the capturing of the received data, when the read pipe is not active (OSPIC_RPIPE_EN = 0).\n\n0: Sampling of the received data with the positive edge of the OSPI_SCK\n1: Sampling of the received data with the negative edge of the OSPI_SCK\n\nThe internal OSPI_SCK clock that is used by the controller for the capturing of the received data has a skew in respect of the OSPI_SCK that is received by the external memory device. In order to be improved the timing requirements of the read path, the controller supports a read pipe register with programmable clock delay. See also the OSPIC_RPIPE_EN register."]
    #[inline(always)]
    pub fn ospic_rxd_neg(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, OqspifCtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,OqspifCtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "This configuration bit is useful when the frequency of the OSPI clock is much lower than the clock of the AMBA bus, in order to not locks the AMBA bus for a long time.\n\n0: Adds wait states via hready signal when an access is performed on the OSPIC_CTRLBUS_REG, OSPIC_WRITEDATA, OSPIC_READDATA and OSPIC_DUMMYDATA registers. It is not needed to check the OSPIC_BUSY of the OSPIC_STATUS_REG.\n1: The controller don\'t adds wait states via the hready signal, when is performed access on the OSPIC_CTRLBUS_REG, OSPIC_WRITEDATA, OSPIC_READDATA and OSPIC_DUMMYDATA registers. The OSPIC_BUSY bit of the OSPIC_STATUS_REG must be checked in order to be detected the completion of the requested access.\n\nIt is applicable only when the controller is in Manual mode. In the case of the Auto mode, the controller always adds wait states via the hready signal."]
    #[inline(always)]
    pub fn ospic_hrdy_md(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, OqspifCtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,OqspifCtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "The value of OSPI_IO3 pad if OSPI_IO3_OEN is 1"]
    #[inline(always)]
    pub fn ospic_io3_dat(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, OqspifCtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,OqspifCtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "The value of OSPI_IO2 pad if OSPI_IO2_OEN is 1"]
    #[inline(always)]
    pub fn ospic_io2_dat(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, OqspifCtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,OqspifCtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Forces the output enable of the OSPI_IO3. Set this bit to 1 only in SPI or Dual SPI mode to control the /HOLD signal. When the Quad or Octal SPI is enabled in the flash device, set this bit to zero.\n0: The OSPI_IO3 pad direction is decided by the controller.\n1: The OSPI_IO3 pad is output. The output value is defined by the OSPIC_IO3_DAT."]
    #[inline(always)]
    pub fn ospic_io3_oen(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, OqspifCtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,OqspifCtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Forces the output enable of the OSPI_IO2. Set this bit to 1 only in SPI or Dual SPI mode to control the /WP signal. When the Quad or Octal SPI is enabled in the flash device, set this bit to zero.\n0: The OSPI_IO2 pad direction is decided by the controller.\n1: The OSPI_IO2 pad is output. The output value is defined by the OSPIC_IO2_DAT."]
    #[inline(always)]
    pub fn ospic_io2_oen(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, OqspifCtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,OqspifCtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Mode of the generated OSPI_SCK clock\n0: Use Mode 0 for the OSPI_CLK. The OSPI_SCK is low when OSPI_CS is high.\n1: Use Mode 3 for the OSPI_CLK. The OSPI_SCK is high when OSPI_CS is high."]
    #[inline(always)]
    pub fn ospic_clk_md(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, OqspifCtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,OqspifCtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Mode of operation\n0: The Manual Mode is selected.\n1: The Auto Mode is selected.\nDuring an erasing the OSPIC_AUTO_MD goes in read only mode (see OSPIC_ERASE_EN)"]
    #[inline(always)]
    pub fn ospic_auto_md(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, OqspifCtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,OqspifCtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for OqspifCtrlmodeReg {
    #[inline(always)]
    fn default() -> OqspifCtrlmodeReg {
        <crate::RegValueT<OqspifCtrlmodeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OqspifCtrCtrlReg_SPEC;
impl crate::sealed::RegSpec for OqspifCtrCtrlReg_SPEC {
    type DataType = u32;
}
#[doc = "Control register for the decryption engine of the OSPIC"]
pub type OqspifCtrCtrlReg = crate::RegValueT<OqspifCtrCtrlReg_SPEC>;

impl OqspifCtrCtrlReg {
    #[doc = "Controls the AES-CTR decryption feature of the OSPIC, which enables the decryption (on-the-fly) of the data that are retrieved from the flash memory device.\n0: The AES-CTR decryption is disabled.\n1: The controller will decrypt the content of the flash memory device that is placed in the address space that is defined by the OSPIC_CTR_SADDR_REG and OSPIC_CTR_EADDR_REG registers. The data that are placed outside the previous space are not decrypted by the OSPIC. The decryption is performed by using the AES-CTR algorithm. The AES key is defined by the OSPIC_CTR_KEY_x_y_REG registers and the nonce value by the OSPIC_CTR_NONCE_x_y_REG registers.\nThis configuration bit has meaning only while the controller is in Auto mode. The on-the-fly decryption is not provided in Manual mode."]
    #[inline(always)]
    pub fn ospic_ctr_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, OqspifCtrCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,OqspifCtrCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for OqspifCtrCtrlReg {
    #[inline(always)]
    fn default() -> OqspifCtrCtrlReg {
        <crate::RegValueT<OqspifCtrCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OqspifCtrEaddrReg_SPEC;
impl crate::sealed::RegSpec for OqspifCtrEaddrReg_SPEC {
    type DataType = u32;
}
#[doc = "End address of the encrypted content in the OSPI flash"]
pub type OqspifCtrEaddrReg = crate::RegValueT<OqspifCtrEaddrReg_SPEC>;

impl OqspifCtrEaddrReg {
    #[doc = "Defines the bits \\[31:10\\] of the end address in the flash memory, where an encrypted image is placed. The bits \\[9:0\\] are considered always as 0x3ff. This has meaning only when the decryption is active. See also the register OSPIC_CTR_CTRL_REG\\[OSPIC_CTR_EN\\]."]
    #[inline(always)]
    pub fn ospic_ctr_eaddr(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3fffff,
        1,
        0,
        u32,
        OqspifCtrEaddrReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3fffff,
            1,
            0,
            u32,
            OqspifCtrEaddrReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OqspifCtrEaddrReg {
    #[inline(always)]
    fn default() -> OqspifCtrEaddrReg {
        <crate::RegValueT<OqspifCtrEaddrReg_SPEC> as RegisterValue<_>>::new(1023)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OqspifCtrKey03Reg_SPEC;
impl crate::sealed::RegSpec for OqspifCtrKey03Reg_SPEC {
    type DataType = u32;
}
#[doc = "Key bytes 0 to 3 for the AES-CTR algorithm"]
pub type OqspifCtrKey03Reg = crate::RegValueT<OqspifCtrKey03Reg_SPEC>;

impl OqspifCtrKey03Reg {
    #[doc = "Defines the key that is used by the AES-CTR algorithm, when the on-the-fly decryption is enabled ( OSPIC_CTR_CTRL_REG\\[OSPIC_CTR_EN\\] = 1 ). The size of the decryption key is 256bits or 32 bytes :\n\nK0 K1 K2 K3...K30 K31.\n\nThe mapping of the bytes to the corresponding OSPIC_CTR_KEY_X_Y_REG registers is the following :\n\n{K0, K1, K2, K3} = OSPIC_CTR_KEY_0_3_REG\\[31:0\\]\n{K4, K5, K6, K7} = OSPIC_CTR_KEY_4_7_REG\\[31:0\\]\n{K8, K9, K10, K11} = OSPIC_CTR_KEY_8_11_REG\\[31:0\\]\n{K12, K13, K14, K15} = OSPIC_CTR_KEY_12_15_REG\\[31:0\\]\n{K16, K17, K18, K19} = OSPIC_CTR_KEY_16_19_REG\\[31:0\\]\n{K20, K21, K22, K23} = OSPIC_CTR_KEY_20_23_REG\\[31:0\\]\n{K24, K25, K26, K27} = OSPIC_CTR_KEY_24_27_REG\\[31:0\\]\n{K28, K29, K30, K31} = OSPIC_CTR_KEY_28_31_REG\\[31:0\\]\n\nAll these registers make sense only when OSPIC_CTR_CTRL_REG\\[OSPIC_CTR_EN\\] = 1. Do not perform access to an encrypted address range while the updating process of the decryption key is in progress."]
    #[inline(always)]
    pub fn ospic_ctr_key_0_3(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        OqspifCtrKey03Reg_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            OqspifCtrKey03Reg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OqspifCtrKey03Reg {
    #[inline(always)]
    fn default() -> OqspifCtrKey03Reg {
        <crate::RegValueT<OqspifCtrKey03Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OqspifCtrKey1215Reg_SPEC;
impl crate::sealed::RegSpec for OqspifCtrKey1215Reg_SPEC {
    type DataType = u32;
}
#[doc = "Key bytes 12 to 15 for the AES-CTR algorithm"]
pub type OqspifCtrKey1215Reg = crate::RegValueT<OqspifCtrKey1215Reg_SPEC>;

impl OqspifCtrKey1215Reg {
    #[doc = "See the description in the OSPIC_CTR_KEY_0_3."]
    #[inline(always)]
    pub fn ospic_ctr_key_12_15(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        OqspifCtrKey1215Reg_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            OqspifCtrKey1215Reg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OqspifCtrKey1215Reg {
    #[inline(always)]
    fn default() -> OqspifCtrKey1215Reg {
        <crate::RegValueT<OqspifCtrKey1215Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OqspifCtrKey1619Reg_SPEC;
impl crate::sealed::RegSpec for OqspifCtrKey1619Reg_SPEC {
    type DataType = u32;
}
#[doc = "Key bytes 16 to 19 for the AES-CTR algorithm"]
pub type OqspifCtrKey1619Reg = crate::RegValueT<OqspifCtrKey1619Reg_SPEC>;

impl OqspifCtrKey1619Reg {
    #[doc = "See the description in the OSPIC_CTR_KEY_0_3."]
    #[inline(always)]
    pub fn ospic_ctr_key_16_19(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        OqspifCtrKey1619Reg_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            OqspifCtrKey1619Reg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OqspifCtrKey1619Reg {
    #[inline(always)]
    fn default() -> OqspifCtrKey1619Reg {
        <crate::RegValueT<OqspifCtrKey1619Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OqspifCtrKey2023Reg_SPEC;
impl crate::sealed::RegSpec for OqspifCtrKey2023Reg_SPEC {
    type DataType = u32;
}
#[doc = "Key bytes 20 to 23 for the AES-CTR algorithm"]
pub type OqspifCtrKey2023Reg = crate::RegValueT<OqspifCtrKey2023Reg_SPEC>;

impl OqspifCtrKey2023Reg {
    #[doc = "See the description in the OSPIC_CTR_KEY_0_3."]
    #[inline(always)]
    pub fn ospic_ctr_key_20_23(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        OqspifCtrKey2023Reg_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            OqspifCtrKey2023Reg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OqspifCtrKey2023Reg {
    #[inline(always)]
    fn default() -> OqspifCtrKey2023Reg {
        <crate::RegValueT<OqspifCtrKey2023Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OqspifCtrKey2427Reg_SPEC;
impl crate::sealed::RegSpec for OqspifCtrKey2427Reg_SPEC {
    type DataType = u32;
}
#[doc = "Key bytes 24 to 27 for the AES-CTR algorithm"]
pub type OqspifCtrKey2427Reg = crate::RegValueT<OqspifCtrKey2427Reg_SPEC>;

impl OqspifCtrKey2427Reg {
    #[doc = "See the description in the OSPIC_CTR_KEY_0_3."]
    #[inline(always)]
    pub fn ospic_ctr_key_24_27(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        OqspifCtrKey2427Reg_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            OqspifCtrKey2427Reg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OqspifCtrKey2427Reg {
    #[inline(always)]
    fn default() -> OqspifCtrKey2427Reg {
        <crate::RegValueT<OqspifCtrKey2427Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OqspifCtrKey2831Reg_SPEC;
impl crate::sealed::RegSpec for OqspifCtrKey2831Reg_SPEC {
    type DataType = u32;
}
#[doc = "Key bytes 28 to 31 for the AES-CTR algorithm"]
pub type OqspifCtrKey2831Reg = crate::RegValueT<OqspifCtrKey2831Reg_SPEC>;

impl OqspifCtrKey2831Reg {
    #[doc = "See the description in the OSPIC_CTR_KEY_0_3."]
    #[inline(always)]
    pub fn ospic_ctr_key_28_31(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        OqspifCtrKey2831Reg_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            OqspifCtrKey2831Reg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OqspifCtrKey2831Reg {
    #[inline(always)]
    fn default() -> OqspifCtrKey2831Reg {
        <crate::RegValueT<OqspifCtrKey2831Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OqspifCtrKey47Reg_SPEC;
impl crate::sealed::RegSpec for OqspifCtrKey47Reg_SPEC {
    type DataType = u32;
}
#[doc = "Key bytes 4 to 7 for the AES-CTR algorithm"]
pub type OqspifCtrKey47Reg = crate::RegValueT<OqspifCtrKey47Reg_SPEC>;

impl OqspifCtrKey47Reg {
    #[doc = "See the description in the OSPIC_CTR_KEY_0_3."]
    #[inline(always)]
    pub fn ospic_ctr_key_4_7(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        OqspifCtrKey47Reg_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            OqspifCtrKey47Reg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OqspifCtrKey47Reg {
    #[inline(always)]
    fn default() -> OqspifCtrKey47Reg {
        <crate::RegValueT<OqspifCtrKey47Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OqspifCtrKey811Reg_SPEC;
impl crate::sealed::RegSpec for OqspifCtrKey811Reg_SPEC {
    type DataType = u32;
}
#[doc = "Key bytes 8 to 11 for the AES-CTR algorithm"]
pub type OqspifCtrKey811Reg = crate::RegValueT<OqspifCtrKey811Reg_SPEC>;

impl OqspifCtrKey811Reg {
    #[doc = "See the description in the OSPIC_CTR_KEY_0_3."]
    #[inline(always)]
    pub fn ospic_ctr_key_8_11(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        OqspifCtrKey811Reg_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            OqspifCtrKey811Reg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OqspifCtrKey811Reg {
    #[inline(always)]
    fn default() -> OqspifCtrKey811Reg {
        <crate::RegValueT<OqspifCtrKey811Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OqspifCtrNonce03Reg_SPEC;
impl crate::sealed::RegSpec for OqspifCtrNonce03Reg_SPEC {
    type DataType = u32;
}
#[doc = "Nonce bytes 0 to 3 for the AES-CTR algorithm"]
pub type OqspifCtrNonce03Reg = crate::RegValueT<OqspifCtrNonce03Reg_SPEC>;

impl OqspifCtrNonce03Reg {
    #[doc = "Defines the 8 bytes of the nonce value (N0 - N7) that is used by the AES-CTR algorithm in order to be constructed the counter block (CTRB). The total size of the counter block is 128 bits or 16 bytes :\n\nCTRB0 CTRB1 CTRB2 CTRB3...CTRB14 CTRB15.\n\nThe first 8 bytes (CTRB0 - CTRB7) of the counter block consisted by the nonce value.\nThe next 8 bytes of the counter block (CTRB8-CTRB15), are produced automatically by the hardware based on the address offset inside the encrypted image, from where are retrieved the requested data.\nThe mapping of the nonce bytes to the corresponding OSPIC_NONCE_X_Y_REG registers is the following :\n\n{CTRB0, CTRB1, CTRB2, CTRB3} = {N0, N1, N2, N3} = OSPIC_NONCE_0_3_REG\\[31:0\\]\n{CTRB4, CTRB5, CTRB6, CTRB7} = {N4, N5, N6, N7} = OSPIC_NONCE_4_7_REG\\[31:0\\]\n\nAll these registers make sense only when OSPIC_CTR_CTRL_REG\\[OSPIC_CTR_EN\\] = 1. Do not perform access to an encrypted address range while the updating process of the nonce value is in progress."]
    #[inline(always)]
    pub fn ospic_ctr_nonce_0_3(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        OqspifCtrNonce03Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            OqspifCtrNonce03Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OqspifCtrNonce03Reg {
    #[inline(always)]
    fn default() -> OqspifCtrNonce03Reg {
        <crate::RegValueT<OqspifCtrNonce03Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OqspifCtrNonce47Reg_SPEC;
impl crate::sealed::RegSpec for OqspifCtrNonce47Reg_SPEC {
    type DataType = u32;
}
#[doc = "Nonce bytes 4 to 7 for the AES-CTR algorithm"]
pub type OqspifCtrNonce47Reg = crate::RegValueT<OqspifCtrNonce47Reg_SPEC>;

impl OqspifCtrNonce47Reg {
    #[doc = "See the description in the OSPIC_NONCE_0_3."]
    #[inline(always)]
    pub fn ospic_ctr_nonce_4_7(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        OqspifCtrNonce47Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            OqspifCtrNonce47Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OqspifCtrNonce47Reg {
    #[inline(always)]
    fn default() -> OqspifCtrNonce47Reg {
        <crate::RegValueT<OqspifCtrNonce47Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OqspifCtrSaddrReg_SPEC;
impl crate::sealed::RegSpec for OqspifCtrSaddrReg_SPEC {
    type DataType = u32;
}
#[doc = "Start address of the encrypted content in the OSPI flash"]
pub type OqspifCtrSaddrReg = crate::RegValueT<OqspifCtrSaddrReg_SPEC>;

impl OqspifCtrSaddrReg {
    #[doc = "Defines the bits \\[31:10\\] of the start address in the flash memory, where an encrypted image is placed. The bits \\[9:0\\] are considered always as zero. This has meaning only when the decryption is active. See also the register OSPIC_CTR_CTRL_REG\\[OSPIC_CTR_EN\\]."]
    #[inline(always)]
    pub fn ospic_ctr_saddr(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3fffff,
        1,
        0,
        u32,
        OqspifCtrSaddrReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3fffff,
            1,
            0,
            u32,
            OqspifCtrSaddrReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OqspifCtrSaddrReg {
    #[inline(always)]
    fn default() -> OqspifCtrSaddrReg {
        <crate::RegValueT<OqspifCtrSaddrReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OqspifDummydataReg_SPEC;
impl crate::sealed::RegSpec for OqspifDummydataReg_SPEC {
    type DataType = u32;
}
#[doc = "Send dummy clocks to SPI Bus for the Manual mode"]
pub type OqspifDummydataReg = crate::RegValueT<OqspifDummydataReg_SPEC>;

impl OqspifDummydataReg {
    #[doc = "Writing to this register generates a number of clock pulses to the SPI bus. During the last clock of this activity in the SPI bus, the OSPI_IOx data pads are in hi-z state (see also the OSPIC_DMY_MD). The data size of the access to this register can be 32-bits / 16-bits/ 8-bits. The number of generated pulses is equal to: (size of AHB bus access) / (size of SPI bus). The size of SPI bus is equal to 1, 2, 4 or 8 for Single, Dual, Quad or Octal SPI mode respectively.\nThis register has meaning only when the controller is in Manual mode."]
    #[inline(always)]
    pub fn ospic_dummydata(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        OqspifDummydataReg_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            OqspifDummydataReg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OqspifDummydataReg {
    #[inline(always)]
    fn default() -> OqspifDummydataReg {
        <crate::RegValueT<OqspifDummydataReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OqspifErasecmdaReg_SPEC;
impl crate::sealed::RegSpec for OqspifErasecmdaReg_SPEC {
    type DataType = u32;
}
#[doc = "The way of erasing in Auto mode (command register A)"]
pub type OqspifErasecmdaReg = crate::RegValueT<OqspifErasecmdaReg_SPEC>;

impl OqspifErasecmdaReg {
    #[doc = "The code value of the erase resume instruction"]
    #[inline(always)]
    pub fn ospic_res_inst(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, OqspifErasecmdaReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8, OqspifErasecmdaReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "The code value of the erase suspend instruction."]
    #[inline(always)]
    pub fn ospic_sus_inst(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, OqspifErasecmdaReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8, OqspifErasecmdaReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "The code value of the write enable instruction."]
    #[inline(always)]
    pub fn ospic_wen_inst(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, OqspifErasecmdaReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8, OqspifErasecmdaReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "The code value of the erase instruction."]
    #[inline(always)]
    pub fn ospic_ers_inst(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, OqspifErasecmdaReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, OqspifErasecmdaReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for OqspifErasecmdaReg {
    #[inline(always)]
    fn default() -> OqspifErasecmdaReg {
        <crate::RegValueT<OqspifErasecmdaReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OqspifErasecmdbReg_SPEC;
impl crate::sealed::RegSpec for OqspifErasecmdbReg_SPEC {
    type DataType = u32;
}
#[doc = "The way of erasing in Auto mode (command register B)"]
pub type OqspifErasecmdbReg = crate::RegValueT<OqspifErasecmdbReg_SPEC>;

impl OqspifErasecmdbReg {
    #[doc = "Defines a timer that counts the minimum allowed delay between an erase suspend command and the previous erase resume command (or the initial erase command).\n0: Don\'t wait. The controller starts immediately to suspend the erase procedure.\n1..255: The controller waits for at least this number of 222kHz clock cycles before the suspension of erasing. Time starts counting after the end of the previous erase resume command (or the initial erase command)"]
    #[inline(always)]
    pub fn ospic_ressus_dly(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, OqspifErasecmdbReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8, OqspifErasecmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "The controller must stay without flash memory reading requests for this number of AMBA hclk clock cycles, before to perform the command of erase or erase resume\n15 - 0"]
    #[inline(always)]
    pub fn ospic_ersres_hld(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, OqspifErasecmdbReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xf,1,0,u8, OqspifErasecmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "After the execution of instructions: write enable, erase, erase suspend and erase resume, the OSPI_CS remains high for at least this number of OSPI bus clock cycles."]
    #[inline(always)]
    pub fn ospic_ers_cs_hi(
        self,
    ) -> crate::common::RegisterField<10, 0x1f, 1, 0, u8, OqspifErasecmdbReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1f,1,0,u8, OqspifErasecmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "The mode of the OSPI Bus during the address phase of the erase instruction\n0x0: Single\n0x1: Dual\n0x2: Quad\n0x3: Octal"]
    #[inline(always)]
    pub fn ospic_ead_tx_md(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, OqspifErasecmdbReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8, OqspifErasecmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "The mode of the OSPI Bus during the transmission of the resume instruction\n0x0: Single\n0x1: Dual\n0x2: Quad\n0x3: Octal"]
    #[inline(always)]
    pub fn ospic_res_tx_md(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, OqspifErasecmdbReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,u8, OqspifErasecmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "The mode of the OSPI Bus during the transmission of the suspend instruction.\n0x0: Single\n0x1: Dual\n0x2: Quad\n0x3: Octal"]
    #[inline(always)]
    pub fn ospic_sus_tx_md(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, OqspifErasecmdbReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3,1,0,u8, OqspifErasecmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "The mode of the OSPI Bus during the transmission of the write enable instruction.\n0x0: Single\n0x1: Dual\n0x2: Quad\n0x3: Octal"]
    #[inline(always)]
    pub fn ospic_wen_tx_md(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, OqspifErasecmdbReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3,1,0,u8, OqspifErasecmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "The mode of the OSPI Bus during the instruction phase of the erase instruction\n0x0: Single\n0x1: Dual\n0x2: Quad\n0x3: Octal"]
    #[inline(always)]
    pub fn ospic_ers_tx_md(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, OqspifErasecmdbReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8, OqspifErasecmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for OqspifErasecmdbReg {
    #[inline(always)]
    fn default() -> OqspifErasecmdbReg {
        <crate::RegValueT<OqspifErasecmdbReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OqspifErasecmdcReg_SPEC;
impl crate::sealed::RegSpec for OqspifErasecmdcReg_SPEC {
    type DataType = u32;
}
#[doc = "The way of erasing in Auto mode (command register C)"]
pub type OqspifErasecmdcReg = crate::RegValueT<OqspifErasecmdcReg_SPEC>;

impl OqspifErasecmdcReg {
    #[doc = "Defines a timer that counts the minimum allowed delay between an erase suspend command and the next read status command.\n0: Don\'t wait. The controller starts immediately to read the status of the flash device.\n1..63: The controller waits for at least this number of 222kHz clock cycles before to read the status of the flash device. Time starts counting when the erase resume command is applied."]
    #[inline(always)]
    pub fn ospic_sussts_dly(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, OqspifErasecmdcReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8, OqspifErasecmdcReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for OqspifErasecmdcReg {
    #[inline(always)]
    fn default() -> OqspifErasecmdcReg {
        <crate::RegValueT<OqspifErasecmdcReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OqspifErasectrlReg_SPEC;
impl crate::sealed::RegSpec for OqspifErasectrlReg_SPEC {
    type DataType = u32;
}
#[doc = "OSPI Erase control register"]
pub type OqspifErasectrlReg = crate::RegValueT<OqspifErasectrlReg_SPEC>;

impl OqspifErasectrlReg {
    #[doc = "This configuration bit has meaning when an erase has been suspended. Normally the erase will be resumed when the flash will stay idle (without read accesses) for a predefined number of clock cycles (see OSPIC_ERASECMDB_REG \\[OSPIC_ERSRES_HLD\\]). By setting this bit the execution of the erase resume process can be postponed.\n0: A suspended erase will be resumed based on the setting in the OSPIC_ERSRES_HLD.\n1: The erase will not be resumed even after the expiration of the OSPIC_ERSRES_HLD. The erase can be resumed again only when the OSPIC_ERS_RES_DIS=0."]
    #[inline(always)]
    pub fn ospic_ers_res_dis(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, OqspifErasectrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<28,1,0,OqspifErasectrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "It shows the progress of sector/block erasing (read only).\n0x0: No Erase.\n0x1: Pending erase request\n0x2: Erase procedure is running\n0x3: Suspended Erase procedure\n0x4: Finishing the Erase procedure\n0x5..0x7: Reserved"]
    #[inline(always)]
    pub fn ospic_ers_state(
        self,
    ) -> crate::common::RegisterField<25, 0x7, 1, 0, u8, OqspifErasectrlReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<25,0x7,1,0,u8, OqspifErasectrlReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "During Manual mode (OSPIC_AUTO_MD = 0). This bit is in read only mode.\nDuring Auto mode (OSPIC_AUTO_MD = 1). To request the erasing of the block/sector (OSPIC_ERS_ADDR, 12\'b0) write 1 to this bit. This bit is cleared automatically with the end of the erasing. Until the end of erasing the OSPIC_ERASE_EN remains in read only mode. During the same time interval the controller remains in Auto Mode (OSPIC_AUTO_MD goes in read only mode)."]
    #[inline(always)]
    pub fn ospic_erase_en(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, OqspifErasectrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<24,1,0,OqspifErasectrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Defines the address of the block/sector that is requested to be erased.\nIf OSPIC_USE_32BA = 0 (24 bits addressing), bits OSPIC_ERASECTRL_REG\\[23-12\\] determine the block/ sector address bits \\[23-12\\]. The OSPIC_ERASECTRL_REG\\[11-4\\] are ignored by the controller.\nIf OSPIC_USE_32BA = 1 (32 bits addressing) bits OSPIC_ERASECTRL_REG\\[23-4\\] determine the block / sectors address bits \\[31:12\\]"]
    #[inline(always)]
    pub fn ospic_ers_addr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0xfffff,
        1,
        0,
        u32,
        OqspifErasectrlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0xfffff,
            1,
            0,
            u32,
            OqspifErasectrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OqspifErasectrlReg {
    #[inline(always)]
    fn default() -> OqspifErasectrlReg {
        <crate::RegValueT<OqspifErasectrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OqspifGpReg_SPEC;
impl crate::sealed::RegSpec for OqspifGpReg_SPEC {
    type DataType = u32;
}
#[doc = "OSPI General Purpose control register"]
pub type OqspifGpReg = crate::RegValueT<OqspifGpReg_SPEC>;

impl OqspifGpReg {
    #[doc = "QSPI pads slew rate control. Indicative values under certain conditions:\n0: Rise=1.7 V/ns, Fall=1.9 V/ns (weak)\n1: Rise=2.0 V/ns, Fall=2.3 V/ns\n2: Rise=2.3 V/ns, Fall=2.6 V/ns\n3: Rise=2.4 V/ns, Fall=2.7 V/ns (strong)\nConditions: FLASH pin capacitance 6 pF, Vcc=1.8V, T=25C and Idrive=16mA."]
    #[inline(always)]
    pub fn ospic_pads_slew(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, OqspifGpReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x3,1,0,u8, OqspifGpReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "QSPI pads drive current\n0: 4 mA\n1: 8 mA\n2: 12 mA\n3: 16 mA"]
    #[inline(always)]
    pub fn ospic_pads_drv(
        self,
    ) -> crate::common::RegisterField<1, 0x3, 1, 0, u8, OqspifGpReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x3,1,0,u8, OqspifGpReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for OqspifGpReg {
    #[inline(always)]
    fn default() -> OqspifGpReg {
        <crate::RegValueT<OqspifGpReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OqspifReaddataReg_SPEC;
impl crate::sealed::RegSpec for OqspifReaddataReg_SPEC {
    type DataType = u32;
}
#[doc = "Read data from SPI Bus for the Manual mode"]
pub type OqspifReaddataReg = crate::RegValueT<OqspifReaddataReg_SPEC>;

impl OqspifReaddataReg {
    #[doc = "A read access at this register generates a data transfer from the external memory device to the OSPIC controller. The data is transferred using the selected mode of the SPI bus (Single SPI, Dual SPI, Quad SPI or Octal SPI). The data size of the access to this register can be 32-bits / 16-bits / 8-bits and is equal to the number of the transferred bits.\nThis register has meaning only when the controller is in Manual mode."]
    #[inline(always)]
    pub fn ospic_readdata(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        OqspifReaddataReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            OqspifReaddataReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OqspifReaddataReg {
    #[inline(always)]
    fn default() -> OqspifReaddataReg {
        <crate::RegValueT<OqspifReaddataReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OqspifRecvdataReg_SPEC;
impl crate::sealed::RegSpec for OqspifRecvdataReg_SPEC {
    type DataType = u32;
}
#[doc = "Received data for the Manual mode"]
pub type OqspifRecvdataReg = crate::RegValueT<OqspifRecvdataReg_SPEC>;

impl OqspifRecvdataReg {
    #[doc = "This register contains the received data when the OSPIC_READDATA_REG register is used in Manual mode, in order to be retrieved data from the external memory device and OSPIC_HRDY_MD=1 && OSPIC_BUSY=0."]
    #[inline(always)]
    pub fn ospic_recvdata(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        OqspifRecvdataReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            OqspifRecvdataReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OqspifRecvdataReg {
    #[inline(always)]
    fn default() -> OqspifRecvdataReg {
        <crate::RegValueT<OqspifRecvdataReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OqspifStatuscmdReg_SPEC;
impl crate::sealed::RegSpec for OqspifStatuscmdReg_SPEC {
    type DataType = u32;
}
#[doc = "The way of reading the status of external device in Auto mode"]
pub type OqspifStatuscmdReg = crate::RegValueT<OqspifStatuscmdReg_SPEC>;

impl OqspifStatuscmdReg {
    #[doc = "Defines the value of that is transferred on the OSPI bus during the phase of the dummy bytes.\n0: The controller keeps the data in the bus unchanged, until to change the bus direction in input mode.\n1: Forces the dummy bytes to get the zero value (only for the cycles that are not in input mode). Only the IO pins that are related with the transfer mode of the dummy bytes (OSPIC_RSTAT_DMY_TX_MD) will get the zero value."]
    #[inline(always)]
    pub fn ospic_rstat_dmy_zero(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, OqspifStatuscmdReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<30,1,0,OqspifStatuscmdReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "It describes the mode of the OSPI bus during the dummy bytes phase.\n0x0: Single SPI\n0x1: Dual\n0x2: Quad\n0x3: Octal"]
    #[inline(always)]
    pub fn ospic_rstat_dmy_tx_md(
        self,
    ) -> crate::common::RegisterField<28, 0x3, 1, 0, u8, OqspifStatuscmdReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x3,1,0,u8, OqspifStatuscmdReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Number of dummy bytes (minus 1). Can be set 1 up to 16 dummy bytes (values 0 up to 15). The dummy bytes are applied only when OSPIC_RSTAT_DMY_EN=1."]
    #[inline(always)]
    pub fn ospic_rstat_dmy_num(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, OqspifStatuscmdReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xf,1,0,u8, OqspifStatuscmdReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enables the transmission of dummy bytes, immediately after the instruction code of the read status command.\n0: Don\'t send the dummy bytes\n1: Send the dummy bytes. The number of the dummy bytes is defined by the OSPIC_RSTAT_DMY_NUM."]
    #[inline(always)]
    pub fn ospic_rstat_dmy_en(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, OqspifStatuscmdReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<23,1,0,OqspifStatuscmdReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Defines the timer which is used to count the delay that it has to wait before to read the FLASH Status Register, after an erase or an erase resume command.\n0: The delay is controlled by the OSPIC_RESSTS_DLY which counts on the OSPI_CLK clock.\n1: The delay is controlled by the OSPIC_RESSUS_DLY which counts on the 222 kHz clock."]
    #[inline(always)]
    pub fn ospic_stsdly_sel(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, OqspifStatuscmdReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<22,1,0,OqspifStatuscmdReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Defines a timer that counts the minimum required delay between the reading of the status register and of the previous erase or erase resume instruction.\n0: Don\'t wait. The controller starts to reading the Flash memory status register immediately.\n1..63: The controller waits for at least this number of OSPI_CLK cycles and afterwards it starts to reading the Flash memory status register. The timer starts to count after the end of the previous erase or erase resume command.\n\nThe actual timer that will be used by the controller before the reading of the Flash memory status register is defined by the OSPIC_STSDLY_SEL."]
    #[inline(always)]
    pub fn ospic_ressts_dly(
        self,
    ) -> crate::common::RegisterField<16, 0x3f, 1, 0, u8, OqspifStatuscmdReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3f,1,0,u8, OqspifStatuscmdReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Defines the value of the Busy bit which means that the flash is busy.\n0: The flash is busy when the Busy bit is equal to 0.\n1: The flash is busy when the Busy bit is equal to 1."]
    #[inline(always)]
    pub fn ospic_busy_val(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, OqspifStatuscmdReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<15,1,0,OqspifStatuscmdReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "It describes who from the bits of status represents the Busy bit (7 - 0)."]
    #[inline(always)]
    pub fn ospic_busy_pos(
        self,
    ) -> crate::common::RegisterField<12, 0x7, 1, 0, u8, OqspifStatuscmdReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x7,1,0,u8, OqspifStatuscmdReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "The mode of the OSPI Bus during the receive status phase of the read status instruction\n0x0: Single\n0x1: Dual\n0x2: Quad\n0x3: Octal"]
    #[inline(always)]
    pub fn ospic_rstat_rx_md(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, OqspifStatuscmdReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,u8, OqspifStatuscmdReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "The mode of the OSPI Bus during the instruction phase of the read status instruction.\n0x0: Single\n0x1: Dual\n0x2: Quad\n0x3: Octal"]
    #[inline(always)]
    pub fn ospic_rstat_tx_md(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, OqspifStatuscmdReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8, OqspifStatuscmdReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "The code value of the read status instruction.\nIt is transmitted during the instruction phase of the read status instruction."]
    #[inline(always)]
    pub fn ospic_rstat_inst(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, OqspifStatuscmdReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, OqspifStatuscmdReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for OqspifStatuscmdReg {
    #[inline(always)]
    fn default() -> OqspifStatuscmdReg {
        <crate::RegValueT<OqspifStatuscmdReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OqspifStatusReg_SPEC;
impl crate::sealed::RegSpec for OqspifStatusReg_SPEC {
    type DataType = u32;
}
#[doc = "The status register of the OSPI controller"]
pub type OqspifStatusReg = crate::RegValueT<OqspifStatusReg_SPEC>;

impl OqspifStatusReg {
    #[doc = "The status of the SPI Bus.\n\n0: The SPI Bus is idle\n1: The SPI Bus is active. Read data, write data or dummy data activity is in progress.\n\nHas meaning only in Manual mode and only when OSPIC_HRDY_MD = 1."]
    #[inline(always)]
    pub fn ospic_busy(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, OqspifStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,OqspifStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for OqspifStatusReg {
    #[inline(always)]
    fn default() -> OqspifStatusReg {
        <crate::RegValueT<OqspifStatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OqspifWritedataReg_SPEC;
impl crate::sealed::RegSpec for OqspifWritedataReg_SPEC {
    type DataType = u32;
}
#[doc = "Write data to SPI Bus for the Manual mode"]
pub type OqspifWritedataReg = crate::RegValueT<OqspifWritedataReg_SPEC>;

impl OqspifWritedataReg {
    #[doc = "Writing to this register is generating a data transfer from the controller to the external memory device. The data written in this register, is then transferred to the memory using the selected mode of the SPI bus (Single SPI, Dual SPI, Quad SPI or Octal SPI). The data size of the access to this register can be 32-bits / 16-bits/ 8-bits and is equal to the number of the transferred bits.\nThis register has meaning only when the controller is in Manual mode."]
    #[inline(always)]
    pub fn ospic_writedata(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        OqspifWritedataReg_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            OqspifWritedataReg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for OqspifWritedataReg {
    #[inline(always)]
    fn default() -> OqspifWritedataReg {
        <crate::RegValueT<OqspifWritedataReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
