use crate::prelude::*;

/// # Store
impl<'a> Discord<'a> {
    pub fn load_skus<F>(&mut self, callback: F)
    where
        F: FnMut(Result<()>),
    {
        unsafe {
            ffi!(self.get_store_manager().fetch_skus(
                Box::into_raw(Box::new(callback)) as *mut _,
                Some(callbacks::result::<F>)
            ))
        }
    }

    pub fn sku<F>(&mut self, id: i64) -> Result<Sku> {
        let mut sku = sys::DiscordSku::default();

        unsafe { ffi!(self.get_store_manager().get_sku(id, &mut sku as *mut _,)) }.to_result()?;

        Ok(Sku::from_sys(&sku))
    }

    pub fn all_skus<F>(&mut self) -> Result<Vec<Sku>> {
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

    pub fn entitlement<F>(&mut self, id: i64) -> Result<Entitlement> {
        let mut entitlement = sys::DiscordEntitlement::default();

        unsafe {
            ffi!(self
                .get_store_manager()
                .get_entitlement(id, &mut entitlement as *mut _,))
        }
        .to_result()?;

        Ok(Entitlement::from_sys(&entitlement))
    }

    pub fn all_entitlements<F>(&mut self) -> Result<Vec<Entitlement>> {
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
        F: FnMut(Result<()>),
    {
        unsafe {
            ffi!(self.get_store_manager().start_purchase(
                sku_id,
                Box::into_raw(Box::new(callback)) as *mut _,
                Some(callbacks::result::<F>)
            ))
        }
    }
}
