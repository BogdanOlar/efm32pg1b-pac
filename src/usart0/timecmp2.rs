///Register `TIMECMP2` reader
pub type R = crate::R<TIMECMP2rs>;
///Register `TIMECMP2` writer
pub type W = crate::W<TIMECMP2rs>;
///Field `TCMPVAL` reader - Timer Comparator 2
pub type TcmpvalR = crate::FieldReader;
///Field `TCMPVAL` writer - Timer Comparator 2
pub type TcmpvalW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Timer Start Source
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TSTART {
    ///0: Comparator 2 is disabled
    Disable = 0,
    ///1: Comparator 2 and timer are started at TX end of frame
    Txeof = 1,
    ///2: Comparator 2 and timer are started at TX Complete
    Txc = 2,
    ///3: Comparator 2 and timer are started at RX going going Active (default: low)
    Rxact = 3,
    ///4: Comparator 2 and timer are started at RX end of frame
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
///Field `TSTART` reader - Timer Start Source
pub type TstartR = crate::FieldReader<TSTART>;
impl TstartR {
    ///Get enumerated values variant
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
    ///Comparator 2 is disabled
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TSTART::Disable
    }
    ///Comparator 2 and timer are started at TX end of frame
    #[inline(always)]
    pub fn is_txeof(&self) -> bool {
        *self == TSTART::Txeof
    }
    ///Comparator 2 and timer are started at TX Complete
    #[inline(always)]
    pub fn is_txc(&self) -> bool {
        *self == TSTART::Txc
    }
    ///Comparator 2 and timer are started at RX going going Active (default: low)
    #[inline(always)]
    pub fn is_rxact(&self) -> bool {
        *self == TSTART::Rxact
    }
    ///Comparator 2 and timer are started at RX end of frame
    #[inline(always)]
    pub fn is_rxeof(&self) -> bool {
        *self == TSTART::Rxeof
    }
}
///Field `TSTART` writer - Timer Start Source
pub type TstartW<'a, REG> = crate::FieldWriter<'a, REG, 3, TSTART>;
impl<'a, REG> TstartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Comparator 2 is disabled
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TSTART::Disable)
    }
    ///Comparator 2 and timer are started at TX end of frame
    #[inline(always)]
    pub fn txeof(self) -> &'a mut crate::W<REG> {
        self.variant(TSTART::Txeof)
    }
    ///Comparator 2 and timer are started at TX Complete
    #[inline(always)]
    pub fn txc(self) -> &'a mut crate::W<REG> {
        self.variant(TSTART::Txc)
    }
    ///Comparator 2 and timer are started at RX going going Active (default: low)
    #[inline(always)]
    pub fn rxact(self) -> &'a mut crate::W<REG> {
        self.variant(TSTART::Rxact)
    }
    ///Comparator 2 and timer are started at RX end of frame
    #[inline(always)]
    pub fn rxeof(self) -> &'a mut crate::W<REG> {
        self.variant(TSTART::Rxeof)
    }
}
///Source Used to Disable Comparator 2
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TSTOP {
    ///0: Comparator 2 is disabled when the counter equals TCMPVAL and triggers a TCMP2 event
    Tcmp2 = 0,
    ///1: Comparator 2 is disabled at TX start TX Engine
    Txst = 1,
    ///2: Comparator 2 is disabled on RX going going Active (default: low)
    Rxact = 2,
    ///3: Comparator 2 is disabled on RX going Inactive
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
///Field `TSTOP` reader - Source Used to Disable Comparator 2
pub type TstopR = crate::FieldReader<TSTOP>;
impl TstopR {
    ///Get enumerated values variant
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
    ///Comparator 2 is disabled when the counter equals TCMPVAL and triggers a TCMP2 event
    #[inline(always)]
    pub fn is_tcmp2(&self) -> bool {
        *self == TSTOP::Tcmp2
    }
    ///Comparator 2 is disabled at TX start TX Engine
    #[inline(always)]
    pub fn is_txst(&self) -> bool {
        *self == TSTOP::Txst
    }
    ///Comparator 2 is disabled on RX going going Active (default: low)
    #[inline(always)]
    pub fn is_rxact(&self) -> bool {
        *self == TSTOP::Rxact
    }
    ///Comparator 2 is disabled on RX going Inactive
    #[inline(always)]
    pub fn is_rxactn(&self) -> bool {
        *self == TSTOP::Rxactn
    }
}
///Field `TSTOP` writer - Source Used to Disable Comparator 2
pub type TstopW<'a, REG> = crate::FieldWriter<'a, REG, 3, TSTOP>;
impl<'a, REG> TstopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Comparator 2 is disabled when the counter equals TCMPVAL and triggers a TCMP2 event
    #[inline(always)]
    pub fn tcmp2(self) -> &'a mut crate::W<REG> {
        self.variant(TSTOP::Tcmp2)
    }
    ///Comparator 2 is disabled at TX start TX Engine
    #[inline(always)]
    pub fn txst(self) -> &'a mut crate::W<REG> {
        self.variant(TSTOP::Txst)
    }
    ///Comparator 2 is disabled on RX going going Active (default: low)
    #[inline(always)]
    pub fn rxact(self) -> &'a mut crate::W<REG> {
        self.variant(TSTOP::Rxact)
    }
    ///Comparator 2 is disabled on RX going Inactive
    #[inline(always)]
    pub fn rxactn(self) -> &'a mut crate::W<REG> {
        self.variant(TSTOP::Rxactn)
    }
}
///Field `RESTARTEN` reader - Restart Timer on TCMP2
pub type RestartenR = crate::BitReader;
///Field `RESTARTEN` writer - Restart Timer on TCMP2
pub type RestartenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - Timer Comparator 2
    #[inline(always)]
    pub fn tcmpval(&self) -> TcmpvalR {
        TcmpvalR::new((self.bits & 0xff) as u8)
    }
    ///Bits 16:18 - Timer Start Source
    #[inline(always)]
    pub fn tstart(&self) -> TstartR {
        TstartR::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bits 20:22 - Source Used to Disable Comparator 2
    #[inline(always)]
    pub fn tstop(&self) -> TstopR {
        TstopR::new(((self.bits >> 20) & 7) as u8)
    }
    ///Bit 24 - Restart Timer on TCMP2
    #[inline(always)]
    pub fn restarten(&self) -> RestartenR {
        RestartenR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMECMP2")
            .field("tcmpval", &self.tcmpval())
            .field("tstart", &self.tstart())
            .field("tstop", &self.tstop())
            .field("restarten", &self.restarten())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Timer Comparator 2
    #[inline(always)]
    #[must_use]
    pub fn tcmpval(&mut self) -> TcmpvalW<TIMECMP2rs> {
        TcmpvalW::new(self, 0)
    }
    ///Bits 16:18 - Timer Start Source
    #[inline(always)]
    #[must_use]
    pub fn tstart(&mut self) -> TstartW<TIMECMP2rs> {
        TstartW::new(self, 16)
    }
    ///Bits 20:22 - Source Used to Disable Comparator 2
    #[inline(always)]
    #[must_use]
    pub fn tstop(&mut self) -> TstopW<TIMECMP2rs> {
        TstopW::new(self, 20)
    }
    ///Bit 24 - Restart Timer on TCMP2
    #[inline(always)]
    #[must_use]
    pub fn restarten(&mut self) -> RestartenW<TIMECMP2rs> {
        RestartenW::new(self, 24)
    }
}
///Used to Generate Interrupts and Various Delays
///
///You can [`read`](crate::Reg::read) this register and get [`timecmp2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timecmp2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct TIMECMP2rs;
impl crate::RegisterSpec for TIMECMP2rs {
    type Ux = u32;
}
///`read()` method returns [`timecmp2::R`](R) reader structure
impl crate::Readable for TIMECMP2rs {}
///`write(|w| ..)` method takes [`timecmp2::W`](W) writer structure
impl crate::Writable for TIMECMP2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TIMECMP2 to value 0
impl crate::Resettable for TIMECMP2rs {
    const RESET_VALUE: u32 = 0;
}
