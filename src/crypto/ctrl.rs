#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRLrs>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRLrs>;
#[doc = "Field `AES` reader - AES Mode"]
pub type AES_R = crate::BitReader;
#[doc = "Field `AES` writer - AES Mode"]
pub type AES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYBUFDIS` reader - Key Buffer Disable"]
pub type KEYBUFDIS_R = crate::BitReader;
#[doc = "Field `KEYBUFDIS` writer - Key Buffer Disable"]
pub type KEYBUFDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHA` reader - SHA Mode"]
pub type SHA_R = crate::BitReader;
#[doc = "Field `SHA` writer - SHA Mode"]
pub type SHA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOBUSYSTALL` reader - No Stalling of Bus When Busy"]
pub type NOBUSYSTALL_R = crate::BitReader;
#[doc = "Field `NOBUSYSTALL` writer - No Stalling of Bus When Busy"]
pub type NOBUSYSTALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INCWIDTH` reader - Increment Width"]
pub type INCWIDTH_R = crate::FieldReader<INCWIDTH>;
#[doc = "Increment Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INCWIDTH {
    #[doc = "0: Byte 15 in DATA1 is used for the increment function."]
    Incwidth1 = 0,
    #[doc = "1: Bytes 14 and 15 in DATA1 are used for the increment function."]
    Incwidth2 = 1,
    #[doc = "2: Bytes 13 to 15 in DATA1 are used for the increment function."]
    Incwidth3 = 2,
    #[doc = "3: Bytes 12 to 15 in DATA1 are used for the increment function."]
    Incwidth4 = 3,
}
impl From<INCWIDTH> for u8 {
    #[inline(always)]
    fn from(variant: INCWIDTH) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for INCWIDTH {
    type Ux = u8;
}
impl INCWIDTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INCWIDTH {
        match self.bits {
            0 => INCWIDTH::Incwidth1,
            1 => INCWIDTH::Incwidth2,
            2 => INCWIDTH::Incwidth3,
            3 => INCWIDTH::Incwidth4,
            _ => unreachable!(),
        }
    }
    #[doc = "Byte 15 in DATA1 is used for the increment function."]
    #[inline(always)]
    pub fn is_incwidth1(&self) -> bool {
        *self == INCWIDTH::Incwidth1
    }
    #[doc = "Bytes 14 and 15 in DATA1 are used for the increment function."]
    #[inline(always)]
    pub fn is_incwidth2(&self) -> bool {
        *self == INCWIDTH::Incwidth2
    }
    #[doc = "Bytes 13 to 15 in DATA1 are used for the increment function."]
    #[inline(always)]
    pub fn is_incwidth3(&self) -> bool {
        *self == INCWIDTH::Incwidth3
    }
    #[doc = "Bytes 12 to 15 in DATA1 are used for the increment function."]
    #[inline(always)]
    pub fn is_incwidth4(&self) -> bool {
        *self == INCWIDTH::Incwidth4
    }
}
#[doc = "Field `INCWIDTH` writer - Increment Width"]
pub type INCWIDTH_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, INCWIDTH>;
impl<'a, REG> INCWIDTH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Byte 15 in DATA1 is used for the increment function."]
    #[inline(always)]
    pub fn incwidth1(self) -> &'a mut crate::W<REG> {
        self.variant(INCWIDTH::Incwidth1)
    }
    #[doc = "Bytes 14 and 15 in DATA1 are used for the increment function."]
    #[inline(always)]
    pub fn incwidth2(self) -> &'a mut crate::W<REG> {
        self.variant(INCWIDTH::Incwidth2)
    }
    #[doc = "Bytes 13 to 15 in DATA1 are used for the increment function."]
    #[inline(always)]
    pub fn incwidth3(self) -> &'a mut crate::W<REG> {
        self.variant(INCWIDTH::Incwidth3)
    }
    #[doc = "Bytes 12 to 15 in DATA1 are used for the increment function."]
    #[inline(always)]
    pub fn incwidth4(self) -> &'a mut crate::W<REG> {
        self.variant(INCWIDTH::Incwidth4)
    }
}
#[doc = "Field `DMA0MODE` reader - DMA0 Read Mode"]
pub type DMA0MODE_R = crate::FieldReader<DMA0MODE>;
#[doc = "DMA0 Read Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMA0MODE {
    #[doc = "0: Target register is fully read/written during every DMA transaction"]
    Full = 0,
    #[doc = "1: Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + necessary zero padding is read. Zero padding is automatically added when writing."]
    Lenlimit = 1,
    #[doc = "2: Target register is fully read/written during every DMA transaction. Bytewise DMA."]
    Fullbyte = 2,
    #[doc = "3: Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + necessary zero padding is read. Bytewise DMA. Zero padding is automatically added when writing."]
    Lenlimitbyte = 3,
}
impl From<DMA0MODE> for u8 {
    #[inline(always)]
    fn from(variant: DMA0MODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DMA0MODE {
    type Ux = u8;
}
impl DMA0MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMA0MODE {
        match self.bits {
            0 => DMA0MODE::Full,
            1 => DMA0MODE::Lenlimit,
            2 => DMA0MODE::Fullbyte,
            3 => DMA0MODE::Lenlimitbyte,
            _ => unreachable!(),
        }
    }
    #[doc = "Target register is fully read/written during every DMA transaction"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == DMA0MODE::Full
    }
    #[doc = "Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + necessary zero padding is read. Zero padding is automatically added when writing."]
    #[inline(always)]
    pub fn is_lenlimit(&self) -> bool {
        *self == DMA0MODE::Lenlimit
    }
    #[doc = "Target register is fully read/written during every DMA transaction. Bytewise DMA."]
    #[inline(always)]
    pub fn is_fullbyte(&self) -> bool {
        *self == DMA0MODE::Fullbyte
    }
    #[doc = "Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + necessary zero padding is read. Bytewise DMA. Zero padding is automatically added when writing."]
    #[inline(always)]
    pub fn is_lenlimitbyte(&self) -> bool {
        *self == DMA0MODE::Lenlimitbyte
    }
}
#[doc = "Field `DMA0MODE` writer - DMA0 Read Mode"]
pub type DMA0MODE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, DMA0MODE>;
impl<'a, REG> DMA0MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Target register is fully read/written during every DMA transaction"]
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0MODE::Full)
    }
    #[doc = "Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + necessary zero padding is read. Zero padding is automatically added when writing."]
    #[inline(always)]
    pub fn lenlimit(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0MODE::Lenlimit)
    }
    #[doc = "Target register is fully read/written during every DMA transaction. Bytewise DMA."]
    #[inline(always)]
    pub fn fullbyte(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0MODE::Fullbyte)
    }
    #[doc = "Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + necessary zero padding is read. Bytewise DMA. Zero padding is automatically added when writing."]
    #[inline(always)]
    pub fn lenlimitbyte(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0MODE::Lenlimitbyte)
    }
}
#[doc = "Field `DMA0RSEL` reader - DMA0 Read Register Select"]
pub type DMA0RSEL_R = crate::FieldReader<DMA0RSEL>;
#[doc = "DMA0 Read Register Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMA0RSEL {
    #[doc = "0: `0`"]
    Data0 = 0,
    #[doc = "1: `1`"]
    Ddata0 = 1,
    #[doc = "2: `10`"]
    Ddata0big = 2,
    #[doc = "3: `11`"]
    Qdata0 = 3,
}
impl From<DMA0RSEL> for u8 {
    #[inline(always)]
    fn from(variant: DMA0RSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DMA0RSEL {
    type Ux = u8;
}
impl DMA0RSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMA0RSEL {
        match self.bits {
            0 => DMA0RSEL::Data0,
            1 => DMA0RSEL::Ddata0,
            2 => DMA0RSEL::Ddata0big,
            3 => DMA0RSEL::Qdata0,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_data0(&self) -> bool {
        *self == DMA0RSEL::Data0
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_ddata0(&self) -> bool {
        *self == DMA0RSEL::Ddata0
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_ddata0big(&self) -> bool {
        *self == DMA0RSEL::Ddata0big
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_qdata0(&self) -> bool {
        *self == DMA0RSEL::Qdata0
    }
}
#[doc = "Field `DMA0RSEL` writer - DMA0 Read Register Select"]
pub type DMA0RSEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, DMA0RSEL>;
impl<'a, REG> DMA0RSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn data0(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0RSEL::Data0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn ddata0(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0RSEL::Ddata0)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn ddata0big(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0RSEL::Ddata0big)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn qdata0(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0RSEL::Qdata0)
    }
}
#[doc = "Field `DMA1MODE` reader - DMA1 Read Mode"]
pub type DMA1MODE_R = crate::FieldReader<DMA1MODE>;
#[doc = "DMA1 Read Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMA1MODE {
    #[doc = "0: Target register is fully read/written during every DMA transaction"]
    Full = 0,
    #[doc = "1: Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + 1 bytes + necessary zero padding is read. Zero padding is automatically added when writing."]
    Lenlimit = 1,
    #[doc = "2: Target register is fully read/written during every DMA transaction. Bytewise DMA."]
    Fullbyte = 2,
    #[doc = "3: Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + 1 bytes + necessary zero padding is read. Bytewise DMA. Zero padding is automatically added when writing."]
    Lenlimitbyte = 3,
}
impl From<DMA1MODE> for u8 {
    #[inline(always)]
    fn from(variant: DMA1MODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DMA1MODE {
    type Ux = u8;
}
impl DMA1MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMA1MODE {
        match self.bits {
            0 => DMA1MODE::Full,
            1 => DMA1MODE::Lenlimit,
            2 => DMA1MODE::Fullbyte,
            3 => DMA1MODE::Lenlimitbyte,
            _ => unreachable!(),
        }
    }
    #[doc = "Target register is fully read/written during every DMA transaction"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == DMA1MODE::Full
    }
    #[doc = "Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + 1 bytes + necessary zero padding is read. Zero padding is automatically added when writing."]
    #[inline(always)]
    pub fn is_lenlimit(&self) -> bool {
        *self == DMA1MODE::Lenlimit
    }
    #[doc = "Target register is fully read/written during every DMA transaction. Bytewise DMA."]
    #[inline(always)]
    pub fn is_fullbyte(&self) -> bool {
        *self == DMA1MODE::Fullbyte
    }
    #[doc = "Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + 1 bytes + necessary zero padding is read. Bytewise DMA. Zero padding is automatically added when writing."]
    #[inline(always)]
    pub fn is_lenlimitbyte(&self) -> bool {
        *self == DMA1MODE::Lenlimitbyte
    }
}
#[doc = "Field `DMA1MODE` writer - DMA1 Read Mode"]
pub type DMA1MODE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, DMA1MODE>;
impl<'a, REG> DMA1MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Target register is fully read/written during every DMA transaction"]
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1MODE::Full)
    }
    #[doc = "Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + 1 bytes + necessary zero padding is read. Zero padding is automatically added when writing."]
    #[inline(always)]
    pub fn lenlimit(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1MODE::Lenlimit)
    }
    #[doc = "Target register is fully read/written during every DMA transaction. Bytewise DMA."]
    #[inline(always)]
    pub fn fullbyte(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1MODE::Fullbyte)
    }
    #[doc = "Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + 1 bytes + necessary zero padding is read. Bytewise DMA. Zero padding is automatically added when writing."]
    #[inline(always)]
    pub fn lenlimitbyte(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1MODE::Lenlimitbyte)
    }
}
#[doc = "Field `DMA1RSEL` reader - DATA0 DMA Unaligned Read Register Select"]
pub type DMA1RSEL_R = crate::FieldReader<DMA1RSEL>;
#[doc = "DATA0 DMA Unaligned Read Register Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMA1RSEL {
    #[doc = "0: `0`"]
    Data1 = 0,
    #[doc = "1: `1`"]
    Ddata1 = 1,
    #[doc = "2: `10`"]
    Qdata1 = 2,
    #[doc = "3: `11`"]
    Qdata1big = 3,
}
impl From<DMA1RSEL> for u8 {
    #[inline(always)]
    fn from(variant: DMA1RSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DMA1RSEL {
    type Ux = u8;
}
impl DMA1RSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMA1RSEL {
        match self.bits {
            0 => DMA1RSEL::Data1,
            1 => DMA1RSEL::Ddata1,
            2 => DMA1RSEL::Qdata1,
            3 => DMA1RSEL::Qdata1big,
            _ => unreachable!(),
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_data1(&self) -> bool {
        *self == DMA1RSEL::Data1
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_ddata1(&self) -> bool {
        *self == DMA1RSEL::Ddata1
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_qdata1(&self) -> bool {
        *self == DMA1RSEL::Qdata1
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_qdata1big(&self) -> bool {
        *self == DMA1RSEL::Qdata1big
    }
}
#[doc = "Field `DMA1RSEL` writer - DATA0 DMA Unaligned Read Register Select"]
pub type DMA1RSEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, DMA1RSEL>;
impl<'a, REG> DMA1RSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn data1(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1RSEL::Data1)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn ddata1(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1RSEL::Ddata1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn qdata1(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1RSEL::Qdata1)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn qdata1big(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1RSEL::Qdata1big)
    }
}
#[doc = "Field `COMBDMA0WEREQ` reader - Combined Data0 Write DMA Request"]
pub type COMBDMA0WEREQ_R = crate::BitReader;
#[doc = "Field `COMBDMA0WEREQ` writer - Combined Data0 Write DMA Request"]
pub type COMBDMA0WEREQ_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - AES Mode"]
    #[inline(always)]
    pub fn aes(&self) -> AES_R {
        AES_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Key Buffer Disable"]
    #[inline(always)]
    pub fn keybufdis(&self) -> KEYBUFDIS_R {
        KEYBUFDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SHA Mode"]
    #[inline(always)]
    pub fn sha(&self) -> SHA_R {
        SHA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 10 - No Stalling of Bus When Busy"]
    #[inline(always)]
    pub fn nobusystall(&self) -> NOBUSYSTALL_R {
        NOBUSYSTALL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Increment Width"]
    #[inline(always)]
    pub fn incwidth(&self) -> INCWIDTH_R {
        INCWIDTH_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - DMA0 Read Mode"]
    #[inline(always)]
    pub fn dma0mode(&self) -> DMA0MODE_R {
        DMA0MODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - DMA0 Read Register Select"]
    #[inline(always)]
    pub fn dma0rsel(&self) -> DMA0RSEL_R {
        DMA0RSEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - DMA1 Read Mode"]
    #[inline(always)]
    pub fn dma1mode(&self) -> DMA1MODE_R {
        DMA1MODE_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - DATA0 DMA Unaligned Read Register Select"]
    #[inline(always)]
    pub fn dma1rsel(&self) -> DMA1RSEL_R {
        DMA1RSEL_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 31 - Combined Data0 Write DMA Request"]
    #[inline(always)]
    pub fn combdma0wereq(&self) -> COMBDMA0WEREQ_R {
        COMBDMA0WEREQ_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AES Mode"]
    #[inline(always)]
    #[must_use]
    pub fn aes(&mut self) -> AES_W<CTRLrs> {
        AES_W::new(self, 0)
    }
    #[doc = "Bit 1 - Key Buffer Disable"]
    #[inline(always)]
    #[must_use]
    pub fn keybufdis(&mut self) -> KEYBUFDIS_W<CTRLrs> {
        KEYBUFDIS_W::new(self, 1)
    }
    #[doc = "Bit 2 - SHA Mode"]
    #[inline(always)]
    #[must_use]
    pub fn sha(&mut self) -> SHA_W<CTRLrs> {
        SHA_W::new(self, 2)
    }
    #[doc = "Bit 10 - No Stalling of Bus When Busy"]
    #[inline(always)]
    #[must_use]
    pub fn nobusystall(&mut self) -> NOBUSYSTALL_W<CTRLrs> {
        NOBUSYSTALL_W::new(self, 10)
    }
    #[doc = "Bits 14:15 - Increment Width"]
    #[inline(always)]
    #[must_use]
    pub fn incwidth(&mut self) -> INCWIDTH_W<CTRLrs> {
        INCWIDTH_W::new(self, 14)
    }
    #[doc = "Bits 16:17 - DMA0 Read Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dma0mode(&mut self) -> DMA0MODE_W<CTRLrs> {
        DMA0MODE_W::new(self, 16)
    }
    #[doc = "Bits 20:21 - DMA0 Read Register Select"]
    #[inline(always)]
    #[must_use]
    pub fn dma0rsel(&mut self) -> DMA0RSEL_W<CTRLrs> {
        DMA0RSEL_W::new(self, 20)
    }
    #[doc = "Bits 24:25 - DMA1 Read Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dma1mode(&mut self) -> DMA1MODE_W<CTRLrs> {
        DMA1MODE_W::new(self, 24)
    }
    #[doc = "Bits 28:29 - DATA0 DMA Unaligned Read Register Select"]
    #[inline(always)]
    #[must_use]
    pub fn dma1rsel(&mut self) -> DMA1RSEL_W<CTRLrs> {
        DMA1RSEL_W::new(self, 28)
    }
    #[doc = "Bit 31 - Combined Data0 Write DMA Request"]
    #[inline(always)]
    #[must_use]
    pub fn combdma0wereq(&mut self) -> COMBDMA0WEREQ_W<CTRLrs> {
        COMBDMA0WEREQ_W::new(self, 31)
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
#[doc = "Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLrs;
impl crate::RegisterSpec for CTRLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRLrs {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRLrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRLrs {
    const RESET_VALUE: u32 = 0;
}
