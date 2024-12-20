///Register `I2SCTRL` reader
pub type R = crate::R<I2SCTRLrs>;
///Register `I2SCTRL` writer
pub type W = crate::W<I2SCTRLrs>;
///Field `EN` reader - Enable I2S Mode
pub type EnR = crate::BitReader;
///Field `EN` writer - Enable I2S Mode
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MONO` reader - Stero or Mono
pub type MonoR = crate::BitReader;
///Field `MONO` writer - Stero or Mono
pub type MonoW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `JUSTIFY` reader - Justification of I2S Data
pub type JustifyR = crate::BitReader;
///Field `JUSTIFY` writer - Justification of I2S Data
pub type JustifyW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMASPLIT` reader - Separate DMA Request for Left/Right Data
pub type DmasplitR = crate::BitReader;
///Field `DMASPLIT` writer - Separate DMA Request for Left/Right Data
pub type DmasplitW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DELAY` reader - Delay on I2S Data
pub type DelayR = crate::BitReader;
///Field `DELAY` writer - Delay on I2S Data
pub type DelayW<'a, REG> = crate::BitWriter<'a, REG>;
///I2S Word Format
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FORMAT {
    ///0: 32-bit word, 32-bit data
    W32d32 = 0,
    ///1: 32-bit word, 32-bit data with 8 lsb masked
    W32d24m = 1,
    ///2: 32-bit word, 24-bit data
    W32d24 = 2,
    ///3: 32-bit word, 16-bit data
    W32d16 = 3,
    ///4: 32-bit word, 8-bit data
    W32d8 = 4,
    ///5: 16-bit word, 16-bit data
    W16d16 = 5,
    ///6: 16-bit word, 8-bit data
    W16d8 = 6,
    ///7: 8-bit word, 8-bit data
    W8d8 = 7,
}
impl From<FORMAT> for u8 {
    #[inline(always)]
    fn from(variant: FORMAT) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FORMAT {
    type Ux = u8;
}
impl crate::IsEnum for FORMAT {}
///Field `FORMAT` reader - I2S Word Format
pub type FormatR = crate::FieldReader<FORMAT>;
impl FormatR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FORMAT {
        match self.bits {
            0 => FORMAT::W32d32,
            1 => FORMAT::W32d24m,
            2 => FORMAT::W32d24,
            3 => FORMAT::W32d16,
            4 => FORMAT::W32d8,
            5 => FORMAT::W16d16,
            6 => FORMAT::W16d8,
            7 => FORMAT::W8d8,
            _ => unreachable!(),
        }
    }
    ///32-bit word, 32-bit data
    #[inline(always)]
    pub fn is_w32d32(&self) -> bool {
        *self == FORMAT::W32d32
    }
    ///32-bit word, 32-bit data with 8 lsb masked
    #[inline(always)]
    pub fn is_w32d24m(&self) -> bool {
        *self == FORMAT::W32d24m
    }
    ///32-bit word, 24-bit data
    #[inline(always)]
    pub fn is_w32d24(&self) -> bool {
        *self == FORMAT::W32d24
    }
    ///32-bit word, 16-bit data
    #[inline(always)]
    pub fn is_w32d16(&self) -> bool {
        *self == FORMAT::W32d16
    }
    ///32-bit word, 8-bit data
    #[inline(always)]
    pub fn is_w32d8(&self) -> bool {
        *self == FORMAT::W32d8
    }
    ///16-bit word, 16-bit data
    #[inline(always)]
    pub fn is_w16d16(&self) -> bool {
        *self == FORMAT::W16d16
    }
    ///16-bit word, 8-bit data
    #[inline(always)]
    pub fn is_w16d8(&self) -> bool {
        *self == FORMAT::W16d8
    }
    ///8-bit word, 8-bit data
    #[inline(always)]
    pub fn is_w8d8(&self) -> bool {
        *self == FORMAT::W8d8
    }
}
///Field `FORMAT` writer - I2S Word Format
pub type FormatW<'a, REG> = crate::FieldWriter<'a, REG, 3, FORMAT, crate::Safe>;
impl<'a, REG> FormatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///32-bit word, 32-bit data
    #[inline(always)]
    pub fn w32d32(self) -> &'a mut crate::W<REG> {
        self.variant(FORMAT::W32d32)
    }
    ///32-bit word, 32-bit data with 8 lsb masked
    #[inline(always)]
    pub fn w32d24m(self) -> &'a mut crate::W<REG> {
        self.variant(FORMAT::W32d24m)
    }
    ///32-bit word, 24-bit data
    #[inline(always)]
    pub fn w32d24(self) -> &'a mut crate::W<REG> {
        self.variant(FORMAT::W32d24)
    }
    ///32-bit word, 16-bit data
    #[inline(always)]
    pub fn w32d16(self) -> &'a mut crate::W<REG> {
        self.variant(FORMAT::W32d16)
    }
    ///32-bit word, 8-bit data
    #[inline(always)]
    pub fn w32d8(self) -> &'a mut crate::W<REG> {
        self.variant(FORMAT::W32d8)
    }
    ///16-bit word, 16-bit data
    #[inline(always)]
    pub fn w16d16(self) -> &'a mut crate::W<REG> {
        self.variant(FORMAT::W16d16)
    }
    ///16-bit word, 8-bit data
    #[inline(always)]
    pub fn w16d8(self) -> &'a mut crate::W<REG> {
        self.variant(FORMAT::W16d8)
    }
    ///8-bit word, 8-bit data
    #[inline(always)]
    pub fn w8d8(self) -> &'a mut crate::W<REG> {
        self.variant(FORMAT::W8d8)
    }
}
impl R {
    ///Bit 0 - Enable I2S Mode
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Stero or Mono
    #[inline(always)]
    pub fn mono(&self) -> MonoR {
        MonoR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Justification of I2S Data
    #[inline(always)]
    pub fn justify(&self) -> JustifyR {
        JustifyR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Separate DMA Request for Left/Right Data
    #[inline(always)]
    pub fn dmasplit(&self) -> DmasplitR {
        DmasplitR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Delay on I2S Data
    #[inline(always)]
    pub fn delay(&self) -> DelayR {
        DelayR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 8:10 - I2S Word Format
    #[inline(always)]
    pub fn format(&self) -> FormatR {
        FormatR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2SCTRL")
            .field("en", &self.en())
            .field("mono", &self.mono())
            .field("justify", &self.justify())
            .field("dmasplit", &self.dmasplit())
            .field("delay", &self.delay())
            .field("format", &self.format())
            .finish()
    }
}
impl W {
    ///Bit 0 - Enable I2S Mode
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<I2SCTRLrs> {
        EnW::new(self, 0)
    }
    ///Bit 1 - Stero or Mono
    #[inline(always)]
    #[must_use]
    pub fn mono(&mut self) -> MonoW<I2SCTRLrs> {
        MonoW::new(self, 1)
    }
    ///Bit 2 - Justification of I2S Data
    #[inline(always)]
    #[must_use]
    pub fn justify(&mut self) -> JustifyW<I2SCTRLrs> {
        JustifyW::new(self, 2)
    }
    ///Bit 3 - Separate DMA Request for Left/Right Data
    #[inline(always)]
    #[must_use]
    pub fn dmasplit(&mut self) -> DmasplitW<I2SCTRLrs> {
        DmasplitW::new(self, 3)
    }
    ///Bit 4 - Delay on I2S Data
    #[inline(always)]
    #[must_use]
    pub fn delay(&mut self) -> DelayW<I2SCTRLrs> {
        DelayW::new(self, 4)
    }
    ///Bits 8:10 - I2S Word Format
    #[inline(always)]
    #[must_use]
    pub fn format(&mut self) -> FormatW<I2SCTRLrs> {
        FormatW::new(self, 8)
    }
}
///I2S Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`i2sctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2sctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct I2SCTRLrs;
impl crate::RegisterSpec for I2SCTRLrs {
    type Ux = u32;
}
///`read()` method returns [`i2sctrl::R`](R) reader structure
impl crate::Readable for I2SCTRLrs {}
///`write(|w| ..)` method takes [`i2sctrl::W`](W) writer structure
impl crate::Writable for I2SCTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets I2SCTRL to value 0
impl crate::Resettable for I2SCTRLrs {
    const RESET_VALUE: u32 = 0;
}
