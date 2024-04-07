#[doc = "Register `DTTIME` reader"]
pub type R = crate::R<DTTIMErs>;
#[doc = "Register `DTTIME` writer"]
pub type W = crate::W<DTTIMErs>;
#[doc = "DTI Prescaler Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DTPRESC {
    #[doc = "0: The HFPERCLK is undivided"]
    Div1 = 0,
    #[doc = "1: The HFPERCLK is divided by 2"]
    Div2 = 1,
    #[doc = "2: The HFPERCLK is divided by 4"]
    Div4 = 2,
    #[doc = "3: The HFPERCLK is divided by 8"]
    Div8 = 3,
    #[doc = "4: The HFPERCLK is divided by 16"]
    Div16 = 4,
    #[doc = "5: The HFPERCLK is divided by 32"]
    Div32 = 5,
    #[doc = "6: The HFPERCLK is divided by 64"]
    Div64 = 6,
    #[doc = "7: The HFPERCLK is divided by 128"]
    Div128 = 7,
    #[doc = "8: The HFPERCLK is divided by 256"]
    Div256 = 8,
    #[doc = "9: The HFPERCLK is divided by 512"]
    Div512 = 9,
    #[doc = "10: The HFPERCLK is divided by 1024"]
    Div1024 = 10,
}
impl From<DTPRESC> for u8 {
    #[inline(always)]
    fn from(variant: DTPRESC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DTPRESC {
    type Ux = u8;
}
impl crate::IsEnum for DTPRESC {}
#[doc = "Field `DTPRESC` reader - DTI Prescaler Setting"]
pub type DtprescR = crate::FieldReader<DTPRESC>;
impl DtprescR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DTPRESC> {
        match self.bits {
            0 => Some(DTPRESC::Div1),
            1 => Some(DTPRESC::Div2),
            2 => Some(DTPRESC::Div4),
            3 => Some(DTPRESC::Div8),
            4 => Some(DTPRESC::Div16),
            5 => Some(DTPRESC::Div32),
            6 => Some(DTPRESC::Div64),
            7 => Some(DTPRESC::Div128),
            8 => Some(DTPRESC::Div256),
            9 => Some(DTPRESC::Div512),
            10 => Some(DTPRESC::Div1024),
            _ => None,
        }
    }
    #[doc = "The HFPERCLK is undivided"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == DTPRESC::Div1
    }
    #[doc = "The HFPERCLK is divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == DTPRESC::Div2
    }
    #[doc = "The HFPERCLK is divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == DTPRESC::Div4
    }
    #[doc = "The HFPERCLK is divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == DTPRESC::Div8
    }
    #[doc = "The HFPERCLK is divided by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == DTPRESC::Div16
    }
    #[doc = "The HFPERCLK is divided by 32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == DTPRESC::Div32
    }
    #[doc = "The HFPERCLK is divided by 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == DTPRESC::Div64
    }
    #[doc = "The HFPERCLK is divided by 128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == DTPRESC::Div128
    }
    #[doc = "The HFPERCLK is divided by 256"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == DTPRESC::Div256
    }
    #[doc = "The HFPERCLK is divided by 512"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == DTPRESC::Div512
    }
    #[doc = "The HFPERCLK is divided by 1024"]
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        *self == DTPRESC::Div1024
    }
}
#[doc = "Field `DTPRESC` writer - DTI Prescaler Setting"]
pub type DtprescW<'a, REG> = crate::FieldWriter<'a, REG, 4, DTPRESC>;
impl<'a, REG> DtprescW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The HFPERCLK is undivided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRESC::Div1)
    }
    #[doc = "The HFPERCLK is divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRESC::Div2)
    }
    #[doc = "The HFPERCLK is divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRESC::Div4)
    }
    #[doc = "The HFPERCLK is divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRESC::Div8)
    }
    #[doc = "The HFPERCLK is divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRESC::Div16)
    }
    #[doc = "The HFPERCLK is divided by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRESC::Div32)
    }
    #[doc = "The HFPERCLK is divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRESC::Div64)
    }
    #[doc = "The HFPERCLK is divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRESC::Div128)
    }
    #[doc = "The HFPERCLK is divided by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRESC::Div256)
    }
    #[doc = "The HFPERCLK is divided by 512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRESC::Div512)
    }
    #[doc = "The HFPERCLK is divided by 1024"]
    #[inline(always)]
    pub fn div1024(self) -> &'a mut crate::W<REG> {
        self.variant(DTPRESC::Div1024)
    }
}
#[doc = "Field `DTRISET` reader - DTI Rise-time"]
pub type DtrisetR = crate::FieldReader;
#[doc = "Field `DTRISET` writer - DTI Rise-time"]
pub type DtrisetW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DTFALLT` reader - DTI Fall-time"]
pub type DtfalltR = crate::FieldReader;
#[doc = "Field `DTFALLT` writer - DTI Fall-time"]
pub type DtfalltW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:3 - DTI Prescaler Setting"]
    #[inline(always)]
    pub fn dtpresc(&self) -> DtprescR {
        DtprescR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:13 - DTI Rise-time"]
    #[inline(always)]
    pub fn dtriset(&self) -> DtrisetR {
        DtrisetR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - DTI Fall-time"]
    #[inline(always)]
    pub fn dtfallt(&self) -> DtfalltR {
        DtfalltR::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - DTI Prescaler Setting"]
    #[inline(always)]
    #[must_use]
    pub fn dtpresc(&mut self) -> DtprescW<DTTIMErs> {
        DtprescW::new(self, 0)
    }
    #[doc = "Bits 8:13 - DTI Rise-time"]
    #[inline(always)]
    #[must_use]
    pub fn dtriset(&mut self) -> DtrisetW<DTTIMErs> {
        DtrisetW::new(self, 8)
    }
    #[doc = "Bits 16:21 - DTI Fall-time"]
    #[inline(always)]
    #[must_use]
    pub fn dtfallt(&mut self) -> DtfalltW<DTTIMErs> {
        DtfalltW::new(self, 16)
    }
}
#[doc = "DTI Time Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dttime::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dttime::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DTTIMErs;
impl crate::RegisterSpec for DTTIMErs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dttime::R`](R) reader structure"]
impl crate::Readable for DTTIMErs {}
#[doc = "`write(|w| ..)` method takes [`dttime::W`](W) writer structure"]
impl crate::Writable for DTTIMErs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DTTIME to value 0"]
impl crate::Resettable for DTTIMErs {
    const RESET_VALUE: u32 = 0;
}
