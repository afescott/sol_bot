/// Formats a list of addresses into comma-separated list.
///
/// # Examples
///
/// ```
/// # use dexscreener::format_addresses;
/// assert_eq!(
///     format_addresses([
///         "0x1111111111111111111111111111111111111111",
///         "0x2222222222222222222222222222222222222222",
///     ]).unwrap(),
///     "0x1111111111111111111111111111111111111111,0x2222222222222222222222222222222222222222"
/// );
/// ```
pub fn format_addresses(
    pair_addresses: impl IntoIterator<Item = impl AsRef<str>>,
) -> crate::error::Result<String> {
    let mut iter = pair_addresses.into_iter();
    let first = match iter.next() {
        Some(first) => first,
        None => return Ok(String::new()),
    };
    let cap = iter.size_hint().1.unwrap_or(5);
    let mut out = String::with_capacity(cap * 45);
    format_address(first.as_ref(), &mut out)?;
    for address in iter {
        out.push(',');
        format_address(address.as_ref(), &mut out)?;
    }
    Ok(out)
}

fn format_address(address: &str, out: &mut String) -> crate::Result<()> {
    match address.len() {
        // Ethereum: `/(0x)?[0-9A-Fa-f]{40}/`
        40 if address.chars().all(|c| c.is_ascii_hexdigit()) => {
            out.push('0');
            out.push('x');
            out.push_str(address);
            Ok(())
        }
        42 if address.starts_with("0x")
            && address.chars().skip(2).all(|c| c.is_ascii_hexdigit()) =>
        {
            out.push_str(address);
            Ok(())
        }

        // Solana: `/[0-9A-Za-z]{44}/`
        44 if address.chars().all(|c| c.is_ascii_alphanumeric()) => {
            out.push_str(address);
            Ok(())
        }
        _ => Err(crate::error::ClientError::InvalidAddress(
            address.to_string(),
        )),
    }
}
