use std::{io::stdin, process::Command};

struct Student{
    id : usize,
    name: String,
    grade: Vec<Option<i32>>,
}

fn main() {
    let mut students: Vec<Student> = Vec::new();
    loop {
        match menu() {
            0 => (),
            1 => {
                add_student(&mut students);
                wait()
            }

            2 => {
                show_students(&students);
                wait()
            }

            3 => {
                search_student(&students);
                wait()
            }

            4 => {
                add_grade(&mut students);
                wait()
            }

            5 => {
                show_average(&students);
                wait()
            }

            6 => {
                delete_student(&mut students);
                wait()
            }

            7 => {
                println!("good bye!");
                return;
            }

            _ => {
                println!("this input false;");
            }
        }
    }

}

#[cfg(target_os = "windows")]
fn clear_screen() {
    Command::new("cmd").args(&["/C", "cls"]).status().unwrap();
}
#[cfg(target_os = "linux")]
fn clear_screen() {
    Command::new("clear").status().unwrap();
}

fn menu() -> i32 {
    clear_screen();

    println!("
        1. Add student \n
        2. Show students \n
        3. Find student \n
        4. Add grade \n
        5. Show average \n
        6. Delete student \n
        7. Exit
    ");

    let mut input= "".to_string();

    stdin().read_line(&mut input).unwrap();

    let result = input.trim().parse::<i32>().unwrap_or_else(|e| {
        eprintln!("plese correct value: {}", e);
        0
    });

    result
}

fn wait() {
    let mut input = "".to_string();
    stdin().read_line(&mut input).ok();
}

fn add_student(students: &mut Vec<Student>){
    clear_screen();

    println!("please input your name:");
    
    let mut name="".to_string();
    stdin().read_line(&mut name).unwrap();
    
    students.push(Student { id: students.len() + 1, name, grade: Vec::new()});
}

fn find_student<T: SearchCriteria>(criteria: T, students: &Vec<Student>){    
    students.iter().for_each(|student| {
        if criteria.matches(student){
            println!("id = {}, name: {}", student.id, student.name);
        }
    });
}

trait SearchCriteria {
    fn matches(&self, student:&Student) -> bool;
}

impl SearchCriteria for usize {
    fn matches(&self, student:&Student) -> bool {
        student.id == *self
    }
}

impl SearchCriteria for String {
    fn matches(&self, student:&Student) -> bool {
        student.name == *self
    }
}

fn search_student(students: &Vec<Student>){
    clear_screen();

    println!("search by id or name?");
    let mut mode = "".to_string();
    stdin().read_line(&mut mode).unwrap();

    match mode.trim() {
        "id" => {
            println!("Enter student id:");
            let mut id = "".to_string();
            stdin().read_line(&mut id).unwrap();
            let id: usize = id.trim().parse().unwrap_or_else(|e| {
                eprintln!("plese correct value: {}", e);
                0
            });
            find_student(id, students);
        },

        "name" => {
            println!("Enter student name:");
            let mut name = "".to_string();
            stdin().read_line(&mut name).unwrap();
            find_student(name, students);
        },

        _ => {
            eprintln!("Invalid input!");
        }
    }
}

fn show_students(students: &Vec<Student>){
    clear_screen();

    students.iter().for_each(|student| {
        println!("\n{}. {}",student.id,student.name)
    });
}

fn add_grade(students: &mut Vec<Student>) {
    clear_screen();

    println!("plese enter id student:");
    let mut id_str = "".to_string();
    stdin().read_line(&mut id_str).unwrap();
    let id  = id_str.trim().parse::<usize>().unwrap();

    println!("plese enter grade for students {}", id);
    let mut grade_str = "".to_string();
    stdin().read_line(&mut grade_str).unwrap();

    let grade: i32 = grade_str.trim().parse().unwrap();

    students[id-1].grade.push(Some(grade));
}

fn show_average(students: &Vec<Student>) {
    clear_screen();

    println!("plese enter id student:");
    let mut id_str = "".to_string();
    stdin().read_line(&mut id_str).unwrap();
    let id: usize = id_str.trim().parse().unwrap();

    let mut average: i32 = 0;
    students[id-1].grade.iter().for_each(|grade| {
        average += grade.unwrap();
    });
    println!("average student {} = {}",id,(average/(students[id-1].grade.len() as i32)));
}

fn delete_student(students: &mut Vec<Student>) {
    clear_screen();

    println!("plese enter id student:");
    let mut id_str = "".to_string();
    stdin().read_line(&mut id_str).unwrap();
    let id: usize = id_str.trim().parse().unwrap();

    students.remove(id-1);
    println!("this student is delete");
}