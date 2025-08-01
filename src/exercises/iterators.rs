use std::cmp::Ordering;
use std::collections::HashMap;

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

    /*println!("1. All employee names:");
    employees.iter().for_each(|employee| println!("{}", employee.name));*/

    /*println!("2. High earners (salary > $80,000):");
    employees.iter()
        .filter(|employee| employee.salary > 80_000.0)
        .for_each(|employee| println!("{:?}", employee));*/

    /*println!("3. Name and salary pairs:");
    employees.iter()
        .map(|employee| (employee.name.clone(), employee.salary))
        .for_each(|entry| println!("{:?}", entry));*/

    /*println!("4. First remote employee:");
    if let Some(employee) = employees.iter().find(|employee| employee.is_remote) {
        println!("{:?}", employee);
    }*/

    /*println!("5. Employee statistics:");
    let min_age = employees.iter()
        .min_by_key(|employee| employee.age)
        .map(|employee| employee.age)
        .unwrap_or_default();

    let max_age = employees.iter()
        .max_by_key( |employee| employee.age)
        .map(|employee| employee.age)
        .unwrap_or_default();

    let min_salary = employees.iter()
        .map(|employee| employee.salary)
        //.fold(f64::INFINITY, |min_salary, salary| if salary < min_salary { salary } else { min_salary });
        .fold(f64::INFINITY, f64::min);

    let max_salary = employees.iter()
        .map(|employee| employee.salary)
        .fold(0.0, f64::max);

    println!("Age: {:?} - {:?}:", min_age, max_age);
    println!("Salary: {:?} - {:?}:", min_salary, max_salary);*/

    /*println!("6. Average salary (using fold):");
    let (total_salary, count) = employees.iter()
        .fold((0.0, 0), |(sum, count), employee| (sum + employee.salary, count + 1));
    println!("Avg salary: {:.2}", total_salary / count as f64);*/

    /*println!("7. Employees by department:");
    let employees_by_department = employees.iter()
        .fold(HashMap::new(), |mut acc, employee| {
            acc.entry(employee.department.clone()).or_insert(Vec::new()).push(employee);
            acc
        });
    for entry in employees_by_department {
        println!("{}", entry.0);
        println!("{:?}", entry.1);
    }*/

    /*println!("8. Senior employees (10+ years) OR high earners (90k+):");
    let senior_employees = employees.iter()
        .filter(|employee| employee.years_experience >= 10);
    let high_earners = employees.iter()
        .filter(|employee| employee.salary >= 90_000.0);

    senior_employees.chain(high_earners).for_each(|employee| println!("{:?}", employee));*/

    /*println!("9. First 5 employees with index:");
    employees.iter()
        .enumerate()
        .take(5)
        .for_each(|employee| println!("{:?}", employee));*/

    /*println!("10. Employee rankings by salary:");
    let mut employees_by_salary = employees.clone();
    employees_by_salary.sort_by(|employee, other_employee| {
            other_employee.salary.partial_cmp(&employee.salary).unwrap_or(Ordering::Equal)
        });

    (1..=5).zip(employees_by_salary)
        .for_each(|entry| println!("{:?}", entry));*/

    /*println!("11. Remote vs Office workers:");
    let (remote, office): (Vec<&Employee>, Vec<&Employee>) = employees.iter()
        .partition(|employee| employee.is_remote);*/

    /*println!("12. Department salary statistics:");

    let process = |mut acc: HashMap<String, (f64, f64, usize)>, employee: &Employee| {
        let entry = acc.entry(employee.department.clone()).or_insert((f64::INFINITY, 0.0, 0));
        entry.0 = entry.0.min(employee.salary);
        entry.1 = entry.1.max(employee.salary);
        entry.2 += 1;
        acc
    };

    let data = employees.iter()
        .fold(HashMap::new(), process);

    for (department, stats) in data {
        println!("Department: {}, count: {}, salary: {} - {}", department, stats.2, stats.0, stats.1);
    }*/

   /* println!("13. Boolean checks:");
    let all_adults = employees.iter().all(|employee| employee.age >= 18);
    let any_remote = employees.iter().any(|employee| employee.is_remote);
    let any_millionaire = employees.iter().any(|employee| employee.salary >= 1_000_000.0);

    println!("   All employees are adults (18+): {}", all_adults);
    println!("   Any remote workers: {}", any_remote);
    println!("   Any millionaires: {}", any_millionaire);*/

    /*println!("14. Pagination example (page 2, 5 per page):");
    employees.iter()
        .skip(5)  // Skip first 5 (page 1)
        .take(5)  // Take next 5 (page 2)
        .for_each(|employee| println!("   {} - {}", employee.name, employee.department));*/

    println!("15. Complex query - Engineering dept, remote, salary > 90k, sorted by experience:");

}