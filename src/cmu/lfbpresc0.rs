#[doc = "Register `LFBPRESC0` reader"]
pub type R = crate::R<LFBPRESC0rs>;
#[doc = "Register `LFBPRESC0` writer"]
pub type W = crate::W<LFBPRESC0rs>;
#[doc = "Field `LEUART0` reader - Low Energy UART 0 Prescaler"]
pub type LEUART0_R = crate::FieldReader<LEUART0>;
#[doc = "Low Energy UART 0 Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LEUART0 {
    #[doc = "0: LFBCLKLEUART0 = LFBCLK"]
    Div1 = 0,
    #[doc = "1: LFBCLKLEUART0 = LFBCLK/2"]
    Div2 = 1,
    #[doc = "2: LFBCLKLEUART0 = LFBCLK/4"]
    Div4 = 2,
    #[doc = "3: LFBCLKLEUART0 = LFBCLK/8"]
    Div8 = 3,
}
impl From<LEUART0> for u8 {
    #[inline(always)]
    fn from(variant: LEUART0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LEUART0 {
    type Ux = u8;
}
impl LEUART0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LEUART0 {
        match self.bits {
            0 => LEUART0::Div1,
            1 => LEUART0::Div2,
            2 => LEUART0::Div4,
            3 => LEUART0::Div8,
            _ => unreachable!(),
        }
    }
    #[doc = "LFBCLKLEUART0 = LFBCLK"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == LEUART0::Div1
    }
    #[doc = "LFBCLKLEUART0 = LFBCLK/2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == LEUART0::Div2
    }
    #[doc = "LFBCLKLEUART0 = LFBCLK/4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == LEUART0::Div4
    }
    #[doc = "LFBCLKLEUART0 = LFBCLK/8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == LEUART0::Div8
    }
}
#[doc = "Field `LEUART0` writer - Low Energy UART 0 Prescaler"]
pub type LEUART0_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, LEUART0>;
impl<'a, REG> LEUART0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LFBCLKLEUART0 = LFBCLK"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(LEUART0::Div1)
    }
    #[doc = "LFBCLKLEUART0 = LFBCLK/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(LEUART0::Div2)
    }
    #[doc = "LFBCLKLEUART0 = LFBCLK/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(LEUART0::Div4)
    }
    #[doc = "LFBCLKLEUART0 = LFBCLK/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(LEUART0::Div8)
    }
}
impl R {
    #[doc = "Bits 0:1 - Low Energy UART 0 Prescaler"]
    #[inline(always)]
    pub fn leuart0(&self) -> LEUART0_R {
        LEUART0_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Low Energy UART 0 Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn leuart0(&mut self) -> LEUART0_W<LFBPRESC0rs> {
        LEUART0_W::new(self, 0)
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
#[doc = "Low Frequency B Prescaler Register 0 (Async Reg)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfbpresc0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfbpresc0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LFBPRESC0rs;
impl crate::RegisterSpec for LFBPRESC0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfbpresc0::R`](R) reader structure"]
impl crate::Readable for LFBPRESC0rs {}
#[doc = "`write(|w| ..)` method takes [`lfbpresc0::W`](W) writer structure"]
impl crate::Writable for LFBPRESC0rs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LFBPRESC0 to value 0"]
impl crate::Resettable for LFBPRESC0rs {
    const RESET_VALUE: u32 = 0;
}
