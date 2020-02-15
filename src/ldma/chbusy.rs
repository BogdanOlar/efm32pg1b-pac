#[doc = "Reader of register CHBUSY"]
pub type R = crate::R<u32, super::CHBUSY>;
#[doc = "Reader of field `BUSY`"]
pub type BUSY_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Channels Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 0xff) as u8)
    }
}
