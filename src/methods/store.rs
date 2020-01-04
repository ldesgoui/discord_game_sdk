use crate::{
    callbacks::ResultCallback, event, iter, sys, to_result::ToResult, Discord, Entitlement, Result,
    Sku,
};

/// # Store
///
/// Some operations must be ran from your game backend:
/// [Reference](https://discordapp.com/developers/docs/game-sdk/store#http-apis).
///
/// > [Chapter in official docs](https://discordapp.com/developers/docs/game-sdk/store)
impl<'a> Discord<'a> {
    /// Fetches the list of SKUs for the current application.
    ///
    /// Only SKUs that have a price set will be fetched.
    /// If you aren't seeing any SKUs being returned, make sure they have a price set.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/store#fetchskus)
    pub fn fetch_skus(&self, callback: impl 'a + FnMut(&Discord<'_>, Result<()>)) {
        unsafe {
            ffi!(self
                .get_store_manager()
                .fetch_skus()
                .and_then(ResultCallback::new(callback)))
        }
    }

    /// Gets a SKU by its ID.
    ///
    /// [`fetch_skus`](#method.fetch_skus) must have completed first.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/store#getsku)
    pub fn sku(&self, id: i64) -> Result<Sku> {
        let mut sku = Sku(sys::DiscordSku::default());

        unsafe { ffi!(self.get_store_manager().get_sku(id, &mut sku.0)) }.to_result()?;

        Ok(sku)
    }

    /// Returns the number of SKUs available.
    ///
    /// [`fetch_skus`](#method.fetch_skus) must have completed first.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/store#countskus)
    pub fn sku_count(&self) -> usize {
        let mut count = 0;

        unsafe { ffi!(self.get_store_manager().count_skus(&mut count)) }

        count as usize
    }

    /// Returns the SKU at a given index.
    ///
    /// [`fetch_skus`](#method.fetch_skus) must have completed first.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/store#getskuat)  
    pub fn sku_at(&self, index: usize) -> Result<Sku> {
        let mut sku = Sku(sys::DiscordSku::default());

        unsafe {
            ffi!(self
                .get_store_manager()
                .get_sku_at(index as i32, &mut sku.0))
        }
        .to_result()?;

        Ok(sku)
    }

    /// Returns an `Iterator` over the SKUs available.
    ///
    /// [`fetch_skus`](#method.fetch_skus) must have completed first.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/store#countskus)
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/store#getskuat)  
    pub fn iter_skus<'b>(&'b self) -> iter::GenericIter<'a, 'b, Result<Sku>> {
        let count = self.sku_count();

        iter::GenericIter::new(self, Box::new(|d, i| d.sku_at(i)), count)
    }

    /// Fetches a list of entitlements to which the user is entitled.
    ///
    /// Applications, DLC, and Bundles will always be returned.
    /// Consumables will be returned until they are consumed by the application via the HTTP endpoint.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/store#fetchentitlements)
    pub fn fetch_entitlements(&self, callback: impl 'a + FnMut(&Discord<'_>, Result<()>)) {
        unsafe {
            ffi!(self
                .get_store_manager()
                .fetch_entitlements()
                .and_then(ResultCallback::new(callback)))
        }
    }

    /// Gets an entitlement by its ID.
    ///
    /// [`fetch_entitlements`](#method.fetch_entitlements) must have completed first.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/store#getentitlement)
    pub fn entitlement(&self, id: i64) -> Result<Entitlement> {
        let mut entitlement = Entitlement(sys::DiscordEntitlement::default());

        unsafe {
            ffi!(self
                .get_store_manager()
                .get_entitlement(id, &mut entitlement.0))
        }
        .to_result()?;

        Ok(entitlement)
    }

    /// Returns the number of Entitlements available.
    ///
    /// [`fetch_entitlements`](#method.fetch_entitlements) must have completed first.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/store#countentitlements)
    pub fn entitlement_count(&self) -> usize {
        let mut count = 0;

        unsafe { ffi!(self.get_store_manager().count_entitlements(&mut count)) }

        count as usize
    }

    /// Returns Entitlement at a given index.
    ///
    /// [`fetch_entitlements`](#method.fetch_entitlements) must have completed first.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/store#getentitlementat)  
    pub fn entitlement_at(&self, index: usize) -> Result<Entitlement> {
        let mut entitlement = Entitlement(sys::DiscordEntitlement::default());

        unsafe {
            ffi!(self
                .get_store_manager()
                .get_entitlement_at(index as i32, &mut entitlement.0))
        }
        .to_result()?;

        Ok(entitlement)
    }

    /// Returns an `Iterator` over the Entitlements available.
    ///
    /// [`fetch_entitlements`](#method.fetch_entitlements) must have completed first.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/store#countentitlements)
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/store#getentitlementat)  
    pub fn iter_entitlements<'b>(&'b self) -> iter::GenericIter<'a, 'b, Result<Entitlement>> {
        let count = self.entitlement_count();

        iter::GenericIter::new(self, Box::new(|d, i| d.entitlement_at(i)), count)
    }

    /// Whether the user is entitled to the given SKU.
    ///
    /// [`fetch_entitlements`](#method.fetch_entitlements) must have completed first.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/store#hasskuentitlement)
    pub fn has_entitlement(&self, sku_id: i64) -> Result<bool> {
        let mut has_entitlement = false;

        unsafe {
            ffi!(self
                .get_store_manager()
                .has_sku_entitlement(sku_id, &mut has_entitlement))
        }
        .to_result()?;

        Ok(has_entitlement)
    }

    /// Opens the overlay to begin the in-app purchase dialogue for the given SKU.
    ///
    /// [`fetch_entitlements`](#method.fetch_entitlements) must have completed first.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/store#startpurchase)
    pub fn start_purchase(&self, sku_id: i64, callback: impl 'a + FnMut(&Discord<'_>, Result<()>)) {
        unsafe {
            ffi!(self
                .get_store_manager()
                .start_purchase(sku_id)
                .and_then(ResultCallback::new(callback)))
        }
    }

    /// Fires when the connected user receives a new entitlement,
    /// either through purchase or through a developer grant.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/store#onentitlementcreate)
    pub fn recv_store_entitlement_create(
        &self,
    ) -> impl '_ + Iterator<Item = event::StoreEntitlementCreate> {
        self.receivers.store_entitlement_create.try_iter()
    }

    /// Fires when the connected user loses an entitlement,
    /// either by expiration, revocation, or consumption in the case of consumable entitlements.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/store#onentitlementdelete)
    pub fn recv_store_entitlement_delete(
        &self,
    ) -> impl '_ + Iterator<Item = event::StoreEntitlementDelete> {
        self.receivers.store_entitlement_delete.try_iter()
    }
}
