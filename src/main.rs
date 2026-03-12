use eframe::*;
use egui::{CentralPanel, ScrollArea};
use std::fs;
use serde_json;
use image::{DynamicImage, imageops::FilterType};

// this is the main app struct
struct SSM {
    directory_path: String,
    selected_file: Option<String>, // Track which file is selected
    preview_texture: Option<egui::TextureHandle>,
}

// this is the main app impl and where the main logic will be implemented
impl eframe::App for SSM {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.label(":3");

            //game path input


            //first load the json file for both game path and directory path and set the values to the input fields so that the user doesn't have to input them every time they open the app
            if let Some(saved_directory) = load_directory_path() {
                self.directory_path = saved_directory; 
            }


            //this will make input for directory from user
            ui.label("Enter path of the folder where VTF spray files are located:");
            ui.label("(example: Steam/steamapps/common/Team Fortress 2/tf/materials/vgui/logos):");

            // Add input field for directory path
            ui.text_edit_singleline(&mut self.directory_path);
            //call function to display vtf files in the directory
            let vtf_files = get_vtf_files(&self.directory_path);
            // Save the directory path to a json file
            save_directory_path(&self.directory_path);
            ui.label("Sprays:");
            
            // Create two columns - file list on left, preview on right
            ui.columns(2, |columns| {
                // Left column - File list
                ScrollArea::vertical()
                    .max_height(600.0)
                    .show(&mut columns[0], |ui| {
                        for file in vtf_files {
                            // Check if this file is selected
                            let text_color = if self.selected_file.as_ref() == Some(&file) {
                                egui::Color32::GREEN
                            } else {
                                egui::Color32::WHITE
                            };
                            
                            // Create colored text with RichText
                            let rich_text = egui::RichText::new(&file).color(text_color);
                            
                            // Show the label and check if clicked
                            if ui.label(rich_text).clicked() {
                                self.selected_file = Some(file.clone());

                                let full_path = format!("{}/{}", self.directory_path, file);

                                if let Ok(file_bytes) = fs::read(full_path) {
                                    if let Ok(color_image) = decode_vtf_preview(&file_bytes) {
                                        self.preview_texture = Some(
                                            ctx.load_texture(
                                                "spray_preview",
                                                color_image,
                                                egui::TextureOptions::LINEAR,
                                            ),
                                        );
                                    }
                                }
                            }
                        }
                    });
                
                // Right column - Preview
                if let Some(selected_file) = &self.selected_file {
                    columns[1].label(format!("Selected spray: {}", selected_file));

                    // Call the function to show the spray image here // use show_spray_dxt to show the spray image in the right column // the current vtf selected is from which filename was highlighted in the left column
                    if let Some(texture) = &self.preview_texture {
                        let sized = egui::load::SizedTexture::from_handle(texture);
                        columns[1].image(egui::ImageSource::Texture(sized));
                    } else {
                        columns[1].label("No preview loaded.");
                    }


                    
                    //button to set spray as active spray in the game
                    if columns[1].button("Set as spray").clicked() {
                        //call set as spray
                        set_as_spray(selected_file, &self.directory_path);
                    }
                }
            });
        });
    }
}

//this function saves the directory path in a json file so that the user doesn't have to input it every time they open the app
//the json contents mock up:
/*
    {
        "sprays_path": "C:/path/to/directory"
    }

file name: paths.json

*/
//save also game path
fn save_directory_path(directory_path: &str) {
    let directory = serde_json::json!({
        "sprays_path": directory_path,
    });
    let json_data = serde_json::to_string(&directory).unwrap();
    fs::write("paths.json", json_data).expect("Unable to write file");
}



//this function will load the json file
fn load_directory_path() -> Option<String> {
    if let Ok(json_data) = fs::read_to_string("paths.json") {
        if let Ok(directory) = serde_json::from_str::<serde_json::Value>(&json_data) {
            if let Some(sprays_path) = directory.get("sprays_path") {
                if let Some(sprays_path_str) = sprays_path.as_str() {
                    return Some(sprays_path_str.to_string());
                }
            }
        }
    }
    None
}


//this function will get the directory path inputted by the user and will find all .vtf files in that directory and subdirectories and will display them in a list
//dont use unwrap i dont want to crash the app if the user inputs an invalid directory path
//make the code very simple just list all the .vtf files in the directory not recursively and display them in a list
fn get_vtf_files(directory_path: &str) -> Vec<String> {
    let mut vtf_files = Vec::new();
    if let Ok(entries) = fs::read_dir(directory_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    if let Some(extension) = path.extension() {
                        if extension == "vtf" {
                            if let Some(file_name) = path.file_name() {
                                if let Some(file_name_str) = file_name.to_str() {
                                    vtf_files.push(file_name_str.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    vtf_files
}

//this function uses vtf to decode vtf files
//converts compressed image data to egui ColorImage format for display
fn decode_vtf_preview(file_bytes: &Vec<u8>) -> Result<egui::ColorImage, Box<dyn std::error::Error>> {
    let vtf = vtf::from_bytes(file_bytes)?;
    let image: DynamicImage = vtf.highres_image.decode(0)?;
    let resized = image.resize_exact(320, 320, FilterType::Triangle);
    let rgba = resized.to_rgba8();
    let (width, height) = rgba.dimensions();
    let color_image = egui::ColorImage::from_rgba_unmultiplied(
        [width as usize, height as usize],
        rgba.as_raw(),
    );

    Ok(color_image)
}



//function when the user clicks the set as spray button it will copy the selected vtf file to the same directory with the name "spray.vtf"
//also create a spray.vmt file with the correct contents to set the spray as active spray in the game
/*
"UnlitGeneric"
{
	"$basetexture"	"vgui/logos/{}"
	"$translucent" "1"
	"$ignorez" "1"
	"$vertexcolor" "1"
	"$vertexalpha" "1"
}
*/
fn set_as_spray(selected_file: &str, directory_path: &str) {
    //create a spray.vmt file with the correct contents to set the spray as active spray in the game
    let vmt_content = format!(
        "\"UnlitGeneric\"\n{{\n\t\"$basetexture\"\t\"vgui/logos/{}\"\n\t\"$translucent\" \"1\"\n\t\"$ignorez\" \"1\"\n\t\"$vertexcolor\" \"1\"\n\t\"$vertexalpha\" \"1\"\n}}",
        selected_file.trim_end_matches(".vtf")
    );
    fs::write(format!("{}/spray.vmt", directory_path), vmt_content).expect("Unable to write file");
    //copy the selected vtf file to the same directory with the name "spray.vtf"
    fs::copy(format!("{}/{}", directory_path, selected_file), format!("{}/spray.vtf", directory_path)).expect("Unable to copy file");
}




//this is the main function where the app is launched
fn main() -> eframe::Result<(), eframe::Error> {
    run_native(
    "Source Spray Selector",
    NativeOptions::default(),
    Box::new(|_cc: &CreationContext<'_>|{
        Ok(Box::new(SSM {
            directory_path: String::new(),
            selected_file: None,
            preview_texture: None,
        }))
    }))
}
