pub fn distance(a: &[u8], b:&[u8]) -> usize {
    let mut a_iter = a.into_iter();
    let mut b_iter = b.into_iter();

    let mut a_elem = a_iter.next();
    let mut b_elem = b_iter.next();

    let mut distance : usize = 0;

    while a_elem.is_some() && b_elem.is_some() {
        distance += bitcount(*a_elem.unwrap() ^ *b_elem.unwrap());

        a_elem = a_iter.next();
        b_elem = b_iter.next();
    }

    while a_elem.is_some() {
        distance += bitcount(*a_elem.unwrap());
        a_elem = a_iter.next();
    }

    while b_elem.is_some() {
        distance += bitcount(*b_elem.unwrap());
        b_elem = b_iter.next();
    }

    distance
}

fn bitcount(v: u8) -> usize {
    let mut c : usize = 0;

    let mut tmp = v;

    while tmp != 0 {
        tmp &= tmp - 1; // clear the least significant bit set
        c += 1;
    }

    c
}