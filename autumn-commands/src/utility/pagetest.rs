use autumn_core::{Context, Error};
use autumn_utils::pagination::{page_window, paginate_embed_pages, total_pages};

use crate::CommandMeta;

pub const META: CommandMeta = CommandMeta {
    name: "pagetest",
    desc: "Test embed pagination behavior.",
    category: "utility",
    usage: "!pagetest [page]",
};

const ITEMS_PER_PAGE: usize = 5;

#[poise::command(prefix_command, slash_command, category = "Utility")]
pub async fn pagetest(
    ctx: Context<'_>,
    #[description = "Starting page"] page: Option<usize>,
) -> Result<(), Error> {
    let items = build_test_items();
    let total = total_pages(items.len(), ITEMS_PER_PAGE);
    let requested_page = page.unwrap_or(1);

    if requested_page == 0 || requested_page > total {
        ctx.say(format!(
            "Page {} does not exist. Available pages: 1-{}.",
            requested_page, total
        ))
        .await?;
        return Ok(());
    }

    let pages = (1..=total)
        .map(|current_page| {
            let (start, end) = page_window(items.len(), ITEMS_PER_PAGE, current_page);
            items[start..end]
                .iter()
                .map(|item| format!("• {}", item))
                .collect::<Vec<_>>()
                .join("\n")
        })
        .collect::<Vec<_>>();
    paginate_embed_pages(ctx, "Pagination Test", &pages, requested_page).await?;
    Ok(())
}

fn build_test_items() -> Vec<String> {
    (1..=24)
        .map(|index| format!("Sample pagination item #{index}"))
        .collect()
}
