#[doc = "Register `HFXOTRIMSTATUS` reader"]
pub type R = crate::R<HFXOTRIMSTATUSrs>;
#[doc = "Field `IBTRIMXOCORE` reader - Value of IBTRIMXOCORE Found By Automatic HFXO Peak Detection Algorithm"]
pub type IBTRIMXOCORE_R = crate::FieldReader;
#[doc = "Field `REGISH` reader - Value of REGISH Found By Automatic HFXO Shunt Current Optimization Algorithm"]
pub type REGISH_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:6 - Value of IBTRIMXOCORE Found By Automatic HFXO Peak Detection Algorithm"]
    #[inline(always)]
    pub fn ibtrimxocore(&self) -> IBTRIMXOCORE_R {
        IBTRIMXOCORE_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:10 - Value of REGISH Found By Automatic HFXO Shunt Current Optimization Algorithm"]
    #[inline(always)]
    pub fn regish(&self) -> REGISH_R {
        REGISH_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
}
#[doc = "HFXO Trim Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfxotrimstatus::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
