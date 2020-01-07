use crate::{sys, to_result::ToResult, Collection, Discord, Entitlement, Result, Sku, Snowflake};

/// # Store
///
/// Some operations must be ran from your game backend:
/// [Reference](https://discordapp.com/developers/docs/game-sdk/store#http-apis).
///
/// > [Chapter in official docs](https://discordapp.com/developers/docs/game-sdk/store)
impl Discord {
    /// Fetches the list of SKUs for the current application.
    ///
    /// Only SKUs that have a price set will be fetched.
    /// If you aren't seeing any SKUs being returned, make sure they have a price set.
    ///
    /// > [Method in official docs](https://discordapp.com/developers/docs/game-sdk/store#fetchskus)
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # fn example(discord: Discord) -> Result<()> {
    /// discord.fetch_skus(|discord, result| {
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
    pub fn fetch_skus(&self, callback: impl 'static + FnOnce(&Discord, Result<()>)) {
        unsafe {
            ffi!(self
                .get_store_manager()
                .fetch_skus()
                .and_then(|res: sys::EDiscordResult| callback::<Result<()>>(res.to_result())))
        }
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
    /// # fn example(discord: Discord) -> Result<()> {
    /// discord.fetch_skus(|discord, result| {
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

        unsafe { ffi!(self.get_store_manager().get_sku(id, &mut sku.0)) }.to_result()?;

        Ok(sku)
    }

    /// Returns the number of SKUs available.
    ///
    /// [`fetch_skus`](#method.fetch_skus) must have completed first.
    ///
    /// Prefer using [`iter_skus`](#method.iter_skus).
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
    /// Prefer using [`iter_skus`](#method.iter_skus).
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
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # fn example(discord: Discord) -> Result<()> {
    /// discord.fetch_skus(|discord, result| {
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
    pub fn iter_skus(&self) -> Collection<Result<Sku>> {
        let count = self.sku_count();

        Collection::new(self, Box::new(|d, i| d.sku_at(i)), count)
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
    /// # fn example(discord: Discord) -> Result<()> {
    /// discord.fetch_entitlements(|discord, result| {
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
    pub fn fetch_entitlements(&self, callback: impl 'static + FnOnce(&Discord, Result<()>)) {
        unsafe {
            ffi!(self
                .get_store_manager()
                .fetch_entitlements()
                .and_then(|res: sys::EDiscordResult| callback::<Result<()>>(res.to_result())))
        }
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
    /// # fn example(discord: Discord) -> Result<()> {
    /// discord.fetch_entitlements(|discord, result| {
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
    /// Prefer using [`iter_entitlements`](#method.iter_entitlements).
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
    /// Prefer using [`iter_entitlements`](#method.iter_entitlements).
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
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # fn example(discord: Discord) -> Result<()> {
    /// discord.fetch_entitlements(|discord, result| {
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
    pub fn iter_entitlements(&self) -> Collection<Result<Entitlement>> {
        let count = self.entitlement_count();

        Collection::new(self, Box::new(|d, i| d.entitlement_at(i)), count)
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
    /// # fn example(discord: Discord) -> Result<()> {
    /// if discord.has_entitlement(SKU_ID)? {
    ///     // ..
    /// }
    /// # Ok(()) }
    /// ```
    pub fn has_entitlement(&self, sku_id: Snowflake) -> Result<bool> {
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
    ///
    /// ```rust
    /// # use discord_game_sdk::*;
    /// # const SKU_ID: Snowflake = 0;
    /// # fn example(discord: Discord) -> Result<()> {
    /// discord.start_purchase(SKU_ID, |discord, result| {
    ///     if let Err(error) = result {
    ///         return eprintln!("failed to start purchase: {}", error);
    ///     }
    /// });
    /// # Ok(()) }
    /// ```
    pub fn start_purchase(
        &self,
        sku_id: Snowflake,
        callback: impl 'static + FnOnce(&Discord, Result<()>),
    ) {
        unsafe {
            ffi!(self
                .get_store_manager()
                .start_purchase(sku_id)
                .and_then(|res: sys::EDiscordResult| callback::<Result<()>>(res.to_result())))
        }
    }
}
