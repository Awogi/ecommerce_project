pub mod header;
pub mod footer;
pub mod product_card;
pub mod filter_sidebar;
pub mod cart_summary;
pub mod loading;
pub mod modal;

// Re-export the existing components
pub use header::Header;
pub use footer::Footer;
pub use product_card::ProductCard;
pub use filter_sidebar::FilterSidebar;
pub use cart_summary::CartSummary;
pub use loading::{Loading, LoadingOverlay};
pub use modal::Modal;