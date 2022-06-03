struct Solution;

impl Solution {
    fn is_cyclic(cur: usize, graph: &Vec<Vec<i32>>, checked: &mut Vec<bool>, path: &mut Vec<bool>) -> bool {
        if checked[cur] {
            return false;   // already checked
        }
        if path[cur] {
            return true;    // currently processing path
        }

        path[cur] = true;

        for next_course in &graph[cur] {
            if Solution::is_cyclic(*next_course as usize, graph, checked, path) {
                return true;
            }
        }

        path[cur] = false;
        checked[cur] = true;

        false
    }

    fn build(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut graph = vec![vec![];num_courses as usize];

        for courses in prerequisites {
            graph[courses[1] as usize].push(courses[0]); // prev -> next
        }

        graph
    }

    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut checked = vec![false; num_courses as usize];
        let mut path = vec![false; num_courses as usize];

        let graph = Solution::build(num_courses, prerequisites);

        for i in 0..num_courses {
            if Solution::is_cyclic(i as usize, &graph, &mut checked, &mut path) {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
        use super::*;

    #[test]
    fn it_works() {
        let v1 = vec![vec![1, 0]];
        assert!(Solution::can_finish(2, v1));

        let v2 = vec![vec![1,0], vec![0, 1]];
        assert!(!Solution::can_finish(2, v2));
    }
}
