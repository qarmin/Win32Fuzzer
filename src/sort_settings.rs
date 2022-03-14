use crate::{get_windows_rs_folder, TypeOfProblem};

pub fn sort_settings(content: &Vec<(&'static str, String, Vec<(&'static str, TypeOfProblem)>)>) {
    println!("vec![");

    let mut content = content.clone();
    content.sort_by_key(|e| e.0);
    for (class_name, full_mod_rs_path, mut outer_vec) in content {
        println!("\t(");
        println!("\t\t\"{}\",", class_name);

        let mod_rs_path = full_mod_rs_path.strip_prefix(&get_windows_rs_folder()).unwrap();

        println!("\t\tformat!(\"{{}}{{}}\",get_windows_rs_folder(),\"{}\"),", mod_rs_path);

        outer_vec.sort_by_key(|a| a.0);

        println!("\t\tvec![");
        for (function_name, type_of_problem) in outer_vec {
            println!("\t\t\t(\"{}\",{}),", function_name, type_of_problem.to_string());
        }
        println!("\t\t],");
        println!("\t),");
    }
    println!("]");
}
