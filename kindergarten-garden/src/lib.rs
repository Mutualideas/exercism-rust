fn plant_name(c: char) -> &'static str {
    match c {
        'V' => "violets",
        'R' => "radishes",
        'C' => "clover",
        'G' => "grass",
        _ => panic!("Unknown plant code"),
    }
}

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let student_names = vec![
        "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
        "Kincaid", "Larry",
    ];

    let student_index = student_names
        .iter()
        .position(|&s| s == student)
        .expect("Student not found");

    let rows: Vec<&str> = diagram.lines().collect();
    let row1 = rows[0].chars().collect::<Vec<char>>();
    let row2 = rows[1].chars().collect::<Vec<char>>();

    vec![
        plant_name(row1[student_index * 2]),
        plant_name(row1[student_index * 2 + 1]),
        plant_name(row2[student_index * 2]),
        plant_name(row2[student_index * 2 + 1]),
    ]
}
