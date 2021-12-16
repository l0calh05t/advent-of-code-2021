fn parse(bits: &[bool]) -> Option<(u64, u64, usize)> {
	let mut offset = 0;
	let to_num = |bits: &[bool]| bits.iter().fold(0u64, |a, b| (a << 1) + *b as u64);

	let mut version_sum = to_num(bits.get(offset..offset + 3)?);
	offset += 3;

	let packet_type = to_num(bits.get(offset..offset + 3)?);
	offset += 3;

	let mut value = 0;

	if packet_type == 4 {
		// literal packet
		while {
			let is_continuation = *bits.get(offset)?;
			offset += 1;
			value = (value << 4) + to_num(bits.get(offset..offset + 4)?);
			offset += 4;
			is_continuation
		} {}
	} else {
		// operator packet
		let length_type = *bits.get(offset)?;
		offset += 1;
		let mut subpacket_values = Vec::new();
		if length_type {
			// count delimited
			let count = to_num(bits.get(offset..offset + 11)?);
			offset += 11;
			subpacket_values.reserve(count as _);
			for _ in 0..count {
				let (subpacket_version, subpacket_value, subpacket_offset) =
					parse(&bits[offset..])?;
				version_sum += subpacket_version;
				offset += subpacket_offset;
				subpacket_values.push(subpacket_value);
			}
		} else {
			// length delimited
			let length = to_num(bits.get(offset..offset + 15)?);
			offset += 15;
			let old_offset = offset;
			while offset < old_offset + length as usize {
				let (subpacket_version, subpacket_value, subpacket_offset) =
					parse(&bits[offset..])?;
				version_sum += subpacket_version;
				offset += subpacket_offset;
				subpacket_values.push(subpacket_value);
			}
			if offset != old_offset + length as usize {
				return None;
			}
		}

		value = match packet_type {
			0 => subpacket_values.iter().sum(),
			1 => subpacket_values.iter().product(),
			2 => *subpacket_values.iter().min()?,
			3 => *subpacket_values.iter().max()?,
			5 => (subpacket_values[0] > subpacket_values[1]) as _,
			6 => (subpacket_values[0] < subpacket_values[1]) as _,
			7 => (subpacket_values[0] == subpacket_values[1]) as _,
			_ => return None,
		};
	}

	Some((version_sum, value, offset))
}

fn main() {
	let input = include_bytes!("../input")
		.strip_suffix(b"\n")
		.unwrap()
		.iter()
		.map(|b| {
			if (b'0'..=b'9').contains(b) {
				*b - b'0'
			} else if (b'A'..=b'F').contains(b) {
				*b - b'A' + 10
			} else {
				unreachable!()
			}
		})
		.flat_map(|b| {
			[
				(b >> 3) & 1 == 1,
				(b >> 2) & 1 == 1,
				(b >> 1) & 1 == 1,
				b & 1 == 1,
			]
		})
		.collect::<Vec<_>>();
	// println!("{:?}", parser().parse(input.as_slice()));

	let (version_sum, value, _) = parse(&input).unwrap();
	println!("{}", version_sum);
	println!("{}", value);
}
