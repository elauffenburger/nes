pub fn get_range<T>(slice: &[T], start: usize, take: usize) -> Result<&[T], String> {
    let end = start + take;

    if end > slice.len() {
        return Err(String::from("end outside bounds of slice"));
    }

    if start < 0 {
        return Err(String::from("start less than 0"));
    }

    Ok(&slice[start..end])
}
