use thiserror::Error;

#[derive(Error, Debug)]
#[error("Invalid index {index} for multi dimensional array of size {product_size}")]
pub struct InvalidIndex {
    index: usize,
    product_size: usize,
}

/// From the 1d index into the carteisian product, get the two indicies to the orignal arrays
/// # Arguments
/// * `index` - The index into the cartesian product
/// * `len_a` - The length of the first vec
/// * `len_b` - The length of the second vec
///
/// a is the first zipped array, so iteration is slowest over a.
pub fn product_index_2d(
    index: usize,
    len_a: usize,
    len_b: usize,
) -> Result<(usize, usize), InvalidIndex> {
    if index >= len_a * len_b {
        return Err(InvalidIndex {
            index,
            product_size: len_a * len_b,
        });
    }

    let i = index / len_b;
    let j = index % len_b;
    Ok((i, j))
}

/// From the 1d index into the carteisian product, get the three indicies to the orignal arrays
/// # Arguments
/// * `index` - The index into the cartesian product
/// * `len_a` - The length of the first vec
/// * `len_b` - The length of the second vec
/// * `len_c` - The length of the third vec
///
/// a is the first zipped array, so iteration is slowest over a.
pub fn product_index_3d(
    index: usize,
    len_a: usize,
    len_b: usize,
    len_c: usize,
) -> Result<(usize, usize, usize), InvalidIndex> {
    if index >= len_a * len_b * len_c {
        return Err(InvalidIndex {
            index,
            product_size: len_a * len_b * len_c,
        });
    }
    let i = index / (len_b * len_c);
    let j = index / len_c % len_b;
    let k = index % len_c;
    Ok((i, j, k))
}

/// From the 1d index into the carteisian product, get the N indicies to the orignal arrays
/// # Arguments
/// * `index` - The index into the cartesian product
/// * `lengths` - The lengths of N arrays combined to get the cartesian product
///
/// a is the first zipped array, so iteration is slowest over a.
#[allow(non_snake_case)]
pub fn product_index_Nd(index: usize, lengths: &[usize]) -> Result<Vec<usize>, InvalidIndex> {
    if lengths.is_empty() {
        return Err(InvalidIndex {
            index,
            product_size: 0,
        });
    }

    let product_size = lengths.iter().product();

    if index >= product_size {
        return Err(InvalidIndex {
            index,
            product_size,
        });
    }

    let n_dims = lengths.len();
    let mut out_indicies: Vec<usize> = Vec::with_capacity(n_dims);

    for (dim, len) in lengths.iter().enumerate() {
        if dim == 0 {
            out_indicies.push(index / lengths.iter().skip(1).product::<usize>());
        } else if dim == n_dims - 1 {
            out_indicies.push(index % lengths[n_dims - 1]);
        } else {
            // Number of dimensions in the lower dimensional 'slice'
            let div: usize = lengths[dim + 1..].iter().product();
            // Wrap to fit in axis.
            let mod_op: usize = *len; //lengths[1..=dim].iter().product();
                                      // dbg!(dim);
                                      // dbg!(div);
                                      // dbg!(mod_op);
            out_indicies.push(index / div % mod_op);
        }
    }

    Ok(out_indicies)
}

#[cfg(test)]
mod tests {
    use super::*;

    use itertools::{iproduct, Itertools};

    #[test]
    fn test_2d() {
        let x = [1, 2, 3, 4, 5, 6, 53452, 345235, 33333, 112312];

        let y = [7, 8, 9, 123, 100, 1232];

        let product = iproduct!(x.iter(), y.iter()).collect_vec();

        assert_eq!(product.len(), x.len() * y.len());
        for (i, expected) in product.iter().enumerate() {
            let (i1, i2) = product_index_2d(i, x.len(), y.len()).unwrap();
            assert_eq!(x[i1], *expected.0);
            assert_eq!(y[i2], *expected.1);
        }
    }

    #[test]
    fn test_3d() {
        let x = [1, 2, 3, 4, 5, 6, 53452, 345235, 33333, 112312];

        let y = [7, 8, 9, 123, 100, 1232];

        let z = [7, 870980, 8098, 9, 123, 100, 1232, 98098908];

        let product = iproduct!(x.iter(), y.iter(), z.iter()).collect_vec();

        assert_eq!(product.len(), x.len() * y.len() * z.len());

        for (i, expected) in product.iter().enumerate() {
            let (i1, i2, i3) = product_index_3d(i, x.len(), y.len(), z.len()).unwrap();
            assert_eq!(x[i1], *expected.0);
            assert_eq!(y[i2], *expected.1);
            assert_eq!(z[i3], *expected.2);
        }
    }

    #[test]
    fn test_4d() {
        let x = [1, 2, 3, 4, 5, 6, 53452, 345235, 33333, 112312];

        let y = [7, 8, 9, 123, 100, 1232];

        let z = [7, 870980, 8098, 9, 123, 100, 1232, 98098908];

        let w = [798098, 34234, 8098, 982120912, 123, 100, 1232, 98098908];

        let product = iproduct!(x.iter(), y.iter(), z.iter(), w.iter()).collect_vec();

        dbg!(&product);

        assert_eq!(product.len(), x.len() * y.len() * z.len() * w.len());

        for (i, expected) in product.iter().enumerate() {
            let indicies =
                product_index_Nd(i, vec![x.len(), y.len(), z.len(), w.len()].as_ref()).unwrap();

            if let &[i1, i2, i3, i4] = &indicies[..] {
                assert_eq!(x[i1], *expected.0);
                assert_eq!(y[i2], *expected.1);
                assert_eq!(z[i3], *expected.2);
                assert_eq!(w[i4], *expected.3);
            } else {
                panic!("{indicies:?} Had a bad shape");
            };
        }
    }

    #[test]
    fn test_5d() {
        let x = [1, 2, 3, 4, 5, 6, 53452, 345235, 33333, 112312];

        let y = [7, 8, 9, 123, 100, 1232];

        let z = [7, 870980, 8098, 9, 123, 100, 1232, 98098908];

        let w = [798098, 34234, 8098, 982120912, 123, 100, 1232, 98098908];
        let u = [
            908908, 34234, 8098, 982120912, 123, 100, 1232, 123123, 123123, 423423, 69,
        ];

        let product = iproduct!(x.iter(), y.iter(), z.iter(), w.iter(), u.iter()).collect_vec();

        //dbg!(&product);

        assert_eq!(
            product.len(),
            x.len() * y.len() * z.len() * w.len() * u.len()
        );

        for (i, expected) in product.iter().enumerate() {
            dbg!(i);
            let indicies = product_index_Nd(
                i,
                vec![x.len(), y.len(), z.len(), w.len(), u.len()].as_ref(),
            )
            .unwrap();

            dbg!(&indicies);
            if let &[i1, i2, i3, i4, i5] = &indicies[..] {
                assert_eq!(x[i1], *expected.0);
                assert_eq!(y[i2], *expected.1);
                assert_eq!(z[i3], *expected.2);
                assert_eq!(w[i4], *expected.3);
                assert_eq!(u[i5], *expected.4);
            } else {
                panic!("{indicies:?} Had a bad shape");
            };
        }
    }

    #[test]
    fn nd_still_same() {
        let index = 40;
        let len_a = 12;
        let len_b = 420;
        let len_c = 69;

        let (ix, iy, iz) = product_index_3d(index, len_a, len_b, len_c).unwrap();
        let nd_indx = product_index_Nd(index, &[len_a, len_b, len_c]).unwrap();
        let (i2x, i2y, i2z) = (nd_indx[0], nd_indx[1], nd_indx[2]);

        assert_eq!(ix, i2x);
        assert_eq!(iy, i2y);
        assert_eq!(iz, i2z);

        let (ix, iy) = product_index_2d(index, len_a, len_b).unwrap();
        let nd_indx = product_index_Nd(index, &[len_a, len_b]).unwrap();
        let (i2x, i2y) = (nd_indx[0], nd_indx[1]);

        assert_eq!(ix, i2x);
        assert_eq!(iy, i2y);
        assert_eq!(iz, i2z);
    }
}
