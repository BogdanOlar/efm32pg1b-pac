#[doc = "Register `HFCOREPRESC` reader"]
pub type R = crate::R<HFCOREPRESCrs>;
#[doc = "Register `HFCOREPRESC` writer"]
pub type W = crate::W<HFCOREPRESCrs>;
#[doc = "HFCORECLK Prescaler\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum PRESC {
    #[doc = "0: `0`"]
    Nodivision = 0,
}
impl From<PRESC> for u16 {
    #[inline(always)]
    fn from(variant: PRESC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRESC {
    type Ux = u16;
}
impl crate::IsEnum for PRESC {}
#[doc = "Field `PRESC` reader - HFCORECLK Prescaler"]
pub type PrescR = crate::FieldReader<PRESC>;
impl PrescR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PRESC> {
        match self.bits {
            0 => Some(PRESC::Nodivision),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_nodivision(&self) -> bool {
        *self == PRESC::Nodivision
    }
}
#[doc = "Field `PRESC` writer - HFCORECLK Prescaler"]
pub type PrescW<'a, REG> = crate::FieldWriter<'a, REG, 9, PRESC>;
impl<'a, REG> PrescW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn nodivision(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Nodivision)
    }
}
impl R {
    #[doc = "Bits 8:16 - HFCORECLK Prescaler"]
    #[inline(always)]
    pub fn presc(&self) -> PrescR {
        PrescR::new(((self.bits >> 8) & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HFCOREPRESC")
            .field("presc", &self.presc())
            .finish()
    }
}
impl W {
    #[doc = "Bits 8:16 - HFCORECLK Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PrescW<HFCOREPRESCrs> {
        PrescW::new(self, 8)
    }
}
#[doc = "High Frequency Core Clock Prescaler Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hfcorepresc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfcorepresc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HFCOREPRESCrs;
impl crate::RegisterSpec for HFCOREPRESCrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfcorepresc::R`](R) reader structure"]
impl crate::Readable for HFCOREPRESCrs {}
#[doc = "`write(|w| ..)` method takes [`hfcorepresc::W`](W) writer structure"]
impl crate::Writable for HFCOREPRESCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HFCOREPRESC to value 0"]
impl crate::Resettable for HFCOREPRESCrs {
    const RESET_VALUE: u32 = 0;
}
