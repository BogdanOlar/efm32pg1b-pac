#[doc = "Reader of register HFXOTRIMSTATUS"]
pub type R = crate::R<u32, super::HFXOTRIMSTATUS>;
#[doc = "Reader of field `IBTRIMXOCORE`"]
pub type IBTRIMXOCORE_R = crate::R<u8, u8>;
#[doc = "Reader of field `REGISH`"]
pub type REGISH_R = crate::R<u8, u8>;
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
