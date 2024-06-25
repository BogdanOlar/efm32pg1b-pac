#[doc = "Register `HFXOTRIMSTATUS` reader"]
pub type R = crate::R<HFXOTRIMSTATUSrs>;
#[doc = "Field `IBTRIMXOCORE` reader - Value of IBTRIMXOCORE Found By Automatic HFXO Peak Detection Algorithm"]
pub type IbtrimxocoreR = crate::FieldReader;
#[doc = "Field `REGISH` reader - Value of REGISH Found By Automatic HFXO Shunt Current Optimization Algorithm"]
pub type RegishR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:6 - Value of IBTRIMXOCORE Found By Automatic HFXO Peak Detection Algorithm"]
    #[inline(always)]
    pub fn ibtrimxocore(&self) -> IbtrimxocoreR {
        IbtrimxocoreR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:10 - Value of REGISH Found By Automatic HFXO Shunt Current Optimization Algorithm"]
    #[inline(always)]
    pub fn regish(&self) -> RegishR {
        RegishR::new(((self.bits >> 7) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HFXOTRIMSTATUS")
            .field("ibtrimxocore", &self.ibtrimxocore())
            .field("regish", &self.regish())
            .finish()
    }
}
#[doc = "HFXO Trim Status\n\nYou can [`read`](crate::Reg::read) this register and get [`hfxotrimstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HFXOTRIMSTATUSrs;
impl crate::RegisterSpec for HFXOTRIMSTATUSrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfxotrimstatus::R`](R) reader structure"]
impl crate::Readable for HFXOTRIMSTATUSrs {}
#[doc = "`reset()` method sets HFXOTRIMSTATUS to value 0x0500"]
impl crate::Resettable for HFXOTRIMSTATUSrs {
    const RESET_VALUE: u32 = 0x0500;
}
