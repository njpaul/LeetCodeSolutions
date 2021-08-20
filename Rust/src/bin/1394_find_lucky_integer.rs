fn main() {
    unimplemented!();
}

pub fn find_lucky(mut arr: Vec<i32>) -> i32 {
    let mut result = -1;
    arr.sort_unstable();

    let mut value = -1;
    let mut count = 0;
    for i in arr {
        if value != i {
            if value == count && value > result {
                result = value;
            }
            value = i;
            count = 0;
        }
        count += 1;
    }

    if value == count && value > result {
        result = value;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        struct Case {
            input: Vec<i32>,
            want: i32,
        }

        let cases = [
            Case {
                input: vec![2, 2, 3, 4],
                want: 2,
            },
            Case {
                input: vec![1, 2, 2, 3, 3, 3],
                want: 3,
            },
            Case {
                input: vec![2, 2, 2, 3, 3],
                want: -1,
            },
            Case {
                input: vec![5],
                want: -1,
            },
            Case {
                input: vec![7, 7, 7, 7, 7, 7, 7],
                want: 7,
            },
        ];

        for c in cases {
            let have = find_lucky(c.input);
            assert_eq!(c.want, have);
        }
    }
}
