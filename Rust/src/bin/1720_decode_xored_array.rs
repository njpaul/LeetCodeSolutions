fn main() {
    unimplemented!();
}

fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
    let mut result = Vec::with_capacity(encoded.len() + 1);
    let mut next = first;

    result.push(first);
    for i in encoded {
        next = next ^ i;
        result.push(next);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        struct Case {
            encoded: Vec<i32>,
            first: i32,
            want: Vec<i32>,
        }

        let cases = [
            Case {
                encoded: vec![1, 2, 3],
                first: 1,
                want: vec![1, 0, 2, 1],
            },
            Case {
                encoded: vec![6, 2, 7, 3],
                first: 4,
                want: vec![4, 2, 0, 7, 4],
            },
            Case {
                encoded: vec![1],
                first: 0,
                want: vec![0, 1],
            },
        ];

        for c in cases {
            let have = decode(c.encoded, c.first);
            assert_eq!(c.want, have);
        }
    }
}
