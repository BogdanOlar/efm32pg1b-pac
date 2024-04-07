#[doc = "Register `ADCCTRL` reader"]
pub type R = crate::R<ADCCTRLrs>;
#[doc = "Register `ADCCTRL` writer"]
pub type W = crate::W<ADCCTRLrs>;
#[doc = "ADC0 Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADC0CLKSEL {
    #[doc = "0: ADC0 is not clocked"]
    Disabled = 0,
    #[doc = "1: AUXHFRCO is clocking ADC0"]
    Auxhfrco = 1,
    #[doc = "2: HFXO is clocking ADC0"]
    Hfxo = 2,
    #[doc = "3: HFSRCCLK is clocking ADC0"]
    Hfsrcclk = 3,
}
impl From<ADC0CLKSEL> for u8 {
    #[inline(always)]
    fn from(variant: ADC0CLKSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADC0CLKSEL {
    type Ux = u8;
}
impl crate::IsEnum for ADC0CLKSEL {}
#[doc = "Field `ADC0CLKSEL` reader - ADC0 Clock Select"]
pub type Adc0clkselR = crate::FieldReader<ADC0CLKSEL>;
impl Adc0clkselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADC0CLKSEL {
        match self.bits {
            0 => ADC0CLKSEL::Disabled,
            1 => ADC0CLKSEL::Auxhfrco,
            2 => ADC0CLKSEL::Hfxo,
            3 => ADC0CLKSEL::Hfsrcclk,
            _ => unreachable!(),
        }
    }
    #[doc = "ADC0 is not clocked"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADC0CLKSEL::Disabled
    }
    #[doc = "AUXHFRCO is clocking ADC0"]
    #[inline(always)]
    pub fn is_auxhfrco(&self) -> bool {
        *self == ADC0CLKSEL::Auxhfrco
    }
    #[doc = "HFXO is clocking ADC0"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == ADC0CLKSEL::Hfxo
    }
    #[doc = "HFSRCCLK is clocking ADC0"]
    #[inline(always)]
    pub fn is_hfsrcclk(&self) -> bool {
        *self == ADC0CLKSEL::Hfsrcclk
    }
}
#[doc = "Field `ADC0CLKSEL` writer - ADC0 Clock Select"]
pub type Adc0clkselW<'a, REG> = crate::FieldWriter<'a, REG, 2, ADC0CLKSEL, crate::Safe>;
impl<'a, REG> Adc0clkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADC0 is not clocked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ADC0CLKSEL::Disabled)
    }
    #[doc = "AUXHFRCO is clocking ADC0"]
    #[inline(always)]
    pub fn auxhfrco(self) -> &'a mut crate::W<REG> {
        self.variant(ADC0CLKSEL::Auxhfrco)
    }
    #[doc = "HFXO is clocking ADC0"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut crate::W<REG> {
        self.variant(ADC0CLKSEL::Hfxo)
    }
    #[doc = "HFSRCCLK is clocking ADC0"]
    #[inline(always)]
    pub fn hfsrcclk(self) -> &'a mut crate::W<REG> {
        self.variant(ADC0CLKSEL::Hfsrcclk)
    }
}
#[doc = "Field `ADC0CLKINV` reader - Invert Clock Selected By ADC0CLKSEL"]
pub type Adc0clkinvR = crate::BitReader;
#[doc = "Field `ADC0CLKINV` writer - Invert Clock Selected By ADC0CLKSEL"]
pub type Adc0clkinvW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 4:5 - ADC0 Clock Select"]
    #[inline(always)]
    pub fn adc0clksel(&self) -> Adc0clkselR {
        Adc0clkselR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - Invert Clock Selected By ADC0CLKSEL"]
    #[inline(always)]
    pub fn adc0clkinv(&self) -> Adc0clkinvR {
        Adc0clkinvR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 4:5 - ADC0 Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn adc0clksel(&mut self) -> Adc0clkselW<ADCCTRLrs> {
        Adc0clkselW::new(self, 4)
    }
    #[doc = "Bit 8 - Invert Clock Selected By ADC0CLKSEL"]
    #[inline(always)]
    #[must_use]
    pub fn adc0clkinv(&mut self) -> Adc0clkinvW<ADCCTRLrs> {
        Adc0clkinvW::new(self, 8)
    }
}
#[doc = "ADC Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADCCTRLrs;
impl crate::RegisterSpec for ADCCTRLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcctrl::R`](R) reader structure"]
impl crate::Readable for ADCCTRLrs {}
#[doc = "`write(|w| ..)` method takes [`adcctrl::W`](W) writer structure"]
impl crate::Writable for ADCCTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADCCTRL to value 0"]
impl crate::Resettable for ADCCTRLrs {
    const RESET_VALUE: u32 = 0;
}
