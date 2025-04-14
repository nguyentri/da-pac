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
// Generated from SVD 1.2, with svd2pac 0.5.0 on Mon, 14 Apr 2025 11:16:41 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"I3C registers"]
unsafe impl ::core::marker::Send for super::I3C {}
unsafe impl ::core::marker::Sync for super::I3C {}
impl super::I3C {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[inline(always)]
    pub const fn i3c_bus_free_avail_timing_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CBusFreeAvailTimingReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CBusFreeAvailTimingReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(212usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_command_queue_port_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CCommandQueuePortReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CCommandQueuePortReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_data_buffer_stat_level_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDataBufferStatLevelReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDataBufferStatLevelReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(80usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_data_buffer_thld_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDataBufferThldCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDataBufferThldCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(32usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_device_addr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDeviceAddrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDeviceAddrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_device_addr_table_ptr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDeviceAddrTablePtrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDeviceAddrTablePtrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(92usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_device_ctrl_extended_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDeviceCtrlExtendedReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDeviceCtrlExtendedReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(176usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_device_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDeviceCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDeviceCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_dev_addr_table_loc1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevAddrTableLoc1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevAddrTableLoc1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(640usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_dev_addr_table_loc2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevAddrTableLoc2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevAddrTableLoc2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(644usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_dev_addr_table_loc3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevAddrTableLoc3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevAddrTableLoc3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(648usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_dev_addr_table_loc4_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevAddrTableLoc4Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevAddrTableLoc4Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(652usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_dev_addr_table_loc5_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevAddrTableLoc5Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevAddrTableLoc5Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(656usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_dev_addr_table_loc6_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevAddrTableLoc6Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevAddrTableLoc6Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(660usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_dev_addr_table_loc7_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevAddrTableLoc7Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevAddrTableLoc7Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(664usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_dev_addr_table_loc8_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevAddrTableLoc8Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevAddrTableLoc8Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(668usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_dev_char_table1_loc1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevCharTable1Loc1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevCharTable1Loc1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(512usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_dev_char_table1_loc2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevCharTable1Loc2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevCharTable1Loc2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(516usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_dev_char_table1_loc3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevCharTable1Loc3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevCharTable1Loc3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(520usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_dev_char_table1_loc4_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevCharTable1Loc4Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevCharTable1Loc4Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(524usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_dev_char_table2_loc1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevCharTable2Loc1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevCharTable2Loc1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(528usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_dev_char_table2_loc2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevCharTable2Loc2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevCharTable2Loc2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(532usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_dev_char_table2_loc3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevCharTable2Loc3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevCharTable2Loc3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(536usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_dev_char_table2_loc4_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevCharTable2Loc4Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevCharTable2Loc4Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(540usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_dev_char_table3_loc1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevCharTable3Loc1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevCharTable3Loc1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(544usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_dev_char_table3_loc2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevCharTable3Loc2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevCharTable3Loc2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(548usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_dev_char_table3_loc3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevCharTable3Loc3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevCharTable3Loc3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(552usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_dev_char_table3_loc4_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevCharTable3Loc4Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevCharTable3Loc4Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(556usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_dev_char_table4_loc1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevCharTable4Loc1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevCharTable4Loc1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(560usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_dev_char_table4_loc2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevCharTable4Loc2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevCharTable4Loc2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(564usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_dev_char_table4_loc3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevCharTable4Loc3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevCharTable4Loc3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(568usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_dev_char_table4_loc4_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevCharTable4Loc4Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevCharTable4Loc4Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(572usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_dev_char_table5_loc1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevCharTable5Loc1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevCharTable5Loc1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(576usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_dev_char_table5_loc2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevCharTable5Loc2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevCharTable5Loc2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(580usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_dev_char_table5_loc3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevCharTable5Loc3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevCharTable5Loc3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(584usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_dev_char_table5_loc4_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevCharTable5Loc4Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevCharTable5Loc4Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(588usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_dev_char_table6_loc1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevCharTable6Loc1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevCharTable6Loc1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(592usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_dev_char_table6_loc2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevCharTable6Loc2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevCharTable6Loc2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(596usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_dev_char_table6_loc3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevCharTable6Loc3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevCharTable6Loc3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(600usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_dev_char_table6_loc4_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevCharTable6Loc4Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevCharTable6Loc4Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(604usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_dev_char_table7_loc1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevCharTable7Loc1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevCharTable7Loc1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(608usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_dev_char_table7_loc2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevCharTable7Loc2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevCharTable7Loc2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(612usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_dev_char_table7_loc3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevCharTable7Loc3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevCharTable7Loc3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(616usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_dev_char_table7_loc4_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevCharTable7Loc4Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevCharTable7Loc4Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(620usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_dev_char_table8_loc1_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevCharTable8Loc1Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevCharTable8Loc1Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(624usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_dev_char_table8_loc2_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevCharTable8Loc2Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevCharTable8Loc2Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(628usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_dev_char_table8_loc3_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevCharTable8Loc3Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevCharTable8Loc3Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(632usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_dev_char_table8_loc4_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevCharTable8Loc4Reg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevCharTable8Loc4Reg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(636usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_dev_char_table_pointer_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CDevCharTablePointerReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CDevCharTablePointerReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(96usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_hw_capability_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CHwCapabilityReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CHwCapabilityReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_ibi_queue_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CIbiQueueCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CIbiQueueCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(36usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_ibi_queue_status_data_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CIbiQueueStatusDataReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CIbiQueueStatusDataReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(24usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_ibi_sir_req_reject_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CIbiSirReqRejectReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CIbiSirReqRejectReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(48usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_intr_force_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CIntrForceReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CIntrForceReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(72usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_intr_signal_en_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CIntrSignalEnReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CIntrSignalEnReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(68usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_intr_status_en_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CIntrStatusEnReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CIntrStatusEnReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(64usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_intr_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CIntrStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CIntrStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(60usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_present_state_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CPresentStateReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CPresentStateReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(84usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_queue_size_capability_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CQueueSizeCapabilityReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CQueueSizeCapabilityReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(232usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_queue_status_level_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CQueueStatusLevelReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CQueueStatusLevelReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(76usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_queue_thld_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CQueueThldCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CQueueThldCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(28usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_reset_ctrl_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CResetCtrlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CResetCtrlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(52usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_response_queue_port_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CResponseQueuePortReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CResponseQueuePortReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_rx_tx_data_port_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CRxTxDataPortReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CRxTxDataPortReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(20usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_scl_ext_lcnt_timing_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CSclExtLcntTimingReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CSclExtLcntTimingReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(200usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_scl_ext_termn_lcnt_time_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CSclExtTermnLcntTimeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CSclExtTermnLcntTimeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(204usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_scl_i2c_fmp_timing_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CSclI2CFmpTimingReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CSclI2CFmpTimingReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(192usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_scl_i2c_fm_timing_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CSclI2CFmTimingReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CSclI2CFmTimingReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(188usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_scl_i3c_od_timing_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CSclI3COdTimingReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CSclI3COdTimingReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(180usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_scl_i3c_pp_timing_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CSclI3CPpTimingReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CSclI3CPpTimingReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(184usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_sda_hold_dly_timing_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CSdaHoldDlyTimingReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CSdaHoldDlyTimingReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(208usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_slv_event_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CSlvEventStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CSlvEventStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(56usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_vendor_specific_reg_ptr_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CVendorSpecificRegPtrReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CVendorSpecificRegPtrReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(108usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_ver_id_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CVerIdReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CVerIdReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(224usize),
            )
        }
    }

    #[inline(always)]
    pub const fn i3c_ver_type_reg(
        &self,
    ) -> &'static crate::common::Reg<self::I3CVerTypeReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::I3CVerTypeReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(228usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CBusFreeAvailTimingReg_SPEC;
impl crate::sealed::RegSpec for I3CBusFreeAvailTimingReg_SPEC {
    type DataType = u32;
}

pub type I3CBusFreeAvailTimingReg = crate::RegValueT<I3CBusFreeAvailTimingReg_SPEC>;

impl I3CBusFreeAvailTimingReg {
    #[inline(always)]
    pub fn bus_free_time(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        I3CBusFreeAvailTimingReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            I3CBusFreeAvailTimingReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CBusFreeAvailTimingReg {
    #[inline(always)]
    fn default() -> I3CBusFreeAvailTimingReg {
        <crate::RegValueT<I3CBusFreeAvailTimingReg_SPEC> as RegisterValue<_>>::new(32)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CCommandQueuePortReg_SPEC;
impl crate::sealed::RegSpec for I3CCommandQueuePortReg_SPEC {
    type DataType = u32;
}

pub type I3CCommandQueuePortReg = crate::RegValueT<I3CCommandQueuePortReg_SPEC>;

impl I3CCommandQueuePortReg {
    #[inline(always)]
    pub fn command(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        I3CCommandQueuePortReg_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            I3CCommandQueuePortReg_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CCommandQueuePortReg {
    #[inline(always)]
    fn default() -> I3CCommandQueuePortReg {
        <crate::RegValueT<I3CCommandQueuePortReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDataBufferStatLevelReg_SPEC;
impl crate::sealed::RegSpec for I3CDataBufferStatLevelReg_SPEC {
    type DataType = u32;
}

pub type I3CDataBufferStatLevelReg = crate::RegValueT<I3CDataBufferStatLevelReg_SPEC>;

impl I3CDataBufferStatLevelReg {
    #[inline(always)]
    pub fn rx_buf_blr(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDataBufferStatLevelReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDataBufferStatLevelReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tx_buf_empty_loc(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDataBufferStatLevelReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDataBufferStatLevelReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDataBufferStatLevelReg {
    #[inline(always)]
    fn default() -> I3CDataBufferStatLevelReg {
        <crate::RegValueT<I3CDataBufferStatLevelReg_SPEC> as RegisterValue<_>>::new(32)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDataBufferThldCtrlReg_SPEC;
impl crate::sealed::RegSpec for I3CDataBufferThldCtrlReg_SPEC {
    type DataType = u32;
}

pub type I3CDataBufferThldCtrlReg = crate::RegValueT<I3CDataBufferThldCtrlReg_SPEC>;

impl I3CDataBufferThldCtrlReg {
    #[inline(always)]
    pub fn rx_start_thld(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x7,
        1,
        0,
        u8,
        u8,
        I3CDataBufferThldCtrlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x7,
            1,
            0,
            u8,
            u8,
            I3CDataBufferThldCtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tx_start_thld(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7,
        1,
        0,
        u8,
        u8,
        I3CDataBufferThldCtrlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7,
            1,
            0,
            u8,
            u8,
            I3CDataBufferThldCtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rx_buf_thld(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x7,
        1,
        0,
        u8,
        u8,
        I3CDataBufferThldCtrlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x7,
            1,
            0,
            u8,
            u8,
            I3CDataBufferThldCtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tx_empty_buf_thld(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7,
        1,
        0,
        u8,
        u8,
        I3CDataBufferThldCtrlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            u8,
            u8,
            I3CDataBufferThldCtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDataBufferThldCtrlReg {
    #[inline(always)]
    fn default() -> I3CDataBufferThldCtrlReg {
        <crate::RegValueT<I3CDataBufferThldCtrlReg_SPEC> as RegisterValue<_>>::new(50529027)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDeviceAddrReg_SPEC;
impl crate::sealed::RegSpec for I3CDeviceAddrReg_SPEC {
    type DataType = u32;
}

pub type I3CDeviceAddrReg = crate::RegValueT<I3CDeviceAddrReg_SPEC>;

impl I3CDeviceAddrReg {
    #[inline(always)]
    pub fn dynamic_addr_valid(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, I3CDeviceAddrReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,I3CDeviceAddrReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dynamic_addr(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7f,
        1,
        0,
        u8,
        u8,
        I3CDeviceAddrReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7f,
            1,
            0,
            u8,
            u8,
            I3CDeviceAddrReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDeviceAddrReg {
    #[inline(always)]
    fn default() -> I3CDeviceAddrReg {
        <crate::RegValueT<I3CDeviceAddrReg_SPEC> as RegisterValue<_>>::new(2147483647)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDeviceAddrTablePtrReg_SPEC;
impl crate::sealed::RegSpec for I3CDeviceAddrTablePtrReg_SPEC {
    type DataType = u32;
}

pub type I3CDeviceAddrTablePtrReg = crate::RegValueT<I3CDeviceAddrTablePtrReg_SPEC>;

impl I3CDeviceAddrTablePtrReg {
    #[inline(always)]
    pub fn dev_addr_table_depth(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        u16,
        u16,
        I3CDeviceAddrTablePtrReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            I3CDeviceAddrTablePtrReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn p_dev_addr_table_start_addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        I3CDeviceAddrTablePtrReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            I3CDeviceAddrTablePtrReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDeviceAddrTablePtrReg {
    #[inline(always)]
    fn default() -> I3CDeviceAddrTablePtrReg {
        <crate::RegValueT<I3CDeviceAddrTablePtrReg_SPEC> as RegisterValue<_>>::new(524928)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDeviceCtrlExtendedReg_SPEC;
impl crate::sealed::RegSpec for I3CDeviceCtrlExtendedReg_SPEC {
    type DataType = u32;
}

pub type I3CDeviceCtrlExtendedReg = crate::RegValueT<I3CDeviceCtrlExtendedReg_SPEC>;

impl I3CDeviceCtrlExtendedReg {
    #[inline(always)]
    pub fn dev_operation_mode(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        u8,
        u8,
        I3CDeviceCtrlExtendedReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            u8,
            u8,
            I3CDeviceCtrlExtendedReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDeviceCtrlExtendedReg {
    #[inline(always)]
    fn default() -> I3CDeviceCtrlExtendedReg {
        <crate::RegValueT<I3CDeviceCtrlExtendedReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDeviceCtrlReg_SPEC;
impl crate::sealed::RegSpec for I3CDeviceCtrlReg_SPEC {
    type DataType = u32;
}

pub type I3CDeviceCtrlReg = crate::RegValueT<I3CDeviceCtrlReg_SPEC>;

impl I3CDeviceCtrlReg {
    #[inline(always)]
    pub fn enable(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, I3CDeviceCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,I3CDeviceCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn resume(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, I3CDeviceCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30,1,0,I3CDeviceCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn abort(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, I3CDeviceCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29,1,0,I3CDeviceCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_enable_i3c(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, I3CDeviceCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28,1,0,I3CDeviceCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn hot_join_ctrl(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, I3CDeviceCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,I3CDeviceCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn i2c_slave_present(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, I3CDeviceCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,I3CDeviceCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn iba_include(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I3CDeviceCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,I3CDeviceCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for I3CDeviceCtrlReg {
    #[inline(always)]
    fn default() -> I3CDeviceCtrlReg {
        <crate::RegValueT<I3CDeviceCtrlReg_SPEC> as RegisterValue<_>>::new(385)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevAddrTableLoc1Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevAddrTableLoc1Reg_SPEC {
    type DataType = u32;
}

pub type I3CDevAddrTableLoc1Reg = crate::RegValueT<I3CDevAddrTableLoc1Reg_SPEC>;

impl I3CDevAddrTableLoc1Reg {
    #[inline(always)]
    pub fn legacy_i2c_device(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, I3CDevAddrTableLoc1Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<31,1,0,I3CDevAddrTableLoc1Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dev_nack_retry_cnt(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x3,
        1,
        0,
        u8,
        u8,
        I3CDevAddrTableLoc1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x3,
            1,
            0,
            u8,
            u8,
            I3CDevAddrTableLoc1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dev_dynamic_addr(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDevAddrTableLoc1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDevAddrTableLoc1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dev_static_addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7f,
        1,
        0,
        u8,
        u8,
        I3CDevAddrTableLoc1Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7f,
            1,
            0,
            u8,
            u8,
            I3CDevAddrTableLoc1Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevAddrTableLoc1Reg {
    #[inline(always)]
    fn default() -> I3CDevAddrTableLoc1Reg {
        <crate::RegValueT<I3CDevAddrTableLoc1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevAddrTableLoc2Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevAddrTableLoc2Reg_SPEC {
    type DataType = u32;
}

pub type I3CDevAddrTableLoc2Reg = crate::RegValueT<I3CDevAddrTableLoc2Reg_SPEC>;

impl I3CDevAddrTableLoc2Reg {
    #[inline(always)]
    pub fn legacy_i2c_device(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, I3CDevAddrTableLoc2Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<31,1,0,I3CDevAddrTableLoc2Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dev_nack_retry_cnt(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x3,
        1,
        0,
        u8,
        u8,
        I3CDevAddrTableLoc2Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x3,
            1,
            0,
            u8,
            u8,
            I3CDevAddrTableLoc2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dev_dynamic_addr(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDevAddrTableLoc2Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDevAddrTableLoc2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dev_static_addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7f,
        1,
        0,
        u8,
        u8,
        I3CDevAddrTableLoc2Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7f,
            1,
            0,
            u8,
            u8,
            I3CDevAddrTableLoc2Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevAddrTableLoc2Reg {
    #[inline(always)]
    fn default() -> I3CDevAddrTableLoc2Reg {
        <crate::RegValueT<I3CDevAddrTableLoc2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevAddrTableLoc3Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevAddrTableLoc3Reg_SPEC {
    type DataType = u32;
}

pub type I3CDevAddrTableLoc3Reg = crate::RegValueT<I3CDevAddrTableLoc3Reg_SPEC>;

impl I3CDevAddrTableLoc3Reg {
    #[inline(always)]
    pub fn legacy_i2c_device(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, I3CDevAddrTableLoc3Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<31,1,0,I3CDevAddrTableLoc3Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dev_nack_retry_cnt(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x3,
        1,
        0,
        u8,
        u8,
        I3CDevAddrTableLoc3Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x3,
            1,
            0,
            u8,
            u8,
            I3CDevAddrTableLoc3Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dev_dynamic_addr(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDevAddrTableLoc3Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDevAddrTableLoc3Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dev_static_addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7f,
        1,
        0,
        u8,
        u8,
        I3CDevAddrTableLoc3Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7f,
            1,
            0,
            u8,
            u8,
            I3CDevAddrTableLoc3Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevAddrTableLoc3Reg {
    #[inline(always)]
    fn default() -> I3CDevAddrTableLoc3Reg {
        <crate::RegValueT<I3CDevAddrTableLoc3Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevAddrTableLoc4Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevAddrTableLoc4Reg_SPEC {
    type DataType = u32;
}

pub type I3CDevAddrTableLoc4Reg = crate::RegValueT<I3CDevAddrTableLoc4Reg_SPEC>;

impl I3CDevAddrTableLoc4Reg {
    #[inline(always)]
    pub fn legacy_i2c_device(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, I3CDevAddrTableLoc4Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<31,1,0,I3CDevAddrTableLoc4Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dev_nack_retry_cnt(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x3,
        1,
        0,
        u8,
        u8,
        I3CDevAddrTableLoc4Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x3,
            1,
            0,
            u8,
            u8,
            I3CDevAddrTableLoc4Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dev_dynamic_addr(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDevAddrTableLoc4Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDevAddrTableLoc4Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dev_static_addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7f,
        1,
        0,
        u8,
        u8,
        I3CDevAddrTableLoc4Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7f,
            1,
            0,
            u8,
            u8,
            I3CDevAddrTableLoc4Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevAddrTableLoc4Reg {
    #[inline(always)]
    fn default() -> I3CDevAddrTableLoc4Reg {
        <crate::RegValueT<I3CDevAddrTableLoc4Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevAddrTableLoc5Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevAddrTableLoc5Reg_SPEC {
    type DataType = u32;
}

pub type I3CDevAddrTableLoc5Reg = crate::RegValueT<I3CDevAddrTableLoc5Reg_SPEC>;

impl I3CDevAddrTableLoc5Reg {
    #[inline(always)]
    pub fn legacy_i2c_device(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, I3CDevAddrTableLoc5Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<31,1,0,I3CDevAddrTableLoc5Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dev_nack_retry_cnt(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x3,
        1,
        0,
        u8,
        u8,
        I3CDevAddrTableLoc5Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x3,
            1,
            0,
            u8,
            u8,
            I3CDevAddrTableLoc5Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dev_dynamic_addr(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDevAddrTableLoc5Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDevAddrTableLoc5Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dev_static_addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7f,
        1,
        0,
        u8,
        u8,
        I3CDevAddrTableLoc5Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7f,
            1,
            0,
            u8,
            u8,
            I3CDevAddrTableLoc5Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevAddrTableLoc5Reg {
    #[inline(always)]
    fn default() -> I3CDevAddrTableLoc5Reg {
        <crate::RegValueT<I3CDevAddrTableLoc5Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevAddrTableLoc6Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevAddrTableLoc6Reg_SPEC {
    type DataType = u32;
}

pub type I3CDevAddrTableLoc6Reg = crate::RegValueT<I3CDevAddrTableLoc6Reg_SPEC>;

impl I3CDevAddrTableLoc6Reg {
    #[inline(always)]
    pub fn legacy_i2c_device(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, I3CDevAddrTableLoc6Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<31,1,0,I3CDevAddrTableLoc6Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dev_nack_retry_cnt(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x3,
        1,
        0,
        u8,
        u8,
        I3CDevAddrTableLoc6Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x3,
            1,
            0,
            u8,
            u8,
            I3CDevAddrTableLoc6Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dev_dynamic_addr(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDevAddrTableLoc6Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDevAddrTableLoc6Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dev_static_addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7f,
        1,
        0,
        u8,
        u8,
        I3CDevAddrTableLoc6Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7f,
            1,
            0,
            u8,
            u8,
            I3CDevAddrTableLoc6Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevAddrTableLoc6Reg {
    #[inline(always)]
    fn default() -> I3CDevAddrTableLoc6Reg {
        <crate::RegValueT<I3CDevAddrTableLoc6Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevAddrTableLoc7Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevAddrTableLoc7Reg_SPEC {
    type DataType = u32;
}

pub type I3CDevAddrTableLoc7Reg = crate::RegValueT<I3CDevAddrTableLoc7Reg_SPEC>;

impl I3CDevAddrTableLoc7Reg {
    #[inline(always)]
    pub fn legacy_i2c_device(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, I3CDevAddrTableLoc7Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<31,1,0,I3CDevAddrTableLoc7Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dev_nack_retry_cnt(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x3,
        1,
        0,
        u8,
        u8,
        I3CDevAddrTableLoc7Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x3,
            1,
            0,
            u8,
            u8,
            I3CDevAddrTableLoc7Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dev_dynamic_addr(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDevAddrTableLoc7Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDevAddrTableLoc7Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dev_static_addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7f,
        1,
        0,
        u8,
        u8,
        I3CDevAddrTableLoc7Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7f,
            1,
            0,
            u8,
            u8,
            I3CDevAddrTableLoc7Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevAddrTableLoc7Reg {
    #[inline(always)]
    fn default() -> I3CDevAddrTableLoc7Reg {
        <crate::RegValueT<I3CDevAddrTableLoc7Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevAddrTableLoc8Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevAddrTableLoc8Reg_SPEC {
    type DataType = u32;
}

pub type I3CDevAddrTableLoc8Reg = crate::RegValueT<I3CDevAddrTableLoc8Reg_SPEC>;

impl I3CDevAddrTableLoc8Reg {
    #[inline(always)]
    pub fn legacy_i2c_device(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, I3CDevAddrTableLoc8Reg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<31,1,0,I3CDevAddrTableLoc8Reg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dev_nack_retry_cnt(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x3,
        1,
        0,
        u8,
        u8,
        I3CDevAddrTableLoc8Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x3,
            1,
            0,
            u8,
            u8,
            I3CDevAddrTableLoc8Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dev_dynamic_addr(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDevAddrTableLoc8Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDevAddrTableLoc8Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dev_static_addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x7f,
        1,
        0,
        u8,
        u8,
        I3CDevAddrTableLoc8Reg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x7f,
            1,
            0,
            u8,
            u8,
            I3CDevAddrTableLoc8Reg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevAddrTableLoc8Reg {
    #[inline(always)]
    fn default() -> I3CDevAddrTableLoc8Reg {
        <crate::RegValueT<I3CDevAddrTableLoc8Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevCharTable1Loc1Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevCharTable1Loc1Reg_SPEC {
    type DataType = u32;
}

pub type I3CDevCharTable1Loc1Reg = crate::RegValueT<I3CDevCharTable1Loc1Reg_SPEC>;

impl I3CDevCharTable1Loc1Reg {
    #[inline(always)]
    pub fn lsb_provisional_id(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        I3CDevCharTable1Loc1Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            I3CDevCharTable1Loc1Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevCharTable1Loc1Reg {
    #[inline(always)]
    fn default() -> I3CDevCharTable1Loc1Reg {
        <crate::RegValueT<I3CDevCharTable1Loc1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevCharTable1Loc2Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevCharTable1Loc2Reg_SPEC {
    type DataType = u32;
}

pub type I3CDevCharTable1Loc2Reg = crate::RegValueT<I3CDevCharTable1Loc2Reg_SPEC>;

impl I3CDevCharTable1Loc2Reg {
    #[inline(always)]
    pub fn msb_provisional_id(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        I3CDevCharTable1Loc2Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            I3CDevCharTable1Loc2Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevCharTable1Loc2Reg {
    #[inline(always)]
    fn default() -> I3CDevCharTable1Loc2Reg {
        <crate::RegValueT<I3CDevCharTable1Loc2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevCharTable1Loc3Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevCharTable1Loc3Reg_SPEC {
    type DataType = u32;
}

pub type I3CDevCharTable1Loc3Reg = crate::RegValueT<I3CDevCharTable1Loc3Reg_SPEC>;

impl I3CDevCharTable1Loc3Reg {
    #[inline(always)]
    pub fn bcr(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDevCharTable1Loc3Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDevCharTable1Loc3Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dcr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDevCharTable1Loc3Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDevCharTable1Loc3Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevCharTable1Loc3Reg {
    #[inline(always)]
    fn default() -> I3CDevCharTable1Loc3Reg {
        <crate::RegValueT<I3CDevCharTable1Loc3Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevCharTable1Loc4Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevCharTable1Loc4Reg_SPEC {
    type DataType = u32;
}

pub type I3CDevCharTable1Loc4Reg = crate::RegValueT<I3CDevCharTable1Loc4Reg_SPEC>;

impl I3CDevCharTable1Loc4Reg {
    #[inline(always)]
    pub fn dev_dynamic_addr_loc4(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDevCharTable1Loc4Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDevCharTable1Loc4Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevCharTable1Loc4Reg {
    #[inline(always)]
    fn default() -> I3CDevCharTable1Loc4Reg {
        <crate::RegValueT<I3CDevCharTable1Loc4Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevCharTable2Loc1Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevCharTable2Loc1Reg_SPEC {
    type DataType = u32;
}

pub type I3CDevCharTable2Loc1Reg = crate::RegValueT<I3CDevCharTable2Loc1Reg_SPEC>;

impl I3CDevCharTable2Loc1Reg {
    #[inline(always)]
    pub fn lsb_provisional_id(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        I3CDevCharTable2Loc1Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            I3CDevCharTable2Loc1Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevCharTable2Loc1Reg {
    #[inline(always)]
    fn default() -> I3CDevCharTable2Loc1Reg {
        <crate::RegValueT<I3CDevCharTable2Loc1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevCharTable2Loc2Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevCharTable2Loc2Reg_SPEC {
    type DataType = u32;
}

pub type I3CDevCharTable2Loc2Reg = crate::RegValueT<I3CDevCharTable2Loc2Reg_SPEC>;

impl I3CDevCharTable2Loc2Reg {
    #[inline(always)]
    pub fn msb_provisional_id(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        I3CDevCharTable2Loc2Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            I3CDevCharTable2Loc2Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevCharTable2Loc2Reg {
    #[inline(always)]
    fn default() -> I3CDevCharTable2Loc2Reg {
        <crate::RegValueT<I3CDevCharTable2Loc2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevCharTable2Loc3Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevCharTable2Loc3Reg_SPEC {
    type DataType = u32;
}

pub type I3CDevCharTable2Loc3Reg = crate::RegValueT<I3CDevCharTable2Loc3Reg_SPEC>;

impl I3CDevCharTable2Loc3Reg {
    #[inline(always)]
    pub fn bcr(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDevCharTable2Loc3Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDevCharTable2Loc3Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dcr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDevCharTable2Loc3Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDevCharTable2Loc3Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevCharTable2Loc3Reg {
    #[inline(always)]
    fn default() -> I3CDevCharTable2Loc3Reg {
        <crate::RegValueT<I3CDevCharTable2Loc3Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevCharTable2Loc4Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevCharTable2Loc4Reg_SPEC {
    type DataType = u32;
}

pub type I3CDevCharTable2Loc4Reg = crate::RegValueT<I3CDevCharTable2Loc4Reg_SPEC>;

impl I3CDevCharTable2Loc4Reg {
    #[inline(always)]
    pub fn dev_dynamic_addr_loc4(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDevCharTable2Loc4Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDevCharTable2Loc4Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevCharTable2Loc4Reg {
    #[inline(always)]
    fn default() -> I3CDevCharTable2Loc4Reg {
        <crate::RegValueT<I3CDevCharTable2Loc4Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevCharTable3Loc1Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevCharTable3Loc1Reg_SPEC {
    type DataType = u32;
}

pub type I3CDevCharTable3Loc1Reg = crate::RegValueT<I3CDevCharTable3Loc1Reg_SPEC>;

impl I3CDevCharTable3Loc1Reg {
    #[inline(always)]
    pub fn lsb_provisional_id(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        I3CDevCharTable3Loc1Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            I3CDevCharTable3Loc1Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevCharTable3Loc1Reg {
    #[inline(always)]
    fn default() -> I3CDevCharTable3Loc1Reg {
        <crate::RegValueT<I3CDevCharTable3Loc1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevCharTable3Loc2Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevCharTable3Loc2Reg_SPEC {
    type DataType = u32;
}

pub type I3CDevCharTable3Loc2Reg = crate::RegValueT<I3CDevCharTable3Loc2Reg_SPEC>;

impl I3CDevCharTable3Loc2Reg {
    #[inline(always)]
    pub fn msb_provisional_id(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        I3CDevCharTable3Loc2Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            I3CDevCharTable3Loc2Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevCharTable3Loc2Reg {
    #[inline(always)]
    fn default() -> I3CDevCharTable3Loc2Reg {
        <crate::RegValueT<I3CDevCharTable3Loc2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevCharTable3Loc3Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevCharTable3Loc3Reg_SPEC {
    type DataType = u32;
}

pub type I3CDevCharTable3Loc3Reg = crate::RegValueT<I3CDevCharTable3Loc3Reg_SPEC>;

impl I3CDevCharTable3Loc3Reg {
    #[inline(always)]
    pub fn bcr(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDevCharTable3Loc3Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDevCharTable3Loc3Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dcr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDevCharTable3Loc3Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDevCharTable3Loc3Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevCharTable3Loc3Reg {
    #[inline(always)]
    fn default() -> I3CDevCharTable3Loc3Reg {
        <crate::RegValueT<I3CDevCharTable3Loc3Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevCharTable3Loc4Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevCharTable3Loc4Reg_SPEC {
    type DataType = u32;
}

pub type I3CDevCharTable3Loc4Reg = crate::RegValueT<I3CDevCharTable3Loc4Reg_SPEC>;

impl I3CDevCharTable3Loc4Reg {
    #[inline(always)]
    pub fn dev_dynamic_addr_loc4(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDevCharTable3Loc4Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDevCharTable3Loc4Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevCharTable3Loc4Reg {
    #[inline(always)]
    fn default() -> I3CDevCharTable3Loc4Reg {
        <crate::RegValueT<I3CDevCharTable3Loc4Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevCharTable4Loc1Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevCharTable4Loc1Reg_SPEC {
    type DataType = u32;
}

pub type I3CDevCharTable4Loc1Reg = crate::RegValueT<I3CDevCharTable4Loc1Reg_SPEC>;

impl I3CDevCharTable4Loc1Reg {
    #[inline(always)]
    pub fn lsb_provisional_id(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        I3CDevCharTable4Loc1Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            I3CDevCharTable4Loc1Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevCharTable4Loc1Reg {
    #[inline(always)]
    fn default() -> I3CDevCharTable4Loc1Reg {
        <crate::RegValueT<I3CDevCharTable4Loc1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevCharTable4Loc2Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevCharTable4Loc2Reg_SPEC {
    type DataType = u32;
}

pub type I3CDevCharTable4Loc2Reg = crate::RegValueT<I3CDevCharTable4Loc2Reg_SPEC>;

impl I3CDevCharTable4Loc2Reg {
    #[inline(always)]
    pub fn msb_provisional_id(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        I3CDevCharTable4Loc2Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            I3CDevCharTable4Loc2Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevCharTable4Loc2Reg {
    #[inline(always)]
    fn default() -> I3CDevCharTable4Loc2Reg {
        <crate::RegValueT<I3CDevCharTable4Loc2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevCharTable4Loc3Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevCharTable4Loc3Reg_SPEC {
    type DataType = u32;
}

pub type I3CDevCharTable4Loc3Reg = crate::RegValueT<I3CDevCharTable4Loc3Reg_SPEC>;

impl I3CDevCharTable4Loc3Reg {
    #[inline(always)]
    pub fn bcr(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDevCharTable4Loc3Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDevCharTable4Loc3Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dcr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDevCharTable4Loc3Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDevCharTable4Loc3Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevCharTable4Loc3Reg {
    #[inline(always)]
    fn default() -> I3CDevCharTable4Loc3Reg {
        <crate::RegValueT<I3CDevCharTable4Loc3Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevCharTable4Loc4Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevCharTable4Loc4Reg_SPEC {
    type DataType = u32;
}

pub type I3CDevCharTable4Loc4Reg = crate::RegValueT<I3CDevCharTable4Loc4Reg_SPEC>;

impl I3CDevCharTable4Loc4Reg {
    #[inline(always)]
    pub fn dev_dynamic_addr_loc4(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDevCharTable4Loc4Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDevCharTable4Loc4Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevCharTable4Loc4Reg {
    #[inline(always)]
    fn default() -> I3CDevCharTable4Loc4Reg {
        <crate::RegValueT<I3CDevCharTable4Loc4Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevCharTable5Loc1Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevCharTable5Loc1Reg_SPEC {
    type DataType = u32;
}

pub type I3CDevCharTable5Loc1Reg = crate::RegValueT<I3CDevCharTable5Loc1Reg_SPEC>;

impl I3CDevCharTable5Loc1Reg {
    #[inline(always)]
    pub fn lsb_provisional_id(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        I3CDevCharTable5Loc1Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            I3CDevCharTable5Loc1Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevCharTable5Loc1Reg {
    #[inline(always)]
    fn default() -> I3CDevCharTable5Loc1Reg {
        <crate::RegValueT<I3CDevCharTable5Loc1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevCharTable5Loc2Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevCharTable5Loc2Reg_SPEC {
    type DataType = u32;
}

pub type I3CDevCharTable5Loc2Reg = crate::RegValueT<I3CDevCharTable5Loc2Reg_SPEC>;

impl I3CDevCharTable5Loc2Reg {
    #[inline(always)]
    pub fn msb_provisional_id(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        I3CDevCharTable5Loc2Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            I3CDevCharTable5Loc2Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevCharTable5Loc2Reg {
    #[inline(always)]
    fn default() -> I3CDevCharTable5Loc2Reg {
        <crate::RegValueT<I3CDevCharTable5Loc2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevCharTable5Loc3Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevCharTable5Loc3Reg_SPEC {
    type DataType = u32;
}

pub type I3CDevCharTable5Loc3Reg = crate::RegValueT<I3CDevCharTable5Loc3Reg_SPEC>;

impl I3CDevCharTable5Loc3Reg {
    #[inline(always)]
    pub fn bcr(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDevCharTable5Loc3Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDevCharTable5Loc3Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dcr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDevCharTable5Loc3Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDevCharTable5Loc3Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevCharTable5Loc3Reg {
    #[inline(always)]
    fn default() -> I3CDevCharTable5Loc3Reg {
        <crate::RegValueT<I3CDevCharTable5Loc3Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevCharTable5Loc4Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevCharTable5Loc4Reg_SPEC {
    type DataType = u32;
}

pub type I3CDevCharTable5Loc4Reg = crate::RegValueT<I3CDevCharTable5Loc4Reg_SPEC>;

impl I3CDevCharTable5Loc4Reg {
    #[inline(always)]
    pub fn dev_dynamic_addr_loc4(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDevCharTable5Loc4Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDevCharTable5Loc4Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevCharTable5Loc4Reg {
    #[inline(always)]
    fn default() -> I3CDevCharTable5Loc4Reg {
        <crate::RegValueT<I3CDevCharTable5Loc4Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevCharTable6Loc1Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevCharTable6Loc1Reg_SPEC {
    type DataType = u32;
}

pub type I3CDevCharTable6Loc1Reg = crate::RegValueT<I3CDevCharTable6Loc1Reg_SPEC>;

impl I3CDevCharTable6Loc1Reg {
    #[inline(always)]
    pub fn lsb_provisional_id(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        I3CDevCharTable6Loc1Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            I3CDevCharTable6Loc1Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevCharTable6Loc1Reg {
    #[inline(always)]
    fn default() -> I3CDevCharTable6Loc1Reg {
        <crate::RegValueT<I3CDevCharTable6Loc1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevCharTable6Loc2Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevCharTable6Loc2Reg_SPEC {
    type DataType = u32;
}

pub type I3CDevCharTable6Loc2Reg = crate::RegValueT<I3CDevCharTable6Loc2Reg_SPEC>;

impl I3CDevCharTable6Loc2Reg {
    #[inline(always)]
    pub fn msb_provisional_id(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        I3CDevCharTable6Loc2Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            I3CDevCharTable6Loc2Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevCharTable6Loc2Reg {
    #[inline(always)]
    fn default() -> I3CDevCharTable6Loc2Reg {
        <crate::RegValueT<I3CDevCharTable6Loc2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevCharTable6Loc3Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevCharTable6Loc3Reg_SPEC {
    type DataType = u32;
}

pub type I3CDevCharTable6Loc3Reg = crate::RegValueT<I3CDevCharTable6Loc3Reg_SPEC>;

impl I3CDevCharTable6Loc3Reg {
    #[inline(always)]
    pub fn bcr(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDevCharTable6Loc3Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDevCharTable6Loc3Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dcr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDevCharTable6Loc3Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDevCharTable6Loc3Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevCharTable6Loc3Reg {
    #[inline(always)]
    fn default() -> I3CDevCharTable6Loc3Reg {
        <crate::RegValueT<I3CDevCharTable6Loc3Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevCharTable6Loc4Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevCharTable6Loc4Reg_SPEC {
    type DataType = u32;
}

pub type I3CDevCharTable6Loc4Reg = crate::RegValueT<I3CDevCharTable6Loc4Reg_SPEC>;

impl I3CDevCharTable6Loc4Reg {
    #[inline(always)]
    pub fn dev_dynamic_addr_loc4(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDevCharTable6Loc4Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDevCharTable6Loc4Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevCharTable6Loc4Reg {
    #[inline(always)]
    fn default() -> I3CDevCharTable6Loc4Reg {
        <crate::RegValueT<I3CDevCharTable6Loc4Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevCharTable7Loc1Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevCharTable7Loc1Reg_SPEC {
    type DataType = u32;
}

pub type I3CDevCharTable7Loc1Reg = crate::RegValueT<I3CDevCharTable7Loc1Reg_SPEC>;

impl I3CDevCharTable7Loc1Reg {
    #[inline(always)]
    pub fn lsb_provisional_id(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        I3CDevCharTable7Loc1Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            I3CDevCharTable7Loc1Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevCharTable7Loc1Reg {
    #[inline(always)]
    fn default() -> I3CDevCharTable7Loc1Reg {
        <crate::RegValueT<I3CDevCharTable7Loc1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevCharTable7Loc2Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevCharTable7Loc2Reg_SPEC {
    type DataType = u32;
}

pub type I3CDevCharTable7Loc2Reg = crate::RegValueT<I3CDevCharTable7Loc2Reg_SPEC>;

impl I3CDevCharTable7Loc2Reg {
    #[inline(always)]
    pub fn msb_provisional_id(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        I3CDevCharTable7Loc2Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            I3CDevCharTable7Loc2Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevCharTable7Loc2Reg {
    #[inline(always)]
    fn default() -> I3CDevCharTable7Loc2Reg {
        <crate::RegValueT<I3CDevCharTable7Loc2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevCharTable7Loc3Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevCharTable7Loc3Reg_SPEC {
    type DataType = u32;
}

pub type I3CDevCharTable7Loc3Reg = crate::RegValueT<I3CDevCharTable7Loc3Reg_SPEC>;

impl I3CDevCharTable7Loc3Reg {
    #[inline(always)]
    pub fn bcr(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDevCharTable7Loc3Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDevCharTable7Loc3Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dcr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDevCharTable7Loc3Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDevCharTable7Loc3Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevCharTable7Loc3Reg {
    #[inline(always)]
    fn default() -> I3CDevCharTable7Loc3Reg {
        <crate::RegValueT<I3CDevCharTable7Loc3Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevCharTable7Loc4Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevCharTable7Loc4Reg_SPEC {
    type DataType = u32;
}

pub type I3CDevCharTable7Loc4Reg = crate::RegValueT<I3CDevCharTable7Loc4Reg_SPEC>;

impl I3CDevCharTable7Loc4Reg {
    #[inline(always)]
    pub fn dev_dynamic_addr_loc4(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDevCharTable7Loc4Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDevCharTable7Loc4Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevCharTable7Loc4Reg {
    #[inline(always)]
    fn default() -> I3CDevCharTable7Loc4Reg {
        <crate::RegValueT<I3CDevCharTable7Loc4Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevCharTable8Loc1Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevCharTable8Loc1Reg_SPEC {
    type DataType = u32;
}

pub type I3CDevCharTable8Loc1Reg = crate::RegValueT<I3CDevCharTable8Loc1Reg_SPEC>;

impl I3CDevCharTable8Loc1Reg {
    #[inline(always)]
    pub fn lsb_provisional_id(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        I3CDevCharTable8Loc1Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            I3CDevCharTable8Loc1Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevCharTable8Loc1Reg {
    #[inline(always)]
    fn default() -> I3CDevCharTable8Loc1Reg {
        <crate::RegValueT<I3CDevCharTable8Loc1Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevCharTable8Loc2Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevCharTable8Loc2Reg_SPEC {
    type DataType = u32;
}

pub type I3CDevCharTable8Loc2Reg = crate::RegValueT<I3CDevCharTable8Loc2Reg_SPEC>;

impl I3CDevCharTable8Loc2Reg {
    #[inline(always)]
    pub fn msb_provisional_id(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        I3CDevCharTable8Loc2Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            I3CDevCharTable8Loc2Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevCharTable8Loc2Reg {
    #[inline(always)]
    fn default() -> I3CDevCharTable8Loc2Reg {
        <crate::RegValueT<I3CDevCharTable8Loc2Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevCharTable8Loc3Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevCharTable8Loc3Reg_SPEC {
    type DataType = u32;
}

pub type I3CDevCharTable8Loc3Reg = crate::RegValueT<I3CDevCharTable8Loc3Reg_SPEC>;

impl I3CDevCharTable8Loc3Reg {
    #[inline(always)]
    pub fn bcr(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDevCharTable8Loc3Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDevCharTable8Loc3Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dcr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDevCharTable8Loc3Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDevCharTable8Loc3Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevCharTable8Loc3Reg {
    #[inline(always)]
    fn default() -> I3CDevCharTable8Loc3Reg {
        <crate::RegValueT<I3CDevCharTable8Loc3Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevCharTable8Loc4Reg_SPEC;
impl crate::sealed::RegSpec for I3CDevCharTable8Loc4Reg_SPEC {
    type DataType = u32;
}

pub type I3CDevCharTable8Loc4Reg = crate::RegValueT<I3CDevCharTable8Loc4Reg_SPEC>;

impl I3CDevCharTable8Loc4Reg {
    #[inline(always)]
    pub fn dev_dynamic_addr_loc4(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CDevCharTable8Loc4Reg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CDevCharTable8Loc4Reg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevCharTable8Loc4Reg {
    #[inline(always)]
    fn default() -> I3CDevCharTable8Loc4Reg {
        <crate::RegValueT<I3CDevCharTable8Loc4Reg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CDevCharTablePointerReg_SPEC;
impl crate::sealed::RegSpec for I3CDevCharTablePointerReg_SPEC {
    type DataType = u32;
}

pub type I3CDevCharTablePointerReg = crate::RegValueT<I3CDevCharTablePointerReg_SPEC>;

impl I3CDevCharTablePointerReg {
    #[inline(always)]
    pub fn present_dev_char_table_indx(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x7,
        1,
        0,
        u8,
        u8,
        I3CDevCharTablePointerReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x7,
            1,
            0,
            u8,
            u8,
            I3CDevCharTablePointerReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn dev_char_table_depth(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x7f,
        1,
        0,
        u8,
        u8,
        I3CDevCharTablePointerReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            12,
            0x7f,
            1,
            0,
            u8,
            u8,
            I3CDevCharTablePointerReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn p_dev_char_table_start_addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xfff,
        1,
        0,
        u16,
        u16,
        I3CDevCharTablePointerReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xfff,
            1,
            0,
            u16,
            u16,
            I3CDevCharTablePointerReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CDevCharTablePointerReg {
    #[inline(always)]
    fn default() -> I3CDevCharTablePointerReg {
        <crate::RegValueT<I3CDevCharTablePointerReg_SPEC> as RegisterValue<_>>::new(131584)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CHwCapabilityReg_SPEC;
impl crate::sealed::RegSpec for I3CHwCapabilityReg_SPEC {
    type DataType = u32;
}

pub type I3CHwCapabilityReg = crate::RegValueT<I3CHwCapabilityReg_SPEC>;

impl I3CHwCapabilityReg {
    #[inline(always)]
    pub fn slv_ibi_cap(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, I3CHwCapabilityReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19,1,0,I3CHwCapabilityReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn slv_hj_cap(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, I3CHwCapabilityReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18,1,0,I3CHwCapabilityReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn dma_en(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, I3CHwCapabilityReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17,1,0,I3CHwCapabilityReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn hdr_tx_clock_period(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x3f,
        1,
        0,
        u8,
        u8,
        I3CHwCapabilityReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            11,
            0x3f,
            1,
            0,
            u8,
            u8,
            I3CHwCapabilityReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn clock_period(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x3f,
        1,
        0,
        u8,
        u8,
        I3CHwCapabilityReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x3f,
            1,
            0,
            u8,
            u8,
            I3CHwCapabilityReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn hdr_ts_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, I3CHwCapabilityReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,I3CHwCapabilityReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn hdr_ddr_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, I3CHwCapabilityReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,I3CHwCapabilityReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn device_role_config(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, u8, I3CHwCapabilityReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            0,
            0x7,
            1,
            0,
            u8,
            u8,
            I3CHwCapabilityReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CHwCapabilityReg {
    #[inline(always)]
    fn default() -> I3CHwCapabilityReg {
        <crate::RegValueT<I3CHwCapabilityReg_SPEC> as RegisterValue<_>>::new(213249)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CIbiQueueCtrlReg_SPEC;
impl crate::sealed::RegSpec for I3CIbiQueueCtrlReg_SPEC {
    type DataType = u32;
}

pub type I3CIbiQueueCtrlReg = crate::RegValueT<I3CIbiQueueCtrlReg_SPEC>;

impl I3CIbiQueueCtrlReg {
    #[inline(always)]
    pub fn notify_sir_rejected(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, I3CIbiQueueCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,I3CIbiQueueCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn notify_hj_rejected(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I3CIbiQueueCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,I3CIbiQueueCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for I3CIbiQueueCtrlReg {
    #[inline(always)]
    fn default() -> I3CIbiQueueCtrlReg {
        <crate::RegValueT<I3CIbiQueueCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CIbiQueueStatusDataReg_SPEC;
impl crate::sealed::RegSpec for I3CIbiQueueStatusDataReg_SPEC {
    type DataType = u32;
}

pub type I3CIbiQueueStatusDataReg = crate::RegValueT<I3CIbiQueueStatusDataReg_SPEC>;

impl I3CIbiQueueStatusDataReg {
    #[inline(always)]
    pub fn ibi_sts(
        self,
    ) -> crate::common::RegisterField<
        28,
        0xf,
        1,
        0,
        u8,
        u8,
        I3CIbiQueueStatusDataReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            28,
            0xf,
            1,
            0,
            u8,
            u8,
            I3CIbiQueueStatusDataReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ibi_id(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CIbiQueueStatusDataReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CIbiQueueStatusDataReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn data_length(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CIbiQueueStatusDataReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CIbiQueueStatusDataReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CIbiQueueStatusDataReg {
    #[inline(always)]
    fn default() -> I3CIbiQueueStatusDataReg {
        <crate::RegValueT<I3CIbiQueueStatusDataReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CIbiSirReqRejectReg_SPEC;
impl crate::sealed::RegSpec for I3CIbiSirReqRejectReg_SPEC {
    type DataType = u32;
}

pub type I3CIbiSirReqRejectReg = crate::RegValueT<I3CIbiSirReqRejectReg_SPEC>;

impl I3CIbiSirReqRejectReg {
    #[inline(always)]
    pub fn sir_req_reject(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        I3CIbiSirReqRejectReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            I3CIbiSirReqRejectReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CIbiSirReqRejectReg {
    #[inline(always)]
    fn default() -> I3CIbiSirReqRejectReg {
        <crate::RegValueT<I3CIbiSirReqRejectReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CIntrForceReg_SPEC;
impl crate::sealed::RegSpec for I3CIntrForceReg_SPEC {
    type DataType = u32;
}

pub type I3CIntrForceReg = crate::RegValueT<I3CIntrForceReg_SPEC>;

impl I3CIntrForceReg {
    #[inline(always)]
    pub fn transfer_err_force_en(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, I3CIntrForceReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<9,1,0,I3CIntrForceReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn transfer_abort_force_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, I3CIntrForceReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5,1,0,I3CIntrForceReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn resp_ready_force_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, I3CIntrForceReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4,1,0,I3CIntrForceReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cmd_queue_ready_force_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, I3CIntrForceReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3,1,0,I3CIntrForceReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ibi_thld_force_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, I3CIntrForceReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2,1,0,I3CIntrForceReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rx_thld_force_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, I3CIntrForceReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1,1,0,I3CIntrForceReg_SPEC,crate::common::W>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tx_thld_force_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I3CIntrForceReg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0,1,0,I3CIntrForceReg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for I3CIntrForceReg {
    #[inline(always)]
    fn default() -> I3CIntrForceReg {
        <crate::RegValueT<I3CIntrForceReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CIntrSignalEnReg_SPEC;
impl crate::sealed::RegSpec for I3CIntrSignalEnReg_SPEC {
    type DataType = u32;
}

pub type I3CIntrSignalEnReg = crate::RegValueT<I3CIntrSignalEnReg_SPEC>;

impl I3CIntrSignalEnReg {
    #[inline(always)]
    pub fn transfer_err_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, I3CIntrSignalEnReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,I3CIntrSignalEnReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn transfer_abort_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, I3CIntrSignalEnReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,I3CIntrSignalEnReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn resp_ready_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, I3CIntrSignalEnReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,I3CIntrSignalEnReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cmd_queue_ready_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, I3CIntrSignalEnReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,I3CIntrSignalEnReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ibi_thld_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, I3CIntrSignalEnReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,I3CIntrSignalEnReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rx_thld_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, I3CIntrSignalEnReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,I3CIntrSignalEnReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tx_thld_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I3CIntrSignalEnReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,I3CIntrSignalEnReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for I3CIntrSignalEnReg {
    #[inline(always)]
    fn default() -> I3CIntrSignalEnReg {
        <crate::RegValueT<I3CIntrSignalEnReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CIntrStatusEnReg_SPEC;
impl crate::sealed::RegSpec for I3CIntrStatusEnReg_SPEC {
    type DataType = u32;
}

pub type I3CIntrStatusEnReg = crate::RegValueT<I3CIntrStatusEnReg_SPEC>;

impl I3CIntrStatusEnReg {
    #[inline(always)]
    pub fn transfer_err_sts_en(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, I3CIntrStatusEnReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,I3CIntrStatusEnReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn transfer_abort_sts_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, I3CIntrStatusEnReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,I3CIntrStatusEnReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn resp_ready_sts_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, I3CIntrStatusEnReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,I3CIntrStatusEnReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cmd_queue_ready_sts_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, I3CIntrStatusEnReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,I3CIntrStatusEnReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ibi_thld_sts_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, I3CIntrStatusEnReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,I3CIntrStatusEnReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rx_thld_sts_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, I3CIntrStatusEnReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,I3CIntrStatusEnReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tx_thld_sts_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I3CIntrStatusEnReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,I3CIntrStatusEnReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for I3CIntrStatusEnReg {
    #[inline(always)]
    fn default() -> I3CIntrStatusEnReg {
        <crate::RegValueT<I3CIntrStatusEnReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CIntrStatusReg_SPEC;
impl crate::sealed::RegSpec for I3CIntrStatusReg_SPEC {
    type DataType = u32;
}

pub type I3CIntrStatusReg = crate::RegValueT<I3CIntrStatusReg_SPEC>;

impl I3CIntrStatusReg {
    #[inline(always)]
    pub fn transfer_err_sts(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, I3CIntrStatusReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,I3CIntrStatusReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn transfer_abort_sts(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, I3CIntrStatusReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,I3CIntrStatusReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn resp_ready_sts(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, I3CIntrStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,I3CIntrStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cmd_queue_ready_sts(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, I3CIntrStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,I3CIntrStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn ibi_thld_sts(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, I3CIntrStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,I3CIntrStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rx_thld_sts(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, I3CIntrStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,I3CIntrStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tx_thld_sts(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I3CIntrStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,I3CIntrStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for I3CIntrStatusReg {
    #[inline(always)]
    fn default() -> I3CIntrStatusReg {
        <crate::RegValueT<I3CIntrStatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CPresentStateReg_SPEC;
impl crate::sealed::RegSpec for I3CPresentStateReg_SPEC {
    type DataType = u32;
}

pub type I3CPresentStateReg = crate::RegValueT<I3CPresentStateReg_SPEC>;

impl I3CPresentStateReg {
    #[inline(always)]
    pub fn master_idle(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, I3CPresentStateReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28,1,0,I3CPresentStateReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cmd_tid(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xf,
        1,
        0,
        u8,
        u8,
        I3CPresentStateReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0xf,
            1,
            0,
            u8,
            u8,
            I3CPresentStateReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cm_tfr_st_sts(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x3f,
        1,
        0,
        u8,
        u8,
        I3CPresentStateReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0x3f,
            1,
            0,
            u8,
            u8,
            I3CPresentStateReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cm_tfr_sts(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3f,
        1,
        0,
        u8,
        u8,
        I3CPresentStateReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x3f,
            1,
            0,
            u8,
            u8,
            I3CPresentStateReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn current_master(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, I3CPresentStateReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,I3CPresentStateReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn sda_line_signal_level(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, I3CPresentStateReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,I3CPresentStateReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[inline(always)]
    pub fn scl_line_signal_level(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I3CPresentStateReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,I3CPresentStateReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for I3CPresentStateReg {
    #[inline(always)]
    fn default() -> I3CPresentStateReg {
        <crate::RegValueT<I3CPresentStateReg_SPEC> as RegisterValue<_>>::new(268435459)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CQueueSizeCapabilityReg_SPEC;
impl crate::sealed::RegSpec for I3CQueueSizeCapabilityReg_SPEC {
    type DataType = u32;
}

pub type I3CQueueSizeCapabilityReg = crate::RegValueT<I3CQueueSizeCapabilityReg_SPEC>;

impl I3CQueueSizeCapabilityReg {
    #[inline(always)]
    pub fn ibi_buf_size(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xf,
        1,
        0,
        u8,
        u8,
        I3CQueueSizeCapabilityReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xf,
            1,
            0,
            u8,
            u8,
            I3CQueueSizeCapabilityReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn resp_buf_size(
        self,
    ) -> crate::common::RegisterField<
        12,
        0xf,
        1,
        0,
        u8,
        u8,
        I3CQueueSizeCapabilityReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            12,
            0xf,
            1,
            0,
            u8,
            u8,
            I3CQueueSizeCapabilityReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmd_buf_size(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xf,
        1,
        0,
        u8,
        u8,
        I3CQueueSizeCapabilityReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xf,
            1,
            0,
            u8,
            u8,
            I3CQueueSizeCapabilityReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn rx_buf_size(
        self,
    ) -> crate::common::RegisterField<
        4,
        0xf,
        1,
        0,
        u8,
        u8,
        I3CQueueSizeCapabilityReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0xf,
            1,
            0,
            u8,
            u8,
            I3CQueueSizeCapabilityReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn tx_buf_size(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        u8,
        u8,
        I3CQueueSizeCapabilityReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            u8,
            u8,
            I3CQueueSizeCapabilityReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CQueueSizeCapabilityReg {
    #[inline(always)]
    fn default() -> I3CQueueSizeCapabilityReg {
        <crate::RegValueT<I3CQueueSizeCapabilityReg_SPEC> as RegisterValue<_>>::new(135748)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CQueueStatusLevelReg_SPEC;
impl crate::sealed::RegSpec for I3CQueueStatusLevelReg_SPEC {
    type DataType = u32;
}

pub type I3CQueueStatusLevelReg = crate::RegValueT<I3CQueueStatusLevelReg_SPEC>;

impl I3CQueueStatusLevelReg {
    #[inline(always)]
    pub fn ibi_sts_cnt(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1f,
        1,
        0,
        u8,
        u8,
        I3CQueueStatusLevelReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0x1f,
            1,
            0,
            u8,
            u8,
            I3CQueueStatusLevelReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn ibi_buf_blr(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CQueueStatusLevelReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CQueueStatusLevelReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn resp_buf_blr(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CQueueStatusLevelReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CQueueStatusLevelReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmd_queue_empty_loc(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CQueueStatusLevelReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CQueueStatusLevelReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CQueueStatusLevelReg {
    #[inline(always)]
    fn default() -> I3CQueueStatusLevelReg {
        <crate::RegValueT<I3CQueueStatusLevelReg_SPEC> as RegisterValue<_>>::new(8)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CQueueThldCtrlReg_SPEC;
impl crate::sealed::RegSpec for I3CQueueThldCtrlReg_SPEC {
    type DataType = u32;
}

pub type I3CQueueThldCtrlReg = crate::RegValueT<I3CQueueThldCtrlReg_SPEC>;

impl I3CQueueThldCtrlReg {
    #[inline(always)]
    pub fn ibi_status_thld(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CQueueThldCtrlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CQueueThldCtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn resp_buf_thld(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CQueueThldCtrlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CQueueThldCtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn cmd_empty_buf_thld(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CQueueThldCtrlReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CQueueThldCtrlReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CQueueThldCtrlReg {
    #[inline(always)]
    fn default() -> I3CQueueThldCtrlReg {
        <crate::RegValueT<I3CQueueThldCtrlReg_SPEC> as RegisterValue<_>>::new(16777473)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CResetCtrlReg_SPEC;
impl crate::sealed::RegSpec for I3CResetCtrlReg_SPEC {
    type DataType = u32;
}

pub type I3CResetCtrlReg = crate::RegValueT<I3CResetCtrlReg_SPEC>;

impl I3CResetCtrlReg {
    #[inline(always)]
    pub fn ibi_queue_rst(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, I3CResetCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,I3CResetCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn rx_fifo_rst(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, I3CResetCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,I3CResetCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn tx_fifo_rst(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, I3CResetCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,I3CResetCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn resp_queue_rst(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, I3CResetCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,I3CResetCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn cmd_queue_rst(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, I3CResetCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,I3CResetCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn soft_rst(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, I3CResetCtrlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,I3CResetCtrlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for I3CResetCtrlReg {
    #[inline(always)]
    fn default() -> I3CResetCtrlReg {
        <crate::RegValueT<I3CResetCtrlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CResponseQueuePortReg_SPEC;
impl crate::sealed::RegSpec for I3CResponseQueuePortReg_SPEC {
    type DataType = u32;
}

pub type I3CResponseQueuePortReg = crate::RegValueT<I3CResponseQueuePortReg_SPEC>;

impl I3CResponseQueuePortReg {
    #[inline(always)]
    pub fn response(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        I3CResponseQueuePortReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            I3CResponseQueuePortReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CResponseQueuePortReg {
    #[inline(always)]
    fn default() -> I3CResponseQueuePortReg {
        <crate::RegValueT<I3CResponseQueuePortReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CRxTxDataPortReg_SPEC;
impl crate::sealed::RegSpec for I3CRxTxDataPortReg_SPEC {
    type DataType = u32;
}

pub type I3CRxTxDataPortReg = crate::RegValueT<I3CRxTxDataPortReg_SPEC>;

impl I3CRxTxDataPortReg {
    #[inline(always)]
    pub fn rx_tx_data_port(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        I3CRxTxDataPortReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            I3CRxTxDataPortReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CRxTxDataPortReg {
    #[inline(always)]
    fn default() -> I3CRxTxDataPortReg {
        <crate::RegValueT<I3CRxTxDataPortReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CSclExtLcntTimingReg_SPEC;
impl crate::sealed::RegSpec for I3CSclExtLcntTimingReg_SPEC {
    type DataType = u32;
}

pub type I3CSclExtLcntTimingReg = crate::RegValueT<I3CSclExtLcntTimingReg_SPEC>;

impl I3CSclExtLcntTimingReg {
    #[inline(always)]
    pub fn i3c_ext_lcnt_4(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CSclExtLcntTimingReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CSclExtLcntTimingReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn i3c_ext_lcnt_3(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CSclExtLcntTimingReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CSclExtLcntTimingReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn i3c_ext_lcnt_2(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CSclExtLcntTimingReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CSclExtLcntTimingReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn i3c_ext_lcnt_1(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CSclExtLcntTimingReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CSclExtLcntTimingReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CSclExtLcntTimingReg {
    #[inline(always)]
    fn default() -> I3CSclExtLcntTimingReg {
        <crate::RegValueT<I3CSclExtLcntTimingReg_SPEC> as RegisterValue<_>>::new(538976288)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CSclExtTermnLcntTimeReg_SPEC;
impl crate::sealed::RegSpec for I3CSclExtTermnLcntTimeReg_SPEC {
    type DataType = u32;
}

pub type I3CSclExtTermnLcntTimeReg = crate::RegValueT<I3CSclExtTermnLcntTimeReg_SPEC>;

impl I3CSclExtTermnLcntTimeReg {
    #[inline(always)]
    pub fn i3c_ext_termn_lcnt(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        u8,
        u8,
        I3CSclExtTermnLcntTimeReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            u8,
            u8,
            I3CSclExtTermnLcntTimeReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CSclExtTermnLcntTimeReg {
    #[inline(always)]
    fn default() -> I3CSclExtTermnLcntTimeReg {
        <crate::RegValueT<I3CSclExtTermnLcntTimeReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CSclI2CFmpTimingReg_SPEC;
impl crate::sealed::RegSpec for I3CSclI2CFmpTimingReg_SPEC {
    type DataType = u32;
}

pub type I3CSclI2CFmpTimingReg = crate::RegValueT<I3CSclI2CFmpTimingReg_SPEC>;

impl I3CSclI2CFmpTimingReg {
    #[inline(always)]
    pub fn i2c_fmp_hcnt(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CSclI2CFmpTimingReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CSclI2CFmpTimingReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn i2c_fmp_lcnt(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        I3CSclI2CFmpTimingReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            I3CSclI2CFmpTimingReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CSclI2CFmpTimingReg {
    #[inline(always)]
    fn default() -> I3CSclI2CFmpTimingReg {
        <crate::RegValueT<I3CSclI2CFmpTimingReg_SPEC> as RegisterValue<_>>::new(1048592)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CSclI2CFmTimingReg_SPEC;
impl crate::sealed::RegSpec for I3CSclI2CFmTimingReg_SPEC {
    type DataType = u32;
}

pub type I3CSclI2CFmTimingReg = crate::RegValueT<I3CSclI2CFmTimingReg_SPEC>;

impl I3CSclI2CFmTimingReg {
    #[inline(always)]
    pub fn i2c_fm_hcnt(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        u16,
        u16,
        I3CSclI2CFmTimingReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            u16,
            u16,
            I3CSclI2CFmTimingReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn i2c_fm_lcnt(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        I3CSclI2CFmTimingReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            I3CSclI2CFmTimingReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CSclI2CFmTimingReg {
    #[inline(always)]
    fn default() -> I3CSclI2CFmTimingReg {
        <crate::RegValueT<I3CSclI2CFmTimingReg_SPEC> as RegisterValue<_>>::new(1048592)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CSclI3COdTimingReg_SPEC;
impl crate::sealed::RegSpec for I3CSclI3COdTimingReg_SPEC {
    type DataType = u32;
}

pub type I3CSclI3COdTimingReg = crate::RegValueT<I3CSclI3COdTimingReg_SPEC>;

impl I3CSclI3COdTimingReg {
    #[inline(always)]
    pub fn i3c_od_hcnt(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CSclI3COdTimingReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CSclI3COdTimingReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn i3c_od_lcnt(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CSclI3COdTimingReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CSclI3COdTimingReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CSclI3COdTimingReg {
    #[inline(always)]
    fn default() -> I3CSclI3COdTimingReg {
        <crate::RegValueT<I3CSclI3COdTimingReg_SPEC> as RegisterValue<_>>::new(655376)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CSclI3CPpTimingReg_SPEC;
impl crate::sealed::RegSpec for I3CSclI3CPpTimingReg_SPEC {
    type DataType = u32;
}

pub type I3CSclI3CPpTimingReg = crate::RegValueT<I3CSclI3CPpTimingReg_SPEC>;

impl I3CSclI3CPpTimingReg {
    #[inline(always)]
    pub fn i3c_pp_hcnt(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CSclI3CPpTimingReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CSclI3CPpTimingReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }

    #[inline(always)]
    pub fn i3c_pp_lcnt(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xff,
        1,
        0,
        u8,
        u8,
        I3CSclI3CPpTimingReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            u8,
            u8,
            I3CSclI3CPpTimingReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CSclI3CPpTimingReg {
    #[inline(always)]
    fn default() -> I3CSclI3CPpTimingReg {
        <crate::RegValueT<I3CSclI3CPpTimingReg_SPEC> as RegisterValue<_>>::new(655370)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CSdaHoldDlyTimingReg_SPEC;
impl crate::sealed::RegSpec for I3CSdaHoldDlyTimingReg_SPEC {
    type DataType = u32;
}

pub type I3CSdaHoldDlyTimingReg = crate::RegValueT<I3CSdaHoldDlyTimingReg_SPEC>;

impl I3CSdaHoldDlyTimingReg {
    #[inline(always)]
    pub fn sda_tx_hold(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7,
        1,
        0,
        u8,
        u8,
        I3CSdaHoldDlyTimingReg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7,
            1,
            0,
            u8,
            u8,
            I3CSdaHoldDlyTimingReg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CSdaHoldDlyTimingReg {
    #[inline(always)]
    fn default() -> I3CSdaHoldDlyTimingReg {
        <crate::RegValueT<I3CSdaHoldDlyTimingReg_SPEC> as RegisterValue<_>>::new(65536)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CSlvEventStatusReg_SPEC;
impl crate::sealed::RegSpec for I3CSlvEventStatusReg_SPEC {
    type DataType = u32;
}

pub type I3CSlvEventStatusReg = crate::RegValueT<I3CSlvEventStatusReg_SPEC>;

impl I3CSlvEventStatusReg {
    #[inline(always)]
    pub fn mwl_updated(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, I3CSlvEventStatusReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<7,1,0,I3CSlvEventStatusReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn mrl_updated(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, I3CSlvEventStatusReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterFieldBool::<6,1,0,I3CSlvEventStatusReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[inline(always)]
    pub fn activity_state(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        u8,
        u8,
        I3CSlvEventStatusReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            u8,
            u8,
            I3CSlvEventStatusReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CSlvEventStatusReg {
    #[inline(always)]
    fn default() -> I3CSlvEventStatusReg {
        <crate::RegValueT<I3CSlvEventStatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CVendorSpecificRegPtrReg_SPEC;
impl crate::sealed::RegSpec for I3CVendorSpecificRegPtrReg_SPEC {
    type DataType = u32;
}

pub type I3CVendorSpecificRegPtrReg = crate::RegValueT<I3CVendorSpecificRegPtrReg_SPEC>;

impl I3CVendorSpecificRegPtrReg {
    #[inline(always)]
    pub fn p_vendor_reg_start_addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffff,
        1,
        0,
        u16,
        u16,
        I3CVendorSpecificRegPtrReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffff,
            1,
            0,
            u16,
            u16,
            I3CVendorSpecificRegPtrReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CVendorSpecificRegPtrReg {
    #[inline(always)]
    fn default() -> I3CVendorSpecificRegPtrReg {
        <crate::RegValueT<I3CVendorSpecificRegPtrReg_SPEC> as RegisterValue<_>>::new(176)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CVerIdReg_SPEC;
impl crate::sealed::RegSpec for I3CVerIdReg_SPEC {
    type DataType = u32;
}

pub type I3CVerIdReg = crate::RegValueT<I3CVerIdReg_SPEC>;

impl I3CVerIdReg {
    #[inline(always)]
    pub fn i3c_ver_id(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        I3CVerIdReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            I3CVerIdReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CVerIdReg {
    #[inline(always)]
    fn default() -> I3CVerIdReg {
        <crate::RegValueT<I3CVerIdReg_SPEC> as RegisterValue<_>>::new(825241642)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3CVerTypeReg_SPEC;
impl crate::sealed::RegSpec for I3CVerTypeReg_SPEC {
    type DataType = u32;
}

pub type I3CVerTypeReg = crate::RegValueT<I3CVerTypeReg_SPEC>;

impl I3CVerTypeReg {
    #[inline(always)]
    pub fn i3c_ver_type(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        u32,
        I3CVerTypeReg_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            u32,
            I3CVerTypeReg_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for I3CVerTypeReg {
    #[inline(always)]
    fn default() -> I3CVerTypeReg {
        <crate::RegValueT<I3CVerTypeReg_SPEC> as RegisterValue<_>>::new(1818439731)
    }
}
