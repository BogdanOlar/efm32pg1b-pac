#[doc = "Register `TIMECMP0` reader"]
pub type R = crate::R<TIMECMP0rs>;
#[doc = "Register `TIMECMP0` writer"]
pub type W = crate::W<TIMECMP0rs>;
#[doc = "Field `TCMPVAL` reader - Timer Comparator 0"]
pub type TcmpvalR = crate::FieldReader;
#[doc = "Field `TCMPVAL` writer - Timer Comparator 0"]
pub type TcmpvalW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Timer Start Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TSTART {
    #[doc = "0: Comparator 0 is disabled"]
    Disable = 0,
    #[doc = "1: Comparator 0 and timer are started at TX end of frame"]
    Txeof = 1,
    #[doc = "2: Comparator 0 and timer are started at TX Complete"]
    Txc = 2,
    #[doc = "3: Comparator 0 and timer are started at RX going Active (default: low)"]
    Rxact = 3,
    #[doc = "4: Comparator 0 and timer are started at RX end of frame"]
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
impl crate::IsEnum for TSTART {}
#[doc = "Field `TSTART` reader - Timer Start Source"]
pub type TstartR = crate::FieldReader<TSTART>;
impl TstartR {
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
    #[doc = "Comparator 0 is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TSTART::Disable
    }
    #[doc = "Comparator 0 and timer are started at TX end of frame"]
    #[inline(always)]
    pub fn is_txeof(&self) -> bool {
        *self == TSTART::Txeof
    }
    #[doc = "Comparator 0 and timer are started at TX Complete"]
    #[inline(always)]
    pub fn is_txc(&self) -> bool {
        *self == TSTART::Txc
    }
    #[doc = "Comparator 0 and timer are started at RX going Active (default: low)"]
    #[inline(always)]
    pub fn is_rxact(&self) -> bool {
        *self == TSTART::Rxact
    }
    #[doc = "Comparator 0 and timer are started at RX end of frame"]
    #[inline(always)]
    pub fn is_rxeof(&self) -> bool {
        *self == TSTART::Rxeof
    }
}
#[doc = "Field `TSTART` writer - Timer Start Source"]
pub type TstartW<'a, REG> = crate::FieldWriter<'a, REG, 3, TSTART>;
impl<'a, REG> TstartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Comparator 0 is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TSTART::Disable)
    }
    #[doc = "Comparator 0 and timer are started at TX end of frame"]
    #[inline(always)]
    pub fn txeof(self) -> &'a mut crate::W<REG> {
        self.variant(TSTART::Txeof)
    }
    #[doc = "Comparator 0 and timer are started at TX Complete"]
    #[inline(always)]
    pub fn txc(self) -> &'a mut crate::W<REG> {
        self.variant(TSTART::Txc)
    }
    #[doc = "Comparator 0 and timer are started at RX going Active (default: low)"]
    #[inline(always)]
    pub fn rxact(self) -> &'a mut crate::W<REG> {
        self.variant(TSTART::Rxact)
    }
    #[doc = "Comparator 0 and timer are started at RX end of frame"]
    #[inline(always)]
    pub fn rxeof(self) -> &'a mut crate::W<REG> {
        self.variant(TSTART::Rxeof)
    }
}
#[doc = "Source Used to Disable Comparator 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TSTOP {
    #[doc = "0: Comparator 0 is disabled when the counter equals TCMPVAL and triggers a TCMP0 event"]
    Tcmp0 = 0,
    #[doc = "1: Comparator 0 is disabled at the start of transmission"]
    Txst = 1,
    #[doc = "2: Comparator 0 is disabled on RX going going Active (default: low)"]
    Rxact = 2,
    #[doc = "3: Comparator 0 is disabled on RX going Inactive"]
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
impl crate::IsEnum for TSTOP {}
#[doc = "Field `TSTOP` reader - Source Used to Disable Comparator 0"]
pub type TstopR = crate::FieldReader<TSTOP>;
impl TstopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TSTOP> {
        match self.bits {
            0 => Some(TSTOP::Tcmp0),
            1 => Some(TSTOP::Txst),
            2 => Some(TSTOP::Rxact),
            3 => Some(TSTOP::Rxactn),
            _ => None,
        }
    }
    #[doc = "Comparator 0 is disabled when the counter equals TCMPVAL and triggers a TCMP0 event"]
    #[inline(always)]
    pub fn is_tcmp0(&self) -> bool {
        *self == TSTOP::Tcmp0
    }
    #[doc = "Comparator 0 is disabled at the start of transmission"]
    #[inline(always)]
    pub fn is_txst(&self) -> bool {
        *self == TSTOP::Txst
    }
    #[doc = "Comparator 0 is disabled on RX going going Active (default: low)"]
    #[inline(always)]
    pub fn is_rxact(&self) -> bool {
        *self == TSTOP::Rxact
    }
    #[doc = "Comparator 0 is disabled on RX going Inactive"]
    #[inline(always)]
    pub fn is_rxactn(&self) -> bool {
        *self == TSTOP::Rxactn
    }
}
#[doc = "Field `TSTOP` writer - Source Used to Disable Comparator 0"]
pub type TstopW<'a, REG> = crate::FieldWriter<'a, REG, 3, TSTOP>;
impl<'a, REG> TstopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Comparator 0 is disabled when the counter equals TCMPVAL and triggers a TCMP0 event"]
    #[inline(always)]
    pub fn tcmp0(self) -> &'a mut crate::W<REG> {
        self.variant(TSTOP::Tcmp0)
    }
    #[doc = "Comparator 0 is disabled at the start of transmission"]
    #[inline(always)]
    pub fn txst(self) -> &'a mut crate::W<REG> {
        self.variant(TSTOP::Txst)
    }
    #[doc = "Comparator 0 is disabled on RX going going Active (default: low)"]
    #[inline(always)]
    pub fn rxact(self) -> &'a mut crate::W<REG> {
        self.variant(TSTOP::Rxact)
    }
    #[doc = "Comparator 0 is disabled on RX going Inactive"]
    #[inline(always)]
    pub fn rxactn(self) -> &'a mut crate::W<REG> {
        self.variant(TSTOP::Rxactn)
    }
}
#[doc = "Field `RESTARTEN` reader - Restart Timer on TCMP0"]
pub type RestartenR = crate::BitReader;
#[doc = "Field `RESTARTEN` writer - Restart Timer on TCMP0"]
pub type RestartenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Timer Comparator 0"]
    #[inline(always)]
    pub fn tcmpval(&self) -> TcmpvalR {
        TcmpvalR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:18 - Timer Start Source"]
    #[inline(always)]
    pub fn tstart(&self) -> TstartR {
        TstartR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Source Used to Disable Comparator 0"]
    #[inline(always)]
    pub fn tstop(&self) -> TstopR {
        TstopR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 24 - Restart Timer on TCMP0"]
    #[inline(always)]
    pub fn restarten(&self) -> RestartenR {
        RestartenR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Timer Comparator 0"]
    #[inline(always)]
    #[must_use]
    pub fn tcmpval(&mut self) -> TcmpvalW<TIMECMP0rs> {
        TcmpvalW::new(self, 0)
    }
    #[doc = "Bits 16:18 - Timer Start Source"]
    #[inline(always)]
    #[must_use]
    pub fn tstart(&mut self) -> TstartW<TIMECMP0rs> {
        TstartW::new(self, 16)
    }
    #[doc = "Bits 20:22 - Source Used to Disable Comparator 0"]
    #[inline(always)]
    #[must_use]
    pub fn tstop(&mut self) -> TstopW<TIMECMP0rs> {
        TstopW::new(self, 20)
    }
    #[doc = "Bit 24 - Restart Timer on TCMP0"]
    #[inline(always)]
    #[must_use]
    pub fn restarten(&mut self) -> RestartenW<TIMECMP0rs> {
        RestartenW::new(self, 24)
    }
}
#[doc = "Used to Generate Interrupts and Various Delays\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timecmp0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timecmp0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMECMP0rs;
impl crate::RegisterSpec for TIMECMP0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timecmp0::R`](R) reader structure"]
impl crate::Readable for TIMECMP0rs {}
#[doc = "`write(|w| ..)` method takes [`timecmp0::W`](W) writer structure"]
impl crate::Writable for TIMECMP0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMECMP0 to value 0"]
impl crate::Resettable for TIMECMP0rs {
    const RESET_VALUE: u32 = 0;
}
