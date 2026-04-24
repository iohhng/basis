use anyhow::{bail, Result};

pub fn split_frontmatter(source: &str) -> Result<(&str, &str)> {
    let after_open = source.strip_prefix("---\n")
    .ok_or_else(|| anyhow::anyhow!("doesn't begin with ---"))?;

    let close = after_open.find("\n---\n")
    .ok_or_else(|| anyhow::anyhow!("no closing ---"))?;

    let frontmatter = &after_open[..close];
    let body = &after_open[close + "\n---\n".len()..];

    Ok((frontmatter, body))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_file() {
        let src = "---\nid = \"0042\"\n---\nbody here\n";
        let (fm, body) = split_frontmatter(src).unwrap();
        assert_eq!(fm, "id = \"0042\"");
        assert_eq!(body, "body here\n");
    }

    #[test]
    fn no_opening_delimiter() {
        let src = "id = \"0042\"\n---\nbody\n";
        assert!(split_frontmatter(src).is_err());
    }

    #[test]
    fn no_closing_delimiter() {
        let src = "---\nid = \"0042\"\nbody with no close\n";
        assert!(split_frontmatter(src).is_err());
    }

    #[test]
    fn dashes_in_body_do_not_close_frontmatter() {
        let src = "---\nid = \"0042\"\n---\nbody\n---\nmore body\n";
        let (fm, body) = split_frontmatter(src).unwrap();
        assert_eq!(fm, "id = \"0042\"");
        assert_eq!(body, "body\n---\nmore body\n");
    }

    #[test]
    fn empty_body() {
        let src = "---\nid = \"0042\"\n---\n";
        let (fm, body) = split_frontmatter(src).unwrap();
        assert_eq!(fm, "id = \"0042\"");
        assert_eq!(body, "");
    }
}
