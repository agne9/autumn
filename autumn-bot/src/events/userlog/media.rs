use tracing::error;

/// Parsed attachment info from the `filename (url)` summary format.
#[derive(Clone, Debug)]
pub struct AttachmentInfo {
    pub filename: String,
    pub url: String,
    pub is_media: bool,
}

/// Parses the `filename (url)\nfilename2 (url2)` summary into structured items.
pub fn parse_attachment_summary(raw: &str) -> Vec<AttachmentInfo> {
    raw.lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                return None;
            }

            let start = trimmed.rfind(" (")?;
            if !trimmed.ends_with(')') {
                return None;
            }

            let filename = trimmed[..start].to_owned();
            let url = trimmed[start + 2..trimmed.len() - 1].to_owned();
            let lower = filename.to_ascii_lowercase();
            let is_media = lower.ends_with(".png")
                || lower.ends_with(".jpg")
                || lower.ends_with(".jpeg")
                || lower.ends_with(".gif")
                || lower.ends_with(".webp")
                || lower.ends_with(".mp4")
                || lower.ends_with(".webm")
                || lower.ends_with(".mov");

            Some(AttachmentInfo {
                filename,
                url,
                is_media,
            })
        })
        .collect()
}

/// Finds the first URL in content that looks like an embeddable media link.
pub fn extract_first_media_url(content: Option<&str>) -> Option<String> {
    let content = content?.trim();
    if content.is_empty() {
        return None;
    }

    content.split_whitespace().find_map(|token| {
        let clean =
            token.trim_matches(|character| matches!(character, '<' | '>' | '(' | ')' | '[' | ']'));
        let lower = clean.to_ascii_lowercase();
        if !lower.starts_with("http://") && !lower.starts_with("https://") {
            return None;
        }

        let looks_embeddable = lower.ends_with(".png")
            || lower.ends_with(".jpg")
            || lower.ends_with(".jpeg")
            || lower.ends_with(".gif")
            || lower.ends_with(".webp")
            || lower.ends_with(".mp4")
            || lower.ends_with(".webm")
            || lower.contains("tenor.com")
            || lower.contains("giphy.com")
            || lower.contains("cdn.discordapp.com")
            || lower.contains("media.discordapp.net");

        if looks_embeddable {
            Some(clean.to_owned())
        } else {
            None
        }
    })
}

/// Checks whether a URL points directly to an image (by extension or Discord CDN params).
pub fn is_direct_image_url(url: &str) -> bool {
    let lower = url.to_ascii_lowercase();
    lower.ends_with(".png")
        || lower.ends_with(".jpg")
        || lower.ends_with(".jpeg")
        || lower.ends_with(".gif")
        || lower.ends_with(".webp")
        || (lower.contains("media.discordapp.net")
            && (lower.contains("format=gif")
                || lower.contains("format=png")
                || lower.contains("format=jpg")
                || lower.contains("format=jpeg")
                || lower.contains("format=webp")))
}

/// Guesses a filename from a media URL, falling back to a default.
pub fn infer_media_filename(url: &str, fallback: &str) -> String {
    let cleaned = url.split('?').next().unwrap_or(url);
    let candidate = cleaned.rsplit('/').next().unwrap_or(fallback);
    if candidate.trim().is_empty() {
        fallback.to_owned()
    } else {
        candidate.to_owned()
    }
}

/// Replaces potentially dangerous characters in an attachment filename.
pub fn sanitize_attachment_filename(filename: &str) -> String {
    let mut out = filename
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || matches!(character, '.' | '_' | '-') {
                character
            } else {
                '_'
            }
        })
        .collect::<String>();

    if out.is_empty() {
        out = "attachment.bin".to_owned();
    }

    out
}

/// Finds the first URL in content that should be sent as a standalone unfurl message.
pub fn extract_first_unfurl_link(content: Option<&str>) -> Option<String> {
    let content = content?.trim();
    if content.is_empty() {
        return None;
    }

    content.split_whitespace().find_map(|token| {
        let clean =
            token.trim_matches(|character| matches!(character, '<' | '>' | '(' | ')' | '[' | ']'));
        let lower = clean.to_ascii_lowercase();
        if !(lower.starts_with("http://") || lower.starts_with("https://")) {
            return None;
        }

        if lower.contains("tenor.com")
            || lower.contains("giphy.com")
            || lower.ends_with(".mp4")
            || lower.ends_with(".webm")
            || lower.ends_with(".mov")
        {
            Some(clean.to_owned())
        } else {
            None
        }
    })
}

/// Downloads media bytes from a URL, respecting a 25 MB size limit.
pub async fn download_media_bytes(url: &str) -> Option<Vec<u8>> {
    let response = reqwest::get(url).await.ok()?;
    if !response.status().is_success() {
        return None;
    }

    let max_size = 25 * 1024 * 1024u64;
    if response
        .content_length()
        .is_some_and(|size| size > max_size)
    {
        return None;
    }

    let bytes = response.bytes().await.ok()?;
    if bytes.len() as u64 > max_size {
        error!(url, "media download exceeded 25 MB limit after fetch");
        return None;
    }

    Some(bytes.to_vec())
}
