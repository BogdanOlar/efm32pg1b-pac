#[doc = "Register `CH6_CTRL` reader"]
pub type R = crate::R<CH6_CTRLrs>;
#[doc = "Register `CH6_CTRL` writer"]
pub type W = crate::W<CH6_CTRLrs>;
#[doc = "Field `STRUCTTYPE` reader - DMA Structure Type"]
pub type STRUCTTYPE_R = crate::FieldReader<STRUCTTYPE>;
#[doc = "DMA Structure Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STRUCTTYPE {
    #[doc = "0: DMA transfer structure type selected."]
    Transfer = 0,
    #[doc = "1: Synchronization structure type selected."]
    Synchronize = 1,
    #[doc = "2: Write immediate value structure type selected."]
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
impl STRUCTTYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<STRUCTTYPE> {
        match self.bits {
            0 => Some(STRUCTTYPE::Transfer),
            1 => Some(STRUCTTYPE::Synchronize),
            2 => Some(STRUCTTYPE::Write),
            _ => None,
        }
    }
    #[doc = "DMA transfer structure type selected."]
    #[inline(always)]
    pub fn is_transfer(&self) -> bool {
        *self == STRUCTTYPE::Transfer
    }
    #[doc = "Synchronization structure type selected."]
    #[inline(always)]
    pub fn is_synchronize(&self) -> bool {
        *self == STRUCTTYPE::Synchronize
    }
    #[doc = "Write immediate value structure type selected."]
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        *self == STRUCTTYPE::Write
    }
}
#[doc = "Field `STRUCTREQ` writer - Structure DMA Transfer Request"]
pub type STRUCTREQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XFERCNT` reader - DMA Unit Data Transfer Count"]
pub type XFERCNT_R = crate::FieldReader<u16>;
#[doc = "Field `XFERCNT` writer - DMA Unit Data Transfer Count"]
pub type XFERCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `BYTESWAP` reader - Endian Byte Swap"]
pub type BYTESWAP_R = crate::BitReader;
#[doc = "Field `BYTESWAP` writer - Endian Byte Swap"]
pub type BYTESWAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLOCKSIZE` reader - Block Transfer Size"]
pub type BLOCKSIZE_R = crate::FieldReader<BLOCKSIZE>;
#[doc = "Block Transfer Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BLOCKSIZE {
    #[doc = "0: One unit transfer per arbitration"]
    Unit1 = 0,
    #[doc = "1: Two unit transfers per arbitration"]
    Unit2 = 1,
    #[doc = "2: Three unit transfers per arbitration"]
    Unit3 = 2,
    #[doc = "3: Four unit transfers per arbitration"]
    Unit4 = 3,
    #[doc = "4: Six unit transfers per arbitration"]
    Unit6 = 4,
    #[doc = "5: Eight unit transfers per arbitration"]
    Unit8 = 5,
    #[doc = "7: Sixteen unit transfers per arbitration"]
    Unit16 = 7,
    #[doc = "9: 32 unit transfers per arbitration"]
    Unit32 = 9,
    #[doc = "10: 64 unit transfers per arbitration"]
    Unit64 = 10,
    #[doc = "11: 128 unit transfers per arbitration"]
    Unit128 = 11,
    #[doc = "12: 256 unit transfers per arbitration"]
    Unit256 = 12,
    #[doc = "13: 512 unit transfers per arbitration"]
    Unit512 = 13,
    #[doc = "14: 1024 unit transfers per arbitration"]
    Unit1024 = 14,
    #[doc = "15: Transfer all units as specified by the XFRCNT field"]
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
impl BLOCKSIZE_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "One unit transfer per arbitration"]
    #[inline(always)]
    pub fn is_unit1(&self) -> bool {
        *self == BLOCKSIZE::Unit1
    }
    #[doc = "Two unit transfers per arbitration"]
    #[inline(always)]
    pub fn is_unit2(&self) -> bool {
        *self == BLOCKSIZE::Unit2
    }
    #[doc = "Three unit transfers per arbitration"]
    #[inline(always)]
    pub fn is_unit3(&self) -> bool {
        *self == BLOCKSIZE::Unit3
    }
    #[doc = "Four unit transfers per arbitration"]
    #[inline(always)]
    pub fn is_unit4(&self) -> bool {
        *self == BLOCKSIZE::Unit4
    }
    #[doc = "Six unit transfers per arbitration"]
    #[inline(always)]
    pub fn is_unit6(&self) -> bool {
        *self == BLOCKSIZE::Unit6
    }
    #[doc = "Eight unit transfers per arbitration"]
    #[inline(always)]
    pub fn is_unit8(&self) -> bool {
        *self == BLOCKSIZE::Unit8
    }
    #[doc = "Sixteen unit transfers per arbitration"]
    #[inline(always)]
    pub fn is_unit16(&self) -> bool {
        *self == BLOCKSIZE::Unit16
    }
    #[doc = "32 unit transfers per arbitration"]
    #[inline(always)]
    pub fn is_unit32(&self) -> bool {
        *self == BLOCKSIZE::Unit32
    }
    #[doc = "64 unit transfers per arbitration"]
    #[inline(always)]
    pub fn is_unit64(&self) -> bool {
        *self == BLOCKSIZE::Unit64
    }
    #[doc = "128 unit transfers per arbitration"]
    #[inline(always)]
    pub fn is_unit128(&self) -> bool {
        *self == BLOCKSIZE::Unit128
    }
    #[doc = "256 unit transfers per arbitration"]
    #[inline(always)]
    pub fn is_unit256(&self) -> bool {
        *self == BLOCKSIZE::Unit256
    }
    #[doc = "512 unit transfers per arbitration"]
    #[inline(always)]
    pub fn is_unit512(&self) -> bool {
        *self == BLOCKSIZE::Unit512
    }
    #[doc = "1024 unit transfers per arbitration"]
    #[inline(always)]
    pub fn is_unit1024(&self) -> bool {
        *self == BLOCKSIZE::Unit1024
    }
    #[doc = "Transfer all units as specified by the XFRCNT field"]
    #[inline(always)]
    pub fn is_all(&self) -> bool {
        *self == BLOCKSIZE::All
    }
}
#[doc = "Field `BLOCKSIZE` writer - Block Transfer Size"]
pub type BLOCKSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 4, BLOCKSIZE>;
impl<'a, REG> BLOCKSIZE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "One unit transfer per arbitration"]
    #[inline(always)]
    pub fn unit1(self) -> &'a mut crate::W<REG> {
        self.variant(BLOCKSIZE::Unit1)
    }
    #[doc = "Two unit transfers per arbitration"]
    #[inline(always)]
    pub fn unit2(self) -> &'a mut crate::W<REG> {
        self.variant(BLOCKSIZE::Unit2)
    }
    #[doc = "Three unit transfers per arbitration"]
    #[inline(always)]
    pub fn unit3(self) -> &'a mut crate::W<REG> {
        self.variant(BLOCKSIZE::Unit3)
    }
    #[doc = "Four unit transfers per arbitration"]
    #[inline(always)]
    pub fn unit4(self) -> &'a mut crate::W<REG> {
        self.variant(BLOCKSIZE::Unit4)
    }
    #[doc = "Six unit transfers per arbitration"]
    #[inline(always)]
    pub fn unit6(self) -> &'a mut crate::W<REG> {
        self.variant(BLOCKSIZE::Unit6)
    }
    #[doc = "Eight unit transfers per arbitration"]
    #[inline(always)]
    pub fn unit8(self) -> &'a mut crate::W<REG> {
        self.variant(BLOCKSIZE::Unit8)
    }
    #[doc = "Sixteen unit transfers per arbitration"]
    #[inline(always)]
    pub fn unit16(self) -> &'a mut crate::W<REG> {
        self.variant(BLOCKSIZE::Unit16)
    }
    #[doc = "32 unit transfers per arbitration"]
    #[inline(always)]
    pub fn unit32(self) -> &'a mut crate::W<REG> {
        self.variant(BLOCKSIZE::Unit32)
    }
    #[doc = "64 unit transfers per arbitration"]
    #[inline(always)]
    pub fn unit64(self) -> &'a mut crate::W<REG> {
        self.variant(BLOCKSIZE::Unit64)
    }
    #[doc = "128 unit transfers per arbitration"]
    #[inline(always)]
    pub fn unit128(self) -> &'a mut crate::W<REG> {
        self.variant(BLOCKSIZE::Unit128)
    }
    #[doc = "256 unit transfers per arbitration"]
    #[inline(always)]
    pub fn unit256(self) -> &'a mut crate::W<REG> {
        self.variant(BLOCKSIZE::Unit256)
    }
    #[doc = "512 unit transfers per arbitration"]
    #[inline(always)]
    pub fn unit512(self) -> &'a mut crate::W<REG> {
        self.variant(BLOCKSIZE::Unit512)
    }
    #[doc = "1024 unit transfers per arbitration"]
    #[inline(always)]
    pub fn unit1024(self) -> &'a mut crate::W<REG> {
        self.variant(BLOCKSIZE::Unit1024)
    }
    #[doc = "Transfer all units as specified by the XFRCNT field"]
    #[inline(always)]
    pub fn all(self) -> &'a mut crate::W<REG> {
        self.variant(BLOCKSIZE::All)
    }
}
#[doc = "Field `DONEIFSEN` reader - DMA Operation Done Interrupt Flag Set Enable"]
pub type DONEIFSEN_R = crate::BitReader;
#[doc = "Field `DONEIFSEN` writer - DMA Operation Done Interrupt Flag Set Enable"]
pub type DONEIFSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REQMODE` reader - DMA Request Transfer Mode Select"]
pub type REQMODE_R = crate::BitReader;
#[doc = "Field `REQMODE` writer - DMA Request Transfer Mode Select"]
pub type REQMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DECLOOPCNT` reader - Decrement Loop Count"]
pub type DECLOOPCNT_R = crate::BitReader;
#[doc = "Field `DECLOOPCNT` writer - Decrement Loop Count"]
pub type DECLOOPCNT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IGNORESREQ` reader - Ignore Sreq"]
pub type IGNORESREQ_R = crate::BitReader;
#[doc = "Field `IGNORESREQ` writer - Ignore Sreq"]
pub type IGNORESREQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRCINC` reader - Source Address Increment Size"]
pub type SRCINC_R = crate::FieldReader<SRCINC>;
#[doc = "Source Address Increment Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SRCINC {
    #[doc = "0: Increment source address by one unit data size after each read"]
    One = 0,
    #[doc = "1: Increment source address by two unit data sizes after each read"]
    Two = 1,
    #[doc = "2: Increment source address by four unit data sizes after each read"]
    Four = 2,
    #[doc = "3: Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO."]
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
impl SRCINC_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "Increment source address by one unit data size after each read"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == SRCINC::One
    }
    #[doc = "Increment source address by two unit data sizes after each read"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == SRCINC::Two
    }
    #[doc = "Increment source address by four unit data sizes after each read"]
    #[inline(always)]
    pub fn is_four(&self) -> bool {
        *self == SRCINC::Four
    }
    #[doc = "Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SRCINC::None
    }
}
#[doc = "Field `SRCINC` writer - Source Address Increment Size"]
pub type SRCINC_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, SRCINC>;
impl<'a, REG> SRCINC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Increment source address by one unit data size after each read"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(SRCINC::One)
    }
    #[doc = "Increment source address by two unit data sizes after each read"]
    #[inline(always)]
    pub fn two(self) -> &'a mut crate::W<REG> {
        self.variant(SRCINC::Two)
    }
    #[doc = "Increment source address by four unit data sizes after each read"]
    #[inline(always)]
    pub fn four(self) -> &'a mut crate::W<REG> {
        self.variant(SRCINC::Four)
    }
    #[doc = "Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(SRCINC::None)
    }
}
#[doc = "Field `SIZE` reader - Unit Data Transfer Size"]
pub type SIZE_R = crate::FieldReader<SIZE>;
#[doc = "Unit Data Transfer Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SIZE {
    #[doc = "0: Each unit transfer is a byte"]
    Byte = 0,
    #[doc = "1: Each unit transfer is a half-word"]
    Halfword = 1,
    #[doc = "2: Each unit transfer is a word"]
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
impl SIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SIZE> {
        match self.bits {
            0 => Some(SIZE::Byte),
            1 => Some(SIZE::Halfword),
            2 => Some(SIZE::Word),
            _ => None,
        }
    }
    #[doc = "Each unit transfer is a byte"]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == SIZE::Byte
    }
    #[doc = "Each unit transfer is a half-word"]
    #[inline(always)]
    pub fn is_halfword(&self) -> bool {
        *self == SIZE::Halfword
    }
    #[doc = "Each unit transfer is a word"]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        *self == SIZE::Word
    }
}
#[doc = "Field `SIZE` writer - Unit Data Transfer Size"]
pub type SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SIZE>;
impl<'a, REG> SIZE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Each unit transfer is a byte"]
    #[inline(always)]
    pub fn byte(self) -> &'a mut crate::W<REG> {
        self.variant(SIZE::Byte)
    }
    #[doc = "Each unit transfer is a half-word"]
    #[inline(always)]
    pub fn halfword(self) -> &'a mut crate::W<REG> {
        self.variant(SIZE::Halfword)
    }
    #[doc = "Each unit transfer is a word"]
    #[inline(always)]
    pub fn word(self) -> &'a mut crate::W<REG> {
        self.variant(SIZE::Word)
    }
}
#[doc = "Field `DSTINC` reader - Destination Address Increment Size"]
pub type DSTINC_R = crate::FieldReader<DSTINC>;
#[doc = "Destination Address Increment Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DSTINC {
    #[doc = "0: Increment destination address by one unit data size after each write"]
    One = 0,
    #[doc = "1: Increment destination address by two unit data sizes after each write"]
    Two = 1,
    #[doc = "2: Increment destination address by four unit data sizes after each write"]
    Four = 2,
    #[doc = "3: Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO."]
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
impl DSTINC_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "Increment destination address by one unit data size after each write"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == DSTINC::One
    }
    #[doc = "Increment destination address by two unit data sizes after each write"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == DSTINC::Two
    }
    #[doc = "Increment destination address by four unit data sizes after each write"]
    #[inline(always)]
    pub fn is_four(&self) -> bool {
        *self == DSTINC::Four
    }
    #[doc = "Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == DSTINC::None
    }
}
#[doc = "Field `DSTINC` writer - Destination Address Increment Size"]
pub type DSTINC_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, DSTINC>;
impl<'a, REG> DSTINC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Increment destination address by one unit data size after each write"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(DSTINC::One)
    }
    #[doc = "Increment destination address by two unit data sizes after each write"]
    #[inline(always)]
    pub fn two(self) -> &'a mut crate::W<REG> {
        self.variant(DSTINC::Two)
    }
    #[doc = "Increment destination address by four unit data sizes after each write"]
    #[inline(always)]
    pub fn four(self) -> &'a mut crate::W<REG> {
        self.variant(DSTINC::Four)
    }
    #[doc = "Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(DSTINC::None)
    }
}
#[doc = "Field `SRCMODE` reader - Source Addressing Mode"]
pub type SRCMODE_R = crate::BitReader;
#[doc = "Field `DSTMODE` reader - Destination Addressing Mode"]
pub type DSTMODE_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - DMA Structure Type"]
    #[inline(always)]
    pub fn structtype(&self) -> STRUCTTYPE_R {
        STRUCTTYPE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:14 - DMA Unit Data Transfer Count"]
    #[inline(always)]
    pub fn xfercnt(&self) -> XFERCNT_R {
        XFERCNT_R::new(((self.bits >> 4) & 0x07ff) as u16)
    }
    #[doc = "Bit 15 - Endian Byte Swap"]
    #[inline(always)]
    pub fn byteswap(&self) -> BYTESWAP_R {
        BYTESWAP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Block Transfer Size"]
    #[inline(always)]
    pub fn blocksize(&self) -> BLOCKSIZE_R {
        BLOCKSIZE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - DMA Operation Done Interrupt Flag Set Enable"]
    #[inline(always)]
    pub fn doneifsen(&self) -> DONEIFSEN_R {
        DONEIFSEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - DMA Request Transfer Mode Select"]
    #[inline(always)]
    pub fn reqmode(&self) -> REQMODE_R {
        REQMODE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Decrement Loop Count"]
    #[inline(always)]
    pub fn decloopcnt(&self) -> DECLOOPCNT_R {
        DECLOOPCNT_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Ignore Sreq"]
    #[inline(always)]
    pub fn ignoresreq(&self) -> IGNORESREQ_R {
        IGNORESREQ_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Source Address Increment Size"]
    #[inline(always)]
    pub fn srcinc(&self) -> SRCINC_R {
        SRCINC_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Unit Data Transfer Size"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Destination Address Increment Size"]
    #[inline(always)]
    pub fn dstinc(&self) -> DSTINC_R {
        DSTINC_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - Source Addressing Mode"]
    #[inline(always)]
    pub fn srcmode(&self) -> SRCMODE_R {
        SRCMODE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Destination Addressing Mode"]
    #[inline(always)]
    pub fn dstmode(&self) -> DSTMODE_R {
        DSTMODE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Structure DMA Transfer Request"]
    #[inline(always)]
    #[must_use]
    pub fn structreq(&mut self) -> STRUCTREQ_W<CH6_CTRLrs> {
        STRUCTREQ_W::new(self, 3)
    }
    #[doc = "Bits 4:14 - DMA Unit Data Transfer Count"]
    #[inline(always)]
    #[must_use]
    pub fn xfercnt(&mut self) -> XFERCNT_W<CH6_CTRLrs> {
        XFERCNT_W::new(self, 4)
    }
    #[doc = "Bit 15 - Endian Byte Swap"]
    #[inline(always)]
    #[must_use]
    pub fn byteswap(&mut self) -> BYTESWAP_W<CH6_CTRLrs> {
        BYTESWAP_W::new(self, 15)
    }
    #[doc = "Bits 16:19 - Block Transfer Size"]
    #[inline(always)]
    #[must_use]
    pub fn blocksize(&mut self) -> BLOCKSIZE_W<CH6_CTRLrs> {
        BLOCKSIZE_W::new(self, 16)
    }
    #[doc = "Bit 20 - DMA Operation Done Interrupt Flag Set Enable"]
    #[inline(always)]
    #[must_use]
    pub fn doneifsen(&mut self) -> DONEIFSEN_W<CH6_CTRLrs> {
        DONEIFSEN_W::new(self, 20)
    }
    #[doc = "Bit 21 - DMA Request Transfer Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn reqmode(&mut self) -> REQMODE_W<CH6_CTRLrs> {
        REQMODE_W::new(self, 21)
    }
    #[doc = "Bit 22 - Decrement Loop Count"]
    #[inline(always)]
    #[must_use]
    pub fn decloopcnt(&mut self) -> DECLOOPCNT_W<CH6_CTRLrs> {
        DECLOOPCNT_W::new(self, 22)
    }
    #[doc = "Bit 23 - Ignore Sreq"]
    #[inline(always)]
    #[must_use]
    pub fn ignoresreq(&mut self) -> IGNORESREQ_W<CH6_CTRLrs> {
        IGNORESREQ_W::new(self, 23)
    }
    #[doc = "Bits 24:25 - Source Address Increment Size"]
    #[inline(always)]
    #[must_use]
    pub fn srcinc(&mut self) -> SRCINC_W<CH6_CTRLrs> {
        SRCINC_W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Unit Data Transfer Size"]
    #[inline(always)]
    #[must_use]
    pub fn size(&mut self) -> SIZE_W<CH6_CTRLrs> {
        SIZE_W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Destination Address Increment Size"]
    #[inline(always)]
    #[must_use]
    pub fn dstinc(&mut self) -> DSTINC_W<CH6_CTRLrs> {
        DSTINC_W::new(self, 28)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Channel Descriptor Control Word Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch6_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch6_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH6_CTRLrs;
impl crate::RegisterSpec for CH6_CTRLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch6_ctrl::R`](R) reader structure"]
impl crate::Readable for CH6_CTRLrs {}
#[doc = "`write(|w| ..)` method takes [`ch6_ctrl::W`](W) writer structure"]
impl crate::Writable for CH6_CTRLrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH6_CTRL to value 0"]
impl crate::Resettable for CH6_CTRLrs {
    const RESET_VALUE: u32 = 0;
}
