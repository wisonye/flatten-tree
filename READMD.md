# Flatten Tree

Simple and easy to use tree structure with the following features:

- Created instance by passing a `Tree-like` `Vector`.

- Allows any `struct` as tree node, just apply the derive macro named **`#[derive(FlattenTreeNode)]`** which is a procedural macro.

- Allows any `struct field` which implemented the `std::fmt::Display` trait to become a title field, by applying the **`#[title]`** attribute on the filed. Only one filed can apply this attribute, as only one `title` can be displayed at any given time for the particular tree node.

    The field be applied this attribute becomes searchable automatic, just like the **`#[searchable]`** attribute below.

- As an option, allows any `struct field` which implemented the `std::fmt::Display` trait to be able to search, by applying the **`#[searchable]`** attribute on the filed, super flexible.

- By flattening all tree nodes into an internal `HashMap` to provide the super fast searching and filtering performance.

<br>

The main goal of this structure is aiming to the UI tree component with the following requirements:

- Load once without changing very often

- Near real-time filtering performance needed when user typing the search key word.

<br>

# Step-by-step usage

1. Define any struct you want to store as a tree structure:

    ```rust
    #[title(field_name = "name")]
    #[searchable(field_names = "ceo")]
    #[derive(FlattenTreeNode, Debug)]
    struct Company {
        name: String,
        address: String,
        ceo: String,
        departments: Option<Vec<Department>>,
    }

    #[title(field_name = "name")]
    #[searchable(field_names = "manager")]
    #[derive(FlattenTreeNode, Debug)]
    struct Department {
        name: String,
        manager: String,
        groups: Option<Vec<DepartmentGroup>>,
    }

    #[title(field_name = "name")]
    #[derive(FlattenTreeNode, Debug)]
    struct DepartmentGroup {
        name: String,
        employees: Option<Vec<Employee>>,
    }

    #[title(field_name = "full_name")]
    #[searchable(field_names = "first_name, last_name, title")]
    #[derive(FlattenTreeNode, Debug)]
    struct Employee {
        full_name: String,
        first_name: String,
        last_name: String,
        sex: EmployeeSex,
        age: u8,
        title: String,
    }
    ```

2. Create the `Tree-like` Vector as the data source:

    ```rust
    let facebook_tree: Vec<Company> = vec![Company {
        name: "Facebook ,Inc".to_string(),
        address: "Menlo Park, California, U.S.".to_string(),
        ceo: "Mark Zuckerberg".to_string(),
        departments: Some(vec![
            Department {
                name: "Software".to_string(),
                manager: "Wison Ye".to_string(),
                groups: Some(vec![
                    DepartmentGroup {
                        name: "Team A".to_string(),
                        employees: Some(vec![
                            Employee {
                                first_name: "Wison".to_string(),
                                last_name: "Ye".to_string(),
                                sex: EmployeeSex::Male,
                                age: 40u8,
                                title: "System Architect".to_string(),
                            },
                            Employee {
                                first_name: "Mike".to_string(),
                                last_name: "Ye".to_string(),
                                sex: EmployeeSex::Male,
                                age: 10u8,
                                title: "Intern".to_string(),
                            },
                        ]),
                    },
                    DepartmentGroup {
                        name: "Team B".to_string(),
                        employees: Some(vec![
                            Employee {
                                first_name: "Fion".to_string(),
                                last_name: "Li".to_string(),
                                sex: EmployeeSex::Female,
                                age: 30u8,
                                title: "Web Developer".to_string(),
                            },
                            Employee {
                                first_name: "Ana".to_string(),
                                last_name: "Li".to_string(),
                                sex: EmployeeSex::Female,
                                age: 28u8,
                                title: "Backend Developer".to_string(),
                            },
                        ]),
                    },
                ]),
            },
            Department {
                name: "Hardware".to_string(),
                manager: "Soul".to_string(),
                groups: Some(vec![
                    DepartmentGroup {
                        name: "Design".to_string(),
                        employees: Some(vec![
                            Employee {
                                first_name: "Tim".to_string(),
                                last_name: "J".to_string(),
                                sex: EmployeeSex::Male,
                                age: 45u8,
                                title: "Group Manger".to_string(),
                            },
                            Employee {
                                first_name: "Lina".to_string(),
                                last_name: "M".to_string(),
                                sex: EmployeeSex::Female,
                                age: 25u8,
                                title: "Assistant".to_string(),
                            },
                        ]),
                    },
                    DepartmentGroup {
                        name: "Market Research".to_string(),
                        employees: Some(vec![
                            Employee {
                                first_name: "Andy".to_string(),
                                last_name: "B".to_string(),
                                sex: EmployeeSex::Female,
                                age: 30u8,
                                title: "Group Manager ".to_string(),
                            },
                            Employee {
                                first_name: "Emali".to_string(),
                                last_name: "L".to_string(),
                                sex: EmployeeSex::Female,
                                age: 28u8,
                                title: "Assistant".to_string(),
                            },
                        ]),
                    },
                ]),
            },
        ]),
    }];
    ```

3. Create `FlattenTree` instance by calling:

    ```rust
    let tree = FlattenTree::From_vec(facebook_tree);

    ```


# Limitaion

- The **`#[title]`** attribute should be able to accept a closure to generate the complicated tree node title for the specified struct.
