#[doc = "Register `BIASPROG` reader"]
pub type R = crate::R<BIASPROGrs>;
#[doc = "Register `BIASPROG` writer"]
pub type W = crate::W<BIASPROGrs>;
#[doc = "Field `ADCBIASPROG` reader - Bias Programming Value of Analog ADC Block"]
pub type ADCBIASPROG_R = crate::FieldReader<ADCBIASPROG>;
#[doc = "Bias Programming Value of Analog ADC Block\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADCBIASPROG {
    #[doc = "0: Normal power (use for 1Msps operation)"]
    Normal = 0,
    #[doc = "4: Scaling bias to 1/2"]
    Scale2 = 4,
    #[doc = "8: Scaling bias to 1/4"]
    Scale4 = 8,
    #[doc = "12: Scaling bias to 1/8"]
    Scale8 = 12,
    #[doc = "14: Scaling bias to 1/16"]
    Scale16 = 14,
    #[doc = "15: Scaling bias to 1/32"]
    Scale32 = 15,
}
impl From<ADCBIASPROG> for u8 {
    #[inline(always)]
    fn from(variant: ADCBIASPROG) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADCBIASPROG {
    type Ux = u8;
}
impl ADCBIASPROG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ADCBIASPROG> {
        match self.bits {
            0 => Some(ADCBIASPROG::Normal),
            4 => Some(ADCBIASPROG::Scale2),
            8 => Some(ADCBIASPROG::Scale4),
            12 => Some(ADCBIASPROG::Scale8),
            14 => Some(ADCBIASPROG::Scale16),
            15 => Some(ADCBIASPROG::Scale32),
            _ => None,
        }
    }
    #[doc = "Normal power (use for 1Msps operation)"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == ADCBIASPROG::Normal
    }
    #[doc = "Scaling bias to 1/2"]
    #[inline(always)]
    pub fn is_scale2(&self) -> bool {
        *self == ADCBIASPROG::Scale2
    }
    #[doc = "Scaling bias to 1/4"]
    #[inline(always)]
    pub fn is_scale4(&self) -> bool {
        *self == ADCBIASPROG::Scale4
    }
    #[doc = "Scaling bias to 1/8"]
    #[inline(always)]
    pub fn is_scale8(&self) -> bool {
        *self == ADCBIASPROG::Scale8
    }
    #[doc = "Scaling bias to 1/16"]
    #[inline(always)]
    pub fn is_scale16(&self) -> bool {
        *self == ADCBIASPROG::Scale16
    }
    #[doc = "Scaling bias to 1/32"]
    #[inline(always)]
    pub fn is_scale32(&self) -> bool {
        *self == ADCBIASPROG::Scale32
    }
}
#[doc = "Field `ADCBIASPROG` writer - Bias Programming Value of Analog ADC Block"]
pub type ADCBIASPROG_W<'a, REG> = crate::FieldWriter<'a, REG, 4, ADCBIASPROG>;
impl<'a, REG> ADCBIASPROG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal power (use for 1Msps operation)"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(ADCBIASPROG::Normal)
    }
    #[doc = "Scaling bias to 1/2"]
    #[inline(always)]
    pub fn scale2(self) -> &'a mut crate::W<REG> {
        self.variant(ADCBIASPROG::Scale2)
    }
    #[doc = "Scaling bias to 1/4"]
    #[inline(always)]
    pub fn scale4(self) -> &'a mut crate::W<REG> {
        self.variant(ADCBIASPROG::Scale4)
    }
    #[doc = "Scaling bias to 1/8"]
    #[inline(always)]
    pub fn scale8(self) -> &'a mut crate::W<REG> {
        self.variant(ADCBIASPROG::Scale8)
    }
    #[doc = "Scaling bias to 1/16"]
    #[inline(always)]
    pub fn scale16(self) -> &'a mut crate::W<REG> {
        self.variant(ADCBIASPROG::Scale16)
    }
    #[doc = "Scaling bias to 1/32"]
    #[inline(always)]
    pub fn scale32(self) -> &'a mut crate::W<REG> {
        self.variant(ADCBIASPROG::Scale32)
    }
}
#[doc = "Field `VFAULTCLR` reader - Clear VREFOF Flag"]
pub type VFAULTCLR_R = crate::BitReader;
#[doc = "Field `VFAULTCLR` writer - Clear VREFOF Flag"]
pub type VFAULTCLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPBIASACC` reader - Accuracy Setting for the System Bias During ADC Operation"]
pub type GPBIASACC_R = crate::BitReader;
#[doc = "Field `GPBIASACC` writer - Accuracy Setting for the System Bias During ADC Operation"]
pub type GPBIASACC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Bias Programming Value of Analog ADC Block"]
    #[inline(always)]
    pub fn adcbiasprog(&self) -> ADCBIASPROG_R {
        ADCBIASPROG_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Clear VREFOF Flag"]
    #[inline(always)]
    pub fn vfaultclr(&self) -> VFAULTCLR_R {
        VFAULTCLR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Accuracy Setting for the System Bias During ADC Operation"]
    #[inline(always)]
    pub fn gpbiasacc(&self) -> GPBIASACC_R {
        GPBIASACC_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Bias Programming Value of Analog ADC Block"]
    #[inline(always)]
    #[must_use]
    pub fn adcbiasprog(&mut self) -> ADCBIASPROG_W<BIASPROGrs> {
        ADCBIASPROG_W::new(self, 0)
    }
    #[doc = "Bit 12 - Clear VREFOF Flag"]
    #[inline(always)]
    #[must_use]
    pub fn vfaultclr(&mut self) -> VFAULTCLR_W<BIASPROGrs> {
        VFAULTCLR_W::new(self, 12)
    }
    #[doc = "Bit 16 - Accuracy Setting for the System Bias During ADC Operation"]
    #[inline(always)]
    #[must_use]
    pub fn gpbiasacc(&mut self) -> GPBIASACC_W<BIASPROGrs> {
        GPBIASACC_W::new(self, 16)
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
#[doc = "Bias Programming Register for Various Analog Blocks Used in ADC Operation\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`biasprog::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`biasprog::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BIASPROGrs;
impl crate::RegisterSpec for BIASPROGrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`biasprog::R`](R) reader structure"]
impl crate::Readable for BIASPROGrs {}
#[doc = "`write(|w| ..)` method takes [`biasprog::W`](W) writer structure"]
impl crate::Writable for BIASPROGrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BIASPROG to value 0"]
impl crate::Resettable for BIASPROGrs {
    const RESET_VALUE: u32 = 0;
}
