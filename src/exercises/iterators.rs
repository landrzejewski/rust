#[derive(Debug, Clone)]
struct Employee {
    id: u32,
    name: String,
    department: String,
    salary: f64,
    age: u32,
    years_experience: u32,
    is_remote: bool,
}

impl Employee {
    fn new(id: u32, name: &str, department: &str, salary: f64, age: u32, years_experience: u32, is_remote: bool) -> Self {
        Employee {
            id,
            name: name.to_string(),
            department: department.to_string(),
            salary,
            age,
            years_experience,
            is_remote,
        }
    }
}

fn create_fake_employees() -> Vec<Employee> {
    vec![
        Employee::new(1, "Alice Johnson", "Engineering", 95000.0, 28, 5, true),
        Employee::new(2, "Bob Smith", "Marketing", 65000.0, 32, 8, false),
        Employee::new(3, "Carol Davis", "Engineering", 110000.0, 35, 12, true),
        Employee::new(4, "David Wilson", "Sales", 75000.0, 29, 6, false),
        Employee::new(5, "Eve Brown", "Engineering", 88000.0, 26, 3, true),
        Employee::new(6, "Frank Miller", "HR", 72000.0, 41, 15, false),
        Employee::new(7, "Grace Lee", "Marketing", 68000.0, 30, 7, true),
        Employee::new(8, "Henry Garcia", "Engineering", 105000.0, 33, 9, false),
        Employee::new(9, "Ivy Martinez", "Sales", 82000.0, 27, 4, true),
        Employee::new(10, "Jack Anderson", "Engineering", 92000.0, 31, 7, false),
        Employee::new(11, "Kate Thompson", "HR", 78000.0, 36, 11, true),
        Employee::new(12, "Liam White", "Marketing", 71000.0, 28, 5, false),
        Employee::new(13, "Mia Harris", "Sales", 79000.0, 34, 10, true),
        Employee::new(14, "Noah Clark", "Engineering", 98000.0, 29, 6, false),
        Employee::new(15, "Olivia Lewis", "Marketing", 64000.0, 25, 2, true),
        Employee::new(16, "Paul Walker", "Sales", 85000.0, 38, 13, false),
        Employee::new(17, "Quinn Hall", "HR", 76000.0, 33, 9, true),
        Employee::new(18, "Ruby Allen", "Engineering", 115000.0, 37, 14, false),
        Employee::new(19, "Sam Young", "Marketing", 69000.0, 31, 8, true),
        Employee::new(20, "Tina King", "Sales", 81000.0, 30, 7, false),
    ]
}

pub fn run() {
    let employees = create_fake_employees();
}