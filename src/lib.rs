#![allow(warnings)]
mod tree;
mod tree_common;

pub use flatten_tree_macros::FlattenTreeNode;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_from_tree_like_vec() {
        #[title(field_name = "name")]
        #[searchable(field_names = "address, ceo")]
        #[derive(FlattenTreeNode, Debug)]
        struct Company {
            name: String,
            address: String,
            ceo: String,
            departments: Option<Vec<Department>>,
        }

        impl std::fmt::Display for Company {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                let departments_string: String = match &self.departments {
                    Some(temp_departments) => {
                        temp_departments.iter().map(|g| g.to_string()).collect()
                    }
                    _ => "".to_string(),
                };
                write!(f, "{}_{}_{}", self.name, self.address, departments_string)
            }
        }

        #[derive(Debug)]
        struct Department {
            // #[title]
            name: String,
            // #[searchable]
            manager: String,
            groups: Option<Vec<DepartmentGroup>>,
        }

        impl std::fmt::Display for Department {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                let groups_string: String = match &self.groups {
                    Some(temp_groups) => temp_groups.iter().map(|g| g.to_string()).collect(),
                    _ => "".to_string(),
                };
                write!(f, "{}_{}_{}", self.name, self.manager, groups_string)
            }
        }

        //
        #[derive(Debug)]
        struct DepartmentGroup {
            // #[title]
            name: String,
            employees: Option<Vec<Employee>>,
        }

        impl std::fmt::Display for DepartmentGroup {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                let employees_string: String = match &self.employees {
                    Some(temp_employees) => temp_employees.iter().map(|e| e.to_string()).collect(),
                    _ => "".to_string(),
                };
                write!(f, "{}_{}", self.name, employees_string)
            }
        }

        //
        #[derive(Debug)]
        enum EmployeeSex {
            Male,
            Female,
        }
        impl std::fmt::Display for EmployeeSex {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                let value_string: String = match self {
                    Self::Male => "Male".to_string(),
                    Self::Female => "Male".to_string(),
                };
                write!(f, "{}", value_string)
            }
        }

        //
        #[derive(Debug)]
        struct Employee {
            // #[title]
            full_name: String,
            // #[searchable]
            first_name: String,
            // #[searchable]
            last_name: String,
            sex: EmployeeSex,
            age: u8,
            // #[searchable]
            title: String,
        }

        impl std::fmt::Display for Employee {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(
                    f,
                    "{}_{}_{}_{}_{}",
                    self.first_name, self.last_name, self.age, self.sex, self.title
                )
            }
        }

        let test_company = Company {
            name: "Facebook .Inc".to_owned(),
            ceo: "Mark".to_owned(),
            address: "US".to_owned(),
            departments: None,
        };

        // let my_company = Company::new();
        // println!("my_company: {:#?}", &my_company);
        // println!(
        // "my_company hashkey value: {:?}",
        // &my_company.generate_tree_node_hashmap_key()
        // );
        // my_company.get_data();

        // println!(
        // "test_company key: {}",
        // test_company.generate_tree_node_hashmap_key()
        // );

        // let facebook_tree: Vec<Company> = vec![Company {
        // name: "Facebook ,Inc".to_string(),
        // address: "Menlo Park, California, U.S.".to_string(),
        // ceo: "Mark Zuckerberg".to_string(),
        // departments: Some(vec![
        // Department {
        // name: "Software".to_string(),
        // manager: "Wison Ye".to_string(),
        // groups: Some(vec![
        // DepartmentGroup {
        // name: "Team A".to_string(),
        // employees: Some(vec![
        // Employee {
        // first_name: "Wison".to_string(),
        // last_name: "Ye".to_string(),
        // sex: EmployeeSex::Male,
        // age: 40u8,
        // title: "System Architect".to_string(),
        // },
        // Employee {
        // first_name: "Mike".to_string(),
        // last_name: "Ye".to_string(),
        // sex: EmployeeSex::Male,
        // age: 10u8,
        // title: "Intern".to_string(),
        // },
        // ]),
        // },
        // DepartmentGroup {
        // name: "Team B".to_string(),
        // employees: Some(vec![
        // Employee {
        // first_name: "Fion".to_string(),
        // last_name: "Li".to_string(),
        // sex: EmployeeSex::Female,
        // age: 30u8,
        // title: "Web Developer".to_string(),
        // },
        // Employee {
        // first_name: "Ana".to_string(),
        // last_name: "Li".to_string(),
        // sex: EmployeeSex::Female,
        // age: 28u8,
        // title: "Backend Developer".to_string(),
        // },
        // ]),
        // },
        // ]),
        // },
        // Department {
        // name: "Hardware".to_string(),
        // manager: "Soul".to_string(),
        // groups: Some(vec![
        // DepartmentGroup {
        // name: "Design".to_string(),
        // employees: Some(vec![
        // Employee {
        // first_name: "Tim".to_string(),
        // last_name: "J".to_string(),
        // sex: EmployeeSex::Male,
        // age: 45u8,
        // title: "Group Manger".to_string(),
        // },
        // Employee {
        // first_name: "Lina".to_string(),
        // last_name: "M".to_string(),
        // sex: EmployeeSex::Female,
        // age: 25u8,
        // title: "Assistant".to_string(),
        // },
        // ]),
        // },
        // DepartmentGroup {
        // name: "Market Research".to_string(),
        // employees: Some(vec![
        // Employee {
        // first_name: "Andy".to_string(),
        // last_name: "B".to_string(),
        // sex: EmployeeSex::Female,
        // age: 30u8,
        // title: "Group Manager ".to_string(),
        // },
        // Employee {
        // first_name: "Emali".to_string(),
        // last_name: "L".to_string(),
        // sex: EmployeeSex::Female,
        // age: 28u8,
        // title: "Assistant".to_string(),
        // },
        // ]),
        // },
        // ]),
        // },
        // ]),
        // }];
    }
}
