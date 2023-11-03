// use druid::widget::{Button, Flex, Label};
// use druid::{AppLauncher, Data, Env, Widget, WindowDesc};
// use druid::WidgetExt;

// #[derive(Clone)]
// struct AppState {
//     message: String,
//     argument: String,
// }

// impl Data for AppState {
//     fn same(&self, other: &Self) -> bool {
//         self.message == other.message  && self.argument == other.argument
//     }
// }

// fn main() {
//     let main_window = WindowDesc::new(ui_builder())
//         .title("Etiquetador")
//         .window_size((300.0, 200.0));

//         let argument = "seu_argumento_aqui".to_string();

//         let initial_state = AppState {
//             message: "Escolha uma opção:".to_string(),
//             argument,
//         };


//     let launcher = AppLauncher::with_window(main_window);

//     launcher
//         .log_to_console()
//         .launch(initial_state)
//         .expect("Failed to launch application");
// }

// fn ui_builder() -> impl Widget<AppState> {
//     Flex::column()
//         .with_child(Label::new(|data: &AppState, _env: &Env| data.message.clone()))
//         .with_child(
//             Button::new("PrusaSlicer")
//                 .on_click(|_ctx, _data: &mut AppState, _env| {
//                     println!("Executando PrusaSlicer...");
//                     // Adicione aqui o código para executar o PrusaSlicer.
//                 })
//                 .padding(10.0),
//         )
//         .with_child(
//             Button::new("IdeaMaker")
//                 .on_click(|_ctx, data: &mut AppState, _env| {
//                     println!("Executando IdeaMaker...");
//                     let _result = std::process::Command::new("C:\\Program Files\\Raise3D\\ideaMaker\\ideaMaker.exe")
//                     .arg(&data.argument)
//                     .spawn();
//                     _ctx.window().close(); // Fecha a janela
//                 })
//                 .padding(10.0),
//         )
//         .with_child(
//             Button::new("Cancelar")
//                 .on_click(|_ctx, _data: &mut AppState, _env| {
//                     println!("Operação cancelada.");
//                     _ctx.window().close(); // Fecha a janela
//                 })
//                 .padding(10.0),
//         )
// }
// Switch to "windows" mode to hide the console in release version
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use druid::widget::{Button, Flex, Label};
use druid::{AppLauncher, Data, Env, Lens, Widget, WindowDesc};
use toml::Value;
use std::fs::File;
use std::io::prelude::*;
use std::env;
use druid::WidgetExt;

#[derive(Clone, Lens)]
struct AppState {
    message: String,
    programs: Vec<ProgramInfo>,
}

#[derive(Clone, Lens, PartialEq)]
struct ProgramInfo {
    name: String,
    path: String,
    params: String,
}

impl Data for AppState {
    fn same(&self, other: &Self) -> bool {
        self.message == other.message && self.programs == other.programs
    }
}

fn main() {

    // formatar para usar o arquivo de configuração esteja na mesma pasta do executavel
    let config_file = format!("{}/opts.cfg", env::current_exe().unwrap().parent().unwrap().display());

    if !config_file_exists(&config_file) {
        create_default_config(&config_file);
        println!("Arquivo 'opts.cfg' criado com configurações padrão. Edite-o antes de continuar.");
        return;
    }

    if env::args().len() == 1 {
        println!("Nenhum argumento foi passado. Abortando.");
        return;
    }
    println!("local do arquivo: {} \n executavel {}", env::current_dir().unwrap().display(), env::current_exe().unwrap().display());  

    let config = std::fs::read_to_string(config_file).expect("Failed to read opts.cfg");
    let parsed_config: Value = toml::from_str(&config).expect("Failed to parse opts.cfg");

    let mut programs = parse_programs(&parsed_config);

    let mut height = 150.0;
    for program in &mut programs {
        program.params = env::args().skip(1).collect::<Vec<String>>().join(" ");
        println!("{}:{} : {}", program.name, program.path, program.params);
        
        // adiciona o espaco necessario para a gui
        height += 40.0;
    }
 

    let main_window = WindowDesc::new(ui_builder(programs.len() as usize))
        .title("Launcher")
        .window_size((300.0, height));

    let initial_state = AppState {
        message: "Escolha uma opção:".to_string(),
        programs,
    };

    let launcher = AppLauncher::with_window(main_window);

    launcher
        .log_to_console()
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn config_file_exists(filename: &str) -> bool {
    std::fs::metadata(filename).is_ok()
}

fn create_default_config(filename: &str) {
    let default_config = r#"
# Program 1
[program.1]
name = "myProgram1"
path = "/path/to/myProgram1"
params = "-param1 value1 -param2 value2"

# Program 2
[program.2]
name = "myProgram2"
path = "/path/to/myProgram2"
params = "-param1 value1 -param2 value2"
"#;

    let mut file = File::create(filename).expect("Failed to create config file");
    file.write_all(default_config.as_bytes()).expect("Failed to write default config");
}

fn parse_programs(config: &Value) -> Vec<ProgramInfo> {
    let mut programs = vec![];
    for i in 1..=25 {
        if let Some(program_info) = config["program"].get(&format!("{}", i)) {
            let name = program_info["name"].as_str().unwrap_or_default();
            let path = program_info["path"].as_str().unwrap_or_default();
            let params = program_info["params"].as_str().unwrap_or_default();

            let program = ProgramInfo {
                name: name.to_string(),
                path: path.to_string(),
                params: params.to_string(),
            };

            programs.push(program);
        }
    }

    programs
}

fn ui_builder( proglen: usize) -> impl Widget<AppState> {

    let mut col = Flex::column().with_child(Label::new(|data: &AppState, _env: &Env| data.message.clone()));

    println!("proglen: {}", proglen);
    for i in 0..=proglen - 1 {
        println!("i: {}", i);
        col.add_child(
            Button::new(move|data: &AppState, _env: &Env| data.programs[i].name.clone())
                .on_click(move|_ctx, data: &mut AppState, _env| {
                    println!("Executando {}...", data.programs[i].name);
                    let _result = std::process::Command::new(&data.programs[i].path)
                        .arg(&data.programs[i].params)
                        .spawn();
                    _ctx.window().close();
                })
                .padding(10.0),
        );
    }
    col.add_child(
        Button::new("Cancelar")
            .on_click(|_ctx, _data: &mut AppState, _env| {
                println!("Operação cancelada.");
                _ctx.window().close();
            })
            .padding(10.0),
    );

    col
    // Flex::column()
    //     .with_child(Label::new(|data: &AppState, _env: &Env| data.message.clone()))
    //     .with_child(
    //         Button::new(|data: &AppState, _env: &Env| data.programs[0].name.clone())
    //             .on_click(|_ctx, data: &mut AppState, _env| {
    //                 println!("Executando {}...", data.programs[0].name);
    //                 let _result = std::process::Command::new(&data.programs[0].path)
    //                     .arg(&data.programs[0].params)
    //                     .spawn();
    //                 _ctx.window().close();
    //             })
    //             .padding(10.0),
    //     )
    //     .with_child(
    //         Button::new(|data: &AppState, _env: &Env| data.programs[1].name.clone())
    //             .on_click(|_ctx, data: &mut AppState, _env| {
    //                 println!("Executando {}...", data.programs[1].name);
    //                 let _result = std::process::Command::new(&data.programs[1].path)
    //                     .arg(&data.programs[1].params)
    //                     .spawn();
    //                 _ctx.window().close();
    //             })
    //             .padding(10.0),
    //     )
    //     .with_child(
    //         Button::new("Cancelar")
    //             .on_click(|_ctx, _data: &mut AppState, _env| {
    //                 println!("Operação cancelada.");
    //                 _ctx.window().close();
    //             })
    //             .padding(10.0),
    //     )
}
