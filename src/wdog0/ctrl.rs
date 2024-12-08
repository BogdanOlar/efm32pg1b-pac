///Register `CTRL` reader
pub type R = crate::R<CTRLrs>;
///Register `CTRL` writer
pub type W = crate::W<CTRLrs>;
///Field `EN` reader - Watchdog Timer Enable
pub type EnR = crate::BitReader;
///Field `EN` writer - Watchdog Timer Enable
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DEBUGRUN` reader - Debug Mode Run Enable
pub type DebugrunR = crate::BitReader;
///Field `DEBUGRUN` writer - Debug Mode Run Enable
pub type DebugrunW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EM2RUN` reader - Energy Mode 2 Run Enable
pub type Em2runR = crate::BitReader;
///Field `EM2RUN` writer - Energy Mode 2 Run Enable
pub type Em2runW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EM3RUN` reader - Energy Mode 3 Run Enable
pub type Em3runR = crate::BitReader;
///Field `EM3RUN` writer - Energy Mode 3 Run Enable
pub type Em3runW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LOCK` reader - Configuration Lock
pub type LockR = crate::BitReader;
///Field `LOCK` writer - Configuration Lock
pub type LockW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EM4BLOCK` reader - Energy Mode 4 Block
pub type Em4blockR = crate::BitReader;
///Field `EM4BLOCK` writer - Energy Mode 4 Block
pub type Em4blockW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWOSCBLOCK` reader - Software Oscillator Disable Block
pub type SwoscblockR = crate::BitReader;
///Field `SWOSCBLOCK` writer - Software Oscillator Disable Block
pub type SwoscblockW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PERSEL` reader - Watchdog Timeout Period Select
pub type PerselR = crate::FieldReader;
///Field `PERSEL` writer - Watchdog Timeout Period Select
pub type PerselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Watchdog Clock Select
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKSEL {
    ///0: ULFRCO
    Ulfrco = 0,
    ///1: LFRCO
    Lfrco = 1,
    ///2: LFXO
    Lfxo = 2,
}
impl From<CLKSEL> for u8 {
    #[inline(always)]
    fn from(variant: CLKSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CLKSEL {
    type Ux = u8;
}
impl crate::IsEnum for CLKSEL {}
///Field `CLKSEL` reader - Watchdog Clock Select
pub type ClkselR = crate::FieldReader<CLKSEL>;
impl ClkselR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CLKSEL> {
        match self.bits {
            0 => Some(CLKSEL::Ulfrco),
            1 => Some(CLKSEL::Lfrco),
            2 => Some(CLKSEL::Lfxo),
            _ => None,
        }
    }
    ///ULFRCO
    #[inline(always)]
    pub fn is_ulfrco(&self) -> bool {
        *self == CLKSEL::Ulfrco
    }
    ///LFRCO
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == CLKSEL::Lfrco
    }
    ///LFXO
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == CLKSEL::Lfxo
    }
}
///Field `CLKSEL` writer - Watchdog Clock Select
pub type ClkselW<'a, REG> = crate::FieldWriter<'a, REG, 2, CLKSEL>;
impl<'a, REG> ClkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///ULFRCO
    #[inline(always)]
    pub fn ulfrco(self) -> &'a mut crate::W<REG> {
        self.variant(CLKSEL::Ulfrco)
    }
    ///LFRCO
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut crate::W<REG> {
        self.variant(CLKSEL::Lfrco)
    }
    ///LFXO
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut crate::W<REG> {
        self.variant(CLKSEL::Lfxo)
    }
}
///Field `WARNSEL` reader - Watchdog Timeout Period Select
pub type WarnselR = crate::FieldReader;
///Field `WARNSEL` writer - Watchdog Timeout Period Select
pub type WarnselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `WINSEL` reader - Watchdog Illegal Window Select
pub type WinselR = crate::FieldReader;
///Field `WINSEL` writer - Watchdog Illegal Window Select
pub type WinselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `CLRSRC` reader - Watchdog Clear Source
pub type ClrsrcR = crate::BitReader;
///Field `CLRSRC` writer - Watchdog Clear Source
pub type ClrsrcW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WDOGRSTDIS` reader - Watchdog Reset Disable
pub type WdogrstdisR = crate::BitReader;
///Field `WDOGRSTDIS` writer - Watchdog Reset Disable
pub type WdogrstdisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Watchdog Timer Enable
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Debug Mode Run Enable
    #[inline(always)]
    pub fn debugrun(&self) -> DebugrunR {
        DebugrunR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Energy Mode 2 Run Enable
    #[inline(always)]
    pub fn em2run(&self) -> Em2runR {
        Em2runR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Energy Mode 3 Run Enable
    #[inline(always)]
    pub fn em3run(&self) -> Em3runR {
        Em3runR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Configuration Lock
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Energy Mode 4 Block
    #[inline(always)]
    pub fn em4block(&self) -> Em4blockR {
        Em4blockR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Software Oscillator Disable Block
    #[inline(always)]
    pub fn swoscblock(&self) -> SwoscblockR {
        SwoscblockR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 8:11 - Watchdog Timeout Period Select
    #[inline(always)]
    pub fn persel(&self) -> PerselR {
        PerselR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:13 - Watchdog Clock Select
    #[inline(always)]
    pub fn clksel(&self) -> ClkselR {
        ClkselR::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 16:17 - Watchdog Timeout Period Select
    #[inline(always)]
    pub fn warnsel(&self) -> WarnselR {
        WarnselR::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 24:26 - Watchdog Illegal Window Select
    #[inline(always)]
    pub fn winsel(&self) -> WinselR {
        WinselR::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bit 30 - Watchdog Clear Source
    #[inline(always)]
    pub fn clrsrc(&self) -> ClrsrcR {
        ClrsrcR::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Watchdog Reset Disable
    #[inline(always)]
    pub fn wdogrstdis(&self) -> WdogrstdisR {
        WdogrstdisR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("en", &self.en())
            .field("debugrun", &self.debugrun())
            .field("em2run", &self.em2run())
            .field("em3run", &self.em3run())
            .field("lock", &self.lock())
            .field("em4block", &self.em4block())
            .field("swoscblock", &self.swoscblock())
            .field("persel", &self.persel())
            .field("clksel", &self.clksel())
            .field("warnsel", &self.warnsel())
            .field("winsel", &self.winsel())
            .field("clrsrc", &self.clrsrc())
            .field("wdogrstdis", &self.wdogrstdis())
            .finish()
    }
}
impl W {
    ///Bit 0 - Watchdog Timer Enable
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<CTRLrs> {
        EnW::new(self, 0)
    }
    ///Bit 1 - Debug Mode Run Enable
    #[inline(always)]
    #[must_use]
    pub fn debugrun(&mut self) -> DebugrunW<CTRLrs> {
        DebugrunW::new(self, 1)
    }
    ///Bit 2 - Energy Mode 2 Run Enable
    #[inline(always)]
    #[must_use]
    pub fn em2run(&mut self) -> Em2runW<CTRLrs> {
        Em2runW::new(self, 2)
    }
    ///Bit 3 - Energy Mode 3 Run Enable
    #[inline(always)]
    #[must_use]
    pub fn em3run(&mut self) -> Em3runW<CTRLrs> {
        Em3runW::new(self, 3)
    }
    ///Bit 4 - Configuration Lock
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LockW<CTRLrs> {
        LockW::new(self, 4)
    }
    ///Bit 5 - Energy Mode 4 Block
    #[inline(always)]
    #[must_use]
    pub fn em4block(&mut self) -> Em4blockW<CTRLrs> {
        Em4blockW::new(self, 5)
    }
    ///Bit 6 - Software Oscillator Disable Block
    #[inline(always)]
    #[must_use]
    pub fn swoscblock(&mut self) -> SwoscblockW<CTRLrs> {
        SwoscblockW::new(self, 6)
    }
    ///Bits 8:11 - Watchdog Timeout Period Select
    #[inline(always)]
    #[must_use]
    pub fn persel(&mut self) -> PerselW<CTRLrs> {
        PerselW::new(self, 8)
    }
    ///Bits 12:13 - Watchdog Clock Select
    #[inline(always)]
    #[must_use]
    pub fn clksel(&mut self) -> ClkselW<CTRLrs> {
        ClkselW::new(self, 12)
    }
    ///Bits 16:17 - Watchdog Timeout Period Select
    #[inline(always)]
    #[must_use]
    pub fn warnsel(&mut self) -> WarnselW<CTRLrs> {
        WarnselW::new(self, 16)
    }
    ///Bits 24:26 - Watchdog Illegal Window Select
    #[inline(always)]
    #[must_use]
    pub fn winsel(&mut self) -> WinselW<CTRLrs> {
        WinselW::new(self, 24)
    }
    ///Bit 30 - Watchdog Clear Source
    #[inline(always)]
    #[must_use]
    pub fn clrsrc(&mut self) -> ClrsrcW<CTRLrs> {
        ClrsrcW::new(self, 30)
    }
    ///Bit 31 - Watchdog Reset Disable
    #[inline(always)]
    #[must_use]
    pub fn wdogrstdis(&mut self) -> WdogrstdisW<CTRLrs> {
        WdogrstdisW::new(self, 31)
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
///`reset()` method sets CTRL to value 0x0f00
impl crate::Resettable for CTRLrs {
    const RESET_VALUE: u32 = 0x0f00;
}
