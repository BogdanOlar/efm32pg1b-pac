///Register `MODEH` reader
pub type R = crate::R<MODEHrs>;
///Register `MODEH` writer
pub type W = crate::W<MODEHrs>;
///Field `MODE8` reader - Pin 8 Mode
pub use super::model::Mode0R as Mode8R;
///Field `MODE9` reader - Pin 9 Mode
pub use super::model::Mode0R as Mode9R;
///Field `MODE10` reader - Pin 10 Mode
pub use super::model::Mode0R as Mode10R;
///Field `MODE11` reader - Pin 11 Mode
pub use super::model::Mode0R as Mode11R;
///Field `MODE12` reader - Pin 12 Mode
pub use super::model::Mode0R as Mode12R;
///Field `MODE13` reader - Pin 13 Mode
pub use super::model::Mode0R as Mode13R;
///Field `MODE14` reader - Pin 14 Mode
pub use super::model::Mode0R as Mode14R;
///Field `MODE15` reader - Pin 15 Mode
pub use super::model::Mode0R as Mode15R;
///Field `MODE8` writer - Pin 8 Mode
pub use super::model::Mode0W as Mode8W;
///Field `MODE9` writer - Pin 9 Mode
pub use super::model::Mode0W as Mode9W;
///Field `MODE10` writer - Pin 10 Mode
pub use super::model::Mode0W as Mode10W;
///Field `MODE11` writer - Pin 11 Mode
pub use super::model::Mode0W as Mode11W;
///Field `MODE12` writer - Pin 12 Mode
pub use super::model::Mode0W as Mode12W;
///Field `MODE13` writer - Pin 13 Mode
pub use super::model::Mode0W as Mode13W;
///Field `MODE14` writer - Pin 14 Mode
pub use super::model::Mode0W as Mode14W;
///Field `MODE15` writer - Pin 15 Mode
pub use super::model::Mode0W as Mode15W;
///Pin 8 Mode
pub use super::model::MODE0 as MODE8;
///Pin 9 Mode
pub use super::model::MODE0 as MODE9;
///Pin 10 Mode
pub use super::model::MODE0 as MODE10;
///Pin 11 Mode
pub use super::model::MODE0 as MODE11;
///Pin 12 Mode
pub use super::model::MODE0 as MODE12;
///Pin 13 Mode
pub use super::model::MODE0 as MODE13;
///Pin 14 Mode
pub use super::model::MODE0 as MODE14;
///Pin 15 Mode
pub use super::model::MODE0 as MODE15;
impl R {
    ///Bits 0:3 - Pin 8 Mode
    #[inline(always)]
    pub fn mode8(&self) -> Mode8R {
        Mode8R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Pin 9 Mode
    #[inline(always)]
    pub fn mode9(&self) -> Mode9R {
        Mode9R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - Pin 10 Mode
    #[inline(always)]
    pub fn mode10(&self) -> Mode10R {
        Mode10R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - Pin 11 Mode
    #[inline(always)]
    pub fn mode11(&self) -> Mode11R {
        Mode11R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - Pin 12 Mode
    #[inline(always)]
    pub fn mode12(&self) -> Mode12R {
        Mode12R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - Pin 13 Mode
    #[inline(always)]
    pub fn mode13(&self) -> Mode13R {
        Mode13R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - Pin 14 Mode
    #[inline(always)]
    pub fn mode14(&self) -> Mode14R {
        Mode14R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - Pin 15 Mode
    #[inline(always)]
    pub fn mode15(&self) -> Mode15R {
        Mode15R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MODEH")
            .field("mode8", &self.mode8())
            .field("mode9", &self.mode9())
            .field("mode10", &self.mode10())
            .field("mode11", &self.mode11())
            .field("mode12", &self.mode12())
            .field("mode13", &self.mode13())
            .field("mode14", &self.mode14())
            .field("mode15", &self.mode15())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Pin 8 Mode
    #[inline(always)]
    #[must_use]
    pub fn mode8(&mut self) -> Mode8W<MODEHrs> {
        Mode8W::new(self, 0)
    }
    ///Bits 4:7 - Pin 9 Mode
    #[inline(always)]
    #[must_use]
    pub fn mode9(&mut self) -> Mode9W<MODEHrs> {
        Mode9W::new(self, 4)
    }
    ///Bits 8:11 - Pin 10 Mode
    #[inline(always)]
    #[must_use]
    pub fn mode10(&mut self) -> Mode10W<MODEHrs> {
        Mode10W::new(self, 8)
    }
    ///Bits 12:15 - Pin 11 Mode
    #[inline(always)]
    #[must_use]
    pub fn mode11(&mut self) -> Mode11W<MODEHrs> {
        Mode11W::new(self, 12)
    }
    ///Bits 16:19 - Pin 12 Mode
    #[inline(always)]
    #[must_use]
    pub fn mode12(&mut self) -> Mode12W<MODEHrs> {
        Mode12W::new(self, 16)
    }
    ///Bits 20:23 - Pin 13 Mode
    #[inline(always)]
    #[must_use]
    pub fn mode13(&mut self) -> Mode13W<MODEHrs> {
        Mode13W::new(self, 20)
    }
    ///Bits 24:27 - Pin 14 Mode
    #[inline(always)]
    #[must_use]
    pub fn mode14(&mut self) -> Mode14W<MODEHrs> {
        Mode14W::new(self, 24)
    }
    ///Bits 28:31 - Pin 15 Mode
    #[inline(always)]
    #[must_use]
    pub fn mode15(&mut self) -> Mode15W<MODEHrs> {
        Mode15W::new(self, 28)
    }
}
///Port Pin Mode High Register
///
///You can [`read`](crate::Reg::read) this register and get [`modeh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modeh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct MODEHrs;
impl crate::RegisterSpec for MODEHrs {
    type Ux = u32;
}
///`read()` method returns [`modeh::R`](R) reader structure
impl crate::Readable for MODEHrs {}
///`write(|w| ..)` method takes [`modeh::W`](W) writer structure
impl crate::Writable for MODEHrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets MODEH to value 0
impl crate::Resettable for MODEHrs {
    const RESET_VALUE: u32 = 0;
}
