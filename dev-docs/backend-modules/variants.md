# Variants Module

## Motivation

The `variants` module is responsible for defining and managing the different game "variants" that the launcher supports (e.g., "Cataclysm: Dark Days Ahead," "Cataclysm: Bright Nights"). It acts as the source of truth for what variants exist and provides functionality to manage their metadata and user-configurable display order.

The primary goals of this module are:
1.  **Definition:** To define the core `GameVariant` enum, which provides a unique, strongly-typed identifier for each supported game version.
2.  **Metadata Management:** To provide access to variant-specific information, such as its full display name and relevant web links, which are configured in `settings.json`.
3.  **User Customization:** To allow the user to define the order in which the game variants are displayed in the UI and to persist this order in the database.
4.  **Business Logic:** To encapsulate logic that is specific to a variant, such as the rules for determining if a release is "Stable" or "Experimental."

## Design

The module is structured around the `GameVariant` enum, with business logic and data access separated into different files.

1.  **Core Definition (`game_variant.rs`):**
    *   **`GameVariant` Enum:** This `strum` and `serde` enabled enum is the heart of the module. It exhaustively lists all supported game variants. The `Display` and `IntoStaticStr` derives allow for easy conversion to and from strings, which are used as IDs.
    *   **Methods on `GameVariant`:**
        *   `id()`: Returns the static string ID for the variant (e.g., "DarkDaysAhead").
        *   `name()` and `links()`: These methods act as convenient accessors that retrieve the variant's display name and links from the managed `Settings` object. This decouples the core enum from the configurable metadata.
        *   `determine_release_type()`: Contains the specific business logic for a variant to classify a GitHub release as `Stable`, `Experimental`, etc., based on its tag name and `prerelease` flag.

2.  **Order Management (`update_game_variant_order.rs`, `get_game_variants_info.rs`):**
    *   **`update_game_variant_order`:** This function takes a `Vec<GameVariant>` representing the new desired order and persists it to the database via the `GameVariantOrderRepository`.
    *   **`get_game_variants_info`:** This function is responsible for composing the complete set of information about all variants for the UI. It:
        1.  Iterates through all possible `GameVariant`s using `strum::IntoEnumIterator`.
        2.  For each variant, it fetches its metadata (name, links) from the `Settings` object.
        3.  It fetches the user-defined sort order from the `GameVariantOrderRepository`.
        4.  It combines this information into a `Vec<GameVariantInfo>` and sorts it according to the user's preferred order, falling back to a default order if none is set.

3.  **Repository (`repository/`):**
    *   Defines the `GameVariantOrderRepository` trait and its `SqliteGameVariantOrderRepository` implementation, which handles the SQL queries for storing and retrieving the serialized `Vec<GameVariant>` that represents the user's preferred sort order.

4.  **Framework Bridge (`commands.rs`):**
    *   **`get_game_variants_info` Command:** Exposes the function of the same name to the frontend, allowing the UI to fetch all the data it needs to render the list of game variants.
    *   **`update_game_variant_order` Command:** Allows the frontend to save the new order after a user drags and drops variants in the settings UI.

## Workings

1.  **Displaying Variants:**
    *   The frontend starts and calls the `get_game_variants_info` command.
    *   The command calls the core `get_game_variants_info` function.
    *   This function gets the list of all variants from the enum's iterator. It gets their names and links from the `Settings`. It gets the custom order (e.g., `[BrightNights, DarkDaysAhead]`) from the database.
    *   It then sorts the list of `GameVariantInfo` structs according to the custom order and returns it to the frontend. The UI can then render the variants in the correct, user-defined order.

2.  **Updating Order:**
    *   The user drags "Dark Days Ahead" to be the first item in the list in the settings page.
    *   The frontend calls the `update_game_variant_order` command with the new list: `[DarkDaysAhead, BrightNights]`.
    *   The command calls the core `update_game_variant_order` function, which serializes the list and saves it to the database, overwriting the previous order.
