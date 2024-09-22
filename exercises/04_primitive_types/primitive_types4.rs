fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        //                 0  1  2  3  4
        let a: [i32; 5] = [1, 2, 3, 4, 5];

        // TODO: Get a slice called `nice_slice` out of the array `a` so that the test passes.
        let nice_slice: &[i32] = &a[1..4];

        assert_eq!([2, 3, 4], nice_slice);
    }
}
