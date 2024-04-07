#[doc = "Register `HFEXPPRESC` reader"]
pub type R = crate::R<HFEXPPRESCrs>;
#[doc = "Register `HFEXPPRESC` writer"]
pub type W = crate::W<HFEXPPRESCrs>;
#[doc = "HFEXPCLK Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRESC {
    #[doc = "0: `0`"]
    Nodivision = 0,
}
impl From<PRESC> for u8 {
    #[inline(always)]
    fn from(variant: PRESC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRESC {
    type Ux = u8;
}
impl crate::IsEnum for PRESC {}
#[doc = "Field `PRESC` reader - HFEXPCLK Prescaler"]
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
#[doc = "Field `PRESC` writer - HFEXPCLK Prescaler"]
pub type PrescW<'a, REG> = crate::FieldWriter<'a, REG, 5, PRESC>;
impl<'a, REG> PrescW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn nodivision(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Nodivision)
    }
}
impl R {
    #[doc = "Bits 8:12 - HFEXPCLK Prescaler"]
    #[inline(always)]
    pub fn presc(&self) -> PrescR {
        PrescR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:12 - HFEXPCLK Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PrescW<HFEXPPRESCrs> {
        PrescW::new(self, 8)
    }
}
#[doc = "High Frequency Export Clock Prescaler Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfexppresc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfexppresc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HFEXPPRESCrs;
impl crate::RegisterSpec for HFEXPPRESCrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfexppresc::R`](R) reader structure"]
impl crate::Readable for HFEXPPRESCrs {}
#[doc = "`write(|w| ..)` method takes [`hfexppresc::W`](W) writer structure"]
impl crate::Writable for HFEXPPRESCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HFEXPPRESC to value 0"]
impl crate::Resettable for HFEXPPRESCrs {
    const RESET_VALUE: u32 = 0;
}
