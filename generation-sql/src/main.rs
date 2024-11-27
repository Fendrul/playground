use std::fs;
use std::fs::File;
use std::io::{Error, Write};
use std::path::Path;

use file_reader::file_reader::FileReader;

fn main() -> std::io::Result<()> {
    // only one of those function is needed to generate what we need un the current process
    // generate_drop_constraints_sql()?;
    generate_fix_table_id()?;

    Ok(())
}

fn generate_fix_table_id() -> Result<(), Error> {
    let reader = FileReader::new("src/table_id")?;

    let table_names_with_id: Vec<_> = reader
        .map(|line| {
            let mut parts = line.split(',');

            let name = parts.next().expect("Couldn't get name");
            let id = parts.next().expect("Couldn't get id").trim();

            if parts.next().is_some() {
                panic!("Too many parts for line {}", line);
            }

            let id = match id.parse::<u32>() {
                Ok(id) => id,
                Err(e) => {
                    println!("Parse error: {}", e);
                    println!("Error parsing id: {}", e);
                    println!("Error kind: {:?}", e.kind());

                    panic!("Couldn't parse number {}", id)
                }
            };

            (name.to_string(), id)
        })
        .collect();

    let mut fix_auto_id_file = File::create("src/generated-sql-scripts/fix_auto_id.sql")?;

    for (table_name, mut id) in &table_names_with_id {
        id += 1;

        writeln!(
            fix_auto_id_file,
            "alter table {table_name}
            alter column ID                set generated always as identity                (start with {id}, increment by 1);
                "        )?
    }

    Ok(())
}

fn generate_drop_constraints_sql() -> std::io::Result<()> {
    let reader = FileReader::new("src/table_names")?;

    let table_names: Vec<_> = reader
        .map(|line| line.trim().to_string())
        .collect();

    create_constraints_sql(&table_names)?;

    generate_select_id_sql(&table_names)?;

    Ok(())
}

fn generate_select_id_sql(table_names: &Vec<String>) -> std::io::Result<()> {
    let mut get_max_id_file = File::create("src/generated-sql-scripts/get_max_id.sql")?;

    writeln!(
        get_max_id_file,
        "create table TEMP_TABLE
        (            table_name varchar(255) not null,            max_count  int          not null        );"    )
        .unwrap();

    for table_name in table_names {
        writeln!(
            get_max_id_file,
            "INSERT INTO TEMP_TABLE (table_name, max_count)
        SELECT '{0}', COALESCE(MAX(ID), 0)
        FROM INES.{0};
        ",
            table_name.clone()
        )?;
    }

    writeln!(get_max_id_file, "select * from TEMP_TABLE;").unwrap();

    writeln!(get_max_id_file, "drop table TEMP_TABLE;").unwrap();

    Ok(())
}

fn create_constraints_sql(table_names: &Vec<String>) -> std::io::Result<()> {
    let mut sql_queries = Vec::new();

    for table_name in table_names {
        let string = format!(
            "alter table {table_name}
    alter column ID        drop identity;        "        );
        sql_queries.push(string);
    }

    let directory_path = Path::new("src/generated-sql-scripts");
    if !directory_path.exists() {
        fs::create_dir(directory_path)?;
    }

    let mut drop_contraints_file = File::create("src/generated-sql-scripts/drop_constraints.sql")?;

    for query in sql_queries {
        writeln!(drop_contraints_file, "{}", query).unwrap();
    }

    Ok(())
}