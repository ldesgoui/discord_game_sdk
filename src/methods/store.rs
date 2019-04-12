use crate::prelude::*;

/// # Store
impl Discord {
    // tested
    pub fn load_skus<F>(&mut self, callback: F)
    where
        F: FnMut(&mut Discord, Result<()>) + 'static,
    {
        unsafe {
            ffi!(self.get_store_manager().fetch_skus()(ResultCallback::new(
                callback
            )))
        }
    }

    pub fn sku(&mut self, id: i64) -> Result<Sku> {
        let mut sku = sys::DiscordSku::default();

        unsafe { ffi!(self.get_store_manager().get_sku(id, &mut sku as *mut _,)) }.to_result()?;

        Ok(Sku::from_sys(&sku))
    }

    // tested, returned []
    pub fn all_skus(&mut self) -> Result<Vec<Sku>> {
        let mut count = 0;

        unsafe { ffi!(self.get_store_manager().count_skus(&mut count)) }

        let mut result = Vec::with_capacity(count as usize);
        let mut sku = sys::DiscordSku::default();

        for index in 0..count {
            unsafe { ffi!(self.get_store_manager().get_sku_at(index as i32, &mut sku)) }
                .to_result()?;

            result.push(Sku::from_sys(&sku))
        }

        Ok(result)
    }

    pub fn entitlement(&mut self, id: i64) -> Result<Entitlement> {
        let mut entitlement = sys::DiscordEntitlement::default();

        unsafe {
            ffi!(self
                .get_store_manager()
                .get_entitlement(id, &mut entitlement as *mut _,))
        }
        .to_result()?;

        Ok(Entitlement::from_sys(&entitlement))
    }

    // tested, returned []
    pub fn all_entitlements(&mut self) -> Result<Vec<Entitlement>> {
        let mut count = 0;

        unsafe { ffi!(self.get_store_manager().count_entitlements(&mut count)) }

        let mut result = Vec::with_capacity(count as usize);
        let mut entitlement = sys::DiscordEntitlement::default();

        for index in 0..count {
            unsafe {
                ffi!(self
                    .get_store_manager()
                    .get_entitlement_at(index as i32, &mut entitlement))
            }
            .to_result()?;

            result.push(Entitlement::from_sys(&entitlement))
        }

        Ok(result)
    }

    pub fn has_entitlement(&mut self, sku_id: i64) -> Result<bool> {
        let mut has_entitlement = false;

        unsafe {
            ffi!(self
                .get_store_manager()
                .has_sku_entitlement(sku_id, &mut has_entitlement as *mut _))
        }
        .to_result()?;

        Ok(has_entitlement)
    }

    pub fn start_purchase<F>(&mut self, sku_id: i64, callback: F)
    where
        F: FnMut(&mut Discord, Result<()>) + 'static,
    {
        unsafe {
            ffi!(self.get_store_manager().start_purchase(sku_id)(
                ResultCallback::new(callback)
            ))
        }
    }
}
