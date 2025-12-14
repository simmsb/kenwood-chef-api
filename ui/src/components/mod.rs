//! The components module contains all shared components for our app. Components are the building blocks of dioxus apps.
//! They can be used to defined common UI elements like buttons, forms, and modals. In this template, we define a Hero
//! component and an Echo component for fullstack apps to be used in our app.

mod hero;
pub use hero::Hero;

mod echo;
pub use echo::Echo;

mod recipe_item;
pub use recipe_item::RecipeItem;
pub mod button;
pub mod paginate;
pub mod label;
pub mod input;
pub mod select;
pub mod textarea;
pub mod card;
pub mod navbar;
pub mod tabs;
