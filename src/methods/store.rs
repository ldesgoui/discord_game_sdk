use crate::{
    callbacks::ResultCallback, sys, to_result::ToResult, Discord, DiscordResult, Entitlement, Sku,
};

/// # Store
impl<'a> Discord<'a> {
    // tested
    pub fn load_skus<F>(&mut self, callback: F)
    where
        F: FnMut(&mut Discord, DiscordResult<()>) + 'a,
    {
        unsafe {
            ffi!(self.get_store_manager().fetch_skus()(ResultCallback::new(
                callback
            )))
        }
    }

    pub fn sku(&mut self, id: i64) -> DiscordResult<Sku> {
        let mut sku = sys::DiscordSku::default();

        unsafe { ffi!(self.get_store_manager().get_sku(id, &mut sku as *mut _,)) }.to_result()?;

        Ok(Sku::from(sku))
    }

    // tested, returned []
    pub fn all_skus(&mut self) -> DiscordResult<Vec<Sku>> {
        let mut count = 0;

        unsafe { ffi!(self.get_store_manager().count_skus(&mut count)) }

        let mut result = Vec::with_capacity(count as usize);
        let mut sku = sys::DiscordSku::default();

        for index in 0..count {
            unsafe { ffi!(self.get_store_manager().get_sku_at(index as i32, &mut sku)) }
                .to_result()?;

            result.push(Sku::from(sku))
        }

        Ok(result)
    }

    pub fn entitlement(&mut self, id: i64) -> DiscordResult<Entitlement> {
        let mut entitlement = sys::DiscordEntitlement::default();

        unsafe {
            ffi!(self
                .get_store_manager()
                .get_entitlement(id, &mut entitlement as *mut _,))
        }
        .to_result()?;

        Ok(Entitlement::from(entitlement))
    }

    // tested, returned []
    pub fn all_entitlements(&mut self) -> DiscordResult<Vec<Entitlement>> {
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

            result.push(Entitlement::from(entitlement))
        }

        Ok(result)
    }

    pub fn has_entitlement(&mut self, sku_id: i64) -> DiscordResult<bool> {
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
        F: FnMut(&mut Discord, DiscordResult<()>) + 'a,
    {
        unsafe {
            ffi!(self.get_store_manager().start_purchase(sku_id)(
                ResultCallback::new(callback)
            ))
        }
    }
}
