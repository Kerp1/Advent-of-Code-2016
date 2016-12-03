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

fn valid_triangle(triangle: (i32, i32, i32)) -> bool {
    return triangle.0 + triangle.1 > triangle.2 &&
        triangle.1 + triangle.2 > triangle.0 &&
        triangle.0 + triangle.2 > triangle.1;
}

pub fn task1() -> i32 {
    let data = common::read_file(String::from("input/day3.txt")).unwrap();

    let triangles = get_triangles(&data);
    let mut valid_number_of_triangles = 0;
    for triangle in triangles {
        if valid_triangle(triangle) {
            valid_number_of_triangles += 1;
        }
    }
    return valid_number_of_triangles;
}
