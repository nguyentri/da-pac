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
// Generated from SVD 1.2, with svd2pac 0.4.0 on Sat, 12 Apr 2025 22:14:17 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"QSPIC registers"]
unsafe impl ::core::marker::Send for super::Qspic {}
unsafe impl ::core::marker::Sync for super::Qspic {}
impl super::Qspic {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    #[doc = "Read break sequence in Auto mode"]
    #[inline(always)]
    pub const fn qspic_burstbrk_reg(
        &self,
    ) -> &'static crate::common::Reg<self::QspicBurstbrkReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::QspicBurstbrkReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[doc = "The way of reading in Auto mode (command register A)"]
    #[inline(always)]
    pub const fn qspic_burstcmda_reg(
        &self,
    ) -> &'static crate::common::Reg<self::QspicBurstcmdaReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::QspicBurstcmdaReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "The way of reading in Auto mode (command register B)"]
    #[inline(always)]
    pub const fn qspic_burstcmdb_reg(
        &self,
    ) -> &'static crate::common::Reg<self::QspicBurstcmdbReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::QspicBurstcmdbReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Check erase progress in Auto mode"]
    #[inline(always)]
    pub const fn qspic_chckerase_reg(
        &self,
    ) -> &'static crate::common::Reg<self::QspicChckeraseReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::QspicChckeraseReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[doc = "SPI Bus control register for the Manual mode"]
    #[inline(always)]
    pub const fn qspic_ctrlbus_reg(
        &self,
    ) -> &'static crate::common::Reg<self::QspicCtrlbusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::QspicCtrlbusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Mode Control register"]
    #[inline(always)]
    pub const fn qspic_ctrlmode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::QspicCtrlmodeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::QspicCtrlmodeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Send dummy clocks to SPI Bus for the Manual mode"]
    #[inline(always)]
    pub const fn qspic_dummydata_reg(
        &self,
    ) -> &'static crate::common::Reg<self::QspicDummydataReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::QspicDummydataReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "The way of erasing in Auto mode (command register A)"]
    #[inline(always)]
    pub const fn qspic_erasecmda_reg(
        &self,
    ) -> &'static crate::common::Reg<self::QspicErasecmdaReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::QspicErasecmdaReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[doc = "The way of erasing in Auto mode (command register B)"]
    #[inline(always)]
    pub const fn qspic_erasecmdb_reg(
        &self,
    ) -> &'static crate::common::Reg<self::QspicErasecmdbReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::QspicErasecmdbReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[doc = "QSPI Erase control register"]
    #[inline(always)]
    pub const fn qspic_erasectrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::QspicErasectrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::QspicErasectrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[doc = "QSPI General Purpose control register"]
    #[inline(always)]
    pub const fn qspic_gp_reg(
        &self,
    ) -> &'static crate::common::Reg<self::QspicGpReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::QspicGpReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }

    #[doc = "Read data from SPI Bus for the Manual mode"]
    #[inline(always)]
    pub const fn qspic_readdata_reg(
        &self,
    ) -> &'static crate::common::Reg<self::QspicReaddataReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::QspicReaddataReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "Received data for the Manual mode"]
    #[inline(always)]
    pub const fn qspic_recvdata_reg(
        &self,
    ) -> &'static crate::common::Reg<self::QspicRecvdataReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::QspicRecvdataReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "The way of reading the status of external device in Auto mode"]
    #[inline(always)]
    pub const fn qspic_statuscmd_reg(
        &self,
    ) -> &'static crate::common::Reg<self::QspicStatuscmdReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::QspicStatuscmdReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[doc = "The status register of the QSPI controller"]
    #[inline(always)]
    pub const fn qspic_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::QspicStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::QspicStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "QSPIC uCode memory"]
    #[inline(always)]
    pub const fn qspic_ucode_start(
        &self,
    ) -> &'static crate::common::Reg<self::QspicUcodeStart_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::QspicUcodeStart_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[doc = "Write data to SPI Bus for the Manual mode"]
    #[inline(always)]
    pub const fn qspic_writedata_reg(
        &self,
    ) -> &'static crate::common::Reg<self::QspicWritedataReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::QspicWritedataReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QspicBurstbrkReg_SPEC;
impl crate::sealed::RegSpec for QspicBurstbrkReg_SPEC {
    type DataType = u32;
}
#[doc = "Read break sequence in Auto mode"]
pub type QspicBurstbrkReg = crate::RegValueT<QspicBurstbrkReg_SPEC>;

impl QspicBurstbrkReg {
    #[doc = "Disable output during the transmission of the second half (QSPIC_BRK_WRD\\[3:0\\]). Setting this bit is only useful if QSPIC_BRK_EN =1 and QSPIC_BRK_SZ= 1.\n0 - The controller drives the QSPI bus during the transmission of the QSPIC_BRK_WRD\\[3:0\\].\n1 - The controller leaves the QSPI bus in Hi-Z during the transmission of the QSPIC_BRK_WORD\\[3:0\\]."]
    #[inline(always)]
    pub fn qspic_sec_hf_ds(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, QspicBurstbrkReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20,1,0,QspicBurstbrkReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "The mode of the QSPI Bus during the transmission of the burst break sequence.\n00 - Single\n01 - Dual\n10 - Quad\n11 - Reserved"]
    #[inline(always)]
    pub fn qspic_brk_tx_md(
        self,
    ) -> crate::common::RegisterField<18, 0x3, 1, 0, u8, QspicBurstbrkReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x3,1,0,u8, QspicBurstbrkReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "The size of Burst Break Sequence\n0 - One byte (Send QSPIC_BRK_WRD\\[15:8\\])\n1 - Two bytes (Send QSPIC_BRK_WRD\\[15:0\\])"]
    #[inline(always)]
    pub fn qspic_brk_sz(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, QspicBurstbrkReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17,1,0,QspicBurstbrkReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Controls the application of a special command (read burst break sequence) that is used in order to force the device to abandon the continuous read mode.\n0 - The special command is not applied\n1 - The special command is applied\n\nThis special command is applied by the controller to the external device under the following conditions:\n- the controller is in Auto mode\n- the QSPIC_INST_MD = 1\n- the previous command that has been applied in the external device was read\n- the controller want to apply to the external device a command different than the read."]
    #[inline(always)]
    pub fn qspic_brk_en(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, QspicBurstbrkReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,QspicBurstbrkReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "This is the value of a special command (read burst break sequence) that is applied by the controller to the external memory device, in order to force the memory device to abandon the continuous read mode."]
    #[inline(always)]
    pub fn qspic_brk_wrd(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, QspicBurstbrkReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16, QspicBurstbrkReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for QspicBurstbrkReg {
    #[inline(always)]
    fn default() -> QspicBurstbrkReg {
        <crate::RegValueT<QspicBurstbrkReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QspicBurstcmdaReg_SPEC;
impl crate::sealed::RegSpec for QspicBurstcmdaReg_SPEC {
    type DataType = u32;
}
#[doc = "The way of reading in Auto mode (command register A)"]
pub type QspicBurstcmdaReg = crate::RegValueT<QspicBurstcmdaReg_SPEC>;

impl QspicBurstcmdaReg {
    #[doc = "It describes the mode of the SPI bus during the Dummy bytes phase.\n00 - Single SPI\n01 - Dual\n10 - Quad\n11 - Reserved"]
    #[inline(always)]
    pub fn qspic_dmy_tx_md(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, u8, QspicBurstcmdaReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<30,0x3,1,0,u8, QspicBurstcmdaReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "It describes the mode of the SPI bus during the Extra Byte phase.\n00 - Single SPI\n01 - Dual\n10 - Quad\n11 - Reserved"]
    #[inline(always)]
    pub fn qspic_ext_tx_md(
        self,
    ) -> crate::common::RegisterField<28, 0x3, 1, 0, u8, QspicBurstcmdaReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x3,1,0,u8, QspicBurstcmdaReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "It describes the mode of the SPI bus during the address phase.\n00 - Single SPI\n01 - Dual\n10 - Quad\n11 - Reserved"]
    #[inline(always)]
    pub fn qspic_adr_tx_md(
        self,
    ) -> crate::common::RegisterField<26, 0x3, 1, 0, u8, QspicBurstcmdaReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<26,0x3,1,0,u8, QspicBurstcmdaReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "It describes the mode of the SPI bus during the instruction phase.\n00 - Single SPI\n01 - Dual\n10 - Quad\n11 - Reserved"]
    #[inline(always)]
    pub fn qspic_inst_tx_md(
        self,
    ) -> crate::common::RegisterField<24, 0x3, 1, 0, u8, QspicBurstcmdaReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x3,1,0,u8, QspicBurstcmdaReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "The value of an extra byte which will be transferred after address (only if QSPIC_EXT_BYTE_EN= 1). Usually this is the Mode Bits in Dual/Quad SPI I/O instructions."]
    #[inline(always)]
    pub fn qspic_ext_byte(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, QspicBurstcmdaReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8, QspicBurstcmdaReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "IInstruction Value for Wrapping Burst. This value is the selected instruction when QSPIC_WRAP_MD is equal to 1 and the access is a wrapping burst of length and size described by the bit fields QSPIC_WRAP_LEN and QSPIC_WRAP_SIZE respectively."]
    #[inline(always)]
    pub fn qspic_inst_wb(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, QspicBurstcmdaReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8, QspicBurstcmdaReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Instruction Value for Incremental Burst or Single read access. This value is the selected instruction at the cases of incremental burst or single read access. Also this value is used when a wrapping burst is not supported (QSPIC_WRAP_MD)"]
    #[inline(always)]
    pub fn qspic_inst(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, QspicBurstcmdaReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, QspicBurstcmdaReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for QspicBurstcmdaReg {
    #[inline(always)]
    fn default() -> QspicBurstcmdaReg {
        <crate::RegValueT<QspicBurstcmdaReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QspicBurstcmdbReg_SPEC;
impl crate::sealed::RegSpec for QspicBurstcmdbReg_SPEC {
    type DataType = u32;
}
#[doc = "The way of reading in Auto mode (command register B)"]
pub type QspicBurstcmdbReg = crate::RegValueT<QspicBurstcmdbReg_SPEC>;

impl QspicBurstcmdbReg {
    #[doc = "By setting this bit, the number of dummy bytes is forced to be equal to 3. In this case the QSPIC_DMY_NUM field is overruled and has no function.\n0 - The number of dummy bytes is controlled by the QSPIC_DMY_NUM field\n1 - Three dummy bytes are used. The QSPIC_DMY_NUM is overruled."]
    #[inline(always)]
    pub fn qspic_dmy_force(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, QspicBurstcmdbReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,QspicBurstcmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Between the transmissions of two different instructions to the flash memory, the SPI bus stays in idle state (QSPI_CS high) for at least this number of QSPI_SCK clock cycles. See the QSPIC_ERS_CS_HI register for some exceptions."]
    #[inline(always)]
    pub fn qspic_cs_high_min(
        self,
    ) -> crate::common::RegisterField<12, 0x7, 1, 0, u8, QspicBurstcmdbReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x7,1,0,u8, QspicBurstcmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "It describes the selected data size of a wrapping burst (QSPIC_WRAP_MD).\n00 - byte access (8-bits)\n01 - half word access (16 bits)\n10 - word access (32-bits)\n11 - Reserved"]
    #[inline(always)]
    pub fn qspic_wrap_size(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, QspicBurstcmdbReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,u8, QspicBurstcmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "It describes the selected length of a wrapping burst (QSPIC_WRAP_MD).\n00 - 4 beat wrapping burst\n01 - 8 beat wrapping burst\n10 - 16 beat wrapping burst\n11 - Reserved"]
    #[inline(always)]
    pub fn qspic_wrap_len(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, QspicBurstcmdbReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8, QspicBurstcmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Wrap mode\n0 - The QSPIC_INST is the selected instruction at any access.\n1 - The QSPIC_INST_WB is the selected instruction at any wrapping burst access of length and size described by the registers QSPIC_WRAP_LEN and QSPIC_WRAP_SIZE respectively. In all other cases the QSPIC_INST is the selected instruction. Use this feature only when the serial FLASH memory supports a special instruction for wrapping burst access."]
    #[inline(always)]
    pub fn qspic_wrap_md(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, QspicBurstcmdbReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,QspicBurstcmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Instruction mode\n0 - Transmit instruction at any burst access.\n1 - Transmit instruction only in the first access after the selection of Auto Mode."]
    #[inline(always)]
    pub fn qspic_inst_md(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, QspicBurstcmdbReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,QspicBurstcmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Number of Dummy Bytes\n\n00 - Zero Dummy Bytes (Don\'t Send Dummy Bytes)\n01 - Send 1 Dummy Byte\n10 - Send 2 Dummy Bytes\n11 - Send 4 Dummy Bytes\n\nWhen QSPIC_DMY_FORCE is enabled, the QSPIC_DMY_NUM is overruled. In this case the number of dummy bytes is defined by the QSPIC_DMY_FORCE and is equal to 3, independent of the value of the QSPIC_DMY_NUM."]
    #[inline(always)]
    pub fn qspic_dmy_num(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, QspicBurstcmdbReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3,1,0,u8, QspicBurstcmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Extra Half Disable Output\n0 - if QSPIC_EXT_BYTE_EN=1, is transmitted the complete QSPIC_EXT_BYTE\n1 - if QSPIC_EXT_BYTE_EN=1, the output is disabled (hi-z) during the transmission of bits \\[3:0\\] of QSPIC_EXT_BYTE"]
    #[inline(always)]
    pub fn qspic_ext_hf_ds(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, QspicBurstcmdbReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,QspicBurstcmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Extra Byte Enable\n0 - Don\'t Send QSPIC_EXT_BYTE\n1 - Send QSPIC_EXT_BYTE"]
    #[inline(always)]
    pub fn qspic_ext_byte_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, QspicBurstcmdbReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,QspicBurstcmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "It describes the mode of the SPI bus during the data phase.\n00 - Single SPI\n01 - Dual\n10 - Quad\n11 - Reserved"]
    #[inline(always)]
    pub fn qspic_dat_rx_md(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, QspicBurstcmdbReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8, QspicBurstcmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for QspicBurstcmdbReg {
    #[inline(always)]
    fn default() -> QspicBurstcmdbReg {
        <crate::RegValueT<QspicBurstcmdbReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QspicChckeraseReg_SPEC;
impl crate::sealed::RegSpec for QspicChckeraseReg_SPEC {
    type DataType = u32;
}
#[doc = "Check erase progress in Auto mode"]
pub type QspicChckeraseReg = crate::RegValueT<QspicChckeraseReg_SPEC>;

impl QspicChckeraseReg {
    #[doc = "Writing any value to this register during erasing, forces the controller to read the flash memory status register. Depending on the value of the Busy bit, it updates the QSPIC_ERASE_EN."]
    #[inline(always)]
    pub fn qspic_chckerase(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        QspicChckeraseReg_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            QspicChckeraseReg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for QspicChckeraseReg {
    #[inline(always)]
    fn default() -> QspicChckeraseReg {
        <crate::RegValueT<QspicChckeraseReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QspicCtrlbusReg_SPEC;
impl crate::sealed::RegSpec for QspicCtrlbusReg_SPEC {
    type DataType = u32;
}
#[doc = "SPI Bus control register for the Manual mode"]
pub type QspicCtrlbusReg = crate::RegValueT<QspicCtrlbusReg_SPEC>;

impl QspicCtrlbusReg {
    #[doc = "Write 1 to disable the chip select (active low) when the controller is in Manual mode."]
    #[inline(always)]
    pub fn qspic_dis_cs(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, QspicCtrlbusReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4,1,0,QspicCtrlbusReg_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Write 1 to enable the chip select (active low) when the controller is in Manual mode."]
    #[inline(always)]
    pub fn qspic_en_cs(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, QspicCtrlbusReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3,1,0,QspicCtrlbusReg_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Write 1 to set the bus mode in Quad mode when the controller is in Manual mode."]
    #[inline(always)]
    pub fn qspic_set_quad(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, QspicCtrlbusReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2,1,0,QspicCtrlbusReg_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Write 1 to set the bus mode in Dual mode when the controller is in Manual mode."]
    #[inline(always)]
    pub fn qspic_set_dual(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, QspicCtrlbusReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1,1,0,QspicCtrlbusReg_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Write 1 to set the bus mode in Single SPI mode when the controller is in Manual mode."]
    #[inline(always)]
    pub fn qspic_set_single(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, QspicCtrlbusReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0,1,0,QspicCtrlbusReg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for QspicCtrlbusReg {
    #[inline(always)]
    fn default() -> QspicCtrlbusReg {
        <crate::RegValueT<QspicCtrlbusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QspicCtrlmodeReg_SPEC;
impl crate::sealed::RegSpec for QspicCtrlmodeReg_SPEC {
    type DataType = u32;
}
#[doc = "Mode Control register"]
pub type QspicCtrlmodeReg = crate::RegValueT<QspicCtrlmodeReg_SPEC>;

impl QspicCtrlmodeReg {
    #[doc = "Controls the length of the address that the external memory device uses.\n0 - The external memory device uses 24 bits address.\n1 - The external memory device uses 32 bits address.\nThe controller uses this bit in order to decide the number of the address bytes that has to transfer to the external device during Auto mode."]
    #[inline(always)]
    pub fn qspic_use_32ba(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, QspicCtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,QspicCtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Controls the way with which is addressed by the QSPI controller a burst request from the AMBA bus.\n\n0: The controller translates a burst access on the AMBA bus as a burst access on the QSPI bus. That results to the minimum number of command/address phases.\n1: The controller will split a burst access on the AMBA bus into a number of single accesses on the QSPI bus. That results to a separate command for each beat of the burst. E.g a 4-beat word incremental AMBA read access will be split into 4 different sequences on the QSPI bus: command/address/extra clock/read data. The QSPI_CS will be low only for the time that is needed for each of these single accesses.\n\nThis configuration bit is usefull when the clock frequency of the QSPI bus is much higher than the clock of the AMBA bus. In this case the interval for which the CS remains low is minimized, achieving lower power dissipation with respect of the case where the QSPIC_FORCENSEQ_EN=0, at cost of performance."]
    #[inline(always)]
    pub fn qspic_forcenseq_en(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, QspicCtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,QspicCtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Read pipe clock delay relative to the falling edge of QSPI_SCK.\nRefer to QSPI Timing for timing parameters and recommended values: 0 to 7"]
    #[inline(always)]
    pub fn qspic_pclk_md(
        self,
    ) -> crate::common::RegisterField<9, 0x7, 1, 0, u8, QspicCtrlmodeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x7,1,0,u8, QspicCtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Controls the use of the data read pipe.\n0 = The read pipe is disabled; the sampling clock is defined according to the QSPIC_RXD_NEG setting.\n1 = The read pipe is enabled. The delay of the sampling clock is defined according to the QSPI_PCLK_MD setting. (Recommended)"]
    #[inline(always)]
    pub fn qspic_rpipe_en(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, QspicCtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,QspicCtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Defines the clock edge that is used for the capturing of the received data, when the read pipe is not active (QSPIC_RPIPE_EN = 0).\n\n0: Sampling of the received data with the positive edge of the QSPI_SCK\n1: Sampling of the received data with the negative edge of the QSPI_SCK\n\nThe internal QSPI_SCK clock that is used by the controller for the capturing of the received data has a skew in respect of the QSPI_SCK that is received by the external memory device. In order to be improved the timing requirements of the read path, the controller supports a read pipe register with programmable clock delay. See also the QSPIC_RPIPE_EN register."]
    #[inline(always)]
    pub fn qspic_rxd_neg(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, QspicCtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,QspicCtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "This configuration bit is useful when the frequency of the QSPI clock is much lower than the clock of the AMBA bus, in order to not locks the AMBA bus for a long time.\n\n0: Adds wait states via hready signal when an access is performed on the QSPIC_WRITEDATA, QSPIC_READDATA and QSPIC_DUMMYDATA registers. It is not needed to checked the QSPIC_BUSY of the QSPIC_STATUS_REG.\n1: The controller don\'t adds wait states via the hready signal, when is performed access on the QSPIC_WRITEDATA, QSPIC_READDATA and QSPIC_DUMMYDATA registers. The QSPIC_BUSY bit of the QSPIC_STATUS_REG must be checked in order to be detected the completion of the requested access.\n\nIt is applicable only when the controller is in Manual mode. In the case of the Auto mode, the controller always adds wait states via the hready signal."]
    #[inline(always)]
    pub fn qspic_hrdy_md(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, QspicCtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,QspicCtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "The value of QSPI_IO3 pad if QSPI_IO3_OEN is 1"]
    #[inline(always)]
    pub fn qspic_io3_dat(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, QspicCtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,QspicCtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "The value of QSPI_IO2 pad if QSPI_IO2_OEN is 1"]
    #[inline(always)]
    pub fn qspic_io2_dat(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, QspicCtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,QspicCtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "QSPI_IO3 output enable. Use this only in SPI or Dual SPI mode to control /HOLD signal. When the Auto Mode is selected (QSPIC_AUTO_MD = 1) and the QUAD SPI is used, set this bit to zero.\n0: The QSPI_IO3 pad is input.\n1: The QSPI_IO3 pad is output."]
    #[inline(always)]
    pub fn qspic_io3_oen(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, QspicCtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,QspicCtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "QSPI_IO2 output enable. Use this only in SPI or Dual SPI mode to control /WP signal. When the Auto Mode is selected (QSPIC_AUTO_MD = 1) and the QUAD SPI is used, set this bit to zero.\n0: The QSPI_IO2 pad is input.\n1: The QSPI_IO2 pad is output."]
    #[inline(always)]
    pub fn qspic_io2_oen(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, QspicCtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,QspicCtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Mode of the generated QSPI_SCK clock\n0: Use Mode 0 for the QSPI_CLK. The QSPI_SCK is low when QSPI_CS is high.\n1: Use Mode 3 for the QSPI_CLK. The QSPI_SCK is high when QSPI_CS is high."]
    #[inline(always)]
    pub fn qspic_clk_md(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, QspicCtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,QspicCtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Mode of operation\n0: The Manual Mode is selected.\n1: The Auto Mode is selected.\nDuring an erasing the QSPIC_AUTO_MD goes in read only mode (see QSPIC_ERASE_EN)"]
    #[inline(always)]
    pub fn qspic_auto_md(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, QspicCtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,QspicCtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for QspicCtrlmodeReg {
    #[inline(always)]
    fn default() -> QspicCtrlmodeReg {
        <crate::RegValueT<QspicCtrlmodeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QspicDummydataReg_SPEC;
impl crate::sealed::RegSpec for QspicDummydataReg_SPEC {
    type DataType = u32;
}
#[doc = "Send dummy clocks to SPI Bus for the Manual mode"]
pub type QspicDummydataReg = crate::RegValueT<QspicDummydataReg_SPEC>;

impl QspicDummydataReg {
    #[doc = "Writing to this register generates a number of clock pulses to the SPI bus. During the last clock of this activity in the SPI bus, the QSPI_IOx data pads are in hi-z state. The data size of the access to this register can be 32-bits / 16-bits/ 8-bits. The number of generated pulses is equal to: (size of AHB bus access) / (size of SPI bus). The size of SPI bus is equal to 1, 2 or 4 for Single, Dual or Quad SPI mode respectively.\nThis register has meaning only when the controller is in Manual mode."]
    #[inline(always)]
    pub fn qspic_dummydata(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        QspicDummydataReg_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            QspicDummydataReg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for QspicDummydataReg {
    #[inline(always)]
    fn default() -> QspicDummydataReg {
        <crate::RegValueT<QspicDummydataReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QspicErasecmdaReg_SPEC;
impl crate::sealed::RegSpec for QspicErasecmdaReg_SPEC {
    type DataType = u32;
}
#[doc = "The way of erasing in Auto mode (command register A)"]
pub type QspicErasecmdaReg = crate::RegValueT<QspicErasecmdaReg_SPEC>;

impl QspicErasecmdaReg {
    #[doc = "The code value of the erase resume instruction"]
    #[inline(always)]
    pub fn qspic_res_inst(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, QspicErasecmdaReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8, QspicErasecmdaReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "The code value of the erase suspend instruction."]
    #[inline(always)]
    pub fn qspic_sus_inst(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, QspicErasecmdaReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8, QspicErasecmdaReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "The code value of the write enable instruction."]
    #[inline(always)]
    pub fn qspic_wen_inst(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, QspicErasecmdaReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8, QspicErasecmdaReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "The code value of the erase instruction."]
    #[inline(always)]
    pub fn qspic_ers_inst(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, QspicErasecmdaReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, QspicErasecmdaReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for QspicErasecmdaReg {
    #[inline(always)]
    fn default() -> QspicErasecmdaReg {
        <crate::RegValueT<QspicErasecmdaReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QspicErasecmdbReg_SPEC;
impl crate::sealed::RegSpec for QspicErasecmdbReg_SPEC {
    type DataType = u32;
}
#[doc = "The way of erasing in Auto mode (command register B)"]
pub type QspicErasecmdbReg = crate::RegValueT<QspicErasecmdbReg_SPEC>;

impl QspicErasecmdbReg {
    #[doc = "Defines a timer that counts the minimum allowed delay between an erase suspend command and the previous erase resume command (or the initial erase command).\n0 = Dont wait. The controller starts immediately to suspend the erase procedure.\n1..63 = The controller waits for at least this number of 222kHz clock cycles before the suspension of erasing. Time starts counting after the end of the previous erase resume command (or the initial erase command)"]
    #[inline(always)]
    pub fn qspic_ressus_dly(
        self,
    ) -> crate::common::RegisterField<24, 0x3f, 1, 0, u8, QspicErasecmdbReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x3f,1,0,u8, QspicErasecmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "The controller must stay without flash memory reading requests for this number of AMBA hclk clock cycles, before to perform the command of erase or erase resume\n15 - 0"]
    #[inline(always)]
    pub fn qspic_ersres_hld(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, QspicErasecmdbReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xf,1,0,u8, QspicErasecmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "After the execution of instructions: write enable, erase, erase suspend and erase resume, the QSPI_CS remains high for at least this number of qspi bus clock cycles."]
    #[inline(always)]
    pub fn qspic_ers_cs_hi(
        self,
    ) -> crate::common::RegisterField<10, 0x1f, 1, 0, u8, QspicErasecmdbReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1f,1,0,u8, QspicErasecmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "The mode of the QSPI Bus during the address phase of the erase instruction\n00 - Single\n01 - Dual\n10 - Quad\n11 - Reserved"]
    #[inline(always)]
    pub fn qspic_ead_tx_md(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, QspicErasecmdbReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8, QspicErasecmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "The mode of the QSPI Bus during the transmission of the resume instruction\n00 - Single\n01 - Dual\n10 - Quad\n11 - Reserved"]
    #[inline(always)]
    pub fn qspic_res_tx_md(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, QspicErasecmdbReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,u8, QspicErasecmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "The mode of the QSPI Bus during the transmission of the suspend instruction.\n00 - Single\n01 - Dual\n10 - Quad\n11 - Reserved"]
    #[inline(always)]
    pub fn qspic_sus_tx_md(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, QspicErasecmdbReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3,1,0,u8, QspicErasecmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "The mode of the QSPI Bus during the transmission of the write enable instruction.\n00 - Single\n01 - Dual\n10 - Quad\n11 - Reserved"]
    #[inline(always)]
    pub fn qspic_wen_tx_md(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, QspicErasecmdbReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3,1,0,u8, QspicErasecmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "The mode of the QSPI Bus during the instruction phase of the erase instruction\n00 - Single\n01 - Dual\n10 - Quad\n11 - Reserved"]
    #[inline(always)]
    pub fn qspic_ers_tx_md(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, QspicErasecmdbReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8, QspicErasecmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for QspicErasecmdbReg {
    #[inline(always)]
    fn default() -> QspicErasecmdbReg {
        <crate::RegValueT<QspicErasecmdbReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QspicErasectrlReg_SPEC;
impl crate::sealed::RegSpec for QspicErasectrlReg_SPEC {
    type DataType = u32;
}
#[doc = "QSPI Erase control register"]
pub type QspicErasectrlReg = crate::RegValueT<QspicErasectrlReg_SPEC>;

impl QspicErasectrlReg {
    #[doc = "It shows the progress of sector/block erasing (read only).\n000 = No Erase.\n001 = Pending erase request\n010 = Erase procedure is running\n011 = Suspended Erase procedure\n100 = Finishing the Erase procedure\n101..111 = Reserved"]
    #[inline(always)]
    pub fn qspic_ers_state(
        self,
    ) -> crate::common::RegisterField<25, 0x7, 1, 0, u8, QspicErasectrlReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<25,0x7,1,0,u8, QspicErasectrlReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "During Manual mode (QSPIC_AUTO_MD = 0). This bit is in read only mode.\nDuring Auto mode (QSPIC_AUTO_MD = 1). To request the erasing of the block/sector (QSPIC_ERS_ADDR, 12\'b0) write 1 to this bit. This bit is cleared automatically with the end of the erasing. Until the end of erasing the QSPIC_ERASE_EN remains in read only mode. During the same period of time the controller remains in Auto Mode (QSPIC_AUTO_MD goes in read only mode)."]
    #[inline(always)]
    pub fn qspic_erase_en(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, QspicErasectrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24,1,0,QspicErasectrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Defines the address of the block/sector that is requested to be erased.\nIf QSPIC_USE_32BA = 0 (24 bits addressing), bits QSPIC_ERASECTRL_REG\\[23-12\\] determine the block/ sector address bits \\[23-12\\].\nQSPIC_ERASECTRL_REG\\[11-4\\] are ignored by the controller.\nIf QSPIC_USE_32BA = 1 (32 bits addressing) bits QSPIC_ERASECTRL_REG\\[23-4\\] determine the block / sectors address bits \\[31:12\\]"]
    #[inline(always)]
    pub fn qspic_ers_addr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0xfffff,
        1,
        0,
        u32,
        QspicErasectrlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0xfffff,
            1,
            0,
            u32,
            QspicErasectrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for QspicErasectrlReg {
    #[inline(always)]
    fn default() -> QspicErasectrlReg {
        <crate::RegValueT<QspicErasectrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QspicGpReg_SPEC;
impl crate::sealed::RegSpec for QspicGpReg_SPEC {
    type DataType = u32;
}
#[doc = "QSPI General Purpose control register"]
pub type QspicGpReg = crate::RegValueT<QspicGpReg_SPEC>;

impl QspicGpReg {
    #[doc = "QSPI pads slew rate control. Indicative values under certain conditions:\n0: Rise=1.7 V/ns, Fall=1.9 V/ns (weak)\n1: Rise=2.0 V/ns, Fall=2.3 V/ns\n2: Rise=2.3 V/ns, Fall=2.6 V/ns\n3: Rise=2.4 V/ns, Fall=2.7 V/ns (strong)\nConditions: FLASH pin capacitance 6 pF, Vcc=1.8V, T=25C and Idrive=16mA."]
    #[inline(always)]
    pub fn qspic_pads_slew(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, QspicGpReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x3,1,0,u8, QspicGpReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "QSPI pads drive current\n0: 4 mA\n1: 8 mA\n2: 12 mA\n3: 16 mA"]
    #[inline(always)]
    pub fn qspic_pads_drv(
        self,
    ) -> crate::common::RegisterField<1, 0x3, 1, 0, u8, QspicGpReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x3,1,0,u8, QspicGpReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for QspicGpReg {
    #[inline(always)]
    fn default() -> QspicGpReg {
        <crate::RegValueT<QspicGpReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QspicReaddataReg_SPEC;
impl crate::sealed::RegSpec for QspicReaddataReg_SPEC {
    type DataType = u32;
}
#[doc = "Read data from SPI Bus for the Manual mode"]
pub type QspicReaddataReg = crate::RegValueT<QspicReaddataReg_SPEC>;

impl QspicReaddataReg {
    #[doc = "A read access at this register generates a data transfer from the external memory device to the QSPIC controller. The data is transferred using the selected mode of the SPI bus (SPI, Dual SPI, Quad SPI). The data size of the access to this register can be 32-bits / 16-bits / 8-bits and is equal to the number of the transferred bits.\nThis register has meaning only when the controller is in Manual mode."]
    #[inline(always)]
    pub fn qspic_readdata(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        QspicReaddataReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            QspicReaddataReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for QspicReaddataReg {
    #[inline(always)]
    fn default() -> QspicReaddataReg {
        <crate::RegValueT<QspicReaddataReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QspicRecvdataReg_SPEC;
impl crate::sealed::RegSpec for QspicRecvdataReg_SPEC {
    type DataType = u32;
}
#[doc = "Received data for the Manual mode"]
pub type QspicRecvdataReg = crate::RegValueT<QspicRecvdataReg_SPEC>;

impl QspicRecvdataReg {
    #[doc = "This register contains the received data when the QSPIC_READDATA_REG register is used in Manual mode, in order to be retrieved data from the external memory device and QSPIC_HRDY_MD=1 && QSPIC_BUSY=0."]
    #[inline(always)]
    pub fn qspic_recvdata(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        QspicRecvdataReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            QspicRecvdataReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for QspicRecvdataReg {
    #[inline(always)]
    fn default() -> QspicRecvdataReg {
        <crate::RegValueT<QspicRecvdataReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QspicStatuscmdReg_SPEC;
impl crate::sealed::RegSpec for QspicStatuscmdReg_SPEC {
    type DataType = u32;
}
#[doc = "The way of reading the status of external device in Auto mode"]
pub type QspicStatuscmdReg = crate::RegValueT<QspicStatuscmdReg_SPEC>;

impl QspicStatuscmdReg {
    #[doc = "Defines the timer which is used to count the delay that it has to wait before to read the FLASH Status Register, after an erase or an erase resume command.\n0 - The delay is controlled by the QSPIC_RESSTS_DLY which counts on the qspi clock.\n1 - The delay is controlled by the QSPIC_RESSUS_DLY which counts on the 222 kHz clock."]
    #[inline(always)]
    pub fn qspic_stsdly_sel(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, QspicStatuscmdReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22,1,0,QspicStatuscmdReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Defines a timer that counts the minimum required delay between the reading of the status register and of the previous erase or erase resume instruction.\n0 - Dont wait. The controller starts to reading the Flash memory status register immediately.\n1..63 - The controller waits for at least this number of QSPI_CLK cycles and afterwards it starts to reading the Flash memory status register. The timer starts to count after the end of the previous erase or erase resume command.\n\nThe actual timer that will be used by the controller before the reading of the Flash memory status register is defined by the QSPIC_STSDLY_SEL."]
    #[inline(always)]
    pub fn qspic_ressts_dly(
        self,
    ) -> crate::common::RegisterField<16, 0x3f, 1, 0, u8, QspicStatuscmdReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3f,1,0,u8, QspicStatuscmdReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Defines the value of the Busy bit which means that the flash is busy.\n0 - The flash is busy when the Busy bit is equal to 0.\n1 - The flash is busy when the Busy bit is equal to 1."]
    #[inline(always)]
    pub fn qspic_busy_val(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, QspicStatuscmdReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,QspicStatuscmdReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "It describes who from the bits of status represents the\nBusy bit (7 - 0)."]
    #[inline(always)]
    pub fn qspic_busy_pos(
        self,
    ) -> crate::common::RegisterField<12, 0x7, 1, 0, u8, QspicStatuscmdReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x7,1,0,u8, QspicStatuscmdReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "The mode of the QSPI Bus during the receive status phase of the read status instruction\n00 - Single\n01 - Dual\n10 - Quad\n11 - Reserved"]
    #[inline(always)]
    pub fn qspic_rstat_rx_md(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, QspicStatuscmdReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,u8, QspicStatuscmdReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "The mode of the QSPI Bus during the instruction phase of the read status instruction.\n00 - Single\n01 - Dual\n10 - Quad\n11 - Reserved"]
    #[inline(always)]
    pub fn qspic_rstat_tx_md(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, QspicStatuscmdReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8, QspicStatuscmdReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "The code value of the read status instruction.\nIt is transmitted during the instruction phase of the read status instruction."]
    #[inline(always)]
    pub fn qspic_rstat_inst(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, QspicStatuscmdReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, QspicStatuscmdReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for QspicStatuscmdReg {
    #[inline(always)]
    fn default() -> QspicStatuscmdReg {
        <crate::RegValueT<QspicStatuscmdReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QspicStatusReg_SPEC;
impl crate::sealed::RegSpec for QspicStatusReg_SPEC {
    type DataType = u32;
}
#[doc = "The status register of the QSPI controller"]
pub type QspicStatusReg = crate::RegValueT<QspicStatusReg_SPEC>;

impl QspicStatusReg {
    #[doc = "The status of the SPI Bus.\n\n0 - The SPI Bus is idle\n1 - The SPI Bus is active. Read data, write data or dummy data activity is in progress.\n\nHas meaning only in Manual mode and only when QSPIC_HRDY_MD = 1."]
    #[inline(always)]
    pub fn qspic_busy(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, QspicStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,QspicStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for QspicStatusReg {
    #[inline(always)]
    fn default() -> QspicStatusReg {
        <crate::RegValueT<QspicStatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QspicUcodeStart_SPEC;
impl crate::sealed::RegSpec for QspicUcodeStart_SPEC {
    type DataType = u32;
}
#[doc = "QSPIC uCode memory"]
pub type QspicUcodeStart = crate::RegValueT<QspicUcodeStart_SPEC>;

impl QspicUcodeStart {
    #[doc = "The first position of the memory (16 words x 32 bits) where a microcode should be placed ( X = 0 to 15). This microcode describes the initialization process of the external flash device."]
    #[inline(always)]
    pub fn qspic_ucode_x(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        QspicUcodeStart_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            QspicUcodeStart_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for QspicUcodeStart {
    #[inline(always)]
    fn default() -> QspicUcodeStart {
        <crate::RegValueT<QspicUcodeStart_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QspicWritedataReg_SPEC;
impl crate::sealed::RegSpec for QspicWritedataReg_SPEC {
    type DataType = u32;
}
#[doc = "Write data to SPI Bus for the Manual mode"]
pub type QspicWritedataReg = crate::RegValueT<QspicWritedataReg_SPEC>;

impl QspicWritedataReg {
    #[doc = "Writing to this register is generating a data transfer from the controller to the external memory device. The data written in this register, is then transferred to the memory using the selected mode of the SPI bus (SPI, Dual SPI, Quad SPI). The data size of the access to this register can be 32-bits / 16-bits/ 8-bits and is equal to the number of the transferred bits.\nThis register has meaning only when the controller is in Manual mode."]
    #[inline(always)]
    pub fn qspic_writedata(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        QspicWritedataReg_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            QspicWritedataReg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for QspicWritedataReg {
    #[inline(always)]
    fn default() -> QspicWritedataReg {
        <crate::RegValueT<QspicWritedataReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
