use std::fs;

fn main() {
    let input: String = fs::read_to_string("input.txt").expect("Error reading file!");
    let coords: Vec<(u32, u32)> = parse_coordinates(input);
    let (best_location, n_of_visible): ((u32, u32), usize) = find_best_location(&coords);

    println!("best: {:?}, n: {}", best_location, n_of_visible);

    // Now get a new list of dir_groups, relative to best_location
    // TODO: Move this to a separate function: calulate_nth_lasered(dir_groups, n)

    let directions: Vec<((u32, u32), (i32, i32))> = list_directions(&coords, &best_location);
    let dir_groups: Vec<Vec<((u32, u32), (i32, i32))>> = group_directions(directions);

    for group in dir_groups {
        println!("{:?}", group);
        println!("");
    }
}

// Find asteroid location where most other asteroids are visible: return 'best' location & n of visible asteroids.
fn find_best_location(coords: &Vec<(u32, u32)>) -> ((u32, u32), usize) {
    let count = coords.iter().count();
    let mut best_loc: (u32, u32) = (0, 0);
    let mut visible: usize = 0;

    for &loc in coords {
        let directions: Vec<((u32, u32), (i32, i32))> = list_directions(&coords, &loc);
        let dir_groups: Vec<Vec<((u32, u32), (i32, i32))>> = group_directions(directions);
        let n: usize = count_visible(count, dir_groups);

        if n > visible {
            visible = n;
            best_loc = loc;
        }
    }

    ((best_loc), visible)
}

// Reduce a two-value coordinate vector (dx, dy) to the smallest possible integer vector representing the same direction.
// Example: direction((9, -3)) => (3, -1)
fn direction((a, b): (i32, i32)) -> (i32, i32) {
    let gcd: i32 = find_gcd(a, b);
    return (a / gcd, b / gcd);
}

// Find greatest common denominator of any 2 integers (+'ve or -'ve, including zero).
fn find_gcd(a: i32, b: i32) -> i32 {
    let mut a = a.abs();
    let mut b = b.abs();

    if a == 0 {
        return b;
    } else if b == 0 {
        return a;
    }

    while a != b {
        if a > b {
            a -= b;
        } else {
            b -= a;
        }
    }

    a
}

// Build list of directions of all asteroids (in a tuple, along with the coordinates of each asteroid) relative to asteroid @ 'loc'.
fn list_directions(coords: &Vec<(u32, u32)>, base_loc: &(u32, u32)) -> Vec<((u32, u32), (i32, i32))> {
    let mut directions: Vec<((u32, u32), (i32, i32))> = vec![];

    for &loc in coords {
        if &loc != base_loc {
            let (x, y) = &base_loc;
            let (a, b) = &loc;
            let dir = direction((*x as i32 - *a as i32, *y as i32 - *b as i32));

            directions.push((loc, dir));
        }
    }

    directions
}

// Build a list of groups (lists) of identical directions (in tuples, along with the coordinates of each asteroid).
fn group_directions(directions: Vec<((u32, u32), (i32, i32))>) -> Vec<Vec<((u32, u32), (i32, i32))>> {
    let mut dir_groups: Vec<Vec<((u32, u32), (i32, i32))>> = vec![];

    for elem in directions {
        let (loc, dir) = elem;
        let mut group_exists: bool = false;

        for i in 0..dir_groups.iter().count() {
            let (_, d) =  dir_groups[i][0];
            if d == dir {
                group_exists = true;
                dir_groups[i].push((loc, dir));
            }
        }

        if group_exists == false {
            dir_groups.push(vec![elem]);
        }
    }

    dir_groups
}

// Calculate n of visible asteroids.
fn count_visible(count: usize, dir_groups: Vec<Vec<((u32, u32), (i32, i32))>>) -> usize {
    let mut n: usize = count - 1;
    for group in dir_groups {
        let elem_count: usize = group.iter().count() - 1;

        n -= elem_count;
    }

    n
}

fn parse_coordinates(input: String) -> Vec<(u32, u32)> {
    let lines: Vec<&str> = input.split('\n').collect();
    let mut coords: Vec<(u32, u32)> = vec![];

    for j in 0..lines.iter().count() {
        let chars: Vec<char> = lines[j].to_string().chars().collect::<Vec<_>>();

        for i in 0..chars.iter().count() {
            if chars[i] == '#' {
                coords.push((i as u32, j as u32));
            }
        }
    }

    coords
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_gcd_test() {
        let gcd = find_gcd(9, 3);
        assert_eq!(gcd, 3);

        let gcd = find_gcd(3, 21);
        assert_eq!(gcd, 3);

        let gcd = find_gcd(-64, 4);
        assert_eq!(gcd, 4);

        let gcd = find_gcd(19, 7);
        assert_eq!(gcd, 1);

        let gcd = find_gcd(7, 19);
        assert_eq!(gcd, 1);

        let gcd = find_gcd(0, -19);
        assert_eq!(gcd, 19);

        let gcd = find_gcd(19, 0);
        assert_eq!(gcd, 19);

        let gcd = find_gcd(-9, -6);
        assert_eq!(gcd, 3);
    }

    #[test]
    fn direction_test() {
        let dir = direction((-64, 4));
        assert_eq!(dir, (-16, 1));

        let dir = direction((32, -2));
        assert_eq!(dir, (16, -1));

        let dir = direction((-48, -3));
        assert_eq!(dir, (-16, -1));

        let dir = direction((-9, -6));
        assert_eq!(dir, (-3, -2));
    }

    // .#..#    (1, 0), (4, 0),
    // .....
    // #####    (0, 2), (1, 2), (2, 2), (3, 2), (4, 2),
    // ....#    (4, 3),
    // ...##    (3, 4), (4, 4)
    #[test]
    fn parse_coordinates_test() {
        let input = String::from(".#..#\n.....\n#####\n....#\n...##");
        let coords = parse_coordinates(input);
        assert_eq!(
            coords,
            vec![
                (1, 0),
                (4, 0),
                (0, 2),
                (1, 2),
                (2, 2),
                (3, 2),
                (4, 2),
                (4, 3),
                (3, 4),
                (4, 4)
            ]
        );
    }

    #[test]
    fn find_best_location_test() {
        let input = String::from(".#..#\n.....\n#####\n....#\n...##");
        let coords = parse_coordinates(input);
        let result = find_best_location(&coords);
        assert_eq!(result, ((3, 4), 8));
    }
}
