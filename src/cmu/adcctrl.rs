#[doc = "Register `ADCCTRL` reader"]
pub type R = crate::R<ADCCTRL_SPEC>;
#[doc = "Register `ADCCTRL` writer"]
pub type W = crate::W<ADCCTRL_SPEC>;
#[doc = "Field `ADC0CLKSEL` reader - ADC0 Clock Select"]
pub type ADC0CLKSEL_R = crate::FieldReader<ADC0CLKSEL_A>;
#[doc = "ADC0 Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADC0CLKSEL_A {
    #[doc = "0: ADC0 is not clocked"]
    DISABLED = 0,
    #[doc = "1: AUXHFRCO is clocking ADC0"]
    AUXHFRCO = 1,
    #[doc = "2: HFXO is clocking ADC0"]
    HFXO = 2,
    #[doc = "3: HFSRCCLK is clocking ADC0"]
    HFSRCCLK = 3,
}
impl From<ADC0CLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC0CLKSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADC0CLKSEL_A {
    type Ux = u8;
}
impl ADC0CLKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADC0CLKSEL_A {
        match self.bits {
            0 => ADC0CLKSEL_A::DISABLED,
            1 => ADC0CLKSEL_A::AUXHFRCO,
            2 => ADC0CLKSEL_A::HFXO,
            3 => ADC0CLKSEL_A::HFSRCCLK,
            _ => unreachable!(),
        }
    }
    #[doc = "ADC0 is not clocked"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADC0CLKSEL_A::DISABLED
    }
    #[doc = "AUXHFRCO is clocking ADC0"]
    #[inline(always)]
    pub fn is_auxhfrco(&self) -> bool {
        *self == ADC0CLKSEL_A::AUXHFRCO
    }
    #[doc = "HFXO is clocking ADC0"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == ADC0CLKSEL_A::HFXO
    }
    #[doc = "HFSRCCLK is clocking ADC0"]
    #[inline(always)]
    pub fn is_hfsrcclk(&self) -> bool {
        *self == ADC0CLKSEL_A::HFSRCCLK
    }
}
#[doc = "Field `ADC0CLKSEL` writer - ADC0 Clock Select"]
pub type ADC0CLKSEL_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, ADC0CLKSEL_A>;
impl<'a, REG, const O: u8> ADC0CLKSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADC0 is not clocked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADC0CLKSEL_A::DISABLED)
    }
    #[doc = "AUXHFRCO is clocking ADC0"]
    #[inline(always)]
    pub fn auxhfrco(self) -> &'a mut crate::W<REG> {
        self.variant(ADC0CLKSEL_A::AUXHFRCO)
    }
    #[doc = "HFXO is clocking ADC0"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut crate::W<REG> {
        self.variant(ADC0CLKSEL_A::HFXO)
    }
    #[doc = "HFSRCCLK is clocking ADC0"]
    #[inline(always)]
    pub fn hfsrcclk(self) -> &'a mut crate::W<REG> {
        self.variant(ADC0CLKSEL_A::HFSRCCLK)
    }
}
#[doc = "Field `ADC0CLKINV` reader - Invert Clock Selected By ADC0CLKSEL"]
pub type ADC0CLKINV_R = crate::BitReader;
#[doc = "Field `ADC0CLKINV` writer - Invert Clock Selected By ADC0CLKSEL"]
pub type ADC0CLKINV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 4:5 - ADC0 Clock Select"]
    #[inline(always)]
    pub fn adc0clksel(&self) -> ADC0CLKSEL_R {
        ADC0CLKSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - Invert Clock Selected By ADC0CLKSEL"]
    #[inline(always)]
    pub fn adc0clkinv(&self) -> ADC0CLKINV_R {
        ADC0CLKINV_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCCTRL")
            .field("adc0clksel", &format_args!("{}", self.adc0clksel().bits()))
            .field("adc0clkinv", &format_args!("{}", self.adc0clkinv().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<ADCCTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 4:5 - ADC0 Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn adc0clksel(&mut self) -> ADC0CLKSEL_W<ADCCTRL_SPEC, 4> {
        ADC0CLKSEL_W::new(self)
    }
    #[doc = "Bit 8 - Invert Clock Selected By ADC0CLKSEL"]
    #[inline(always)]
    #[must_use]
    pub fn adc0clkinv(&mut self) -> ADC0CLKINV_W<ADCCTRL_SPEC, 8> {
        ADC0CLKINV_W::new(self)
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
#[doc = "ADC Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADCCTRL_SPEC;
impl crate::RegisterSpec for ADCCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcctrl::R`](R) reader structure"]
impl crate::Readable for ADCCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adcctrl::W`](W) writer structure"]
impl crate::Writable for ADCCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCCTRL to value 0"]
impl crate::Resettable for ADCCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
