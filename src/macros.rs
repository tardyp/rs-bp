// define macro tag with additional context
#[macro_export]
macro_rules! context_tag {
    ($tag:expr) => {
        context(
            $tag,
            delimited(space_or_comments, tag($tag), space_or_comments),
        )
    };
}
// define macro ending delimiter with optional comma
#[macro_export]
macro_rules! end_delimiter {
    ($tag:expr) => {
        tuple((
            space_or_comments,
            opt(char(',')),
            space_or_comments,
            cut(tag($tag)),
            space_or_comments,
        ))
    };
}
