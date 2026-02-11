use serde::{Deserialize, Serialize};

use crate::types::PostRef;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Paginator {
    pub current_page: usize,
    pub total_pages: usize,
    pub total_items: usize,
    pub items_per_page: usize,
    pub has_prev: bool,
    pub has_next: bool,
    pub prev_path: Option<String>,
    pub next_path: Option<String>,
    pub pages: Vec<PageLink>,
    pub items: Vec<PostRef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageLink {
    pub number: usize,
    pub path: String,
    pub is_current: bool,
}

impl Paginator {
    pub fn new(
        all_items: &[PostRef],
        items_per_page: usize,
        current_page: usize,
        base_path: &str,
    ) -> Self {
        let total_items = all_items.len();
        let total_pages = if total_items == 0 {
            1
        } else {
            (total_items + items_per_page - 1) / items_per_page
        };

        let current_page = current_page.clamp(1, total_pages);

        let start = (current_page - 1) * items_per_page;
        let end = (start + items_per_page).min(total_items);
        let items = all_items[start..end].to_vec();

        let has_prev = current_page > 1;
        let has_next = current_page < total_pages;

        let prev_path = if has_prev {
            Some(page_path(base_path, current_page - 1))
        } else {
            None
        };

        let next_path = if has_next {
            Some(page_path(base_path, current_page + 1))
        } else {
            None
        };

        let pages = (1..=total_pages)
            .map(|n| PageLink {
                number: n,
                path: page_path(base_path, n),
                is_current: n == current_page,
            })
            .collect();

        Self {
            current_page,
            total_pages,
            total_items,
            items_per_page,
            has_prev,
            has_next,
            prev_path,
            next_path,
            pages,
            items,
        }
    }

    /// Create all page instances for a set of items
    pub fn paginate_all(
        all_items: &[PostRef],
        items_per_page: usize,
        base_path: &str,
    ) -> Vec<Paginator> {
        let total_items = all_items.len();
        let total_pages = if total_items == 0 {
            1
        } else {
            (total_items + items_per_page - 1) / items_per_page
        };

        (1..=total_pages)
            .map(|page| Paginator::new(all_items, items_per_page, page, base_path))
            .collect()
    }
}

fn page_path(base_path: &str, page: usize) -> String {
    let base = base_path.trim_end_matches('/');
    if page == 1 {
        format!("{base}/")
    } else {
        format!("{base}/page/{page}/")
    }
}
