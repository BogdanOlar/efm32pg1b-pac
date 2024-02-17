#[doc = "Register `HFCOREPRESC` reader"]
pub type R = crate::R<HFCOREPRESCrs>;
#[doc = "Register `HFCOREPRESC` writer"]
pub type W = crate::W<HFCOREPRESCrs>;
#[doc = "Field `PRESC` reader - HFCORECLK Prescaler"]
pub type PRESC_R = crate::FieldReader<PRESC>;
#[doc = "HFCORECLK Prescaler\n\nValue on reset: 0"]
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
impl PRESC_R {
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
pub type PRESC_W<'a, REG> = crate::FieldWriter<'a, REG, 9, PRESC>;
impl<'a, REG> PRESC_W<'a, REG>
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
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 8) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 8:16 - HFCORECLK Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PRESC_W<HFCOREPRESCrs> {
        PRESC_W::new(self, 8)
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
#[doc = "High Frequency Core Clock Prescaler Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfcorepresc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfcorepresc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HFCOREPRESCrs;
impl crate::RegisterSpec for HFCOREPRESCrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfcorepresc::R`](R) reader structure"]
impl crate::Readable for HFCOREPRESCrs {}
#[doc = "`write(|w| ..)` method takes [`hfcorepresc::W`](W) writer structure"]
impl crate::Writable for HFCOREPRESCrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HFCOREPRESC to value 0"]
impl crate::Resettable for HFCOREPRESCrs {
    const RESET_VALUE: u32 = 0;
}
