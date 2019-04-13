#[derive(Clone, Debug)]
pub(crate) enum MetadataUpdate<'a> {
    Add(&'a str, &'a str),
    Delete(&'a str),
}
