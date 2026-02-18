/// Format a case ID label (e.g., "WARN", 5 -> "WARN5").
pub fn format_case_label(case_code: &str, action_case_number: u64) -> String {
    format!("{}{}", case_code.to_ascii_uppercase(), action_case_number)
}

/// Convert internal action identifiers to user-facing names.
pub fn action_display_name(action: &str) -> String {
    match action {
        "warn" => "Warned".to_owned(),
        "ban" => "Banned".to_owned(),
        "kick" => "Kicked".to_owned(),
        "timeout" => "Timeout".to_owned(),
        "unban" => "Unbanned".to_owned(),
        "untimeout" => "Untimeout".to_owned(),
        "unwarn" => "Unwarned".to_owned(),
        "unwarn_all" => "Unwarned All".to_owned(),
        "purge" => "Purged".to_owned(),
        "terminate" => "Terminated".to_owned(),
        other => {
            let mut chars = other.chars();
            match chars.next() {
                Some(first) => format!("{}{}", first.to_uppercase(), chars.as_str()),
                None => "Unknown".to_owned(),
            }
        }
    }
}

/// Parse a case label like "WARN5" into ("WARN", 5).
pub fn parse_case_label(raw: &str) -> Option<(String, u64)> {
    let input = raw.trim();
    if input.is_empty() {
        return None;
    }

    let mut split_idx = 0;
    for (idx, ch) in input.char_indices() {
        if ch.is_ascii_alphabetic() {
            split_idx = idx + ch.len_utf8();
            continue;
        }
        break;
    }

    if split_idx == 0 || split_idx >= input.len() {
        return None;
    }

    let (code, number_part) = input.split_at(split_idx);
    let number = number_part.parse::<u64>().ok().filter(|value| *value > 0)?;
    Some((code.to_ascii_uppercase(), number))
}

/// Format seconds into a compact human-readable duration (e.g. 59s, 1m, 1h, 1d, 1h 30m).
pub fn format_compact_duration(total_seconds: u64) -> String {
    let days = total_seconds / 86_400;
    let hours = (total_seconds % 86_400) / 3_600;
    let minutes = (total_seconds % 3_600) / 60;
    let seconds = total_seconds % 60;

    if days > 0 {
        return if hours > 0 {
            format!("{}d {}h", days, hours)
        } else {
            format!("{}d", days)
        };
    }

    if hours > 0 {
        return if minutes > 0 {
            format!("{}h {}m", hours, minutes)
        } else {
            format!("{}h", hours)
        };
    }

    if minutes > 0 {
        return if seconds > 0 {
            format!("{}m {}s", minutes, seconds)
        } else {
            format!("{}m", minutes)
        };
    }

    format!("{}s", seconds)
}

/// Map internal case event keys to user-facing labels.
pub fn event_display_name(event_type: &str) -> &'static str {
    match event_type {
        "created" => "Created",
        "reason_updated" => "Reason Updated",
        "note_added" => "Note Added",
        _ => "Updated",
    }
}
