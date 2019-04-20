pub fn take_elems<T>(slice: &[T], start: usize, take: usize) -> Result<&[T], String> {
    let end = start + take;

    if end > slice.len() {
        return Err(String::from("end outside bounds of slice"));
    }

    Ok(&slice[start..end])
}
