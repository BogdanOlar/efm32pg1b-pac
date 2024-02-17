#[doc = "Register `TIMECMP2` reader"]
pub type R = crate::R<TIMECMP2rs>;
#[doc = "Register `TIMECMP2` writer"]
pub type W = crate::W<TIMECMP2rs>;
#[doc = "Field `TCMPVAL` reader - Timer Comparator 2"]
pub type TCMPVAL_R = crate::FieldReader;
#[doc = "Field `TCMPVAL` writer - Timer Comparator 2"]
pub type TCMPVAL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TSTART` reader - Timer Start Source"]
pub type TSTART_R = crate::FieldReader<TSTART>;
#[doc = "Timer Start Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TSTART {
    #[doc = "0: Comparator 2 is disabled"]
    Disable = 0,
    #[doc = "1: Comparator 2 and timer are started at TX end of frame"]
    Txeof = 1,
    #[doc = "2: Comparator 2 and timer are started at TX Complete"]
    Txc = 2,
    #[doc = "3: Comparator 2 and timer are started at RX going going Active (default: low)"]
    Rxact = 3,
    #[doc = "4: Comparator 2 and timer are started at RX end of frame"]
    Rxeof = 4,
}
impl From<TSTART> for u8 {
    #[inline(always)]
    fn from(variant: TSTART) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TSTART {
    type Ux = u8;
}
impl TSTART_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TSTART> {
        match self.bits {
            0 => Some(TSTART::Disable),
            1 => Some(TSTART::Txeof),
            2 => Some(TSTART::Txc),
            3 => Some(TSTART::Rxact),
            4 => Some(TSTART::Rxeof),
            _ => None,
        }
    }
    #[doc = "Comparator 2 is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TSTART::Disable
    }
    #[doc = "Comparator 2 and timer are started at TX end of frame"]
    #[inline(always)]
    pub fn is_txeof(&self) -> bool {
        *self == TSTART::Txeof
    }
    #[doc = "Comparator 2 and timer are started at TX Complete"]
    #[inline(always)]
    pub fn is_txc(&self) -> bool {
        *self == TSTART::Txc
    }
    #[doc = "Comparator 2 and timer are started at RX going going Active (default: low)"]
    #[inline(always)]
    pub fn is_rxact(&self) -> bool {
        *self == TSTART::Rxact
    }
    #[doc = "Comparator 2 and timer are started at RX end of frame"]
    #[inline(always)]
    pub fn is_rxeof(&self) -> bool {
        *self == TSTART::Rxeof
    }
}
#[doc = "Field `TSTART` writer - Timer Start Source"]
pub type TSTART_W<'a, REG> = crate::FieldWriter<'a, REG, 3, TSTART>;
impl<'a, REG> TSTART_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Comparator 2 is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TSTART::Disable)
    }
    #[doc = "Comparator 2 and timer are started at TX end of frame"]
    #[inline(always)]
    pub fn txeof(self) -> &'a mut crate::W<REG> {
        self.variant(TSTART::Txeof)
    }
    #[doc = "Comparator 2 and timer are started at TX Complete"]
    #[inline(always)]
    pub fn txc(self) -> &'a mut crate::W<REG> {
        self.variant(TSTART::Txc)
    }
    #[doc = "Comparator 2 and timer are started at RX going going Active (default: low)"]
    #[inline(always)]
    pub fn rxact(self) -> &'a mut crate::W<REG> {
        self.variant(TSTART::Rxact)
    }
    #[doc = "Comparator 2 and timer are started at RX end of frame"]
    #[inline(always)]
    pub fn rxeof(self) -> &'a mut crate::W<REG> {
        self.variant(TSTART::Rxeof)
    }
}
#[doc = "Field `TSTOP` reader - Source Used to Disable Comparator 2"]
pub type TSTOP_R = crate::FieldReader<TSTOP>;
#[doc = "Source Used to Disable Comparator 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TSTOP {
    #[doc = "0: Comparator 2 is disabled when the counter equals TCMPVAL and triggers a TCMP2 event"]
    Tcmp2 = 0,
    #[doc = "1: Comparator 2 is disabled at TX start TX Engine"]
    Txst = 1,
    #[doc = "2: Comparator 2 is disabled on RX going going Active (default: low)"]
    Rxact = 2,
    #[doc = "3: Comparator 2 is disabled on RX going Inactive"]
    Rxactn = 3,
}
impl From<TSTOP> for u8 {
    #[inline(always)]
    fn from(variant: TSTOP) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TSTOP {
    type Ux = u8;
}
impl TSTOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TSTOP> {
        match self.bits {
            0 => Some(TSTOP::Tcmp2),
            1 => Some(TSTOP::Txst),
            2 => Some(TSTOP::Rxact),
            3 => Some(TSTOP::Rxactn),
            _ => None,
        }
    }
    #[doc = "Comparator 2 is disabled when the counter equals TCMPVAL and triggers a TCMP2 event"]
    #[inline(always)]
    pub fn is_tcmp2(&self) -> bool {
        *self == TSTOP::Tcmp2
    }
    #[doc = "Comparator 2 is disabled at TX start TX Engine"]
    #[inline(always)]
    pub fn is_txst(&self) -> bool {
        *self == TSTOP::Txst
    }
    #[doc = "Comparator 2 is disabled on RX going going Active (default: low)"]
    #[inline(always)]
    pub fn is_rxact(&self) -> bool {
        *self == TSTOP::Rxact
    }
    #[doc = "Comparator 2 is disabled on RX going Inactive"]
    #[inline(always)]
    pub fn is_rxactn(&self) -> bool {
        *self == TSTOP::Rxactn
    }
}
#[doc = "Field `TSTOP` writer - Source Used to Disable Comparator 2"]
pub type TSTOP_W<'a, REG> = crate::FieldWriter<'a, REG, 3, TSTOP>;
impl<'a, REG> TSTOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Comparator 2 is disabled when the counter equals TCMPVAL and triggers a TCMP2 event"]
    #[inline(always)]
    pub fn tcmp2(self) -> &'a mut crate::W<REG> {
        self.variant(TSTOP::Tcmp2)
    }
    #[doc = "Comparator 2 is disabled at TX start TX Engine"]
    #[inline(always)]
    pub fn txst(self) -> &'a mut crate::W<REG> {
        self.variant(TSTOP::Txst)
    }
    #[doc = "Comparator 2 is disabled on RX going going Active (default: low)"]
    #[inline(always)]
    pub fn rxact(self) -> &'a mut crate::W<REG> {
        self.variant(TSTOP::Rxact)
    }
    #[doc = "Comparator 2 is disabled on RX going Inactive"]
    #[inline(always)]
    pub fn rxactn(self) -> &'a mut crate::W<REG> {
        self.variant(TSTOP::Rxactn)
    }
}
#[doc = "Field `RESTARTEN` reader - Restart Timer on TCMP2"]
pub type RESTARTEN_R = crate::BitReader;
#[doc = "Field `RESTARTEN` writer - Restart Timer on TCMP2"]
pub type RESTARTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Timer Comparator 2"]
    #[inline(always)]
    pub fn tcmpval(&self) -> TCMPVAL_R {
        TCMPVAL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:18 - Timer Start Source"]
    #[inline(always)]
    pub fn tstart(&self) -> TSTART_R {
        TSTART_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Source Used to Disable Comparator 2"]
    #[inline(always)]
    pub fn tstop(&self) -> TSTOP_R {
        TSTOP_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 24 - Restart Timer on TCMP2"]
    #[inline(always)]
    pub fn restarten(&self) -> RESTARTEN_R {
        RESTARTEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Timer Comparator 2"]
    #[inline(always)]
    #[must_use]
    pub fn tcmpval(&mut self) -> TCMPVAL_W<TIMECMP2rs> {
        TCMPVAL_W::new(self, 0)
    }
    #[doc = "Bits 16:18 - Timer Start Source"]
    #[inline(always)]
    #[must_use]
    pub fn tstart(&mut self) -> TSTART_W<TIMECMP2rs> {
        TSTART_W::new(self, 16)
    }
    #[doc = "Bits 20:22 - Source Used to Disable Comparator 2"]
    #[inline(always)]
    #[must_use]
    pub fn tstop(&mut self) -> TSTOP_W<TIMECMP2rs> {
        TSTOP_W::new(self, 20)
    }
    #[doc = "Bit 24 - Restart Timer on TCMP2"]
    #[inline(always)]
    #[must_use]
    pub fn restarten(&mut self) -> RESTARTEN_W<TIMECMP2rs> {
        RESTARTEN_W::new(self, 24)
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
#[doc = "Used to Generate Interrupts and Various Delays\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timecmp2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timecmp2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMECMP2rs;
impl crate::RegisterSpec for TIMECMP2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timecmp2::R`](R) reader structure"]
impl crate::Readable for TIMECMP2rs {}
#[doc = "`write(|w| ..)` method takes [`timecmp2::W`](W) writer structure"]
impl crate::Writable for TIMECMP2rs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMECMP2 to value 0"]
impl crate::Resettable for TIMECMP2rs {
    const RESET_VALUE: u32 = 0;
}
