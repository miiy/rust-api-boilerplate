use time::{OffsetDateTime, format_description::well_known::Iso8601, error::Parse};

/// Parse an ISO8601 datetime string into OffsetDateTime
///
/// # Arguments
/// * `datetime_str` - An optional ISO8601 formatted datetime string
///
/// # Returns
/// * `Ok(Some(OffsetDateTime))` - If string is present and valid
/// * `Ok(None)` - If input is None
/// * `Err(Parse)` - If parsing fails
pub fn parse_iso8601_datetime(datetime_str: Option<String>) -> Result<Option<OffsetDateTime>, Parse> {
    match datetime_str {
        Some(dt_str) => {
            if dt_str.is_empty() {
                return Ok(None);
            }
            OffsetDateTime::parse(&dt_str, &Iso8601::DEFAULT)
                .map(Some)
        },
        None => Ok(None)
    }
}
