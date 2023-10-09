pub mod lexical_order_string {
    use std::collections::HashMap;

    pub fn is_ordered(worlds: &mut [&str], alph: &str) -> bool {
        let mut map = HashMap::<u8, usize>::new();

        for (i, letter) in alph.as_bytes().iter().enumerate() {
            map.insert(*letter, i);
        }

        let mut i = 1;

        while i < worlds.len() {
            if !compare(worlds[i - 1], worlds[i], &map) {
                return false;
            }
            i += 1
        }

        true
    }

    fn compare(first_world: &str, second_world: &str, dict: &HashMap<u8, usize>) -> bool {
        let longitude = std::cmp::min(first_world.len(), second_world.len());

        let first_world_as_bytes = first_world.as_bytes();
        let second_world_as_bytes = second_world.as_bytes();

        for i in 0..longitude {
            // println!("i {}", i);
            // println!("bytes fw: {}", first_world_as_bytes[i]);
            // println!("bytes sw: {}", second_world_as_bytes[i]);
            if dict[&first_world_as_bytes[i]] < dict[&second_world_as_bytes[i]] {
                return true;
            }

            if dict[&first_world_as_bytes[i]] > dict[&second_world_as_bytes[i]] {
                return false;
            }
        }

        first_world.len() <= second_world.len()
    }
}

pub mod merge_two_sorted_list {
    pub fn m(nums1: &mut [i32], nums2: &mut [i32]) {
        let n = nums1.len() - 1 - (nums2.len() - 1) - 1;
        let m = nums2.len() - 1;

        let mut zeros_pointer = nums1.len() - 1;
        let mut num1_pointer = n as isize;
        let mut num2_pointer = m as isize;

        while num1_pointer >= 0 && num2_pointer >= 0 {
            if nums1[num1_pointer as usize] < nums2[num2_pointer as usize] {
                nums1[zeros_pointer] = nums2[num2_pointer as usize];
                num2_pointer -= 1
            } else {
                nums1[zeros_pointer] = nums1[num1_pointer as usize];
                num1_pointer -= 1;
            }
            zeros_pointer -= 1;
        }

        if n != m {
            nums1[..((num2_pointer + 1) as usize)]
                .copy_from_slice(&nums2[..((num2_pointer + 1) as usize)]);
        }
    }
}

pub mod container_with_most_water {
    pub fn calc(nums: &[i32]) -> isize {
        let mut i = 0;
        let mut i2 = nums.len() - 1;

        let mut max_area = 0;

        while i != i2 {
            let min_height = std::cmp::min(nums[i], nums[i2]);
            let long = (i2 - i) as isize;

            let product = long * (min_height as isize);

            if product > max_area {
                max_area = product;
            }

            if min_height == nums[i] {
                i += 1;
            } else {
                i2 -= 1;
            }
        }

        max_area
    }
}

pub mod longest_substr_wo_repeating_chars {
    use std::collections::HashMap;

    pub fn c(s: &str) -> usize {
        let s_as_bytes = s.as_bytes();
        let mut i = 0;
        let mut i2 = 0;

        let mut counter = 0;

        let mut map = HashMap::<u8, bool>::new();

        while i2 < s_as_bytes.len() {
            let curr = s_as_bytes[i];
            map.insert(curr, true);

            let next = s_as_bytes[i2];

            if let std::collections::hash_map::Entry::Vacant(e) = map.entry(next) {
                e.insert(true);
                i2 += 1;
            } else {
                if map.len() > counter {
                    counter = map.len();
                }
                map = HashMap::new();
                i = i2;
                i2 += 1;
            }
        }

        counter
    }
}

pub mod binary_search {
    pub fn perform(n: &[i32], search: i32) -> isize {
        let mut p1 = 0;
        let mut p2 = n.len() - 1;

        let mut mid = p1 + (p2 - p1) / 2;

        while p2 >= p1 {
            // println!("mid {mid} p1 {p1} p2 {p2} n {}", n[mid]);
            if n[mid] == search {
                return mid as isize;
            }

            if n[mid] < search {
                p1 = mid + 1;
                mid = p1 + (p2 - p1) / 2
            } else {
                p2 = mid - 1;
                mid = p1 + (p2 - p1) / 2
            }
        }

        -1
    }
}

pub mod search_in_rotated_arrays {
    pub fn s(n: &[i32], target: i32) -> isize {
        let mut p1 = 0;
        let mut p2 = n.len() - 1;

        // [4, 5, 6, 7, 0, 1, 2]
        let mut mid = p1 + (p2 - p1) / 2;

        while p2 >= p1 {
            // println!("mid {mid} p1 {p1} p2 {p2}");
            if n[mid] == target {
                return mid as isize;
            }

            if n[mid] >= n[p1] {
                if target >= n[p1] && target < n[mid] {
                    p2 = mid - 1;
                } else {
                    p1 = mid + 1;
                }
            } else {
                #[allow(clippy::collapsible_else_if)]
                if target >= n[p2] && target < n[mid] {
                    p1 = mid + 1;
                } else {
                    p2 = mid - 1;
                }
            }
            mid = p1 + (p2 - p1) / 2;
        }

        -1
    }
}

pub mod search_in_2d_array {
    pub fn perform(arr: &[&[i32]], target: i32) -> bool {
        let mut p1 = 0;
        let mut p2 = arr.len() - 1;

        while p1 < p2 {
            let mid = p1 + (p2 - p1) / 2 + 1;
            // println!("vertical: mid {mid} p1 {p1} p2 {p2}");
            #[allow(clippy::comparison_chain)]
            if arr[mid][0] == target {
                return true;
            } else if arr[mid][0] < target {
                p1 = mid
            } else {
                p2 = mid - 1
            }
        }
        // println!("vertical: p1 {p1} p2 {p2}");

        let row = arr[p1];
        let mut p1 = 0;
        let mut p2 = row.len() - 1;

        while p2 >= p1 {
            let mid = p1 + (p2 - p1) / 2;
            // println!("row: mid {mid} p1 {p1} p2 {p2}");

            #[allow(clippy::comparison_chain)]
            if row[mid] == target {
                return true;
            } else if row[mid] < target {
                p1 = mid + 1;
            } else {
                p2 = mid - 1;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexical_order_string_works() {
        let a = lexical_order_string::is_ordered(
            &mut ["como", "conoce"],
            "abcdefghijklmnñopqrstuvwxyz",
        );

        assert!(a)
    }

    #[test]
    fn merge_two_sorted_lists() {
        let n = &mut [1, 2, 3, 7, 0, 0, 0, 0, 0];
        merge_two_sorted_list::m(n, &mut [-4, 2, 5, 6, 8]);
        println!("{:?}", n)
    }

    #[test]
    fn container_with_most_water() {
        assert_eq!(
            container_with_most_water::calc(&[1, 8, 6, 2, 5, 4, 8, 3, 7]),
            49
        )
    }

    #[test]
    fn longest_substr() {
        assert_eq!(
            longest_substr_wo_repeating_chars::c("jdkafnlcdsalkxcmpoiuytfccv"),
            15
        );

        assert_eq!(longest_substr_wo_repeating_chars::c("abcabcbb"), 3)
    }

    #[test]
    fn binary_search() {
        assert_eq!(
            binary_search::perform(
                &[0, 3, 4, 5, 28, 30, 59, 90, 102, 344, 555, 878, 9392, 292929, 29292929,],
                0,
            ),
            0
        );
    }

    #[test]
    fn search_in_rotated_array() {
        assert_eq!(search_in_rotated_arrays::s(&[4, 5, 6, 7, 0, 1, 2], 0), 4);
        // println!(
        //     "{}",
        //     binary_search::perform(&[4, 5, 6, 7, 8, 9, 10, 11, 23, 0, 1, 2], 0)
        // )
    }

    #[test]
    fn search_in_2d_array() {
        assert!(search_in_2d_array::perform(
            &[
                &[1, 2, 5, 6],
                &[10, 20, 23, 25],
                &[49, 69, 90, 100, 123],
                &[139, 158, 180]
            ],
            100
        ));

        assert!(!search_in_2d_array::perform(
            &[
                &[1, 2, 5, 6],
                &[10, 20, 23, 25],
                &[49, 69, 90, 100, 123],
                &[139, 158, 180]
            ],
            101
        ))
    }
}
