use common;

fn get_triangles(data: &str) -> Vec<(i32, i32, i32)> {
    let mut triangles = Vec::new();
    let rows = data.split_terminator("\n");

    for row in rows {
        let numbers: Vec<&str> = row.split_whitespace().collect();
        triangles.push((numbers[0].parse::<i32>().unwrap(),
                        numbers[1].parse::<i32>().unwrap(),
                        numbers[2].parse::<i32>().unwrap()));
    }
    return triangles;
}

fn get_triangles_columns(data: &Vec<(i32, i32, i32)>) -> Vec<(i32, i32, i32)> {
    let mut c1 = Vec::new();
    let mut c2 = Vec::new();
    let mut c3 = Vec::new();
    let mut triangles = Vec::new();

    for triangle in data {
        c1.push(triangle.0);
        c2.push(triangle.1);
        c3.push(triangle.2);
    }
    triangles.extend(construct_triangles(c1));
    triangles.extend(construct_triangles(c2));
    triangles.extend(construct_triangles(c3));

    return triangles;

}

fn construct_triangles(mut numbers: Vec<i32>) -> Vec<(i32, i32, i32)> {
    let mut triangles = Vec::new();
    while !numbers.is_empty() {
        triangles.push((numbers.pop().unwrap(), numbers.pop().unwrap(), numbers.pop().unwrap()));
    }
    return triangles;
}

fn valid_triangle(triangle: (i32, i32, i32)) -> bool {
    return triangle.0 + triangle.1 > triangle.2 && triangle.1 + triangle.2 > triangle.0 &&
           triangle.0 + triangle.2 > triangle.1;
}

fn count_valid_triangles(triangles: Vec<(i32, i32, i32)>) -> i32 {
    let mut valid_number_of_triangles = 0;
    for triangle in triangles {
        if valid_triangle(triangle) {
            valid_number_of_triangles += 1;
        }
    }
    return valid_number_of_triangles;
}

pub fn task1() -> i32 {
    let data = common::read_file(String::from("input/day3.txt")).unwrap();

    count_valid_triangles(get_triangles(&data))
}

pub fn task2() -> i32 {
    let data = common::read_file(String::from("input/day3.txt")).unwrap();

    count_valid_triangles(get_triangles_columns(&get_triangles(&data)))
}
