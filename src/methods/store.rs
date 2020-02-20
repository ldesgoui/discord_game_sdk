use crate::{
    callback, iter, sys, to_result::ToResult, Discord, Entitlement, Result, Sku, Snowflake,
};
use std::convert::TryInto;

/// # Store
///
/// Some operations must be ran from your game backend:
/// [Reference](https://discordapp.com/developers/docs/game-sdk/store#http-apis).
///
/// > [Chapter in official docs](https://discordapp.com/developers/docs/game-sdk/store)
impl<E> Discord<E> {
    /// Fetches the list of SKUs for the current application.
    ///
    /// Only SKUs that have a price set will be fetched.
    /// If you aren't seeing any SKUs being returned, make sure they have a price set.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/store#fetchskus)
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # fn example(discord: Discord<()>) -> Result<()> {
    /// discord.fetch_skus(|result| {
    ///     if let Err(error) = result {
    ///         return eprintln!("failed to fetch skus: {}", error);
    ///     }
    ///
    ///     for sku in discord.iter_skus() {
    ///         // ..
    ///     }
    /// });
    /// # Ok(()) }
    /// ```
    pub fn fetch_skus<'d>(&'d self, callback: impl 'd + FnOnce(Result<()>)) {
        self.with_store_manager(|mgr| {
            let (ptr, fun) =
                callback::one_param(|res: sys::EDiscordResult| callback(res.to_result()));

            unsafe { mgr.fetch_skus.unwrap()(mgr, ptr, fun) }
        })
    }

    /// Gets a SKU by its ID.
    ///
    /// [`fetch_skus`](#method.fetch_skus) must have completed first.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/store#getsku)
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # const SKU_ID: Snowflake = 0;
    /// # fn example(discord: Discord<()>) -> Result<()> {
    /// discord.fetch_skus(|result| {
    ///     if let Err(error) = result {
    ///         return eprintln!("failed to fetch skus: {}", error);
    ///     }
    ///
    ///     match discord.sku(SKU_ID) {
    ///         Ok(sku) => {
    ///             // ...
    ///         }
    ///         Err(error) => eprintln!("failed to get sku: {}", error),
    ///     }
    /// });
    /// # Ok(()) }
    /// ```
    pub fn sku(&self, id: Snowflake) -> Result<Sku> {
        let mut sku = Sku(sys::DiscordSku::default());

        self.with_store_manager(|mgr| unsafe { mgr.get_sku.unwrap()(mgr, id, &mut sku.0) })
            .to_result()?;

        Ok(sku)
    }

    /// Returns the number of SKUs available.
    ///
    /// [`fetch_skus`](#method.fetch_skus) must have completed first.
    ///
    /// Prefer using [`iter_skus`](#method.iter_skus).
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/store#countskus)
    pub fn sku_count(&self) -> u32 {
        let mut count = 0;

        self.with_store_manager(|mgr| unsafe { mgr.count_skus.unwrap()(mgr, &mut count) });

        // XXX: i32 should be u32
        count.try_into().unwrap()
    }

    /// Returns the SKU at a given index.
    ///
    /// [`fetch_skus`](#method.fetch_skus) must have completed first.
    ///
    /// Prefer using [`iter_skus`](#method.iter_skus).
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/store#getskuat)  
    pub fn sku_at(&self, index: u32) -> Result<Sku> {
        let mut sku = Sku(sys::DiscordSku::default());

        self.with_store_manager(|mgr| unsafe {
            mgr.get_sku_at.unwrap()(
                mgr,
                // XXX: i32 should be u32
                index.try_into().unwrap(),
                &mut sku.0,
            )
        })
        .to_result()?;

        Ok(sku)
    }

    /// Returns an `Iterator` over the SKUs available.
    ///
    /// [`fetch_skus`](#method.fetch_skus) must have completed first.
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # fn example(discord: Discord<()>) -> Result<()> {
    /// discord.fetch_skus(|result| {
    ///     if let Err(error) = result {
    ///         return eprintln!("failed to fetch skus: {}", error);
    ///     }
    ///
    ///     for sku in discord.iter_skus() {
    ///         // ..
    ///     }
    /// });
    /// # Ok(()) }
    /// ```
    pub fn iter_skus<'d>(
        &'d self,
    ) -> impl 'd
           + Iterator<Item = Result<Sku>>
           + DoubleEndedIterator
           + ExactSizeIterator
           + std::iter::FusedIterator
           + std::fmt::Debug {
        iter::Collection::new(self, Self::sku_at, self.sku_count())
    }

    /// Fetches a list of entitlements to which the user is entitled.
    ///
    /// Applications, DLC, and Bundles will always be returned.
    /// Consumables will be returned until they are consumed by the application via the HTTP endpoint.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/store#fetchentitlements)
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # fn example(discord: Discord<()>) -> Result<()> {
    /// discord.fetch_entitlements(|result| {
    ///     if let Err(error) = result {
    ///         return eprintln!("failed to fetch entitlements: {}", error);
    ///     }
    ///
    ///     for entitlement in discord.iter_entitlements() {
    ///         // ..
    ///     }
    /// });
    /// # Ok(()) }
    /// ```
    pub fn fetch_entitlements<'d>(&'d self, callback: impl 'd + FnOnce(Result<()>)) {
        self.with_store_manager(|mgr| {
            let (ptr, fun) =
                callback::one_param(|res: sys::EDiscordResult| callback(res.to_result()));

            unsafe { mgr.fetch_entitlements.unwrap()(mgr, ptr, fun) }
        })
    }

    /// Gets an entitlement by its ID.
    ///
    /// [`fetch_entitlements`](#method.fetch_entitlements) must have completed first.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/store#getentitlement)
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # const ENTITLEMENT_ID: Snowflake = 0;
    /// # fn example(discord: Discord<()>) -> Result<()> {
    /// discord.fetch_entitlements(|result| {
    ///     if let Err(error) = result {
    ///         return eprintln!("failed to fetch entitlements: {}", error);
    ///     }
    ///
    ///     match discord.entitlement(ENTITLEMENT_ID) {
    ///         Ok(entitlement) => {
    ///             // ...
    ///         }
    ///         Err(error) => eprintln!("failed to get entitlement: {}", error),
    ///     }
    /// });
    /// # Ok(()) }
    /// ```
    pub fn entitlement(&self, id: Snowflake) -> Result<Entitlement> {
        let mut entitlement = Entitlement(sys::DiscordEntitlement::default());

        self.with_store_manager(|mgr| unsafe {
            mgr.get_entitlement.unwrap()(mgr, id, &mut entitlement.0)
        })
        .to_result()?;

        Ok(entitlement)
    }

    /// Returns the number of Entitlements available.
    ///
    /// [`fetch_entitlements`](#method.fetch_entitlements) must have completed first.
    ///
    /// Prefer using [`iter_entitlements`](#method.iter_entitlements).
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/store#countentitlements)
    pub fn entitlement_count(&self) -> u32 {
        let mut count = 0;

        self.with_store_manager(|mgr| unsafe { mgr.count_entitlements.unwrap()(mgr, &mut count) });

        // XXX: i32 should be u32
        count.try_into().unwrap()
    }

    /// Returns Entitlement at a given index.
    ///
    /// [`fetch_entitlements`](#method.fetch_entitlements) must have completed first.
    ///
    /// Prefer using [`iter_entitlements`](#method.iter_entitlements).
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/store#getentitlementat)  
    pub fn entitlement_at(&self, index: u32) -> Result<Entitlement> {
        let mut entitlement = Entitlement(sys::DiscordEntitlement::default());

        self.with_store_manager(|mgr| unsafe {
            mgr.get_entitlement_at.unwrap()(
                mgr,
                // XXX: i32 should be u32
                index.try_into().unwrap(),
                &mut entitlement.0,
            )
        })
        .to_result()?;

        Ok(entitlement)
    }

    /// Returns an `Iterator` over the Entitlements available.
    ///
    /// [`fetch_entitlements`](#method.fetch_entitlements) must have completed first.
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # fn example(discord: Discord<()>) -> Result<()> {
    /// discord.fetch_entitlements(|result| {
    ///     if let Err(error) = result {
    ///         return eprintln!("failed to fetch entitlements: {}", error);
    ///     }
    ///
    ///     for entitlement in discord.iter_entitlements() {
    ///         // ..
    ///     }
    /// });
    /// # Ok(()) }
    /// ```
    pub fn iter_entitlements<'d>(
        &'d self,
    ) -> impl 'd
           + Iterator<Item = Result<Entitlement>>
           + DoubleEndedIterator
           + ExactSizeIterator
           + std::iter::FusedIterator
           + std::fmt::Debug {
        iter::Collection::new(self, Self::entitlement_at, self.entitlement_count())
    }

    /// Whether the user is entitled to the given SKU.
    ///
    /// [`fetch_entitlements`](#method.fetch_entitlements) must have completed first.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/store#hasskuentitlement)
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # const SKU_ID: Snowflake = 0;
    /// # fn example(discord: Discord<()>) -> Result<()> {
    /// if discord.has_entitlement(SKU_ID)? {
    ///     // ..
    /// }
    /// # Ok(()) }
    /// ```
    pub fn has_entitlement(&self, sku_id: Snowflake) -> Result<bool> {
        let mut has_entitlement = false;

        self.with_store_manager(|mgr| unsafe {
            mgr.has_sku_entitlement.unwrap()(mgr, sku_id, &mut has_entitlement)
        })
        .to_result()?;

        Ok(has_entitlement)
    }

    /// Opens the overlay to begin the in-app purchase dialogue for the given SKU.
    ///
    /// [`fetch_entitlements`](#method.fetch_entitlements) must have completed first.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/store#startpurchase)
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # const SKU_ID: Snowflake = 0;
    /// # fn example(discord: Discord<()>) -> Result<()> {
    /// discord.start_purchase(SKU_ID, |result| {
    ///     if let Err(error) = result {
    ///         return eprintln!("failed to start purchase: {}", error);
    ///     }
    /// });
    /// # Ok(()) }
    /// ```
    pub fn start_purchase<'d>(&'d self, sku_id: Snowflake, callback: impl 'd + FnOnce(Result<()>)) {
        self.with_store_manager(|mgr| {
            let (ptr, fun) =
                callback::one_param(|res: sys::EDiscordResult| callback(res.to_result()));

            unsafe { mgr.start_purchase.unwrap()(mgr, sku_id, ptr, fun) }
        })
    }
}
