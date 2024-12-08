///Register `CH6_CTRL` reader
pub type R = crate::R<CH6_CTRLrs>;
///Register `CH6_CTRL` writer
pub type W = crate::W<CH6_CTRLrs>;
///DMA Structure Type
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STRUCTTYPE {
    ///0: DMA transfer structure type selected.
    Transfer = 0,
    ///1: Synchronization structure type selected.
    Synchronize = 1,
    ///2: Write immediate value structure type selected.
    Write = 2,
}
impl From<STRUCTTYPE> for u8 {
    #[inline(always)]
    fn from(variant: STRUCTTYPE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for STRUCTTYPE {
    type Ux = u8;
}
impl crate::IsEnum for STRUCTTYPE {}
///Field `STRUCTTYPE` reader - DMA Structure Type
pub type StructtypeR = crate::FieldReader<STRUCTTYPE>;
impl StructtypeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<STRUCTTYPE> {
        match self.bits {
            0 => Some(STRUCTTYPE::Transfer),
            1 => Some(STRUCTTYPE::Synchronize),
            2 => Some(STRUCTTYPE::Write),
            _ => None,
        }
    }
    ///DMA transfer structure type selected.
    #[inline(always)]
    pub fn is_transfer(&self) -> bool {
        *self == STRUCTTYPE::Transfer
    }
    ///Synchronization structure type selected.
    #[inline(always)]
    pub fn is_synchronize(&self) -> bool {
        *self == STRUCTTYPE::Synchronize
    }
    ///Write immediate value structure type selected.
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        *self == STRUCTTYPE::Write
    }
}
///Field `STRUCTREQ` writer - Structure DMA Transfer Request
pub type StructreqW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XFERCNT` reader - DMA Unit Data Transfer Count
pub type XfercntR = crate::FieldReader<u16>;
///Field `XFERCNT` writer - DMA Unit Data Transfer Count
pub type XfercntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `BYTESWAP` reader - Endian Byte Swap
pub type ByteswapR = crate::BitReader;
///Field `BYTESWAP` writer - Endian Byte Swap
pub type ByteswapW<'a, REG> = crate::BitWriter<'a, REG>;
///Block Transfer Size
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BLOCKSIZE {
    ///0: One unit transfer per arbitration
    Unit1 = 0,
    ///1: Two unit transfers per arbitration
    Unit2 = 1,
    ///2: Three unit transfers per arbitration
    Unit3 = 2,
    ///3: Four unit transfers per arbitration
    Unit4 = 3,
    ///4: Six unit transfers per arbitration
    Unit6 = 4,
    ///5: Eight unit transfers per arbitration
    Unit8 = 5,
    ///7: Sixteen unit transfers per arbitration
    Unit16 = 7,
    ///9: 32 unit transfers per arbitration
    Unit32 = 9,
    ///10: 64 unit transfers per arbitration
    Unit64 = 10,
    ///11: 128 unit transfers per arbitration
    Unit128 = 11,
    ///12: 256 unit transfers per arbitration
    Unit256 = 12,
    ///13: 512 unit transfers per arbitration
    Unit512 = 13,
    ///14: 1024 unit transfers per arbitration
    Unit1024 = 14,
    ///15: Transfer all units as specified by the XFRCNT field
    All = 15,
}
impl From<BLOCKSIZE> for u8 {
    #[inline(always)]
    fn from(variant: BLOCKSIZE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BLOCKSIZE {
    type Ux = u8;
}
impl crate::IsEnum for BLOCKSIZE {}
///Field `BLOCKSIZE` reader - Block Transfer Size
pub type BlocksizeR = crate::FieldReader<BLOCKSIZE>;
impl BlocksizeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<BLOCKSIZE> {
        match self.bits {
            0 => Some(BLOCKSIZE::Unit1),
            1 => Some(BLOCKSIZE::Unit2),
            2 => Some(BLOCKSIZE::Unit3),
            3 => Some(BLOCKSIZE::Unit4),
            4 => Some(BLOCKSIZE::Unit6),
            5 => Some(BLOCKSIZE::Unit8),
            7 => Some(BLOCKSIZE::Unit16),
            9 => Some(BLOCKSIZE::Unit32),
            10 => Some(BLOCKSIZE::Unit64),
            11 => Some(BLOCKSIZE::Unit128),
            12 => Some(BLOCKSIZE::Unit256),
            13 => Some(BLOCKSIZE::Unit512),
            14 => Some(BLOCKSIZE::Unit1024),
            15 => Some(BLOCKSIZE::All),
            _ => None,
        }
    }
    ///One unit transfer per arbitration
    #[inline(always)]
    pub fn is_unit1(&self) -> bool {
        *self == BLOCKSIZE::Unit1
    }
    ///Two unit transfers per arbitration
    #[inline(always)]
    pub fn is_unit2(&self) -> bool {
        *self == BLOCKSIZE::Unit2
    }
    ///Three unit transfers per arbitration
    #[inline(always)]
    pub fn is_unit3(&self) -> bool {
        *self == BLOCKSIZE::Unit3
    }
    ///Four unit transfers per arbitration
    #[inline(always)]
    pub fn is_unit4(&self) -> bool {
        *self == BLOCKSIZE::Unit4
    }
    ///Six unit transfers per arbitration
    #[inline(always)]
    pub fn is_unit6(&self) -> bool {
        *self == BLOCKSIZE::Unit6
    }
    ///Eight unit transfers per arbitration
    #[inline(always)]
    pub fn is_unit8(&self) -> bool {
        *self == BLOCKSIZE::Unit8
    }
    ///Sixteen unit transfers per arbitration
    #[inline(always)]
    pub fn is_unit16(&self) -> bool {
        *self == BLOCKSIZE::Unit16
    }
    ///32 unit transfers per arbitration
    #[inline(always)]
    pub fn is_unit32(&self) -> bool {
        *self == BLOCKSIZE::Unit32
    }
    ///64 unit transfers per arbitration
    #[inline(always)]
    pub fn is_unit64(&self) -> bool {
        *self == BLOCKSIZE::Unit64
    }
    ///128 unit transfers per arbitration
    #[inline(always)]
    pub fn is_unit128(&self) -> bool {
        *self == BLOCKSIZE::Unit128
    }
    ///256 unit transfers per arbitration
    #[inline(always)]
    pub fn is_unit256(&self) -> bool {
        *self == BLOCKSIZE::Unit256
    }
    ///512 unit transfers per arbitration
    #[inline(always)]
    pub fn is_unit512(&self) -> bool {
        *self == BLOCKSIZE::Unit512
    }
    ///1024 unit transfers per arbitration
    #[inline(always)]
    pub fn is_unit1024(&self) -> bool {
        *self == BLOCKSIZE::Unit1024
    }
    ///Transfer all units as specified by the XFRCNT field
    #[inline(always)]
    pub fn is_all(&self) -> bool {
        *self == BLOCKSIZE::All
    }
}
///Field `BLOCKSIZE` writer - Block Transfer Size
pub type BlocksizeW<'a, REG> = crate::FieldWriter<'a, REG, 4, BLOCKSIZE>;
impl<'a, REG> BlocksizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///One unit transfer per arbitration
    #[inline(always)]
    pub fn unit1(self) -> &'a mut crate::W<REG> {
        self.variant(BLOCKSIZE::Unit1)
    }
    ///Two unit transfers per arbitration
    #[inline(always)]
    pub fn unit2(self) -> &'a mut crate::W<REG> {
        self.variant(BLOCKSIZE::Unit2)
    }
    ///Three unit transfers per arbitration
    #[inline(always)]
    pub fn unit3(self) -> &'a mut crate::W<REG> {
        self.variant(BLOCKSIZE::Unit3)
    }
    ///Four unit transfers per arbitration
    #[inline(always)]
    pub fn unit4(self) -> &'a mut crate::W<REG> {
        self.variant(BLOCKSIZE::Unit4)
    }
    ///Six unit transfers per arbitration
    #[inline(always)]
    pub fn unit6(self) -> &'a mut crate::W<REG> {
        self.variant(BLOCKSIZE::Unit6)
    }
    ///Eight unit transfers per arbitration
    #[inline(always)]
    pub fn unit8(self) -> &'a mut crate::W<REG> {
        self.variant(BLOCKSIZE::Unit8)
    }
    ///Sixteen unit transfers per arbitration
    #[inline(always)]
    pub fn unit16(self) -> &'a mut crate::W<REG> {
        self.variant(BLOCKSIZE::Unit16)
    }
    ///32 unit transfers per arbitration
    #[inline(always)]
    pub fn unit32(self) -> &'a mut crate::W<REG> {
        self.variant(BLOCKSIZE::Unit32)
    }
    ///64 unit transfers per arbitration
    #[inline(always)]
    pub fn unit64(self) -> &'a mut crate::W<REG> {
        self.variant(BLOCKSIZE::Unit64)
    }
    ///128 unit transfers per arbitration
    #[inline(always)]
    pub fn unit128(self) -> &'a mut crate::W<REG> {
        self.variant(BLOCKSIZE::Unit128)
    }
    ///256 unit transfers per arbitration
    #[inline(always)]
    pub fn unit256(self) -> &'a mut crate::W<REG> {
        self.variant(BLOCKSIZE::Unit256)
    }
    ///512 unit transfers per arbitration
    #[inline(always)]
    pub fn unit512(self) -> &'a mut crate::W<REG> {
        self.variant(BLOCKSIZE::Unit512)
    }
    ///1024 unit transfers per arbitration
    #[inline(always)]
    pub fn unit1024(self) -> &'a mut crate::W<REG> {
        self.variant(BLOCKSIZE::Unit1024)
    }
    ///Transfer all units as specified by the XFRCNT field
    #[inline(always)]
    pub fn all(self) -> &'a mut crate::W<REG> {
        self.variant(BLOCKSIZE::All)
    }
}
///Field `DONEIFSEN` reader - DMA Operation Done Interrupt Flag Set Enable
pub type DoneifsenR = crate::BitReader;
///Field `DONEIFSEN` writer - DMA Operation Done Interrupt Flag Set Enable
pub type DoneifsenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `REQMODE` reader - DMA Request Transfer Mode Select
pub type ReqmodeR = crate::BitReader;
///Field `REQMODE` writer - DMA Request Transfer Mode Select
pub type ReqmodeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DECLOOPCNT` reader - Decrement Loop Count
pub type DecloopcntR = crate::BitReader;
///Field `DECLOOPCNT` writer - Decrement Loop Count
pub type DecloopcntW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IGNORESREQ` reader - Ignore Sreq
pub type IgnoresreqR = crate::BitReader;
///Field `IGNORESREQ` writer - Ignore Sreq
pub type IgnoresreqW<'a, REG> = crate::BitWriter<'a, REG>;
///Source Address Increment Size
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SRCINC {
    ///0: Increment source address by one unit data size after each read
    One = 0,
    ///1: Increment source address by two unit data sizes after each read
    Two = 1,
    ///2: Increment source address by four unit data sizes after each read
    Four = 2,
    ///3: Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO.
    None = 3,
}
impl From<SRCINC> for u8 {
    #[inline(always)]
    fn from(variant: SRCINC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SRCINC {
    type Ux = u8;
}
impl crate::IsEnum for SRCINC {}
///Field `SRCINC` reader - Source Address Increment Size
pub type SrcincR = crate::FieldReader<SRCINC>;
impl SrcincR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SRCINC {
        match self.bits {
            0 => SRCINC::One,
            1 => SRCINC::Two,
            2 => SRCINC::Four,
            3 => SRCINC::None,
            _ => unreachable!(),
        }
    }
    ///Increment source address by one unit data size after each read
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == SRCINC::One
    }
    ///Increment source address by two unit data sizes after each read
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == SRCINC::Two
    }
    ///Increment source address by four unit data sizes after each read
    #[inline(always)]
    pub fn is_four(&self) -> bool {
        *self == SRCINC::Four
    }
    ///Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO.
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SRCINC::None
    }
}
///Field `SRCINC` writer - Source Address Increment Size
pub type SrcincW<'a, REG> = crate::FieldWriter<'a, REG, 2, SRCINC, crate::Safe>;
impl<'a, REG> SrcincW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Increment source address by one unit data size after each read
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(SRCINC::One)
    }
    ///Increment source address by two unit data sizes after each read
    #[inline(always)]
    pub fn two(self) -> &'a mut crate::W<REG> {
        self.variant(SRCINC::Two)
    }
    ///Increment source address by four unit data sizes after each read
    #[inline(always)]
    pub fn four(self) -> &'a mut crate::W<REG> {
        self.variant(SRCINC::Four)
    }
    ///Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO.
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(SRCINC::None)
    }
}
///Unit Data Transfer Size
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SIZE {
    ///0: Each unit transfer is a byte
    Byte = 0,
    ///1: Each unit transfer is a half-word
    Halfword = 1,
    ///2: Each unit transfer is a word
    Word = 2,
}
impl From<SIZE> for u8 {
    #[inline(always)]
    fn from(variant: SIZE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SIZE {
    type Ux = u8;
}
impl crate::IsEnum for SIZE {}
///Field `SIZE` reader - Unit Data Transfer Size
pub type SizeR = crate::FieldReader<SIZE>;
impl SizeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SIZE> {
        match self.bits {
            0 => Some(SIZE::Byte),
            1 => Some(SIZE::Halfword),
            2 => Some(SIZE::Word),
            _ => None,
        }
    }
    ///Each unit transfer is a byte
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == SIZE::Byte
    }
    ///Each unit transfer is a half-word
    #[inline(always)]
    pub fn is_halfword(&self) -> bool {
        *self == SIZE::Halfword
    }
    ///Each unit transfer is a word
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        *self == SIZE::Word
    }
}
///Field `SIZE` writer - Unit Data Transfer Size
pub type SizeW<'a, REG> = crate::FieldWriter<'a, REG, 2, SIZE>;
impl<'a, REG> SizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Each unit transfer is a byte
    #[inline(always)]
    pub fn byte(self) -> &'a mut crate::W<REG> {
        self.variant(SIZE::Byte)
    }
    ///Each unit transfer is a half-word
    #[inline(always)]
    pub fn halfword(self) -> &'a mut crate::W<REG> {
        self.variant(SIZE::Halfword)
    }
    ///Each unit transfer is a word
    #[inline(always)]
    pub fn word(self) -> &'a mut crate::W<REG> {
        self.variant(SIZE::Word)
    }
}
///Destination Address Increment Size
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DSTINC {
    ///0: Increment destination address by one unit data size after each write
    One = 0,
    ///1: Increment destination address by two unit data sizes after each write
    Two = 1,
    ///2: Increment destination address by four unit data sizes after each write
    Four = 2,
    ///3: Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO.
    None = 3,
}
impl From<DSTINC> for u8 {
    #[inline(always)]
    fn from(variant: DSTINC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DSTINC {
    type Ux = u8;
}
impl crate::IsEnum for DSTINC {}
///Field `DSTINC` reader - Destination Address Increment Size
pub type DstincR = crate::FieldReader<DSTINC>;
impl DstincR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DSTINC {
        match self.bits {
            0 => DSTINC::One,
            1 => DSTINC::Two,
            2 => DSTINC::Four,
            3 => DSTINC::None,
            _ => unreachable!(),
        }
    }
    ///Increment destination address by one unit data size after each write
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == DSTINC::One
    }
    ///Increment destination address by two unit data sizes after each write
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == DSTINC::Two
    }
    ///Increment destination address by four unit data sizes after each write
    #[inline(always)]
    pub fn is_four(&self) -> bool {
        *self == DSTINC::Four
    }
    ///Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO.
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == DSTINC::None
    }
}
///Field `DSTINC` writer - Destination Address Increment Size
pub type DstincW<'a, REG> = crate::FieldWriter<'a, REG, 2, DSTINC, crate::Safe>;
impl<'a, REG> DstincW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Increment destination address by one unit data size after each write
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(DSTINC::One)
    }
    ///Increment destination address by two unit data sizes after each write
    #[inline(always)]
    pub fn two(self) -> &'a mut crate::W<REG> {
        self.variant(DSTINC::Two)
    }
    ///Increment destination address by four unit data sizes after each write
    #[inline(always)]
    pub fn four(self) -> &'a mut crate::W<REG> {
        self.variant(DSTINC::Four)
    }
    ///Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO.
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(DSTINC::None)
    }
}
///Field `SRCMODE` reader - Source Addressing Mode
pub type SrcmodeR = crate::BitReader;
///Field `DSTMODE` reader - Destination Addressing Mode
pub type DstmodeR = crate::BitReader;
impl R {
    ///Bits 0:1 - DMA Structure Type
    #[inline(always)]
    pub fn structtype(&self) -> StructtypeR {
        StructtypeR::new((self.bits & 3) as u8)
    }
    ///Bits 4:14 - DMA Unit Data Transfer Count
    #[inline(always)]
    pub fn xfercnt(&self) -> XfercntR {
        XfercntR::new(((self.bits >> 4) & 0x07ff) as u16)
    }
    ///Bit 15 - Endian Byte Swap
    #[inline(always)]
    pub fn byteswap(&self) -> ByteswapR {
        ByteswapR::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:19 - Block Transfer Size
    #[inline(always)]
    pub fn blocksize(&self) -> BlocksizeR {
        BlocksizeR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bit 20 - DMA Operation Done Interrupt Flag Set Enable
    #[inline(always)]
    pub fn doneifsen(&self) -> DoneifsenR {
        DoneifsenR::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - DMA Request Transfer Mode Select
    #[inline(always)]
    pub fn reqmode(&self) -> ReqmodeR {
        ReqmodeR::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Decrement Loop Count
    #[inline(always)]
    pub fn decloopcnt(&self) -> DecloopcntR {
        DecloopcntR::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Ignore Sreq
    #[inline(always)]
    pub fn ignoresreq(&self) -> IgnoresreqR {
        IgnoresreqR::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 24:25 - Source Address Increment Size
    #[inline(always)]
    pub fn srcinc(&self) -> SrcincR {
        SrcincR::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 26:27 - Unit Data Transfer Size
    #[inline(always)]
    pub fn size(&self) -> SizeR {
        SizeR::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bits 28:29 - Destination Address Increment Size
    #[inline(always)]
    pub fn dstinc(&self) -> DstincR {
        DstincR::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bit 30 - Source Addressing Mode
    #[inline(always)]
    pub fn srcmode(&self) -> SrcmodeR {
        SrcmodeR::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Destination Addressing Mode
    #[inline(always)]
    pub fn dstmode(&self) -> DstmodeR {
        DstmodeR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH6_CTRL")
            .field("structtype", &self.structtype())
            .field("xfercnt", &self.xfercnt())
            .field("byteswap", &self.byteswap())
            .field("blocksize", &self.blocksize())
            .field("doneifsen", &self.doneifsen())
            .field("reqmode", &self.reqmode())
            .field("decloopcnt", &self.decloopcnt())
            .field("ignoresreq", &self.ignoresreq())
            .field("srcinc", &self.srcinc())
            .field("size", &self.size())
            .field("dstinc", &self.dstinc())
            .field("srcmode", &self.srcmode())
            .field("dstmode", &self.dstmode())
            .finish()
    }
}
impl W {
    ///Bit 3 - Structure DMA Transfer Request
    #[inline(always)]
    #[must_use]
    pub fn structreq(&mut self) -> StructreqW<CH6_CTRLrs> {
        StructreqW::new(self, 3)
    }
    ///Bits 4:14 - DMA Unit Data Transfer Count
    #[inline(always)]
    #[must_use]
    pub fn xfercnt(&mut self) -> XfercntW<CH6_CTRLrs> {
        XfercntW::new(self, 4)
    }
    ///Bit 15 - Endian Byte Swap
    #[inline(always)]
    #[must_use]
    pub fn byteswap(&mut self) -> ByteswapW<CH6_CTRLrs> {
        ByteswapW::new(self, 15)
    }
    ///Bits 16:19 - Block Transfer Size
    #[inline(always)]
    #[must_use]
    pub fn blocksize(&mut self) -> BlocksizeW<CH6_CTRLrs> {
        BlocksizeW::new(self, 16)
    }
    ///Bit 20 - DMA Operation Done Interrupt Flag Set Enable
    #[inline(always)]
    #[must_use]
    pub fn doneifsen(&mut self) -> DoneifsenW<CH6_CTRLrs> {
        DoneifsenW::new(self, 20)
    }
    ///Bit 21 - DMA Request Transfer Mode Select
    #[inline(always)]
    #[must_use]
    pub fn reqmode(&mut self) -> ReqmodeW<CH6_CTRLrs> {
        ReqmodeW::new(self, 21)
    }
    ///Bit 22 - Decrement Loop Count
    #[inline(always)]
    #[must_use]
    pub fn decloopcnt(&mut self) -> DecloopcntW<CH6_CTRLrs> {
        DecloopcntW::new(self, 22)
    }
    ///Bit 23 - Ignore Sreq
    #[inline(always)]
    #[must_use]
    pub fn ignoresreq(&mut self) -> IgnoresreqW<CH6_CTRLrs> {
        IgnoresreqW::new(self, 23)
    }
    ///Bits 24:25 - Source Address Increment Size
    #[inline(always)]
    #[must_use]
    pub fn srcinc(&mut self) -> SrcincW<CH6_CTRLrs> {
        SrcincW::new(self, 24)
    }
    ///Bits 26:27 - Unit Data Transfer Size
    #[inline(always)]
    #[must_use]
    pub fn size(&mut self) -> SizeW<CH6_CTRLrs> {
        SizeW::new(self, 26)
    }
    ///Bits 28:29 - Destination Address Increment Size
    #[inline(always)]
    #[must_use]
    pub fn dstinc(&mut self) -> DstincW<CH6_CTRLrs> {
        DstincW::new(self, 28)
    }
}
///Channel Descriptor Control Word Register
///
///You can [`read`](crate::Reg::read) this register and get [`ch6_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CH6_CTRLrs;
impl crate::RegisterSpec for CH6_CTRLrs {
    type Ux = u32;
}
///`read()` method returns [`ch6_ctrl::R`](R) reader structure
impl crate::Readable for CH6_CTRLrs {}
///`write(|w| ..)` method takes [`ch6_ctrl::W`](W) writer structure
impl crate::Writable for CH6_CTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CH6_CTRL to value 0
impl crate::Resettable for CH6_CTRLrs {
    const RESET_VALUE: u32 = 0;
}
