const DL_PREFIX: &'static str = "dotenv-linter";
const ON_PREFIX: &'static str = "on";
const OFF_PREFIX: &'static str = "off";

#[derive(Debug)]
pub struct Comment<'a> {
    disable: bool,
    pub checks: Vec<&'a str>,
}

pub fn parse(s: &str) -> Option<Comment> {
    let line_comment = s[1..].trim();

    if !line_comment.starts_with(DL_PREFIX) {
        return None;
    }

    let comments: Vec<&str> = line_comment.split(':').collect();

    let right_part = comments.last()?;
    let comments: Vec<&str> = right_part.split(' ').collect();

    let (&flag, checks) = comments.split_first()?;

    if flag != ON_PREFIX && flag != OFF_PREFIX {
        return None;
    }

    let checks: Vec<&str> = checks
        .iter()
        .map(|&s| {
            s.split(',')
                .filter(|&s| !s.is_empty())
                .collect::<Vec<&str>>()
        })
        .flatten()
        .collect();

    let mut disable: bool = false;
    if flag == OFF_PREFIX {
        disable = true;
    }

    Some(Comment { disable, checks })
}

impl Comment<'_> {
    pub fn is_disabled(&self) -> bool {
        self.disable
    }
}
