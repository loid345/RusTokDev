// DTOs for pages-related requests/responses.
pub mod block;
pub mod menu;
pub mod page;

pub use block::{BlockResponse, CreateBlockInput, UpdateBlockInput};
pub use menu::{CreateMenuInput, MenuResponse, UpdateMenuInput};
pub use page::{
    CreatePageInput, ListPagesFilter, PageBodyInput, PageBodyResponse, PageListItem, PageResponse,
    PageTranslationInput, PageTranslationResponse, UpdatePageInput,
};
