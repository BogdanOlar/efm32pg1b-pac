///Register `CTRL` reader
pub type R = crate::R<CTRLrs>;
///Register `CTRL` writer
pub type W = crate::W<CTRLrs>;
///Field `AES` reader - AES Mode
pub type AesR = crate::BitReader;
///Field `AES` writer - AES Mode
pub type AesW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `KEYBUFDIS` reader - Key Buffer Disable
pub type KeybufdisR = crate::BitReader;
///Field `KEYBUFDIS` writer - Key Buffer Disable
pub type KeybufdisW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SHA` reader - SHA Mode
pub type ShaR = crate::BitReader;
///Field `SHA` writer - SHA Mode
pub type ShaW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NOBUSYSTALL` reader - No Stalling of Bus When Busy
pub type NobusystallR = crate::BitReader;
///Field `NOBUSYSTALL` writer - No Stalling of Bus When Busy
pub type NobusystallW<'a, REG> = crate::BitWriter<'a, REG>;
///Increment Width
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INCWIDTH {
    ///0: Byte 15 in DATA1 is used for the increment function.
    Incwidth1 = 0,
    ///1: Bytes 14 and 15 in DATA1 are used for the increment function.
    Incwidth2 = 1,
    ///2: Bytes 13 to 15 in DATA1 are used for the increment function.
    Incwidth3 = 2,
    ///3: Bytes 12 to 15 in DATA1 are used for the increment function.
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
impl crate::IsEnum for INCWIDTH {}
///Field `INCWIDTH` reader - Increment Width
pub type IncwidthR = crate::FieldReader<INCWIDTH>;
impl IncwidthR {
    ///Get enumerated values variant
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
    ///Byte 15 in DATA1 is used for the increment function.
    #[inline(always)]
    pub fn is_incwidth1(&self) -> bool {
        *self == INCWIDTH::Incwidth1
    }
    ///Bytes 14 and 15 in DATA1 are used for the increment function.
    #[inline(always)]
    pub fn is_incwidth2(&self) -> bool {
        *self == INCWIDTH::Incwidth2
    }
    ///Bytes 13 to 15 in DATA1 are used for the increment function.
    #[inline(always)]
    pub fn is_incwidth3(&self) -> bool {
        *self == INCWIDTH::Incwidth3
    }
    ///Bytes 12 to 15 in DATA1 are used for the increment function.
    #[inline(always)]
    pub fn is_incwidth4(&self) -> bool {
        *self == INCWIDTH::Incwidth4
    }
}
///Field `INCWIDTH` writer - Increment Width
pub type IncwidthW<'a, REG> = crate::FieldWriter<'a, REG, 2, INCWIDTH, crate::Safe>;
impl<'a, REG> IncwidthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Byte 15 in DATA1 is used for the increment function.
    #[inline(always)]
    pub fn incwidth1(self) -> &'a mut crate::W<REG> {
        self.variant(INCWIDTH::Incwidth1)
    }
    ///Bytes 14 and 15 in DATA1 are used for the increment function.
    #[inline(always)]
    pub fn incwidth2(self) -> &'a mut crate::W<REG> {
        self.variant(INCWIDTH::Incwidth2)
    }
    ///Bytes 13 to 15 in DATA1 are used for the increment function.
    #[inline(always)]
    pub fn incwidth3(self) -> &'a mut crate::W<REG> {
        self.variant(INCWIDTH::Incwidth3)
    }
    ///Bytes 12 to 15 in DATA1 are used for the increment function.
    #[inline(always)]
    pub fn incwidth4(self) -> &'a mut crate::W<REG> {
        self.variant(INCWIDTH::Incwidth4)
    }
}
///DMA0 Read Mode
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMA0MODE {
    ///0: Target register is fully read/written during every DMA transaction
    Full = 0,
    ///1: Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + necessary zero padding is read. Zero padding is automatically added when writing.
    Lenlimit = 1,
    ///2: Target register is fully read/written during every DMA transaction. Bytewise DMA.
    Fullbyte = 2,
    ///3: Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + necessary zero padding is read. Bytewise DMA. Zero padding is automatically added when writing.
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
impl crate::IsEnum for DMA0MODE {}
///Field `DMA0MODE` reader - DMA0 Read Mode
pub type Dma0modeR = crate::FieldReader<DMA0MODE>;
impl Dma0modeR {
    ///Get enumerated values variant
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
    ///Target register is fully read/written during every DMA transaction
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == DMA0MODE::Full
    }
    ///Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + necessary zero padding is read. Zero padding is automatically added when writing.
    #[inline(always)]
    pub fn is_lenlimit(&self) -> bool {
        *self == DMA0MODE::Lenlimit
    }
    ///Target register is fully read/written during every DMA transaction. Bytewise DMA.
    #[inline(always)]
    pub fn is_fullbyte(&self) -> bool {
        *self == DMA0MODE::Fullbyte
    }
    ///Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + necessary zero padding is read. Bytewise DMA. Zero padding is automatically added when writing.
    #[inline(always)]
    pub fn is_lenlimitbyte(&self) -> bool {
        *self == DMA0MODE::Lenlimitbyte
    }
}
///Field `DMA0MODE` writer - DMA0 Read Mode
pub type Dma0modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, DMA0MODE, crate::Safe>;
impl<'a, REG> Dma0modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Target register is fully read/written during every DMA transaction
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0MODE::Full)
    }
    ///Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + necessary zero padding is read. Zero padding is automatically added when writing.
    #[inline(always)]
    pub fn lenlimit(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0MODE::Lenlimit)
    }
    ///Target register is fully read/written during every DMA transaction. Bytewise DMA.
    #[inline(always)]
    pub fn fullbyte(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0MODE::Fullbyte)
    }
    ///Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + necessary zero padding is read. Bytewise DMA. Zero padding is automatically added when writing.
    #[inline(always)]
    pub fn lenlimitbyte(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0MODE::Lenlimitbyte)
    }
}
///DMA0 Read Register Select
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMA0RSEL {
    ///0: `0`
    Data0 = 0,
    ///1: `1`
    Ddata0 = 1,
    ///2: `10`
    Ddata0big = 2,
    ///3: `11`
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
impl crate::IsEnum for DMA0RSEL {}
///Field `DMA0RSEL` reader - DMA0 Read Register Select
pub type Dma0rselR = crate::FieldReader<DMA0RSEL>;
impl Dma0rselR {
    ///Get enumerated values variant
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
    ///`0`
    #[inline(always)]
    pub fn is_data0(&self) -> bool {
        *self == DMA0RSEL::Data0
    }
    ///`1`
    #[inline(always)]
    pub fn is_ddata0(&self) -> bool {
        *self == DMA0RSEL::Ddata0
    }
    ///`10`
    #[inline(always)]
    pub fn is_ddata0big(&self) -> bool {
        *self == DMA0RSEL::Ddata0big
    }
    ///`11`
    #[inline(always)]
    pub fn is_qdata0(&self) -> bool {
        *self == DMA0RSEL::Qdata0
    }
}
///Field `DMA0RSEL` writer - DMA0 Read Register Select
pub type Dma0rselW<'a, REG> = crate::FieldWriter<'a, REG, 2, DMA0RSEL, crate::Safe>;
impl<'a, REG> Dma0rselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///`0`
    #[inline(always)]
    pub fn data0(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0RSEL::Data0)
    }
    ///`1`
    #[inline(always)]
    pub fn ddata0(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0RSEL::Ddata0)
    }
    ///`10`
    #[inline(always)]
    pub fn ddata0big(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0RSEL::Ddata0big)
    }
    ///`11`
    #[inline(always)]
    pub fn qdata0(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0RSEL::Qdata0)
    }
}
///DMA1 Read Mode
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMA1MODE {
    ///0: Target register is fully read/written during every DMA transaction
    Full = 0,
    ///1: Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + 1 bytes + necessary zero padding is read. Zero padding is automatically added when writing.
    Lenlimit = 1,
    ///2: Target register is fully read/written during every DMA transaction. Bytewise DMA.
    Fullbyte = 2,
    ///3: Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + 1 bytes + necessary zero padding is read. Bytewise DMA. Zero padding is automatically added when writing.
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
impl crate::IsEnum for DMA1MODE {}
///Field `DMA1MODE` reader - DMA1 Read Mode
pub type Dma1modeR = crate::FieldReader<DMA1MODE>;
impl Dma1modeR {
    ///Get enumerated values variant
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
    ///Target register is fully read/written during every DMA transaction
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == DMA1MODE::Full
    }
    ///Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + 1 bytes + necessary zero padding is read. Zero padding is automatically added when writing.
    #[inline(always)]
    pub fn is_lenlimit(&self) -> bool {
        *self == DMA1MODE::Lenlimit
    }
    ///Target register is fully read/written during every DMA transaction. Bytewise DMA.
    #[inline(always)]
    pub fn is_fullbyte(&self) -> bool {
        *self == DMA1MODE::Fullbyte
    }
    ///Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + 1 bytes + necessary zero padding is read. Bytewise DMA. Zero padding is automatically added when writing.
    #[inline(always)]
    pub fn is_lenlimitbyte(&self) -> bool {
        *self == DMA1MODE::Lenlimitbyte
    }
}
///Field `DMA1MODE` writer - DMA1 Read Mode
pub type Dma1modeW<'a, REG> = crate::FieldWriter<'a, REG, 2, DMA1MODE, crate::Safe>;
impl<'a, REG> Dma1modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Target register is fully read/written during every DMA transaction
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1MODE::Full)
    }
    ///Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + 1 bytes + necessary zero padding is read. Zero padding is automatically added when writing.
    #[inline(always)]
    pub fn lenlimit(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1MODE::Lenlimit)
    }
    ///Target register is fully read/written during every DMA transaction. Bytewise DMA.
    #[inline(always)]
    pub fn fullbyte(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1MODE::Fullbyte)
    }
    ///Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + 1 bytes + necessary zero padding is read. Bytewise DMA. Zero padding is automatically added when writing.
    #[inline(always)]
    pub fn lenlimitbyte(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1MODE::Lenlimitbyte)
    }
}
///DATA0 DMA Unaligned Read Register Select
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMA1RSEL {
    ///0: `0`
    Data1 = 0,
    ///1: `1`
    Ddata1 = 1,
    ///2: `10`
    Qdata1 = 2,
    ///3: `11`
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
impl crate::IsEnum for DMA1RSEL {}
///Field `DMA1RSEL` reader - DATA0 DMA Unaligned Read Register Select
pub type Dma1rselR = crate::FieldReader<DMA1RSEL>;
impl Dma1rselR {
    ///Get enumerated values variant
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
    ///`0`
    #[inline(always)]
    pub fn is_data1(&self) -> bool {
        *self == DMA1RSEL::Data1
    }
    ///`1`
    #[inline(always)]
    pub fn is_ddata1(&self) -> bool {
        *self == DMA1RSEL::Ddata1
    }
    ///`10`
    #[inline(always)]
    pub fn is_qdata1(&self) -> bool {
        *self == DMA1RSEL::Qdata1
    }
    ///`11`
    #[inline(always)]
    pub fn is_qdata1big(&self) -> bool {
        *self == DMA1RSEL::Qdata1big
    }
}
///Field `DMA1RSEL` writer - DATA0 DMA Unaligned Read Register Select
pub type Dma1rselW<'a, REG> = crate::FieldWriter<'a, REG, 2, DMA1RSEL, crate::Safe>;
impl<'a, REG> Dma1rselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///`0`
    #[inline(always)]
    pub fn data1(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1RSEL::Data1)
    }
    ///`1`
    #[inline(always)]
    pub fn ddata1(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1RSEL::Ddata1)
    }
    ///`10`
    #[inline(always)]
    pub fn qdata1(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1RSEL::Qdata1)
    }
    ///`11`
    #[inline(always)]
    pub fn qdata1big(self) -> &'a mut crate::W<REG> {
        self.variant(DMA1RSEL::Qdata1big)
    }
}
///Field `COMBDMA0WEREQ` reader - Combined Data0 Write DMA Request
pub type Combdma0wereqR = crate::BitReader;
///Field `COMBDMA0WEREQ` writer - Combined Data0 Write DMA Request
pub type Combdma0wereqW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - AES Mode
    #[inline(always)]
    pub fn aes(&self) -> AesR {
        AesR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Key Buffer Disable
    #[inline(always)]
    pub fn keybufdis(&self) -> KeybufdisR {
        KeybufdisR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - SHA Mode
    #[inline(always)]
    pub fn sha(&self) -> ShaR {
        ShaR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 10 - No Stalling of Bus When Busy
    #[inline(always)]
    pub fn nobusystall(&self) -> NobusystallR {
        NobusystallR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bits 14:15 - Increment Width
    #[inline(always)]
    pub fn incwidth(&self) -> IncwidthR {
        IncwidthR::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:17 - DMA0 Read Mode
    #[inline(always)]
    pub fn dma0mode(&self) -> Dma0modeR {
        Dma0modeR::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 20:21 - DMA0 Read Register Select
    #[inline(always)]
    pub fn dma0rsel(&self) -> Dma0rselR {
        Dma0rselR::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 24:25 - DMA1 Read Mode
    #[inline(always)]
    pub fn dma1mode(&self) -> Dma1modeR {
        Dma1modeR::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 28:29 - DATA0 DMA Unaligned Read Register Select
    #[inline(always)]
    pub fn dma1rsel(&self) -> Dma1rselR {
        Dma1rselR::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bit 31 - Combined Data0 Write DMA Request
    #[inline(always)]
    pub fn combdma0wereq(&self) -> Combdma0wereqR {
        Combdma0wereqR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("aes", &self.aes())
            .field("keybufdis", &self.keybufdis())
            .field("sha", &self.sha())
            .field("nobusystall", &self.nobusystall())
            .field("incwidth", &self.incwidth())
            .field("dma0mode", &self.dma0mode())
            .field("dma0rsel", &self.dma0rsel())
            .field("dma1mode", &self.dma1mode())
            .field("dma1rsel", &self.dma1rsel())
            .field("combdma0wereq", &self.combdma0wereq())
            .finish()
    }
}
impl W {
    ///Bit 0 - AES Mode
    #[inline(always)]
    #[must_use]
    pub fn aes(&mut self) -> AesW<CTRLrs> {
        AesW::new(self, 0)
    }
    ///Bit 1 - Key Buffer Disable
    #[inline(always)]
    #[must_use]
    pub fn keybufdis(&mut self) -> KeybufdisW<CTRLrs> {
        KeybufdisW::new(self, 1)
    }
    ///Bit 2 - SHA Mode
    #[inline(always)]
    #[must_use]
    pub fn sha(&mut self) -> ShaW<CTRLrs> {
        ShaW::new(self, 2)
    }
    ///Bit 10 - No Stalling of Bus When Busy
    #[inline(always)]
    #[must_use]
    pub fn nobusystall(&mut self) -> NobusystallW<CTRLrs> {
        NobusystallW::new(self, 10)
    }
    ///Bits 14:15 - Increment Width
    #[inline(always)]
    #[must_use]
    pub fn incwidth(&mut self) -> IncwidthW<CTRLrs> {
        IncwidthW::new(self, 14)
    }
    ///Bits 16:17 - DMA0 Read Mode
    #[inline(always)]
    #[must_use]
    pub fn dma0mode(&mut self) -> Dma0modeW<CTRLrs> {
        Dma0modeW::new(self, 16)
    }
    ///Bits 20:21 - DMA0 Read Register Select
    #[inline(always)]
    #[must_use]
    pub fn dma0rsel(&mut self) -> Dma0rselW<CTRLrs> {
        Dma0rselW::new(self, 20)
    }
    ///Bits 24:25 - DMA1 Read Mode
    #[inline(always)]
    #[must_use]
    pub fn dma1mode(&mut self) -> Dma1modeW<CTRLrs> {
        Dma1modeW::new(self, 24)
    }
    ///Bits 28:29 - DATA0 DMA Unaligned Read Register Select
    #[inline(always)]
    #[must_use]
    pub fn dma1rsel(&mut self) -> Dma1rselW<CTRLrs> {
        Dma1rselW::new(self, 28)
    }
    ///Bit 31 - Combined Data0 Write DMA Request
    #[inline(always)]
    #[must_use]
    pub fn combdma0wereq(&mut self) -> Combdma0wereqW<CTRLrs> {
        Combdma0wereqW::new(self, 31)
    }
}
///Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CTRLrs;
impl crate::RegisterSpec for CTRLrs {
    type Ux = u32;
}
///`read()` method returns [`ctrl::R`](R) reader structure
impl crate::Readable for CTRLrs {}
///`write(|w| ..)` method takes [`ctrl::W`](W) writer structure
impl crate::Writable for CTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CTRL to value 0
impl crate::Resettable for CTRLrs {
    const RESET_VALUE: u32 = 0;
}
