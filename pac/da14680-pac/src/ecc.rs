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
// Generated from SVD 1.2, with svd2pac 0.6.0 on Thu, 24 Jul 2025 04:44:57 +0000

#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"ECC registers"]
unsafe impl ::core::marker::Send for super::Ecc {}
unsafe impl ::core::marker::Sync for super::Ecc {}
impl super::Ecc {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn _svd2pac_as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    #[doc = "Command register"]
    #[inline(always)]
    pub const fn ecc_command_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EccCommandReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EccCommandReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(4usize),
            )
        }
    }

    #[doc = "Configuration register"]
    #[inline(always)]
    pub const fn ecc_config_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EccConfigReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EccConfigReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(0usize),
            )
        }
    }

    #[doc = "Control register"]
    #[inline(always)]
    pub const fn ecc_control_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EccControlReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EccControlReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(8usize),
            )
        }
    }

    #[doc = "Status register"]
    #[inline(always)]
    pub const fn ecc_status_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EccStatusReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EccStatusReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(12usize),
            )
        }
    }

    #[doc = "Version register"]
    #[inline(always)]
    pub const fn ecc_version_reg(
        &self,
    ) -> &'static crate::common::Reg<self::EccVersionReg_SPEC, crate::common::RW> {
        unsafe {
            crate::common::Reg::<self::EccVersionReg_SPEC, crate::common::RW>::from_ptr(
                self._svd2pac_as_ptr().add(16usize),
            )
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EccCommandReg_SPEC;
impl crate::sealed::RegSpec for EccCommandReg_SPEC {
    type DataType = u32;
}

#[doc = "Command register"]
pub type EccCommandReg = crate::RegValueT<EccCommandReg_SPEC>;

impl EccCommandReg {
    #[doc = "This bit indicates if the IP has to calculate R mod N for the next operation. This bit must be set to 1 when a new prime number has been programmed. This bit is automatically cleared when R mod N has been calculated.\n\'0\': no effect\n\'1\': forces the IP to re-calculate R mod N"]
    #[inline(always)]
    pub fn ecc_calcr2(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, EccCommandReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,EccCommandReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Sign of parameter B in equation y2=x3+Ax+B\n\'0\': B is positive\n\'1\': B is negative"]
    #[inline(always)]
    pub fn ecc_signb(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, EccCommandReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30,1,0,EccCommandReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Sign of parameter A in equation y2=x3+Ax+B\n\'0\': A is positive\n\'1\': A is negative"]
    #[inline(always)]
    pub fn ecc_signa(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, EccCommandReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29,1,0,EccCommandReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "This field defines the size (= number of 64-bit double words) of the operands for the current operation. Possible values are limited by the generic parameter g_Log2MaxDataSize that defines the max space allocated or reserved to each operand.\n\nArbitrary Data/Key size from 128 up to 2566 are supported:\n0x02 (02d) -> 128-bit Data/Key size\n0x03 (02d) -> 256-bit Data/Key size\nECC-ECDSA - Prime Field F(p)\n0x03 -> 192-bit (Curve P-192)\n0x04 -> 256-bit (Curves P-224 & P-256)\nECC-ECDSA - Binary Field F(2m)\n0x03 -> 192-bit (Curve K-163)\n0x04 -> 256-bit (Curve K-233)\n- 4 Xers: 0x01, 0x02, 0x4, 0x6 -> 64, 128 & multiples of 128 bits"]
    #[inline(always)]
    pub fn ecc_sizeofoperands(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, EccCommandReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,EccCommandReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "\'0\': Field is F(p)\n\'1\': Field is F(2m)"]
    #[inline(always)]
    pub fn ecc_field(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, EccCommandReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,EccCommandReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "Primitive Arithmetic Operations F(p) & F(2m)\n\\[6:4\\] = 0x0\n\\[3:0\\] =\n0x0 -> Reserved\n0x1 -> Modular Addition\n0x2 -> Modular Subtraction\n0x3 -> Modular Multiplication (Odd N)\n0x4 -> Modular Reduction (Odd N)\n0x5 -> Modular Division (Odd N)\n0x6 -> Modular Inversion (Odd N)\n0x7 -> Reserved\n0x8 -> Multiplication\n0x9 -> Modular Inversion (Even N)\n0xA -> Modular Reduction (Even N)\nothers -> Reserved\nC = A + B mod N\nC = A - B mod N\nC = A * B mod N\nC = B mod N\nC = A/B mod N\nC = 1/B mod N\nC = A * B\nC = 1/B mod N\nC = B mod N\nHigh-level RSA, CRT & DSA Operations - F(p) only\n(\\[7\\] forced to 0)\n\\[6:4\\] = 0x1\n\\[3:0\\] =\n0x0 -> MulModN\n0x1 -> MulAddN\n0x2 -> ECMQV (part1)\nothers -> Reserved\n\nPrimitive ECC & Check Point Operations F(p) & F(2m)\n\\[6:4\\] = 0x2\n\\[3:0\\] =\n0x0 -> Point Doubling (Projective Coord.)\n0x1 -> ptAdd3\n0x2 -> GenSessionKey\n0x3 -> Check_AB (ECDSA)\n0x4 -> Check_n (ECDSA)\n0x5 -> Check single value less than N\n0x6 -> Check_Point_On_Curve\n0x7-> Reserved\n0x8 -> Curve25519 point multiplication\n0x9 -> Ed25519 Check point on curve\n0xA -> Ed25519 ScalarMult\n0xB -> Ed25519 CheckValid\nothers -> Reserved\n\nHigh-level ECC ECDSA Operations F(p) & F(2m)\n\\[6:4\\] = 0x3\n\\[3:0\\] =\n0x0 -> ECMQV (part 2)\n0x1 -> Verify ZKP\n0x2 -> ECDSA Domain Parameters Validation\nothers -> Reserved\n\n\n\\[6:4\\]=0x4, 0x5, 0x6, 0x7 -> Reserved"]
    #[inline(always)]
    pub fn ecc_typeoperation(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, u8, EccCommandReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7f,1,0,u8,u8,EccCommandReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for EccCommandReg {
    #[inline(always)]
    fn default() -> EccCommandReg {
        <crate::RegValueT<EccCommandReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EccConfigReg_SPEC;
impl crate::sealed::RegSpec for EccConfigReg_SPEC {
    type DataType = u32;
}

#[doc = "Configuration register"]
pub type EccConfigReg = crate::RegValueT<EccConfigReg_SPEC>;

impl EccConfigReg {
    #[doc = "When executing primitive arithmetic operations, this pointer defines the location where the result will be stored in Memory."]
    #[inline(always)]
    pub fn ecc_opptrc(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, u8, EccConfigReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1f,1,0,u8,u8,EccConfigReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "When executing primitive arithmetic operations, this Pointer defines where operand B is located in memory."]
    #[inline(always)]
    pub fn ecc_opptrb(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, u8, EccConfigReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1f,1,0,u8,u8,EccConfigReg_SPEC,crate::common::RW>::from_register(self,0)
    }

    #[doc = "When executing primitive arithmetic operations, this Pointer defines where operand A is located in memory."]
    #[inline(always)]
    pub fn ecc_opptra(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, u8, EccConfigReg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,u8,u8,EccConfigReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for EccConfigReg {
    #[inline(always)]
    fn default() -> EccConfigReg {
        <crate::RegValueT<EccConfigReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EccControlReg_SPEC;
impl crate::sealed::RegSpec for EccControlReg_SPEC {
    type DataType = u32;
}

#[doc = "Control register"]
pub type EccControlReg = crate::RegValueT<EccControlReg_SPEC>;

impl EccControlReg {
    #[doc = "The Start signal is activated when all data and key inputs have been loaded in the external crypto memory and are available for processing. This signal is active high and is sampled on the rising edge of Clk.\nWhen this signal goes high, the PK Command present in the PK_CommandReg\\[\\] is initiated and executed. The PK_Start signal is ignored when the core is already processing data and is automatically cleared when the operation is finished"]
    #[inline(always)]
    pub fn ecc_start(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, EccControlReg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,EccControlReg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for EccControlReg {
    #[inline(always)]
    fn default() -> EccControlReg {
        <crate::RegValueT<EccControlReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EccStatusReg_SPEC;
impl crate::sealed::RegSpec for EccStatusReg_SPEC {
    type DataType = u32;
}

#[doc = "Status register"]
pub type EccStatusReg = crate::RegValueT<EccStatusReg_SPEC>;

impl EccStatusReg {
    #[doc = "This Status Signal indicates that the core is processing data. This signal is active high and goes low when the selected algorithm is finished."]
    #[inline(always)]
    pub fn ecc_busy(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, EccStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16,1,0,EccStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "After the Miller-Rabin Primality test, this flag is:\n- set to 0 when the random number under test is probably prime\n- cleared to 1 when the random number under test is composite"]
    #[inline(always)]
    pub fn ecc_primalitytestresult(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, EccStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12,1,0,EccStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "This flag is set to 1 when executing a modular inversion (PK_CommandReg\\[3:0\\] = 0x6 or 0x9) if the operand is not invertible."]
    #[inline(always)]
    pub fn ecc_notinvertible(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, EccStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11,1,0,EccStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Status signal set to 1 when parameters A and B are not valid, i.e 4A+ 27B = 0. This flag is updated after execution of the command Check_AB."]
    #[inline(always)]
    pub fn ecc_param_ab_notvalid(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, EccStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,EccStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "This flag indicates if the signature can be accepted or must be rejected.\nThis flag is set to 1 when the signature is not valid and is updated after execution of the command ECDSA_Generation, ECDSA_Verification, DSA_Generation, DSA_Verification."]
    #[inline(always)]
    pub fn ecc_signature_notvalid(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, EccStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9,1,0,EccStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Status signal set to 1 when Parameter n is not valid.\nThis flag is updated after execution of the command Check_n."]
    #[inline(always)]
    pub fn ecc_param_n_notvalid(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, EccStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,EccStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Status signal set to 1 when couple x, y is not valid (i.e. not smaller than the prime).\nThis flag is updated after execution of the command Check_Couple_Less_Prime."]
    #[inline(always)]
    pub fn ecc_couple_notvalid(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, EccStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,EccStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Status signal set to 1 when Point Px is at the infinity.\nThis flag is updated after execution of an ECC operation."]
    #[inline(always)]
    pub fn ecc_point_px_atinfinity(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, EccStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,EccStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Status signal set to 1 when Point Px is not on the defined EC. This flag is updated after execution of the command Check_Point_OnCurve."]
    #[inline(always)]
    pub fn ecc_point_px_notoncurve(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, EccStatusReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,EccStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Address of the last Point detected as Not On Curve, Not Valid or at the infinity."]
    #[inline(always)]
    pub fn ecc_fail_address(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, u8, EccStatusReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xf,1,0,u8,u8,EccStatusReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for EccStatusReg {
    #[inline(always)]
    fn default() -> EccStatusReg {
        <crate::RegValueT<EccStatusReg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EccVersionReg_SPEC;
impl crate::sealed::RegSpec for EccVersionReg_SPEC {
    type DataType = u32;
}

#[doc = "Version register"]
pub type EccVersionReg = crate::RegValueT<EccVersionReg_SPEC>;

impl EccVersionReg {
    #[doc = "Version of IP to be read via CPU interface."]
    #[inline(always)]
    pub fn ecc_hvn(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, u8, EccVersionReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8,u8,EccVersionReg_SPEC,crate::common::R>::from_register(self,0)
    }

    #[doc = "Version of Crypto code to be read via CPU interface.Note that this should be read before ECC is used since it corrupts its contents."]
    #[inline(always)]
    pub fn ecc_svn(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, u8, EccVersionReg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8,u8,EccVersionReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for EccVersionReg {
    #[inline(always)]
    fn default() -> EccVersionReg {
        <crate::RegValueT<EccVersionReg_SPEC> as RegisterValue<_>>::new(1024)
    }
}
