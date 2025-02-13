use array2d::Array2D;
use std::{
    fmt::Debug,
    io::{self, Read},
};
use vector2d::Vector2D;

fn main() {
    let input = {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).unwrap();
        buffer
    };

    let chars: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => None,
                    _ => Some(c),
                })
                .collect()
        })
        .collect();

    let arr = Array2D::from_rows(chars.as_slice()).unwrap();

    println!(
        "input arr:\n{}",
        arr_to_str(&arr, |cell| match cell {
            Some(c) => *c,
            _ => '.',
        })
    );

    let antinodes = find_antinodes(&arr);
    println!(
        "antinodes:\n{}",
        arr_to_str(&antinodes, |b| match b {
            true => '#',
            false => '.',
        })
    );

    let result = antinodes.elements_row_major_iter().filter(|&c| *c).count();
    println!("part 1: {}", result);

    let antinodes = find_antinodes_harmonics(&arr);
    println!(
        "antinodes with harmonics:\n{}",
        arr_to_str(&antinodes, |b| match b {
            true => '#',
            false => '.',
        })
    );

    let result = antinodes.elements_row_major_iter().filter(|&c| *c).count();
    println!("part 2: {}", result);
}

fn arr_to_str<T, F>(arr: &Array2D<T>, f: F) -> String
where
    F: Fn(&T) -> char,
{
    let mut s = String::new();

    for row in arr.rows_iter() {
        for cell in row {
            let c = f(cell);
            s.push(c);
        }
        s.push('\n');
    }

    return s;
}

fn find_antinodes<Frequency>(arr: &Array2D<Option<Frequency>>) -> Array2D<bool>
where
    Frequency: Eq + Debug,
{
    let mut antinodes = Array2D::filled_with(false, arr.num_rows(), arr.num_columns());

    let frequencies: Vec<_> = arr
        .indices_row_major()
        .filter_map(|coordinate| {
            arr.get(coordinate.0, coordinate.1)
                .unwrap()
                .as_ref()
                .and_then(|frequency| Some((coordinate, frequency)))
        })
        .collect();

    // println!("frequencies:\n{:?}", &frequencies);
    if frequencies.len() <= 1 {
        return antinodes;
    }

    for i in 0..frequencies.len() - 1 {
        let f1 = frequencies[i];

        for j in (i + 1)..frequencies.len() {
            let f2 = frequencies[j];

            //check matching frequency
            if *f1.1 != *f2.1 {
                continue;
            }

            //calculate antinode locations
            let a = f1.0;
            let b = f2.0;

            let antinode_coordinates = [
                ((2 * a.0).checked_sub(b.0), (2 * a.1).checked_sub(b.1)),
                ((2 * b.0).checked_sub(a.0), (2 * b.1).checked_sub(a.1)),
            ];

            for coordinate in antinode_coordinates {
                let (Some(row), Some(column)) = coordinate else {
                    continue;
                };
                antinodes.set(row, column, true).unwrap_or(())
            }
        }
    }

    return antinodes;
}

fn find_antinodes_harmonics<Frequency>(arr: &Array2D<Option<Frequency>>) -> Array2D<bool>
where
    Frequency: Eq + Debug,
{
    let mut antinodes = Array2D::filled_with(false, arr.num_rows(), arr.num_columns());

    let frequencies: Vec<_> = arr
        .indices_row_major()
        .filter_map(|coordinate| {
            arr.get(coordinate.0, coordinate.1)
                .unwrap()
                .as_ref()
                .and_then(|frequency| Some((Vector2D::new(coordinate.0, coordinate.1), frequency)))
        })
        .collect();

    // println!("frequencies:\n{:?}", &frequencies);
    if frequencies.len() <= 1 {
        return antinodes;
    }

    for i in 0..frequencies.len() - 1 {
        let f1 = frequencies[i];

        for j in (i + 1)..frequencies.len() {
            let f2 = frequencies[j];

            //check matching frequency
            if *f1.1 != *f2.1 {
                continue;
            }

            //calculate antinode locations
            let a = f1.0;
            let b = f2.0;

            let mut harmonic = 0;
            loop {
                let possible_antinodes = [
                    (a * (harmonic + 1)).checked_sub(&(b * harmonic)),
                    (b * (harmonic + 1)).checked_sub(&(a * harmonic)),
                ];

                let mut any_added = false;

                for possible_antinode in possible_antinodes {
                    let Some(v) = possible_antinode else {
                        continue;
                    };

                    let add_result = antinodes.set(v.x, v.y, true);

                    if add_result.is_ok() {
                        any_added = true;
                    }
                }

                if !any_added {
                    break;
                }

                harmonic += 1;
            }
        }
    }

    return antinodes;
}

trait CheckedSub
where
    Self: Sized,
{
    fn checked_sub(&self, rhs: &Self) -> Option<Self>;
}

impl CheckedSub for usize {
    fn checked_sub(&self, rhs: &Self) -> Option<Self> {
        usize::checked_sub(*self, *rhs)
    }
}

impl<T> CheckedSub for Vector2D<T>
where
    T: CheckedSub + Copy,
{
    fn checked_sub(&self, rhs: &Self) -> Option<Self> {
        match (self.x.checked_sub(&rhs.x), self.y.checked_sub(&rhs.y)) {
            (Some(x), Some(y)) => Some(Vector2D::new(x, y)),
            _ => None,
        }
    }
}
