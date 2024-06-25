#[doc = "Register `STARTUP` reader"]
pub type R = crate::R<STARTUPrs>;
#[doc = "Register `STARTUP` writer"]
pub type W = crate::W<STARTUPrs>;
#[doc = "Field `STDLY0` reader - Startup Delay 0"]
pub type Stdly0R = crate::FieldReader<u16>;
#[doc = "Field `STDLY0` writer - Startup Delay 0"]
pub type Stdly0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `STDLY1` reader - Startup Delay 0"]
pub type Stdly1R = crate::FieldReader<u16>;
#[doc = "Field `STDLY1` writer - Startup Delay 0"]
pub type Stdly1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `ASTWAIT` reader - Active Startup Wait"]
pub type AstwaitR = crate::BitReader;
#[doc = "Field `ASTWAIT` writer - Active Startup Wait"]
pub type AstwaitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STWSEN` reader - Startup Waitstates Enable"]
pub type StwsenR = crate::BitReader;
#[doc = "Field `STWSEN` writer - Startup Waitstates Enable"]
pub type StwsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STWSAEN` reader - Startup Waitstates Always Enable"]
pub type StwsaenR = crate::BitReader;
#[doc = "Field `STWSAEN` writer - Startup Waitstates Always Enable"]
pub type StwsaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STWS` reader - Startup Waitstates"]
pub type StwsR = crate::FieldReader;
#[doc = "Field `STWS` writer - Startup Waitstates"]
pub type StwsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:9 - Startup Delay 0"]
    #[inline(always)]
    pub fn stdly0(&self) -> Stdly0R {
        Stdly0R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 12:21 - Startup Delay 0"]
    #[inline(always)]
    pub fn stdly1(&self) -> Stdly1R {
        Stdly1R::new(((self.bits >> 12) & 0x03ff) as u16)
    }
    #[doc = "Bit 24 - Active Startup Wait"]
    #[inline(always)]
    pub fn astwait(&self) -> AstwaitR {
        AstwaitR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Startup Waitstates Enable"]
    #[inline(always)]
    pub fn stwsen(&self) -> StwsenR {
        StwsenR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Startup Waitstates Always Enable"]
    #[inline(always)]
    pub fn stwsaen(&self) -> StwsaenR {
        StwsaenR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 28:30 - Startup Waitstates"]
    #[inline(always)]
    pub fn stws(&self) -> StwsR {
        StwsR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STARTUP")
            .field("stdly0", &self.stdly0())
            .field("stdly1", &self.stdly1())
            .field("astwait", &self.astwait())
            .field("stwsen", &self.stwsen())
            .field("stwsaen", &self.stwsaen())
            .field("stws", &self.stws())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:9 - Startup Delay 0"]
    #[inline(always)]
    #[must_use]
    pub fn stdly0(&mut self) -> Stdly0W<STARTUPrs> {
        Stdly0W::new(self, 0)
    }
    #[doc = "Bits 12:21 - Startup Delay 0"]
    #[inline(always)]
    #[must_use]
    pub fn stdly1(&mut self) -> Stdly1W<STARTUPrs> {
        Stdly1W::new(self, 12)
    }
    #[doc = "Bit 24 - Active Startup Wait"]
    #[inline(always)]
    #[must_use]
    pub fn astwait(&mut self) -> AstwaitW<STARTUPrs> {
        AstwaitW::new(self, 24)
    }
    #[doc = "Bit 25 - Startup Waitstates Enable"]
    #[inline(always)]
    #[must_use]
    pub fn stwsen(&mut self) -> StwsenW<STARTUPrs> {
        StwsenW::new(self, 25)
    }
    #[doc = "Bit 26 - Startup Waitstates Always Enable"]
    #[inline(always)]
    #[must_use]
    pub fn stwsaen(&mut self) -> StwsaenW<STARTUPrs> {
        StwsaenW::new(self, 26)
    }
    #[doc = "Bits 28:30 - Startup Waitstates"]
    #[inline(always)]
    #[must_use]
    pub fn stws(&mut self) -> StwsW<STARTUPrs> {
        StwsW::new(self, 28)
    }
}
#[doc = "Startup Control\n\nYou can [`read`](crate::Reg::read) this register and get [`startup::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`startup::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STARTUPrs;
impl crate::RegisterSpec for STARTUPrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`startup::R`](R) reader structure"]
impl crate::Readable for STARTUPrs {}
#[doc = "`write(|w| ..)` method takes [`startup::W`](W) writer structure"]
impl crate::Writable for STARTUPrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STARTUP to value 0x1300_104d"]
impl crate::Resettable for STARTUPrs {
    const RESET_VALUE: u32 = 0x1300_104d;
}
