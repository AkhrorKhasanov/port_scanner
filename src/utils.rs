pub fn parse_port_range(range: &str) ->Result<(u16, u16), String> {
    let parts: Vec<&str> = range.split('-').collect();

    if parts.len() != 2 {
        return Err("Port range format is invalid. Example: '80-443'".to_string());
    }

    let start = parts[0].parse::<u16>()
        .map_err(|_| "Invalid start port")?;
    let end = parts[1].parse::<u16>()
        .map_err(|_| "Invalid end port")?;

    if start > end {
        return Err("Start port cannot be greater than end port".to_string());
    }

    Ok((start, end))
}