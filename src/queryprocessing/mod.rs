pub mod ddl;
pub mod dml;
pub mod sql;

pub fn execute(statement: dml::DMLStatement) -> serde_json::Value {
    match statement {
        dml::DMLStatement::Insert(insert) => serde_json::json!({
            "result": handle_insert(insert)
        }),
        _ => todo!("Implement other DML statements"),
    }
}

fn handle_insert(insert_statement: dml::DMLInsertStatement) -> serde_json::Value {
    if !std::path::Path::new(&global_file_path()).exists() {
        let _ = std::fs::create_dir_all(".adaptivedb");
        let _ = std::fs::write(&global_file_path(), "[{
            \"name\": \"Employee\",
            \"rows\": []
        }]");
    }

    let mut map = std::collections::HashMap::new();
    for i in 0..insert_statement.columns.len() {
        map.insert(
            insert_statement.columns[i].clone(),
            insert_statement.values[i].clone(),
        );
    }

    let data = std::fs::read_to_string(&global_file_path()).unwrap();
    let mut data: serde_json::Value = serde_json::from_str(&data).unwrap();
    let table_name = insert_statement.table_name;
    let table_index = data
        .as_array()
        .unwrap()
        .iter()
        .position(|x| x["name"] == table_name)
        .unwrap();
    let table = &mut data[table_index]["rows"];
    table.as_array_mut().unwrap().push(serde_json::json!(map));
    let data = serde_json::to_string_pretty(&data).unwrap();
    let _ = std::fs::write(&global_file_path(), data);


    serde_json::json!({
        "success": true
    })
}

fn global_file_path() -> std::path::PathBuf {
    ".adaptivedb/data.json".into()
}
