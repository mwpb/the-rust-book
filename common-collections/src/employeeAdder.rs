use std::collections::HashMap;
use std::io;
// private let employee_to_department = HashMap::new();

fn list_employees_in_department(
  department_name: String,
  employee_to_department: &HashMap<String, String>,
) -> Vec<String> {
  let mut employees: Vec<String> = Vec::new();
  for (employeeName, departmentName) in employee_to_department {
    let employee_name = String::from(employeeName);
    if &department_name == departmentName {
      employees.push(employee_name);
    }
  }
  return employees;
}

pub fn run() {
  let mut employee_to_department: HashMap<String, String> = HashMap::new();

  loop {
    println!("Enter 'add' to add an employee to a department.");
    println!("Enter 'list' to list the employees by department.");
    println!("Enter 'q' to quit.");

    let mut command = String::new();
    io::stdin()
      .read_line(&mut command)
      .expect("Failed to read line.");
    let command: String = match command.trim().parse() {
      Ok(s) => s,
      Err(_) => continue,
    };
    if command == "q" {
      break;
    } else if command == "add" {
      println!("Enter employee name:\n");
      let mut employee = String::new();
      io::stdin()
        .read_line(&mut employee)
        .expect("Failed to read line.");
      let employee: String = match employee.trim().parse() {
        Ok(s) => s,
        Err(_) => continue,
      };

      if employee.len() == 0 {
        println!("Employee must have length greater than 0.");
        continue;
      }

      println!("Enter department name:\n");
      let mut department = String::new();
      io::stdin()
        .read_line(&mut department)
        .expect("Failed to read line.");
      let department: String = match department.trim().parse() {
        Ok(s) => s,
        Err(_) => continue,
      };

      if department.len() == 0 {
        println!("Department must have length greater than 0.");
        continue;
      }

      employee_to_department.insert(employee, department);
    } else if command == "list" {
      println!("Enter department name:\n");
      let mut department = String::new();
      io::stdin()
        .read_line(&mut department)
        .expect("Failed to read line.");
      let department: String = match department.trim().parse() {
        Ok(s) => s,
        Err(_) => continue,
      };

      println!(
        "{}",
        list_employees_in_department(department, &employee_to_department).join(" ")
      );
    }
  }
}
