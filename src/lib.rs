#![allow(warnings)]
mod tree;

use simple_fast_tree_macros::Searchable;
use simple_fast_tree_traits::GenerateTreeNodeHashmapKey;
use tree::SimpleFastTreeNode;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_from_tree_like_vec() {
        #[derive(Searchable, Debug)]
        struct Company {
            // #[searchable]
            name: String,
            address: String,
            ceo: String,
            // departments: Option<Vec<Department>>,
        }
        // #[derive(Debug)]
        // struct Department {
        // // #[Searchable]
        // name: String,
        // manager: String,
        // groups: Vec<DepartmentGroup>,
        // }
        //
        // #[derive(Debug)]
        // struct DepartmentGroup {
        // // #[searchable]
        // name: String,
        // employees: Vec<Employee>,
        // }
        //
        // #[derive(Debug)]
        // enum EmployeeSex {
        // Male,
        // Female,
        // }
        //
        // #[derive(Debug)]
        // struct Employee {
        // // #[searchable]
        // first_name: String,
        // // #[searchable]
        // last_name: String,
        // sex: EmployeeSex,
        // age: u8,
        // // #[searchable]
        // title: String,
        // }

        // let test_company = Company {
        // name: "Facebook .Inc".to_owned(),
        // ceo: "Mark".to_owned(),
        // address: "US".to_owned(),
        // departments: None
        // };

        let my_company = Company::new();
        println!("my_company: {:#?}", &my_company);
        println!("my_company hashkey value: {}", &my_company.generate_tree_node_hashmap_key());
        my_company.get_data();

        // println!(
        // "test_company key: {}",
        // test_company.generate_tree_node_hashmap_key()
        // );

        // let facebook_tree: Vec<Company> = vec![Company {
        // name: "Facebook ,Inc".to_string(),
        // address: "Menlo Park, California, U.S.".to_string(),
        // ceo: "Mark Zuckerberg".to_string(),
        // departments: vec![
        // Department {
        // name: "Software".to_string(),
        // manager: "Wison Ye".to_string(),
        // groups: vec![
        // DepartmentGroup {
        // name: "Team A".to_string(),
        // employees: vec![
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
        // ],
        // },
        // DepartmentGroup {
        // name: "Team B".to_string(),
        // employees: vec![
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
        // ],
        // },
        // ],
        // },
        // Department {
        // name: "Hardware".to_string(),
        // manager: "Soul".to_string(),
        // groups: vec![
        // DepartmentGroup {
        // name: "Design".to_string(),
        // employees: vec![
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
        // ],
        // },
        // DepartmentGroup {
        // name: "Market Research".to_string(),
        // employees: vec![
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
        // ],
        // },
        // ],
        // },
        // ],
        // }];
    }
}
