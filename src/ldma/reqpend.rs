#[doc = "Reader of register REQPEND"]
pub type R = crate::R<u32, super::REQPEND>;
#[doc = "Reader of field `REQPEND`"]
pub type REQPEND_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - DMA Requests Pending"]
    #[inline(always)]
    pub fn reqpend(&self) -> REQPEND_R {
        REQPEND_R::new((self.bits & 0xff) as u8)
    }
}
