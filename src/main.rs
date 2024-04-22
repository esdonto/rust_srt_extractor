use std::process::{Command, Output};
use eframe::egui;
use regex;
use native_dialog;

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_drag_and_drop(true).with_inner_size([260.0, 180.0]),
        ..Default::default()
    };
    let _ = eframe::run_native("Subtitle extraction", options, Box::new(|cc| Box::new(MyEguiApp::new(cc))));
}

//--------------------------------------------------------------------------
// GUI code

struct MyEguiApp {}

impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self { }
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::LeftToRight), |ui| {
                ui.heading("Drag file into window");
            });
        });

        // Treating the file being dragged into the window
        ctx.input(|i| {
            if !i.raw.dropped_files.is_empty() {
                let selected_file = <Option<std::path::PathBuf> as Clone>::clone(&i.raw.dropped_files[0].path).unwrap().into_os_string().into_string().unwrap();
                // Extracting the subtitles, getting the amount of successfull attempts
                let quant = extract_subtitles(&selected_file);
                // Dialog with the amount of extracted subtitles
                native_dialog::MessageDialog::new()
                    .set_type(native_dialog::MessageType::Info)
                    .set_title("Extraction results")
                    .set_text(&format!("{} subtitle(s) extracted", quant))
                    .show_alert()
                    .unwrap();
            }
        });
    }
}

//--------------------------------------------------------------------------

// Run the given command in the native terminal - Taken from the std::process:Command docs
fn run_command(command_text: &str) -> Output {
    if cfg!(target_os = "windows") {
        Command::new("powershell")
        .args(["-c", command_text])
        .output()
        .expect("failed to execute process")
    } else {
        Command::new("sh")
        .arg("-c")
        .arg(command_text)
        .output()
        .expect("failed to execute process")
    }
}

// Using ffmpeg to extract the subtitles from a video file saving them as .srt files
fn extract_subtitles(path: &str) -> u32 {
    // Getting folder of the selected file
    let re = regex::Regex::new(r"[^\/\\]+$").unwrap();
    let out_path = re.replace(path, "");

    // Getting the amount of available subtitles embedded in the video file
    let info = run_command(&format!("ffmpeg -i \"{}\"", path));println!("ffmpeg -i \"{}\"", path);

    let ree = regex::Regex::new(r"Stream #0:[^:]*: Subtitle").unwrap();
    let mut success = 0;

    for i in 0..ree.find_iter(&String::from_utf8(info.stderr).expect("Failure at reading ffmpeg output")).count() {
        let command_result = run_command(&format!("ffmpeg -i \"{}\" -map 0:s:{} \"{}/sub{}.srt\"", path, i, out_path, success));
        if command_result.status.success() {
            success += 1;
        }
    }
    return success;
}
