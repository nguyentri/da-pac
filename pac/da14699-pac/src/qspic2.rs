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
// Generated from SVD 1.2, with svd2pac 0.4.0 on Sat, 12 Apr 2025 22:54:18 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"QSPIC2 registers"]
unsafe impl ::core::marker::Send for super::Qspic2 {}
unsafe impl ::core::marker::Sync for super::Qspic2 {}
impl super::Qspic2 {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }
    #[doc = "The way of writing in Auto mode when the external device is a serial SRAM"]
    #[inline(always)]
    pub const fn qspic2_awritecmd_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Qspic2AwritecmdReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Qspic2AwritecmdReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[doc = "Read break sequence in Auto mode"]
    #[inline(always)]
    pub const fn qspic2_burstbrk_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Qspic2BurstbrkReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Qspic2BurstbrkReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[doc = "The way of reading in Auto mode (command register A)"]
    #[inline(always)]
    pub const fn qspic2_burstcmda_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Qspic2BurstcmdaReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Qspic2BurstcmdaReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "The way of reading in Auto mode (command register B)"]
    #[inline(always)]
    pub const fn qspic2_burstcmdb_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Qspic2BurstcmdbReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Qspic2BurstcmdbReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[doc = "Check erase progress in Auto mode"]
    #[inline(always)]
    pub const fn qspic2_chckerase_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Qspic2ChckeraseReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Qspic2ChckeraseReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[doc = "SPI Bus control register for the Manual mode"]
    #[inline(always)]
    pub const fn qspic2_ctrlbus_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Qspic2CtrlbusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Qspic2CtrlbusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Mode control register"]
    #[inline(always)]
    pub const fn qspic2_ctrlmode_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Qspic2CtrlmodeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Qspic2CtrlmodeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Send dummy clocks to SPI Bus for the Manual mode"]
    #[inline(always)]
    pub const fn qspic2_dummydata_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Qspic2DummydataReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Qspic2DummydataReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[doc = "The way of erasing in Auto mode (command register A)"]
    #[inline(always)]
    pub const fn qspic2_erasecmda_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Qspic2ErasecmdaReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Qspic2ErasecmdaReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(40usize),
            )
        }
    }

    #[doc = "The way of erasing in Auto mode (command register B)"]
    #[inline(always)]
    pub const fn qspic2_erasecmdb_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Qspic2ErasecmdbReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Qspic2ErasecmdbReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(44usize),
            )
        }
    }

    #[doc = "Erase control register"]
    #[inline(always)]
    pub const fn qspic2_erasectrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Qspic2ErasectrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Qspic2ErasectrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[doc = "General purpose QSPIC2 register"]
    #[inline(always)]
    pub const fn qspic2_gp_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Qspic2GpReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Qspic2GpReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }

    #[doc = "External memory burst length configuration"]
    #[inline(always)]
    pub const fn qspic2_memblen_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Qspic2MemblenReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Qspic2MemblenReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(68usize),
            )
        }
    }

    #[doc = "Read data from SPI Bus for the Manual mode"]
    #[inline(always)]
    pub const fn qspic2_readdata_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Qspic2ReaddataReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Qspic2ReaddataReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[doc = "Received data for the Manual mode"]
    #[inline(always)]
    pub const fn qspic2_recvdata_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Qspic2RecvdataReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Qspic2RecvdataReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "The way of reading the status of external device in Auto mode"]
    #[inline(always)]
    pub const fn qspic2_statuscmd_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Qspic2StatuscmdReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Qspic2StatuscmdReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[doc = "The status register of the QSPI controller"]
    #[inline(always)]
    pub const fn qspic2_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Qspic2StatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Qspic2StatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[doc = "Write data to SPI Bus for the Manual mode"]
    #[inline(always)]
    pub const fn qspic2_writedata_reg(
        &self,
    ) -> &'static crate::common::Reg<self::Qspic2WritedataReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::Qspic2WritedataReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qspic2AwritecmdReg_SPEC;
impl crate::sealed::RegSpec for Qspic2AwritecmdReg_SPEC {
    type DataType = u32;
}
#[doc = "The way of writing in Auto mode when the external device is a serial SRAM"]
pub type Qspic2AwritecmdReg = crate::RegValueT<Qspic2AwritecmdReg_SPEC>;

impl Qspic2AwritecmdReg {
    #[doc = "After the execution of the write command, the QSPI_CS remains high for at least this number of QSPI_SCK clock cycles."]
    #[inline(always)]
    pub fn qspic_wr_cs_high_min(
        self,
    ) -> crate::common::RegisterField<14, 0x1f, 1, 0, u8, Qspic2AwritecmdReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1f,1,0,u8, Qspic2AwritecmdReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "The mode of the SPI Bus during the data phase of the write command.\n0x0: Single SPI\n0x1: Dual\n0x2: Quad\n0x3: Reserved"]
    #[inline(always)]
    pub fn qspic_wr_dat_tx_md(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, Qspic2AwritecmdReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,u8, Qspic2AwritecmdReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "The mode of the SPI Bus during the adress phase of the write command.\n0x0: Single SPI\n0x1: Dual\n0x2: Quad\n0x3: Reserved"]
    #[inline(always)]
    pub fn qspic_wr_adr_tx_md(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, Qspic2AwritecmdReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,u8, Qspic2AwritecmdReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "The mode of the SPI Bus during the instruction phase of the write command.\n0x0: Single SPI\n0x1: Dual\n0x2: Quad\n0x3: Reserved"]
    #[inline(always)]
    pub fn qspic_wr_inst_tx_md(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, Qspic2AwritecmdReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8, Qspic2AwritecmdReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "This is the value of the instruction that is used, in order to be programmed the external SRAM device."]
    #[inline(always)]
    pub fn qspic_wr_inst(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Qspic2AwritecmdReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, Qspic2AwritecmdReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Qspic2AwritecmdReg {
    #[inline(always)]
    fn default() -> Qspic2AwritecmdReg {
        <crate::RegValueT<Qspic2AwritecmdReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qspic2BurstbrkReg_SPEC;
impl crate::sealed::RegSpec for Qspic2BurstbrkReg_SPEC {
    type DataType = u32;
}
#[doc = "Read break sequence in Auto mode"]
pub type Qspic2BurstbrkReg = crate::RegValueT<Qspic2BurstbrkReg_SPEC>;

impl Qspic2BurstbrkReg {
    #[doc = "Disable output during the transmission of the second half (QSPIC_BRK_WRD\\[3:0\\]). Setting this bit is only useful if QSPIC_BRK_EN =1 and QSPIC_BRK_SZ= 1.\n0: The controller drives the SPI bus during the transmission of the QSPIC_BRK_WRD\\[3:0\\].\n1: The controller leaves the SPI bus in Hi-Z during the transmission of the QSPIC_BRK_WORD\\[3:0\\]."]
    #[inline(always)]
    pub fn qspic_sec_hf_ds(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Qspic2BurstbrkReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20,1,0,Qspic2BurstbrkReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "The mode of the SPI Bus during the transmission of the read break sequence.\n0x0: Single SPI\n0x1: Dual\n0x2: Quad\n0x3: Reserved"]
    #[inline(always)]
    pub fn qspic_brk_tx_md(
        self,
    ) -> crate::common::RegisterField<18, 0x3, 1, 0, u8, Qspic2BurstbrkReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x3,1,0,u8, Qspic2BurstbrkReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "The size of the read break sequence.\n0: One byte (Send QSPIC_BRK_WRD\\[15:8\\])\n1: Two bytes (Send QSPIC_BRK_WRD\\[15:0\\])"]
    #[inline(always)]
    pub fn qspic_brk_sz(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Qspic2BurstbrkReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17,1,0,Qspic2BurstbrkReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Controls the application of a special command (read break sequence) that is used in order to force the device to abandon the continuous read mode.\n0: The special command is not applied\n1: The special command is applied\n\nThis special command is applied by the controller to the external device under the following conditions:\n- the controller is in Auto mode\n- the QSPIC_INST_MD = 1\n- the previous command that has been applied in the external device was read\n- the controller want to apply to the external device a command different than the read."]
    #[inline(always)]
    pub fn qspic_brk_en(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Qspic2BurstbrkReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,Qspic2BurstbrkReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "This is the value of a special command (read break sequence) that is applied by the controller to the external memory device, in order to force the memory device to abandon the continuous read mode."]
    #[inline(always)]
    pub fn qspic_brk_wrd(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Qspic2BurstbrkReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            Qspic2BurstbrkReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Qspic2BurstbrkReg {
    #[inline(always)]
    fn default() -> Qspic2BurstbrkReg {
        <crate::RegValueT<Qspic2BurstbrkReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qspic2BurstcmdaReg_SPEC;
impl crate::sealed::RegSpec for Qspic2BurstcmdaReg_SPEC {
    type DataType = u32;
}
#[doc = "The way of reading in Auto mode (command register A)"]
pub type Qspic2BurstcmdaReg = crate::RegValueT<Qspic2BurstcmdaReg_SPEC>;

impl Qspic2BurstcmdaReg {
    #[doc = "It describes the mode of the SPI bus during the Dummy bytes phase.\n0x0: Single SPI\n0x1: Dual\n0x2: Quad\n0x3: Reserved"]
    #[inline(always)]
    pub fn qspic_dmy_tx_md(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, u8, Qspic2BurstcmdaReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<30,0x3,1,0,u8, Qspic2BurstcmdaReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "It describes the mode of the SPI bus during the Extra Byte phase.\n0x0: Single SPI\n0x1: Dual\n0x2: Quad\n0x3: Reserved"]
    #[inline(always)]
    pub fn qspic_ext_tx_md(
        self,
    ) -> crate::common::RegisterField<28, 0x3, 1, 0, u8, Qspic2BurstcmdaReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x3,1,0,u8, Qspic2BurstcmdaReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "It describes the mode of the SPI bus during the address phase.\n0x0: Single SPI\n0x1: Dual\n0x2: Quad\n0x3: Reserved"]
    #[inline(always)]
    pub fn qspic_adr_tx_md(
        self,
    ) -> crate::common::RegisterField<26, 0x3, 1, 0, u8, Qspic2BurstcmdaReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<26,0x3,1,0,u8, Qspic2BurstcmdaReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "It describes the mode of the SPI bus during the instruction phase.\n0x0: Single SPI\n0x1: Dual\n0x2: Quad\n0x3: Reserved"]
    #[inline(always)]
    pub fn qspic_inst_tx_md(
        self,
    ) -> crate::common::RegisterField<24, 0x3, 1, 0, u8, Qspic2BurstcmdaReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x3,1,0,u8, Qspic2BurstcmdaReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "The value of an extra byte which will be transferred after address (only if QSPIC_EXT_BYTE_EN= 1). Usually this is the Mode Bits in Dual/Quad SPI I/O instructions."]
    #[inline(always)]
    pub fn qspic_ext_byte(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Qspic2BurstcmdaReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8, Qspic2BurstcmdaReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Instruction Value for Wrapping Burst. This value is the selected instruction when QSPIC_WRAP_MD is equal to 1 and the access is a wrapping burst of length and size described by the bit fields QSPIC_WRAP_LEN and QSPIC_WRAP_SIZE respectively."]
    #[inline(always)]
    pub fn qspic_inst_wb(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Qspic2BurstcmdaReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8, Qspic2BurstcmdaReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Instruction Value for Incremental Burst or Single read access. This value is the selected instruction at the cases of incremental burst or single read access. Also this value is used when a wrapping burst is not supported (QSPIC_WRAP_MD)"]
    #[inline(always)]
    pub fn qspic_inst(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Qspic2BurstcmdaReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, Qspic2BurstcmdaReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Qspic2BurstcmdaReg {
    #[inline(always)]
    fn default() -> Qspic2BurstcmdaReg {
        <crate::RegValueT<Qspic2BurstcmdaReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qspic2BurstcmdbReg_SPEC;
impl crate::sealed::RegSpec for Qspic2BurstcmdbReg_SPEC {
    type DataType = u32;
}
#[doc = "The way of reading in Auto mode (command register B)"]
pub type Qspic2BurstcmdbReg = crate::RegValueT<Qspic2BurstcmdbReg_SPEC>;

impl Qspic2BurstcmdbReg {
    #[doc = "By setting this bit, the number of dummy bytes is forced to be equal to 3. In this case the QSPIC_DMY_NUM field is overruled and has no function.\n0: The number of dummy bytes is controlled by the QSPIC_DMY_NUM field\n1: Three dummy bytes are used. The QSPIC_DMY_NUM is overruled."]
    #[inline(always)]
    pub fn qspic_dmy_force(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Qspic2BurstcmdbReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<15,1,0,Qspic2BurstcmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Between the transmission of two different instructions to the flash memory, the qspi bus stays in idle state (QSPI_CS high) for at least this number of QSPI_SCK clock cycles. See the QSPIC_ERS_CS_HI and the QSPIC_WR_CS_HIGH_MIN registers for some exceptions."]
    #[inline(always)]
    pub fn qspic_cs_high_min(
        self,
    ) -> crate::common::RegisterField<12, 0x7, 1, 0, u8, Qspic2BurstcmdbReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x7,1,0,u8, Qspic2BurstcmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "It describes the selected data size of a wrapping burst (QSPIC_WRAP_MD).\n0x0: Byte access (8-bits)\n0x1: Half word access (16 bits)\n0x2: Word access (32-bits)\n0x3: Reserved"]
    #[inline(always)]
    pub fn qspic_wrap_size(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, Qspic2BurstcmdbReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,u8, Qspic2BurstcmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "It describes the selected length of a wrapping burst (QSPIC_WRAP_MD).\n0x0: 4 beat wrapping burst\n0x1: 8 beat wrapping burst\n0x2: 16 beat wrapping burst\n0x3: Reserved"]
    #[inline(always)]
    pub fn qspic_wrap_len(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, Qspic2BurstcmdbReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8, Qspic2BurstcmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Wrap mode\n0: The QSPIC_INST is the selected instruction at any access.\n1: The QSPIC_INST_WB is the selected instruction at any wrapping burst access of length and size described by the registers QSPIC_WRAP_LEN and QSPIC_WRAP_SIZE respectively. In all other cases the QSPIC_INST is the selected instruction. Use this feature only when the serial FLASH memory supports a special instruction for wrapping burst access."]
    #[inline(always)]
    pub fn qspic_wrap_md(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Qspic2BurstcmdbReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,Qspic2BurstcmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Instruction mode\n0: Transmit instruction at any burst access.\n1: Transmit instruction only in the first access after the selection of Auto Mode."]
    #[inline(always)]
    pub fn qspic_inst_md(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Qspic2BurstcmdbReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,Qspic2BurstcmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Number of Dummy Bytes\n\n0x0: Zero Dummy Bytes (Don\'t Send Dummy Bytes)\n0x1: Send 1 Dummy Byte\n0x2: Send 2 Dummy Bytes\n0x3: Send 4 Dummy Bytes\n\nWhen QSPIC_DMY_FORCE is enabled, the QSPIC_DMY_NUM is overruled. In this case the number of dummy bytes is defined by the QSPIC_DMY_FORCE and is equal to 3, independent of the value of the QSPIC_DMY_NUM."]
    #[inline(always)]
    pub fn qspic_dmy_num(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, Qspic2BurstcmdbReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3,1,0,u8, Qspic2BurstcmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Extra Half Disable Output\n0: if QSPIC_EXT_BYTE_EN=1 then transmit the complete QSPIC_EXT_BYTE\n1: if QSPIC_EXT_BYTE_EN=1 then disable (hi-z) output during the transmission of bits \\[3:0\\] of QSPIC_EXT_BYTE"]
    #[inline(always)]
    pub fn qspic_ext_hf_ds(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Qspic2BurstcmdbReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,Qspic2BurstcmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Extra Byte Enable\n0: Don\'t Send QSPIC_EXT_BYTE\n1: Send QSPIC_EXT_BYTE"]
    #[inline(always)]
    pub fn qspic_ext_byte_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Qspic2BurstcmdbReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,Qspic2BurstcmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "It describes the mode of the SPI bus during the data phase.\n0x0: Single SPI\n0x1: Dual\n0x2: Quad\n0x3: Reserved"]
    #[inline(always)]
    pub fn qspic_dat_rx_md(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Qspic2BurstcmdbReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8, Qspic2BurstcmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Qspic2BurstcmdbReg {
    #[inline(always)]
    fn default() -> Qspic2BurstcmdbReg {
        <crate::RegValueT<Qspic2BurstcmdbReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qspic2ChckeraseReg_SPEC;
impl crate::sealed::RegSpec for Qspic2ChckeraseReg_SPEC {
    type DataType = u32;
}
#[doc = "Check erase progress in Auto mode"]
pub type Qspic2ChckeraseReg = crate::RegValueT<Qspic2ChckeraseReg_SPEC>;

impl Qspic2ChckeraseReg {
    #[doc = "Writing any value to this register during erasing, forces the controller to read the flash memory status register. Depending on the value of the Busy bit, it updates the QSPIC_ERASE_EN.\nThis register has meaning only when the controller is in Auto mode and there is an erase in progress (QSPIC_ERASE_EN =1). It has no meaning when the external device is a serial SRAM."]
    #[inline(always)]
    pub fn qspic_chckerase(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        Qspic2ChckeraseReg_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            Qspic2ChckeraseReg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Qspic2ChckeraseReg {
    #[inline(always)]
    fn default() -> Qspic2ChckeraseReg {
        <crate::RegValueT<Qspic2ChckeraseReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qspic2CtrlbusReg_SPEC;
impl crate::sealed::RegSpec for Qspic2CtrlbusReg_SPEC {
    type DataType = u32;
}
#[doc = "SPI Bus control register for the Manual mode"]
pub type Qspic2CtrlbusReg = crate::RegValueT<Qspic2CtrlbusReg_SPEC>;

impl Qspic2CtrlbusReg {
    #[doc = "Write 1 to disable the chip select (active low) when the controller is in Manual mode."]
    #[inline(always)]
    pub fn qspic_dis_cs(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Qspic2CtrlbusReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4,1,0,Qspic2CtrlbusReg_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Write 1 to enable the chip select (active low) when the controller is in Manual mode."]
    #[inline(always)]
    pub fn qspic_en_cs(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Qspic2CtrlbusReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3,1,0,Qspic2CtrlbusReg_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Write 1 to set the bus mode in Quad mode when the controller is in Manual mode."]
    #[inline(always)]
    pub fn qspic_set_quad(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Qspic2CtrlbusReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2,1,0,Qspic2CtrlbusReg_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Write 1 to set the bus mode in Dual mode when the controller is in Manual mode."]
    #[inline(always)]
    pub fn qspic_set_dual(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Qspic2CtrlbusReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1,1,0,Qspic2CtrlbusReg_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Write 1 to set the bus mode in Single SPI mode when the controller is in Manual mode."]
    #[inline(always)]
    pub fn qspic_set_single(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Qspic2CtrlbusReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0,1,0,Qspic2CtrlbusReg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Qspic2CtrlbusReg {
    #[inline(always)]
    fn default() -> Qspic2CtrlbusReg {
        <crate::RegValueT<Qspic2CtrlbusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qspic2CtrlmodeReg_SPEC;
impl crate::sealed::RegSpec for Qspic2CtrlmodeReg_SPEC {
    type DataType = u32;
}
#[doc = "Mode control register"]
pub type Qspic2CtrlmodeReg = crate::RegValueT<Qspic2CtrlmodeReg_SPEC>;

impl Qspic2CtrlmodeReg {
    #[doc = "Controls the behavior of the QSPI_SCK when the QSPI_CS is high and the QSPIC_CS_MD=1.\n0: Is produced one QSPI_SCK clock pulse after each 0 to 1 transition in the QSPI_CS.\n1: The QSPI_SCK clock remains always active, while the QSPI_CS is inactive.\n\nThis setting has meaning only when the QSPIC_CS_MD=1."]
    #[inline(always)]
    pub fn qspic_clk_free_en(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Qspic2CtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,Qspic2CtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Controls the clock edge with which is produced the QSPI_CS signal.\n0: The QSPI_CS is produced with the rising edge of the QSPI_SCK. The QSPI_SCK is always inactive while the QSPI_CS is high.\n1: The QSPI_CS is produced with the falling edge of the QSPI_SCK. The behavior of the QSPI_SCK while the QSPI_CS is high, is controlled by the QSPIC_CLK_FREE_EN."]
    #[inline(always)]
    pub fn qspic_cs_md(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Qspic2CtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,Qspic2CtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Defines the type of the external device that is connected on the QSPIC controller\n0: The external memory device is a serial Flash\n1: The external memory device is a serial SRAM\n\nWhen the external device is a serial SRAM, the erase suspend/ resume functionality of the controller is disabled. In this case the writing of the QSPIC_ERASECTRL_REG\\[QSPIC_ERASE_EN\\] bit has no effect. Also, the memory space where the external device is mapped, is considered as writable."]
    #[inline(always)]
    pub fn qspic_sram_en(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Qspic2CtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,Qspic2CtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Controls the length of the address that the external memory device uses.\n0: The external memory device uses 24 bits address.\n1: The external memory device uses 32 bits address.\nThe controller uses this bit in order to decide the number of the address bytes that has to transfer to the external device during Auto mode."]
    #[inline(always)]
    pub fn qspic_use_32ba(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Qspic2CtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,Qspic2CtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Controls the way with which is addressed by the QSPI controller a burst request from the AMBA bus.\n\n0: The controller translates a burst access on the AMBA bus as a burst access on the QSPI bus. That results to the minimum number of command/address phases.\n1: The controller will split a burst access on the AMBA bus into a number of single accesses on the QSPI bus. That results to a separate command for each beat of the burst. E.g a 4-beat word incremental AMBA read access will be split into 4 different sequences on the QSPI bus: command/address/extra clock/read data. The QSPI_CS will be low only for the time that is needed for each of these single access.\n\nThis configuration bit is usefull when the clock frequency of the QSPI bus is much higher than the clock of the AMBA bus. In this case the interval for which the CS remains low is minimized, achieving lower power dissipation with respect of the case where the QSPIC_FORCENSEQ_EN=0, at cost of performance."]
    #[inline(always)]
    pub fn qspic_forcenseq_en(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Qspic2CtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,Qspic2CtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Controls the read pipe clock delay relative to the falling edge of QSPI_SCK. Refer to QSPI Timing for timing parameters"]
    #[inline(always)]
    pub fn qspic_pclk_md(
        self,
    ) -> crate::common::RegisterField<9, 0x7, 1, 0, u8, Qspic2CtrlmodeReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x7,1,0,u8, Qspic2CtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Controls the use of the data read pipe.\n0: The read pipe is disabled, the sampling clock is defined according to the QSPIC_RXD_NEG setting.\n1: The read pipe is enabled. The delay of the sampling clock is defined according to the QSPI_PCLK_MD setting. (Recommended)"]
    #[inline(always)]
    pub fn qspic_rpipe_en(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Qspic2CtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,Qspic2CtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Defines the clock edge that is used for the capturing of the received data, when the read pipe is not active (QSPIC_RPIPE_EN = 0).\n\n0: Sampling of the received data with the positive edge of the QSPI_SCK\n1: Sampling of the received data with the negative edge of the QSPI_SCK\n\nThe internal QSPI_SCK clock that is used by the controller for the capturing of the received data has a skew in respect of the QSPI_SCK that is received by the external memory device. In order to be improved the timing requirements of the read path, the controller supports a read pipe register with programmable clock delay. See also the QSPIC_RPIPE_EN register."]
    #[inline(always)]
    pub fn qspic_rxd_neg(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Qspic2CtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,Qspic2CtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "This configuration bit is useful when the frequency of the QSPI clock is much lower than the clock of the AMBA bus, in order to not locks the AMBA bus for a long time.\n\n0: Adds wait states via hready signal when an access is performed on the QSPIC_WRITEDATA, QSPIC_READDATA and QSPIC_DUMMYDATA registers. It is not needed to checked the QSPIC_BUSY of the QSPIC_STATUS_REG.\n1: The controller don\'t adds wait states via the hready signal, when is performed access on the QSPIC_WRITEDATA, QSPIC_READDATA and QSPIC_DUMMYDATA registers. The QSPIC_BUSY bit of the QSPIC_STATUS_REG must be checked in order to be detected the completion of the requested access.\n\nIt is applicable only when the controller is in Manual mode. In the case of the Auto mode, the controller always adds wait states via the hready signal."]
    #[inline(always)]
    pub fn qspic_hrdy_md(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Qspic2CtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,Qspic2CtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "The value of QSPI_IO3 pad if QSPI_IO3_OEN is 1"]
    #[inline(always)]
    pub fn qspic_io3_dat(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Qspic2CtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,Qspic2CtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "The value of QSPI_IO2 pad if QSPI_IO2_OEN is 1"]
    #[inline(always)]
    pub fn qspic_io2_dat(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Qspic2CtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,Qspic2CtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "QSPI_IO3 output enable. Use this only in SPI or Dual SPI mode to control /HOLD signal. When the Auto Mode is selected (QSPIC_AUTO_MD = 1) and the QUAD SPI is used, set this bit to zero.\n0: The QSPI_IO3 pad is input.\n1: The QSPI_IO3 pad is output."]
    #[inline(always)]
    pub fn qspic_io3_oen(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Qspic2CtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,Qspic2CtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "QSPI_IO2 output enable. Use this only in SPI or Dual SPI mode to control /WP signal. When the Auto Mode is selected (QSPIC_AUTO_MD = 1) and the QUAD SPI is used, set this bit to zero.\n0: The QSPI_IO2 pad is input.\n1: The QSPI_IO2 pad is output."]
    #[inline(always)]
    pub fn qspic_io2_oen(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Qspic2CtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,Qspic2CtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Mode of the generated QSPI_SCK clock\n\n0: Use Mode 0 for the QSPI_CLK. The QSPI_SCK is low when QSPI_CS is high.\n1: Use Mode 3 for the QSPI_CLK. The QSPI_SCK is high when QSPI_CS is high.\n\nSee also the register QSPIC_CS_MD and the QSPIC_CLK_FREE_EN"]
    #[inline(always)]
    pub fn qspic_clk_md(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Qspic2CtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,Qspic2CtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Mode of operation\n0: The Manual Mode is selected.\n1: The Auto Mode is selected.\nDuring an erasing the QSPIC_AUTO_MD goes in read only mode (see QSPIC_ERASE_EN)"]
    #[inline(always)]
    pub fn qspic_auto_md(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Qspic2CtrlmodeReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Qspic2CtrlmodeReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Qspic2CtrlmodeReg {
    #[inline(always)]
    fn default() -> Qspic2CtrlmodeReg {
        <crate::RegValueT<Qspic2CtrlmodeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qspic2DummydataReg_SPEC;
impl crate::sealed::RegSpec for Qspic2DummydataReg_SPEC {
    type DataType = u32;
}
#[doc = "Send dummy clocks to SPI Bus for the Manual mode"]
pub type Qspic2DummydataReg = crate::RegValueT<Qspic2DummydataReg_SPEC>;

impl Qspic2DummydataReg {
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
        Qspic2DummydataReg_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            Qspic2DummydataReg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Qspic2DummydataReg {
    #[inline(always)]
    fn default() -> Qspic2DummydataReg {
        <crate::RegValueT<Qspic2DummydataReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qspic2ErasecmdaReg_SPEC;
impl crate::sealed::RegSpec for Qspic2ErasecmdaReg_SPEC {
    type DataType = u32;
}
#[doc = "The way of erasing in Auto mode (command register A)"]
pub type Qspic2ErasecmdaReg = crate::RegValueT<Qspic2ErasecmdaReg_SPEC>;

impl Qspic2ErasecmdaReg {
    #[doc = "The code value of the erase resume instruction"]
    #[inline(always)]
    pub fn qspic_res_inst(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Qspic2ErasecmdaReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xff,1,0,u8, Qspic2ErasecmdaReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "The code value of the erase suspend instruction."]
    #[inline(always)]
    pub fn qspic_sus_inst(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Qspic2ErasecmdaReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8, Qspic2ErasecmdaReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "The code value of the write enable instruction."]
    #[inline(always)]
    pub fn qspic_wen_inst(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Qspic2ErasecmdaReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8, Qspic2ErasecmdaReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "The code value of the erase instruction."]
    #[inline(always)]
    pub fn qspic_ers_inst(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Qspic2ErasecmdaReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, Qspic2ErasecmdaReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Qspic2ErasecmdaReg {
    #[inline(always)]
    fn default() -> Qspic2ErasecmdaReg {
        <crate::RegValueT<Qspic2ErasecmdaReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qspic2ErasecmdbReg_SPEC;
impl crate::sealed::RegSpec for Qspic2ErasecmdbReg_SPEC {
    type DataType = u32;
}
#[doc = "The way of erasing in Auto mode (command register B)"]
pub type Qspic2ErasecmdbReg = crate::RegValueT<Qspic2ErasecmdbReg_SPEC>;

impl Qspic2ErasecmdbReg {
    #[doc = "Defines a timer that counts the minimum allowed delay between an erase suspend command and the previous erase resume command (or the initial erase command).\n0x00: Dont wait. The controller starts immediately to suspend the erase procedure.\n0x01..0x3F: The controller waits for at least this number of 288 KHz clock cycles before the suspension of erasing. Time starts counting after the end of the previous erase resume command (or the initial erase command)"]
    #[inline(always)]
    pub fn qspic_ressus_dly(
        self,
    ) -> crate::common::RegisterField<24, 0x3f, 1, 0, u8, Qspic2ErasecmdbReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x3f,1,0,u8, Qspic2ErasecmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "The controller must stay without flash memory reading requests for this number of AMBA hclk clock cycles, before to perform the command of erase or erase resume. Allowable range : 0xF - 0x0"]
    #[inline(always)]
    pub fn qspic_ersres_hld(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Qspic2ErasecmdbReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xf,1,0,u8, Qspic2ErasecmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "After the execution of instructions: write enable, erase, erase suspend and erase resume, the QSPI_CS remains high for at least this number of QSPI_SCK clock cycles."]
    #[inline(always)]
    pub fn qspic_ers_cs_hi(
        self,
    ) -> crate::common::RegisterField<10, 0x1f, 1, 0, u8, Qspic2ErasecmdbReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1f,1,0,u8, Qspic2ErasecmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "The mode of the SPI Bus during the address phase of the erase instruction\n0x0: Single SPI\n0x1: Dual\n0x2: Quad\n0x3: Reserved"]
    #[inline(always)]
    pub fn qspic_ead_tx_md(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, Qspic2ErasecmdbReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8, Qspic2ErasecmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "The mode of the SPI Bus during the transmission of the resume instruction\n0x0: Single SPI\n0x1: Dual\n0x2: Quad\n0x3: Reserved"]
    #[inline(always)]
    pub fn qspic_res_tx_md(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, Qspic2ErasecmdbReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,u8, Qspic2ErasecmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "The mode of the SPI Bus during the transmission of the suspend instruction.\n0x0: Single SPI\n0x1: Dual\n0x2: Quad\n0x3: Reserved"]
    #[inline(always)]
    pub fn qspic_sus_tx_md(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, Qspic2ErasecmdbReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3,1,0,u8, Qspic2ErasecmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "The mode of the SPI Bus during the transmission of the write enable instruction.\n0x0: Single SPI\n0x1: Dual\n0x2: Quad\n0x3: Reserved"]
    #[inline(always)]
    pub fn qspic_wen_tx_md(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, Qspic2ErasecmdbReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3,1,0,u8, Qspic2ErasecmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "The mode of the SPI Bus during the instruction phase of the erase instruction\n0x0: Single SPI\n0x1: Dual\n0x2: Quad\n0x3: Reserved"]
    #[inline(always)]
    pub fn qspic_ers_tx_md(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Qspic2ErasecmdbReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,u8, Qspic2ErasecmdbReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Qspic2ErasecmdbReg {
    #[inline(always)]
    fn default() -> Qspic2ErasecmdbReg {
        <crate::RegValueT<Qspic2ErasecmdbReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qspic2ErasectrlReg_SPEC;
impl crate::sealed::RegSpec for Qspic2ErasectrlReg_SPEC {
    type DataType = u32;
}
#[doc = "Erase control register"]
pub type Qspic2ErasectrlReg = crate::RegValueT<Qspic2ErasectrlReg_SPEC>;

impl Qspic2ErasectrlReg {
    #[doc = "It shows the progress of sector/block erasing (read only).\n0x0: No Erase.\n0x1: Pending erase request\n0x2: Erase procedure is running\n0x3: Suspended Erase procedure\n0x4: Finishing the Erase procedure\n0x5..0x7: Reserved"]
    #[inline(always)]
    pub fn qspic_ers_state(
        self,
    ) -> crate::common::RegisterField<25, 0x7, 1, 0, u8, Qspic2ErasectrlReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<25,0x7,1,0,u8, Qspic2ErasectrlReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "This bit has meaning only when the external device is a serial FLASH (QSPIC_SRAM_EN=0). \n\nDuring Manual mode (QSPIC_AUTO_MD = 0) : This bit is in read only mode.\nDuring Auto mode (QSPIC_AUTO_MD = 1). To request the erasing of the block/sector (QSPIC_ERS_ADDR, 12\'b0) write 1 to this bit. This bit is cleared automatically with the end of the erasing. Until the end of erasing the QSPIC_ERASE_EN remains in read only mode. During the same period of time the controller remains in Auto Mode (QSPIC_AUTO_MD goes in read only mode).\n\nIn the case where the external device is a serial SRAM (QSPIC_SRAM_EN=1) this bit is in read only mode."]
    #[inline(always)]
    pub fn qspic_erase_en(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Qspic2ErasectrlReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<24,1,0,Qspic2ErasectrlReg_SPEC,crate::common::RW>::from_register(self,0)
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
        Qspic2ErasectrlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0xfffff,
            1,
            0,
            u32,
            Qspic2ErasectrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Qspic2ErasectrlReg {
    #[inline(always)]
    fn default() -> Qspic2ErasectrlReg {
        <crate::RegValueT<Qspic2ErasectrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qspic2GpReg_SPEC;
impl crate::sealed::RegSpec for Qspic2GpReg_SPEC {
    type DataType = u32;
}
#[doc = "General purpose QSPIC2 register"]
pub type Qspic2GpReg = crate::RegValueT<Qspic2GpReg_SPEC>;

impl Qspic2GpReg {
    #[doc = "QSPI pads slew rate control. Indicative values under certain conditions:\n0x0 : Rise=1.7 V/ns, Fall=1.9 V/ns (weak)\n0x1 : Rise=2.0 V/ns, Fall=2.3 V/ns\n0x2 : Rise=2.3 V/ns, Fall=2.6 V/ns\n0x3 : Rise=2.4 V/ns, Fall=2.7 V/ns (strong)\nConditions: FLASH pin capacitance 6pF, Vcc=1.8V, T=25C and Idrive=16mA"]
    #[inline(always)]
    pub fn qspic_pads_slew(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, Qspic2GpReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x3,1,0,u8, Qspic2GpReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "QSPI pads drive current\n0x0 : 4 mA\n0x1 : 8 mA\n0x2 : 12 mA\n0x3 : 16 mA"]
    #[inline(always)]
    pub fn qspic_pads_drv(
        self,
    ) -> crate::common::RegisterField<1, 0x3, 1, 0, u8, Qspic2GpReg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x3,1,0,u8, Qspic2GpReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Qspic2GpReg {
    #[inline(always)]
    fn default() -> Qspic2GpReg {
        <crate::RegValueT<Qspic2GpReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qspic2MemblenReg_SPEC;
impl crate::sealed::RegSpec for Qspic2MemblenReg_SPEC {
    type DataType = u32;
}
#[doc = "External memory burst length configuration"]
pub type Qspic2MemblenReg = crate::RegValueT<Qspic2MemblenReg_SPEC>;

impl Qspic2MemblenReg {
    #[doc = "Defines the maximum allowed time tCEM for which the QSPIC_CS can stay active (QSPI_CS=0). It has meaning only when QSPIC_T_CEM_EN is equal to 1. See also the description of the QSPIC_T_CEM_EN for more details.\n\nThe tCEM is expressed in number of qspi clock cycles and can be calculated as follows :\n\ntCEM / (qspi_clock_period)\n\nIf the result of the above equation is higher than 0x3FF, use the value 0x3FF."]
    #[inline(always)]
    pub fn qspic_t_cem_cc(
        self,
    ) -> crate::common::RegisterField<4, 0x3ff, 1, 0, u16, Qspic2MemblenReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3ff,1,0,u16, Qspic2MemblenReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "This bit enables the controlling of the maximum time tCEM for which the QSPI_CS remains active. It has meaning only when the Auto mode is active (QSPIC_AUTO_MD=1) and the external device is a serial SRAM (QSPIC_SRAM_EN=1). In the case where the external device is a serial Flash (QSPIC_SRAM_EN=0) or the controller is in Manual mode (QSPIC_AUTO_MD=0), this field has no any effect.\n\nThis feature is usefull in the case where the external serial device is a dynamic RAM that requires refresh. If the refresh is applied only when the device is in the idle state (QSPI_CS = 1), the time for which the device remains in the active state (QSPI_CS = 0) should be limited by a maximum threshold.\n\n0:There is no any constraint regarding the maximum allowed time for which the QSPI_CS can stay active. This is the case also when QSPIC_SRAM_EN=0 or QSPIC_AUTO_MD=0.\n1:There is a maximum allowed time interval tCEM for which the QSPI_CS can stay active during a burst access (for reading or writting of data). For the controller this is considered as equal to QSPIC_T_CEM_CC x qspi_clock_period. In the case where the data transfer requires the QSPI_CS to stays active for more than QSPIC_T_CEM_CC qspi clock cycles, the QSPI controller splits the access on the SPI bus in more than one bursts, by inserting inactive periods (QSPI_CS = 0) between them. This will cost extra clock cycles for the realization of the original acceess, due to the additional commands that are required in the SPI bus.\n\nThe value in the QSPIC_T_CEM_CC should be updated every time where the frequency of the qspi clock is modified. The qspi clock frequency should not be decreased more than a lowest frequency. This is the lowest frequency that enables the be performed a 32-bit word read and write access, without violating the tCEM timing requirement (the QSPI controller allows to be performed at least the transferring of one beat of the requested burst, independent of the QSPIC_T_CEM_CC limit)."]
    #[inline(always)]
    pub fn qspic_t_cem_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Qspic2MemblenReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,Qspic2MemblenReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "In this register is defined the expected behavior of the external memory device regarding the length of a burst operation :\n\n0x0: The external memory device is capable to implement incremental burst of unspecified length.\n0x1: The external memory device implements a wrapping burst of length 4 bytes.\n0x2: The external memory device implements a wrapping burst of length 8 bytes.\n0x3: The external memory device implements a wrapping burst of length 16 bytes.\n0x4: The external memory device implements a wrapping burst of length 32 bytes.\n0x5: The external memory device implements a wrapping burst of length 64 bytes.\n0x6 - 0x7 : Reserved\n\nThis setting is used by the QSPI controller when the Auto mode is enabled (QSPIC_AUTO_MD=1), in order to handle the various burst requests of the AHB bus, in respect of the requirements of the external memory device.\nThe external memory device may need to be configured by applying special instruction, in order to be defined the kind of the burst operation. This can be implemented by applying this special instruction with the QSPI controller in Manual mode (QSPIC_AUTO_MD=1). Refer to the datasheet of the external device for more information."]
    #[inline(always)]
    pub fn qspic_memblen(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, Qspic2MemblenReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,u8, Qspic2MemblenReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Qspic2MemblenReg {
    #[inline(always)]
    fn default() -> Qspic2MemblenReg {
        <crate::RegValueT<Qspic2MemblenReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qspic2ReaddataReg_SPEC;
impl crate::sealed::RegSpec for Qspic2ReaddataReg_SPEC {
    type DataType = u32;
}
#[doc = "Read data from SPI Bus for the Manual mode"]
pub type Qspic2ReaddataReg = crate::RegValueT<Qspic2ReaddataReg_SPEC>;

impl Qspic2ReaddataReg {
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
        Qspic2ReaddataReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            Qspic2ReaddataReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Qspic2ReaddataReg {
    #[inline(always)]
    fn default() -> Qspic2ReaddataReg {
        <crate::RegValueT<Qspic2ReaddataReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qspic2RecvdataReg_SPEC;
impl crate::sealed::RegSpec for Qspic2RecvdataReg_SPEC {
    type DataType = u32;
}
#[doc = "Received data for the Manual mode"]
pub type Qspic2RecvdataReg = crate::RegValueT<Qspic2RecvdataReg_SPEC>;

impl Qspic2RecvdataReg {
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
        Qspic2RecvdataReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            Qspic2RecvdataReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Qspic2RecvdataReg {
    #[inline(always)]
    fn default() -> Qspic2RecvdataReg {
        <crate::RegValueT<Qspic2RecvdataReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qspic2StatuscmdReg_SPEC;
impl crate::sealed::RegSpec for Qspic2StatuscmdReg_SPEC {
    type DataType = u32;
}
#[doc = "The way of reading the status of external device in Auto mode"]
pub type Qspic2StatuscmdReg = crate::RegValueT<Qspic2StatuscmdReg_SPEC>;

impl Qspic2StatuscmdReg {
    #[doc = "Defines the timer which is used to count the delay that it has to wait before to read the FLASH Status Register, after an erase or an erase resume command.\n0: The delay is controlled by the QSPIC_RESSTS_DLY which counts on the qspi clock.\n1: The delay is controlled by the QSPIC_RESSUS_DLY which counts on the 288 kHz clock."]
    #[inline(always)]
    pub fn qspic_stsdly_sel(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Qspic2StatuscmdReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<22,1,0,Qspic2StatuscmdReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Defines a timer that counts the minimum required delay between the reading of the status register and of the previous erase or erase resume instruction.\n0x00: Dont wait. The controller starts to reading the Flash memory status register immediately.\n0x01..0x3F: The controller waits for at least this number of QSPI_CLK cycles and afterwards it starts to reading the Flash memory status register. The timer starts to count after the end of the previous erase or erase resume command.\n\nThe actual timer that will be used by the controller before the reading of the Flash memory status register is defined by the QSPIC_STSDLY_SEL."]
    #[inline(always)]
    pub fn qspic_ressts_dly(
        self,
    ) -> crate::common::RegisterField<16, 0x3f, 1, 0, u8, Qspic2StatuscmdReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3f,1,0,u8, Qspic2StatuscmdReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Defines the value of the Busy bit which means that the flash is busy.\n0: The flash is busy when the Busy bit is equal to 0.\n1: The flash is busy when the Busy bit is equal to 1."]
    #[inline(always)]
    pub fn qspic_busy_val(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Qspic2StatuscmdReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<15,1,0,Qspic2StatuscmdReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Defines the bit of the Flash status register which represents the Busy bit (0x7 - 0x0)."]
    #[inline(always)]
    pub fn qspic_busy_pos(
        self,
    ) -> crate::common::RegisterField<12, 0x7, 1, 0, u8, Qspic2StatuscmdReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x7,1,0,u8, Qspic2StatuscmdReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "The mode of the SPI Bus during the reception phase of the read status instruction, where the value of status register is retrieved.\n0x0: Single SPI\n0x1: Dual\n0x2: Quad\n0x3: Reserved"]
    #[inline(always)]
    pub fn qspic_rstat_rx_md(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, Qspic2StatuscmdReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,u8, Qspic2StatuscmdReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "The mode of the SPI Bus during the instruction phase of the read status instruction.\n0x0: Single SPI\n0x1: Dual\n0x2: Quad\n0x3: Reserved"]
    #[inline(always)]
    pub fn qspic_rstat_tx_md(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, Qspic2StatuscmdReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,u8, Qspic2StatuscmdReg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "The code value of the read status instruction.\nIt is transmitted during the instruction phase of the read status instruction."]
    #[inline(always)]
    pub fn qspic_rstat_inst(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Qspic2StatuscmdReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, Qspic2StatuscmdReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Qspic2StatuscmdReg {
    #[inline(always)]
    fn default() -> Qspic2StatuscmdReg {
        <crate::RegValueT<Qspic2StatuscmdReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qspic2StatusReg_SPEC;
impl crate::sealed::RegSpec for Qspic2StatusReg_SPEC {
    type DataType = u32;
}
#[doc = "The status register of the QSPI controller"]
pub type Qspic2StatusReg = crate::RegValueT<Qspic2StatusReg_SPEC>;

impl Qspic2StatusReg {
    #[doc = "The status of the SPI Bus.\n\n0: The SPI Bus is idle\n1: The SPI Bus is active. Read data, write data or dummy data activity is in progress.\n\nHas meaning only in Manual mode and only when QSPIC_HRDY_MD = 1."]
    #[inline(always)]
    pub fn qspic_busy(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Qspic2StatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,Qspic2StatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Qspic2StatusReg {
    #[inline(always)]
    fn default() -> Qspic2StatusReg {
        <crate::RegValueT<Qspic2StatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qspic2WritedataReg_SPEC;
impl crate::sealed::RegSpec for Qspic2WritedataReg_SPEC {
    type DataType = u32;
}
#[doc = "Write data to SPI Bus for the Manual mode"]
pub type Qspic2WritedataReg = crate::RegValueT<Qspic2WritedataReg_SPEC>;

impl Qspic2WritedataReg {
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
        Qspic2WritedataReg_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            Qspic2WritedataReg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Qspic2WritedataReg {
    #[inline(always)]
    fn default() -> Qspic2WritedataReg {
        <crate::RegValueT<Qspic2WritedataReg_SPEC> as RegisterValue<_>>::new(0)
    }
}
