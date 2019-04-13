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
        let mut sku = Sku(sys::DiscordSku::default());

        unsafe { ffi!(self.get_store_manager().get_sku(id, &mut sku.0 as *mut _,)) }.to_result()?;

        Ok(sku)
    }

    // tested, returned []
    pub fn all_skus(&mut self) -> DiscordResult<Vec<Sku>> {
        let mut count = 0;

        unsafe { ffi!(self.get_store_manager().count_skus(&mut count)) }

        let mut result = Vec::with_capacity(count as usize);
        let mut sku = Sku(sys::DiscordSku::default());

        for index in 0..count {
            unsafe {
                ffi!(self
                    .get_store_manager()
                    .get_sku_at(index as i32, &mut sku.0))
            }
            .to_result()?;

            result.push(sku)
        }

        Ok(result)
    }

    pub fn entitlement(&mut self, id: i64) -> DiscordResult<Entitlement> {
        let mut entitlement = Entitlement(sys::DiscordEntitlement::default());

        unsafe {
            ffi!(self
                .get_store_manager()
                .get_entitlement(id, &mut entitlement.0 as *mut _,))
        }
        .to_result()?;

        Ok(entitlement)
    }

    // tested, returned []
    pub fn all_entitlements(&mut self) -> DiscordResult<Vec<Entitlement>> {
        let mut count = 0;

        unsafe { ffi!(self.get_store_manager().count_entitlements(&mut count)) }

        let mut result = Vec::with_capacity(count as usize);
        let mut entitlement = Entitlement(sys::DiscordEntitlement::default());

        for index in 0..count {
            unsafe {
                ffi!(self
                    .get_store_manager()
                    .get_entitlement_at(index as i32, &mut entitlement.0))
            }
            .to_result()?;

            result.push(entitlement);
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
