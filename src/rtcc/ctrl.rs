#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRLrs>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRLrs>;
#[doc = "Field `ENABLE` reader - RTCC Enable"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - RTCC Enable"]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEBUGRUN` reader - Debug Mode Run Enable"]
pub type DEBUGRUN_R = crate::BitReader;
#[doc = "Field `DEBUGRUN` writer - Debug Mode Run Enable"]
pub type DEBUGRUN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRECCV0TOP` reader - Pre-counter CCV0 Top Value Enable"]
pub type PRECCV0TOP_R = crate::BitReader;
#[doc = "Field `PRECCV0TOP` writer - Pre-counter CCV0 Top Value Enable"]
pub type PRECCV0TOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCV1TOP` reader - CCV1 Top Value Enable"]
pub type CCV1TOP_R = crate::BitReader;
#[doc = "Field `CCV1TOP` writer - CCV1 Top Value Enable"]
pub type CCV1TOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTPRESC` reader - Counter Prescaler Value"]
pub type CNTPRESC_R = crate::FieldReader<CNTPRESC>;
#[doc = "Counter Prescaler Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CNTPRESC {
    #[doc = "0: CLKCNT = LFECLKRTCC/1"]
    Div1 = 0,
    #[doc = "1: CLKCNT = LFECLKRTCC/2"]
    Div2 = 1,
    #[doc = "2: CLKCNT = LFECLKRTCC/4"]
    Div4 = 2,
    #[doc = "3: CLKCNT = LFECLKRTCC/8"]
    Div8 = 3,
    #[doc = "4: CLKCNT = LFECLKRTCC/16"]
    Div16 = 4,
    #[doc = "5: CLKCNT = LFECLKRTCC/32"]
    Div32 = 5,
    #[doc = "6: CLKCNT = LFECLKRTCC/64"]
    Div64 = 6,
    #[doc = "7: CLKCNT = LFECLKRTCC/128"]
    Div128 = 7,
    #[doc = "8: CLKCNT = LFECLKRTCC/256"]
    Div256 = 8,
    #[doc = "9: CLKCNT = LFECLKRTCC/512"]
    Div512 = 9,
    #[doc = "10: CLKCNT = LFECLKRTCC/1024"]
    Div1024 = 10,
    #[doc = "11: CLKCNT = LFECLKRTCC/2048"]
    Div2048 = 11,
    #[doc = "12: CLKCNT = LFECLKRTCC/4096"]
    Div4096 = 12,
    #[doc = "13: CLKCNT = LFECLKRTCC/8192"]
    Div8192 = 13,
    #[doc = "14: CLKCNT = LFECLKRTCC/16384"]
    Div16384 = 14,
    #[doc = "15: CLKCNT = LFECLKRTCC/32768"]
    Div32768 = 15,
}
impl From<CNTPRESC> for u8 {
    #[inline(always)]
    fn from(variant: CNTPRESC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CNTPRESC {
    type Ux = u8;
}
impl CNTPRESC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CNTPRESC {
        match self.bits {
            0 => CNTPRESC::Div1,
            1 => CNTPRESC::Div2,
            2 => CNTPRESC::Div4,
            3 => CNTPRESC::Div8,
            4 => CNTPRESC::Div16,
            5 => CNTPRESC::Div32,
            6 => CNTPRESC::Div64,
            7 => CNTPRESC::Div128,
            8 => CNTPRESC::Div256,
            9 => CNTPRESC::Div512,
            10 => CNTPRESC::Div1024,
            11 => CNTPRESC::Div2048,
            12 => CNTPRESC::Div4096,
            13 => CNTPRESC::Div8192,
            14 => CNTPRESC::Div16384,
            15 => CNTPRESC::Div32768,
            _ => unreachable!(),
        }
    }
    #[doc = "CLKCNT = LFECLKRTCC/1"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == CNTPRESC::Div1
    }
    #[doc = "CLKCNT = LFECLKRTCC/2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == CNTPRESC::Div2
    }
    #[doc = "CLKCNT = LFECLKRTCC/4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == CNTPRESC::Div4
    }
    #[doc = "CLKCNT = LFECLKRTCC/8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == CNTPRESC::Div8
    }
    #[doc = "CLKCNT = LFECLKRTCC/16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == CNTPRESC::Div16
    }
    #[doc = "CLKCNT = LFECLKRTCC/32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == CNTPRESC::Div32
    }
    #[doc = "CLKCNT = LFECLKRTCC/64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == CNTPRESC::Div64
    }
    #[doc = "CLKCNT = LFECLKRTCC/128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == CNTPRESC::Div128
    }
    #[doc = "CLKCNT = LFECLKRTCC/256"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == CNTPRESC::Div256
    }
    #[doc = "CLKCNT = LFECLKRTCC/512"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == CNTPRESC::Div512
    }
    #[doc = "CLKCNT = LFECLKRTCC/1024"]
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        *self == CNTPRESC::Div1024
    }
    #[doc = "CLKCNT = LFECLKRTCC/2048"]
    #[inline(always)]
    pub fn is_div2048(&self) -> bool {
        *self == CNTPRESC::Div2048
    }
    #[doc = "CLKCNT = LFECLKRTCC/4096"]
    #[inline(always)]
    pub fn is_div4096(&self) -> bool {
        *self == CNTPRESC::Div4096
    }
    #[doc = "CLKCNT = LFECLKRTCC/8192"]
    #[inline(always)]
    pub fn is_div8192(&self) -> bool {
        *self == CNTPRESC::Div8192
    }
    #[doc = "CLKCNT = LFECLKRTCC/16384"]
    #[inline(always)]
    pub fn is_div16384(&self) -> bool {
        *self == CNTPRESC::Div16384
    }
    #[doc = "CLKCNT = LFECLKRTCC/32768"]
    #[inline(always)]
    pub fn is_div32768(&self) -> bool {
        *self == CNTPRESC::Div32768
    }
}
#[doc = "Field `CNTPRESC` writer - Counter Prescaler Value"]
pub type CNTPRESC_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, CNTPRESC>;
impl<'a, REG> CNTPRESC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CLKCNT = LFECLKRTCC/1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(CNTPRESC::Div1)
    }
    #[doc = "CLKCNT = LFECLKRTCC/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(CNTPRESC::Div2)
    }
    #[doc = "CLKCNT = LFECLKRTCC/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(CNTPRESC::Div4)
    }
    #[doc = "CLKCNT = LFECLKRTCC/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(CNTPRESC::Div8)
    }
    #[doc = "CLKCNT = LFECLKRTCC/16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(CNTPRESC::Div16)
    }
    #[doc = "CLKCNT = LFECLKRTCC/32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(CNTPRESC::Div32)
    }
    #[doc = "CLKCNT = LFECLKRTCC/64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(CNTPRESC::Div64)
    }
    #[doc = "CLKCNT = LFECLKRTCC/128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(CNTPRESC::Div128)
    }
    #[doc = "CLKCNT = LFECLKRTCC/256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(CNTPRESC::Div256)
    }
    #[doc = "CLKCNT = LFECLKRTCC/512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut crate::W<REG> {
        self.variant(CNTPRESC::Div512)
    }
    #[doc = "CLKCNT = LFECLKRTCC/1024"]
    #[inline(always)]
    pub fn div1024(self) -> &'a mut crate::W<REG> {
        self.variant(CNTPRESC::Div1024)
    }
    #[doc = "CLKCNT = LFECLKRTCC/2048"]
    #[inline(always)]
    pub fn div2048(self) -> &'a mut crate::W<REG> {
        self.variant(CNTPRESC::Div2048)
    }
    #[doc = "CLKCNT = LFECLKRTCC/4096"]
    #[inline(always)]
    pub fn div4096(self) -> &'a mut crate::W<REG> {
        self.variant(CNTPRESC::Div4096)
    }
    #[doc = "CLKCNT = LFECLKRTCC/8192"]
    #[inline(always)]
    pub fn div8192(self) -> &'a mut crate::W<REG> {
        self.variant(CNTPRESC::Div8192)
    }
    #[doc = "CLKCNT = LFECLKRTCC/16384"]
    #[inline(always)]
    pub fn div16384(self) -> &'a mut crate::W<REG> {
        self.variant(CNTPRESC::Div16384)
    }
    #[doc = "CLKCNT = LFECLKRTCC/32768"]
    #[inline(always)]
    pub fn div32768(self) -> &'a mut crate::W<REG> {
        self.variant(CNTPRESC::Div32768)
    }
}
#[doc = "Field `CNTTICK` reader - Counter Prescaler Mode"]
pub type CNTTICK_R = crate::BitReader;
#[doc = "Field `CNTTICK` writer - Counter Prescaler Mode"]
pub type CNTTICK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSCFDETEN` reader - Oscillator Failure Detection Enable"]
pub type OSCFDETEN_R = crate::BitReader;
#[doc = "Field `OSCFDETEN` writer - Oscillator Failure Detection Enable"]
pub type OSCFDETEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTMODE` reader - Main Counter Mode"]
pub type CNTMODE_R = crate::BitReader;
#[doc = "Field `CNTMODE` writer - Main Counter Mode"]
pub type CNTMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LYEARCORRDIS` reader - Leap Year Correction Disabled"]
pub type LYEARCORRDIS_R = crate::BitReader;
#[doc = "Field `LYEARCORRDIS` writer - Leap Year Correction Disabled"]
pub type LYEARCORRDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RTCC Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Debug Mode Run Enable"]
    #[inline(always)]
    pub fn debugrun(&self) -> DEBUGRUN_R {
        DEBUGRUN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Pre-counter CCV0 Top Value Enable"]
    #[inline(always)]
    pub fn preccv0top(&self) -> PRECCV0TOP_R {
        PRECCV0TOP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CCV1 Top Value Enable"]
    #[inline(always)]
    pub fn ccv1top(&self) -> CCV1TOP_R {
        CCV1TOP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Counter Prescaler Value"]
    #[inline(always)]
    pub fn cntpresc(&self) -> CNTPRESC_R {
        CNTPRESC_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Counter Prescaler Mode"]
    #[inline(always)]
    pub fn cnttick(&self) -> CNTTICK_R {
        CNTTICK_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - Oscillator Failure Detection Enable"]
    #[inline(always)]
    pub fn oscfdeten(&self) -> OSCFDETEN_R {
        OSCFDETEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Main Counter Mode"]
    #[inline(always)]
    pub fn cntmode(&self) -> CNTMODE_R {
        CNTMODE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Leap Year Correction Disabled"]
    #[inline(always)]
    pub fn lyearcorrdis(&self) -> LYEARCORRDIS_R {
        LYEARCORRDIS_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTCC Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<CTRLrs> {
        ENABLE_W::new(self, 0)
    }
    #[doc = "Bit 2 - Debug Mode Run Enable"]
    #[inline(always)]
    #[must_use]
    pub fn debugrun(&mut self) -> DEBUGRUN_W<CTRLrs> {
        DEBUGRUN_W::new(self, 2)
    }
    #[doc = "Bit 4 - Pre-counter CCV0 Top Value Enable"]
    #[inline(always)]
    #[must_use]
    pub fn preccv0top(&mut self) -> PRECCV0TOP_W<CTRLrs> {
        PRECCV0TOP_W::new(self, 4)
    }
    #[doc = "Bit 5 - CCV1 Top Value Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccv1top(&mut self) -> CCV1TOP_W<CTRLrs> {
        CCV1TOP_W::new(self, 5)
    }
    #[doc = "Bits 8:11 - Counter Prescaler Value"]
    #[inline(always)]
    #[must_use]
    pub fn cntpresc(&mut self) -> CNTPRESC_W<CTRLrs> {
        CNTPRESC_W::new(self, 8)
    }
    #[doc = "Bit 12 - Counter Prescaler Mode"]
    #[inline(always)]
    #[must_use]
    pub fn cnttick(&mut self) -> CNTTICK_W<CTRLrs> {
        CNTTICK_W::new(self, 12)
    }
    #[doc = "Bit 15 - Oscillator Failure Detection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn oscfdeten(&mut self) -> OSCFDETEN_W<CTRLrs> {
        OSCFDETEN_W::new(self, 15)
    }
    #[doc = "Bit 16 - Main Counter Mode"]
    #[inline(always)]
    #[must_use]
    pub fn cntmode(&mut self) -> CNTMODE_W<CTRLrs> {
        CNTMODE_W::new(self, 16)
    }
    #[doc = "Bit 17 - Leap Year Correction Disabled"]
    #[inline(always)]
    #[must_use]
    pub fn lyearcorrdis(&mut self) -> LYEARCORRDIS_W<CTRLrs> {
        LYEARCORRDIS_W::new(self, 17)
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
#[doc = "Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLrs;
impl crate::RegisterSpec for CTRLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRLrs {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRLrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRLrs {
    const RESET_VALUE: u32 = 0;
}
