#[doc = "Register `HFPRESC` reader"]
pub type R = crate::R<HFPRESCrs>;
#[doc = "Register `HFPRESC` writer"]
pub type W = crate::W<HFPRESCrs>;
#[doc = "Field `PRESC` reader - HFCLK Prescaler"]
pub type PRESC_R = crate::FieldReader<PRESC>;
#[doc = "HFCLK Prescaler\n\nValue on reset: 0"]
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
#[doc = "Field `PRESC` writer - HFCLK Prescaler"]
pub type PRESC_W<'a, REG> = crate::FieldWriter<'a, REG, 5, PRESC>;
impl<'a, REG> PRESC_W<'a, REG>
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
#[doc = "Field `HFCLKLEPRESC` reader - HFCLKLE Prescaler"]
pub type HFCLKLEPRESC_R = crate::BitReader<HFCLKLEPRESC>;
#[doc = "HFCLKLE Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HFCLKLEPRESC {
    #[doc = "0: HFCLKLE is HFBUSCLKLE divided by 2."]
    Div2 = 0,
    #[doc = "1: HFCLKLE is HFBUSCLKLE divided by 4."]
    Div4 = 1,
}
impl From<HFCLKLEPRESC> for bool {
    #[inline(always)]
    fn from(variant: HFCLKLEPRESC) -> Self {
        variant as u8 != 0
    }
}
impl HFCLKLEPRESC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HFCLKLEPRESC {
        match self.bits {
            false => HFCLKLEPRESC::Div2,
            true => HFCLKLEPRESC::Div4,
        }
    }
    #[doc = "HFCLKLE is HFBUSCLKLE divided by 2."]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == HFCLKLEPRESC::Div2
    }
    #[doc = "HFCLKLE is HFBUSCLKLE divided by 4."]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == HFCLKLEPRESC::Div4
    }
}
#[doc = "Field `HFCLKLEPRESC` writer - HFCLKLE Prescaler"]
pub type HFCLKLEPRESC_W<'a, REG> = crate::BitWriter<'a, REG, HFCLKLEPRESC>;
impl<'a, REG> HFCLKLEPRESC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HFCLKLE is HFBUSCLKLE divided by 2."]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(HFCLKLEPRESC::Div2)
    }
    #[doc = "HFCLKLE is HFBUSCLKLE divided by 4."]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(HFCLKLEPRESC::Div4)
    }
}
impl R {
    #[doc = "Bits 8:12 - HFCLK Prescaler"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - HFCLKLE Prescaler"]
    #[inline(always)]
    pub fn hfclklepresc(&self) -> HFCLKLEPRESC_R {
        HFCLKLEPRESC_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 8:12 - HFCLK Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PRESC_W<HFPRESCrs> {
        PRESC_W::new(self, 8)
    }
    #[doc = "Bit 24 - HFCLKLE Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn hfclklepresc(&mut self) -> HFCLKLEPRESC_W<HFPRESCrs> {
        HFCLKLEPRESC_W::new(self, 24)
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
#[doc = "High Frequency Clock Prescaler Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfpresc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfpresc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HFPRESCrs;
impl crate::RegisterSpec for HFPRESCrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfpresc::R`](R) reader structure"]
impl crate::Readable for HFPRESCrs {}
#[doc = "`write(|w| ..)` method takes [`hfpresc::W`](W) writer structure"]
impl crate::Writable for HFPRESCrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HFPRESC to value 0"]
impl crate::Resettable for HFPRESCrs {
    const RESET_VALUE: u32 = 0;
}
