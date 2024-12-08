///Register `CSTATUS` reader
pub type R = crate::R<CSTATUSrs>;
///Selected ALU Operand 0
///
///Value on reset: 1
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum V0 {
    ///0: `0`
    Ddata0 = 0,
    ///1: `1`
    Ddata1 = 1,
    ///2: `10`
    Ddata2 = 2,
    ///3: `11`
    Ddata3 = 3,
    ///4: `100`
    Ddata4 = 4,
    ///5: `101`
    Data0 = 5,
    ///6: `110`
    Data1 = 6,
    ///7: `111`
    Data2 = 7,
}
impl From<V0> for u8 {
    #[inline(always)]
    fn from(variant: V0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for V0 {
    type Ux = u8;
}
impl crate::IsEnum for V0 {}
///Field `V0` reader - Selected ALU Operand 0
pub type V0R = crate::FieldReader<V0>;
impl V0R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> V0 {
        match self.bits {
            0 => V0::Ddata0,
            1 => V0::Ddata1,
            2 => V0::Ddata2,
            3 => V0::Ddata3,
            4 => V0::Ddata4,
            5 => V0::Data0,
            6 => V0::Data1,
            7 => V0::Data2,
            _ => unreachable!(),
        }
    }
    ///`0`
    #[inline(always)]
    pub fn is_ddata0(&self) -> bool {
        *self == V0::Ddata0
    }
    ///`1`
    #[inline(always)]
    pub fn is_ddata1(&self) -> bool {
        *self == V0::Ddata1
    }
    ///`10`
    #[inline(always)]
    pub fn is_ddata2(&self) -> bool {
        *self == V0::Ddata2
    }
    ///`11`
    #[inline(always)]
    pub fn is_ddata3(&self) -> bool {
        *self == V0::Ddata3
    }
    ///`100`
    #[inline(always)]
    pub fn is_ddata4(&self) -> bool {
        *self == V0::Ddata4
    }
    ///`101`
    #[inline(always)]
    pub fn is_data0(&self) -> bool {
        *self == V0::Data0
    }
    ///`110`
    #[inline(always)]
    pub fn is_data1(&self) -> bool {
        *self == V0::Data1
    }
    ///`111`
    #[inline(always)]
    pub fn is_data2(&self) -> bool {
        *self == V0::Data2
    }
}
///Selected ALU Operand 1
///
///Value on reset: 2
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum V1 {
    ///0: `0`
    Ddata0 = 0,
    ///1: `1`
    Ddata1 = 1,
    ///2: `10`
    Ddata2 = 2,
    ///3: `11`
    Ddata3 = 3,
    ///4: `100`
    Ddata4 = 4,
    ///5: `101`
    Data0 = 5,
    ///6: `110`
    Data1 = 6,
    ///7: `111`
    Data2 = 7,
}
impl From<V1> for u8 {
    #[inline(always)]
    fn from(variant: V1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for V1 {
    type Ux = u8;
}
impl crate::IsEnum for V1 {}
///Field `V1` reader - Selected ALU Operand 1
pub type V1R = crate::FieldReader<V1>;
impl V1R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> V1 {
        match self.bits {
            0 => V1::Ddata0,
            1 => V1::Ddata1,
            2 => V1::Ddata2,
            3 => V1::Ddata3,
            4 => V1::Ddata4,
            5 => V1::Data0,
            6 => V1::Data1,
            7 => V1::Data2,
            _ => unreachable!(),
        }
    }
    ///`0`
    #[inline(always)]
    pub fn is_ddata0(&self) -> bool {
        *self == V1::Ddata0
    }
    ///`1`
    #[inline(always)]
    pub fn is_ddata1(&self) -> bool {
        *self == V1::Ddata1
    }
    ///`10`
    #[inline(always)]
    pub fn is_ddata2(&self) -> bool {
        *self == V1::Ddata2
    }
    ///`11`
    #[inline(always)]
    pub fn is_ddata3(&self) -> bool {
        *self == V1::Ddata3
    }
    ///`100`
    #[inline(always)]
    pub fn is_ddata4(&self) -> bool {
        *self == V1::Ddata4
    }
    ///`101`
    #[inline(always)]
    pub fn is_data0(&self) -> bool {
        *self == V1::Data0
    }
    ///`110`
    #[inline(always)]
    pub fn is_data1(&self) -> bool {
        *self == V1::Data1
    }
    ///`111`
    #[inline(always)]
    pub fn is_data2(&self) -> bool {
        *self == V1::Data2
    }
}
///Field `SEQPART` reader - Sequence Part
pub type SeqpartR = crate::BitReader;
///Field `SEQSKIP` reader - Sequence Skip Next Instruction
pub type SeqskipR = crate::BitReader;
///Field `SEQIP` reader - Sequence Next Instruction Pointer
pub type SeqipR = crate::FieldReader;
impl R {
    ///Bits 0:2 - Selected ALU Operand 0
    #[inline(always)]
    pub fn v0(&self) -> V0R {
        V0R::new((self.bits & 7) as u8)
    }
    ///Bits 8:10 - Selected ALU Operand 1
    #[inline(always)]
    pub fn v1(&self) -> V1R {
        V1R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bit 16 - Sequence Part
    #[inline(always)]
    pub fn seqpart(&self) -> SeqpartR {
        SeqpartR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Sequence Skip Next Instruction
    #[inline(always)]
    pub fn seqskip(&self) -> SeqskipR {
        SeqskipR::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bits 20:24 - Sequence Next Instruction Pointer
    #[inline(always)]
    pub fn seqip(&self) -> SeqipR {
        SeqipR::new(((self.bits >> 20) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSTATUS")
            .field("v0", &self.v0())
            .field("v1", &self.v1())
            .field("seqpart", &self.seqpart())
            .field("seqskip", &self.seqskip())
            .field("seqip", &self.seqip())
            .finish()
    }
}
///Control Status Register
///
///You can [`read`](crate::Reg::read) this register and get [`cstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CSTATUSrs;
impl crate::RegisterSpec for CSTATUSrs {
    type Ux = u32;
}
///`read()` method returns [`cstatus::R`](R) reader structure
impl crate::Readable for CSTATUSrs {}
///`reset()` method sets CSTATUS to value 0x0201
impl crate::Resettable for CSTATUSrs {
    const RESET_VALUE: u32 = 0x0201;
}
