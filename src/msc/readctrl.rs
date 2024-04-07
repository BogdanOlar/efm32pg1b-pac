#[doc = "Register `READCTRL` reader"]
pub type R = crate::R<READCTRLrs>;
#[doc = "Register `READCTRL` writer"]
pub type W = crate::W<READCTRLrs>;
#[doc = "Field `IFCDIS` reader - Internal Flash Cache Disable"]
pub type IfcdisR = crate::BitReader;
#[doc = "Field `IFCDIS` writer - Internal Flash Cache Disable"]
pub type IfcdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AIDIS` reader - Automatic Invalidate Disable"]
pub type AidisR = crate::BitReader;
#[doc = "Field `AIDIS` writer - Automatic Invalidate Disable"]
pub type AidisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICCDIS` reader - Interrupt Context Cache Disable"]
pub type IccdisR = crate::BitReader;
#[doc = "Field `ICCDIS` writer - Interrupt Context Cache Disable"]
pub type IccdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PREFETCH` reader - Prefetch Mode"]
pub type PrefetchR = crate::BitReader;
#[doc = "Field `PREFETCH` writer - Prefetch Mode"]
pub type PrefetchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USEHPROT` reader - AHB_HPROT Mode"]
pub type UsehprotR = crate::BitReader;
#[doc = "Field `USEHPROT` writer - AHB_HPROT Mode"]
pub type UsehprotW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Read Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE {
    #[doc = "0: Zero wait-states inserted in fetch or read transfers"]
    Ws0 = 0,
    #[doc = "1: One wait-state inserted for each fetch or read transfer. See Flash Wait-States table for details"]
    Ws1 = 1,
}
impl From<MODE> for u8 {
    #[inline(always)]
    fn from(variant: MODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE {
    type Ux = u8;
}
impl crate::IsEnum for MODE {}
#[doc = "Field `MODE` reader - Read Mode"]
pub type ModeR = crate::FieldReader<MODE>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MODE> {
        match self.bits {
            0 => Some(MODE::Ws0),
            1 => Some(MODE::Ws1),
            _ => None,
        }
    }
    #[doc = "Zero wait-states inserted in fetch or read transfers"]
    #[inline(always)]
    pub fn is_ws0(&self) -> bool {
        *self == MODE::Ws0
    }
    #[doc = "One wait-state inserted for each fetch or read transfer. See Flash Wait-States table for details"]
    #[inline(always)]
    pub fn is_ws1(&self) -> bool {
        *self == MODE::Ws1
    }
}
#[doc = "Field `MODE` writer - Read Mode"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, MODE>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Zero wait-states inserted in fetch or read transfers"]
    #[inline(always)]
    pub fn ws0(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Ws0)
    }
    #[doc = "One wait-state inserted for each fetch or read transfer. See Flash Wait-States table for details"]
    #[inline(always)]
    pub fn ws1(self) -> &'a mut crate::W<REG> {
        self.variant(MODE::Ws1)
    }
}
#[doc = "Field `SCBTP` reader - Suppress Conditional Branch Target Perfetch"]
pub type ScbtpR = crate::BitReader;
#[doc = "Field `SCBTP` writer - Suppress Conditional Branch Target Perfetch"]
pub type ScbtpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - Internal Flash Cache Disable"]
    #[inline(always)]
    pub fn ifcdis(&self) -> IfcdisR {
        IfcdisR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Automatic Invalidate Disable"]
    #[inline(always)]
    pub fn aidis(&self) -> AidisR {
        AidisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt Context Cache Disable"]
    #[inline(always)]
    pub fn iccdis(&self) -> IccdisR {
        IccdisR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Prefetch Mode"]
    #[inline(always)]
    pub fn prefetch(&self) -> PrefetchR {
        PrefetchR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AHB_HPROT Mode"]
    #[inline(always)]
    pub fn usehprot(&self) -> UsehprotR {
        UsehprotR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Read Mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 28 - Suppress Conditional Branch Target Perfetch"]
    #[inline(always)]
    pub fn scbtp(&self) -> ScbtpR {
        ScbtpR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Internal Flash Cache Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ifcdis(&mut self) -> IfcdisW<READCTRLrs> {
        IfcdisW::new(self, 3)
    }
    #[doc = "Bit 4 - Automatic Invalidate Disable"]
    #[inline(always)]
    #[must_use]
    pub fn aidis(&mut self) -> AidisW<READCTRLrs> {
        AidisW::new(self, 4)
    }
    #[doc = "Bit 5 - Interrupt Context Cache Disable"]
    #[inline(always)]
    #[must_use]
    pub fn iccdis(&mut self) -> IccdisW<READCTRLrs> {
        IccdisW::new(self, 5)
    }
    #[doc = "Bit 8 - Prefetch Mode"]
    #[inline(always)]
    #[must_use]
    pub fn prefetch(&mut self) -> PrefetchW<READCTRLrs> {
        PrefetchW::new(self, 8)
    }
    #[doc = "Bit 9 - AHB_HPROT Mode"]
    #[inline(always)]
    #[must_use]
    pub fn usehprot(&mut self) -> UsehprotW<READCTRLrs> {
        UsehprotW::new(self, 9)
    }
    #[doc = "Bits 24:25 - Read Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<READCTRLrs> {
        ModeW::new(self, 24)
    }
    #[doc = "Bit 28 - Suppress Conditional Branch Target Perfetch"]
    #[inline(always)]
    #[must_use]
    pub fn scbtp(&mut self) -> ScbtpW<READCTRLrs> {
        ScbtpW::new(self, 28)
    }
}
#[doc = "Read Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`readctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`readctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct READCTRLrs;
impl crate::RegisterSpec for READCTRLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`readctrl::R`](R) reader structure"]
impl crate::Readable for READCTRLrs {}
#[doc = "`write(|w| ..)` method takes [`readctrl::W`](W) writer structure"]
impl crate::Writable for READCTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets READCTRL to value 0x0100_0100"]
impl crate::Resettable for READCTRLrs {
    const RESET_VALUE: u32 = 0x0100_0100;
}
