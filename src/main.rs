#![windows_subsystem = "windows"]
use eframe::*;
use egui::{CentralPanel, ScrollArea, IconData, CursorIcon};
use std::fs;
use serde_json;
use image::{DynamicImage, imageops::FilterType};
use std::sync::Arc;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use egui::{FontData, FontDefinitions, FontFamily};
use serde::{Serialize, Deserialize};


// this is the main app struct
struct SSS {
    directory_path: String,
    selected_file: Option<String>,
    preview_texture: Option<egui::TextureHandle>,
    show_favorites: bool,
    truepath: Option<PathBuf>,
    show_gallery: bool,
    gallery_textures: HashMap<String, Option<egui::TextureHandle>>,
    show_about: bool,
    show_profiles: bool,
    profiles: Vec<Gamepaths>,
    new_profile_game: Option<String>,
    new_profile_path: Option<String>,
}

fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();

    fonts.font_data.insert(
        "lexend".to_owned(),
        Arc::new(FontData::from_static(include_bytes!("../assets/LexendDeca-Regular.ttf"))),
    );

    fonts.families.insert(FontFamily::Proportional, vec!["lexend".to_owned()]);
    fonts.families.insert(FontFamily::Monospace, vec!["lexend".to_owned()]);

    ctx.set_fonts(fonts);
}

// this is the main app impl and where the main logic will be implemented
impl eframe::App for SSS {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        catppuccin_egui::set_theme(ctx, catppuccin_egui::MOCHA);
        CentralPanel::default().show(ctx, |ui| {

            //ui.label(":3");
            //create a buttons
            ui.horizontal(|ui| {
                if ui.button("Profiles").clicked() {
                    self.show_profiles = !self.show_profiles;
                }
                if ui.button("Favorites").clicked() {
                    self.show_favorites = !self.show_favorites;
                }

                if ui.button("Gallery View").clicked() {
                    self.show_gallery = !self.show_gallery;
                }

                if ui.button("About").clicked() {
                    self.show_about = !self.show_about;
                }
            });

            //first load the json file for both game path and directory path and set the values to the input fields so that the user doesn't have to input them every time they open the app
       //     if let Some(saved_directory) = load_directory_path() {
         //       self.directory_path = saved_directory; 
       //     }   


            //this will make input for directory from user
            ui.label("Enter path of the folder where VTF spray files are located:");
            ui.label("(example: Steam/steamapps/common/Team Fortress 2/tf/materials/vgui/logos)");

            // Add input field for directory path // Save the directory path to a json file
            if ui.text_edit_singleline(&mut self.directory_path).changed() {
                save_directory_path(&self.directory_path);
            }
            
            //call function to display vtf files in the directory
            let vtf_files = get_vtf_files(&self.directory_path);
            

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
                            
                            // Show the label with selection highlight
                            let is_selected = self.selected_file.as_ref() == Some(&file);
                            let response = ui.selectable_label(is_selected, rich_text);
                            if response.hovered() {
                                ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
                            }

                            if response.clicked() {
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

                            if response.secondary_clicked() {
                                favorites_json(&self.directory_path, &file);
                            }
                        }
                    });
                
                // Right column
                if let Some(selected_file) = &self.selected_file {
                    columns[1].label(format!("Selected spray: {}", selected_file));

                    // Call the function to show the spray image here
                    if let Some(texture) = &self.preview_texture {
                        let sized = egui::load::SizedTexture::from_handle(texture);
                        columns[1].image(egui::ImageSource::Texture(sized));
                    } else {
                        columns[1].label("No preview loaded.");
                    }


                    
                    //button to set spray as active spray in the game
                    if columns[1].button("Set as spray").clicked() {
                        set_as_spray(selected_file, &self.directory_path, &self.truepath);
                    }
                }
            });
        });

        //the window to show favorites
        if self.show_favorites {
            egui::Window::new("Favorites")
            .open(&mut self.show_favorites)
            .show(ctx, |ui| {
                let favorites = get_favorite_sprays();
                if favorites.is_empty() {
                    ui.label("Right Click on the file name to add Favorites");
                } else {
                    for fav in favorites {
                        let path = Path::new(&fav);
                        let file_name = path
                            .file_name()
                            .and_then(|s| s.to_str())
                            .map(|s| s.to_string())
                            .unwrap_or_else(|| fav.clone());

                        let is_selected = self.selected_file.as_ref() == Some(&file_name);
                        let resp = ui.selectable_label(is_selected, &file_name);
                        if resp.clicked() {
                            self.selected_file = Some(file_name.clone());

                            self.truepath = Some(path.to_path_buf());

                //            if let Some(parent) = path.parent().and_then(|p| p.to_str()) {
                   //             self.directory_path = parent.to_string();
                  //          } 

                            if let Ok(file_bytes) = fs::read(&fav) {
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
                }
            });
        }


        //the window to show gallery
        if self.show_gallery {
            egui::Window::new("Gallery")
            .resizable(true)
            .open(&mut self.show_gallery)
            .show(ctx, |ui| {
                let vtf_files = get_vtf_files(&self.directory_path);
                if vtf_files.is_empty() {
                    ui.label("Loading.. will take a while depending on how many vtf files in folder (or you have none)");
                    return;
                }
                egui::ScrollArea::vertical().show(ui, |ui| {
                    egui::Grid::new("gallery_grid")
                        .num_columns(4)
                        .spacing(egui::vec2(12.0, 12.0))
                        .show(ui, |ui| {
                            for file in vtf_files {
                                let full_path = format!("{}/{}", self.directory_path, file);

                                if !self.gallery_textures.contains_key(&full_path) {
                                    let tex = fs::read(&full_path)
                                        .ok()
                                        .and_then(|bytes| decode_vtf_preview(&bytes).ok())
                                        .map(|img| {
                                            ctx.load_texture(
                                                full_path.clone(),
                                                img,
                                                egui::TextureOptions::LINEAR,
                                            )
                                        });
                                    self.gallery_textures.insert(full_path.clone(), tex);
                                }

                                let clicked = if let Some(Some(tex)) = self.gallery_textures.get(&full_path) {
                                    let sized = egui::load::SizedTexture::from_handle(tex);
                                    let image = egui::Image::new(egui::ImageSource::Texture(sized))
                                        .fit_to_exact_size(egui::Vec2::splat(96.0));
                                    let resp = ui.add(image);
                                    ui.label(&file);
                                    resp.clicked()
                                } else {
                                    ui.label("Loading...");
                                    ui.label(&file);
                                    false
                                };

                                if clicked {
                                    self.selected_file = Some(file.clone());
                                    self.preview_texture = self
                                        .gallery_textures
                                        .get(&full_path)
                                        .and_then(|t| t.clone());
                                }

                                ui.end_row();
                            }
                        });
                });
            });
        }

        if self.show_about{
            egui::Window::new("About")
            .resizable(false)
            .open(&mut self.show_about)
            .show(ctx, |ui| {
                ui.label(egui::RichText::new("Source Spray Selector").heading());
                ui.label("Version 1.1.0");
                ui.horizontal(|ui| {
                    ui.hyperlink_to("Github", "https://github.com/ShinySir/Source-Spray-Selector");
                    ui.hyperlink_to("Check for Updates", "https://github.com/ShinySir/Source-Spray-Selector/releases");
                    ui.hyperlink_to("license", "https://github.com/ShinySir/Source-Spray-Selector/blob/master/LICENSE");
                });
            });
            
        }

        if self.show_profiles {
            egui::Window::new("Game Profiles")
                .resizable(false)
                .open(&mut self.show_profiles)
                .show(ctx, |ui| {
                    ui.label("Profiles");
                    egui::ScrollArea::vertical().show(ui, |ui| {
                    // Show existing profiles
                    for profile in self.profiles.iter().filter(|p| p.gamename != "default") {
                        ui.horizontal(|ui| {
                            if ui.button(&profile.gamename).clicked() {
                                // Load this profile into directory_path
                                load_profile(&profile.gamename, &mut self.directory_path);
                            }
                            ui.label("->");
                            ui.label(&profile.sprays_path);
                        });
                    }

                    ui.separator();

                    // Button to add a new profile
                    if ui.button("+ Add Profile").clicked() {
                        self.new_profile_game = Some(String::new());
                        self.new_profile_path = Some(String::new());
                    }

                    // If the new profile input fields are active
                    if self.new_profile_game.is_some() && self.new_profile_path.is_some() {
                        // take() temporarily moves the value out of the Option
                        let mut game_input = self.new_profile_game.take().unwrap();
                        let mut path_input = self.new_profile_path.take().unwrap();

                        ui.horizontal(|ui| {
                            ui.label("Game:");
                            ui.text_edit_singleline(&mut game_input);

                            ui.label("Path:");
                            ui.text_edit_singleline(&mut path_input);

                            if ui.button("Save").clicked() {
                                add_profile(&game_input, &path_input);
                                self.profiles = load_profiles();
                            }
                        });

                        // Put back the inputs if not saved
                        self.new_profile_game = Some(game_input);
                        self.new_profile_path = Some(path_input);
                    }
                });
            });
        }

    }
}


//this function saves the directory path in a json file so that the user doesn't have to input it every time they open the app
//the json contents mock up:
/*
    {
        "sprays_path": "C:/path/to/directory"
    }
*/
fn save_directory_path(directory_path: &str) {
    let directory = serde_json::json!({
        "sprays_path": directory_path,
    });
    let json_data = serde_json::to_string(&directory).unwrap();
    fs::write("gamepath.json", json_data).expect("Unable to write json");
}

//this function will load the json file
fn load_directory_path() -> Option<String> {
    if let Ok(json_data) = fs::read_to_string("gamepath.json") {
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
    let resized = image.resize_exact(360, 360, FilterType::Triangle);
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
fn set_as_spray(selected_file: &str, directory_path: &str, truepath: &Option<std::path::PathBuf>) {
    //create a spray.vmt file with the correct contents to set the spray as active spray in the game
    let vmt_content = format!(
        "\"UnlitGeneric\"\n{{\n\t\"$basetexture\"\t\"vgui/logos/{}\"\n\t\"$translucent\" \"1\"\n\t\"$ignorez\" \"1\"\n\t\"$vertexcolor\" \"1\"\n\t\"$vertexalpha\" \"1\"\n}}",
        selected_file.trim_end_matches(".vtf")
    );
    fs::write(format!("{}/spray.vmt", directory_path), vmt_content).expect("Unable to write file");
    //copy the selected vtf file to the same directory with the name "spray.vtf"
    let check = format!("{}/{}", directory_path, selected_file);
    if Path::new(&check).exists() {
    fs::copy(format!("{}/{}", directory_path, selected_file), format!("{}/spray.vtf", directory_path)).expect("Unable to copy file");
    } else {
    println!("not in filepath, assuming it's from favorites");
        if let Some(tp) = truepath {
            println!("DEBUG: copying from {:?}", tp);
            fs::copy(tp, format!("{}/spray.vtf", directory_path)).expect("Unable to copy file");
        }
    }
}



//function to implement favorites
//when the user right clicks on a spray in the list it will add it to the favorites list and save it in a json file
fn favorites_json(directory_path: &str, selected_file: &str) {
    let full_path = format!("{}/{}", directory_path, selected_file);

    let mut root: serde_json::Value = if let Ok(json_data) = fs::read_to_string("favs.json") {
        serde_json::from_str(&json_data).unwrap_or_else(|_| serde_json::json!({}))
    } else {
        serde_json::json!({})
    };

    let mut favorites = root
        .get("favorites")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    if !favorites.contains(&full_path) {
        favorites.push(full_path);
    }

    root["favorites"] = serde_json::Value::Array(
        favorites.into_iter().map(serde_json::Value::String).collect(),
    );

    let json_data = serde_json::to_string_pretty(&root).unwrap();
    fs::write("favs.json", json_data).expect("Unable to write json");
}


//this function is similar to get_vtf_files but it will get the favorite sprays from the json file and display them in a separate list
fn get_favorite_sprays() -> Vec<String> {
    if let Ok(json_data) = fs::read_to_string("favs.json") {
        if let Ok(root) = serde_json::from_str::<serde_json::Value>(&json_data) {
            if let Some(favorites) = root.get("favorites").and_then(|v| v.as_array()) {
                return favorites
                    .iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect();
            }
        }
    }
    Vec::new()
}


//fucntions to implement game profiles
#[derive(Debug, Deserialize, Serialize)]
struct Gamepaths {
    gamename: String,
    sprays_path: String,
}


// function to load the game profiles
fn load_profiles() -> Vec<Gamepaths> {
    if let Ok(json_data) = fs::read_to_string("profiles.json") {
        if let Ok(mut profiles) = serde_json::from_str::<Vec<Gamepaths>>(&json_data) {
            if !profiles.iter().any(|p| p.gamename == "default") {
                profiles.insert(0, Gamepaths {
                    gamename: "default".to_string(),
                    sprays_path: String::new(),
                });
            }
            return profiles;
        }
    }

    // If file missing or invalid, return default profile
    vec![Gamepaths { gamename: "default".to_string(), sprays_path: String::new() }]
}

fn save_profiles(profiles: &Vec<Gamepaths>) {
    let json_data = serde_json::to_string_pretty(profiles).unwrap();
    fs::write("profiles.json", json_data).expect("Unable to write profiles.json");
}

// Add a new profile
fn add_profile(gamename: &str, sprays_path: &str) {
    let mut profiles = load_profiles();

    // Check if profile exists
    if let Some(existing) = profiles.iter_mut().find(|p| p.gamename == gamename) {
        // overwrite the path
        existing.sprays_path = sprays_path.to_string();
    } else {
        // add new profile
        profiles.push(Gamepaths {
            gamename: gamename.to_string(),
            sprays_path: sprays_path.to_string(),
        });
    }

    // save updated profiles
    save_profiles(&profiles);
}

//function to load the game profile to the directory path
fn load_profile(gamename: &str, directory_path: &mut String) {
    let profiles = load_profiles();

    if let Some(profile) = profiles.iter().find(|p| p.gamename == gamename) {
        *directory_path = profile.sprays_path.clone();
    } else {
        // fallback to default
        if let Some(default) = profiles.iter().find(|p| p.gamename == "default") {
            *directory_path = default.sprays_path.clone();
        } else {
            *directory_path = String::new();
        }
    }
}


//this is the main function
fn main() -> eframe::Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        
        viewport: egui::ViewportBuilder::default()
            .with_icon(Arc::new(load_icon()))
            .with_app_id("Source_Spray_Selector"),
            
        ..Default::default()
    };

    let directory_path = load_directory_path().unwrap_or_default();

    run_native(
        "Source Spray Selector :3",
        options,
        Box::new(|cc| {
            setup_fonts(&cc.egui_ctx);
            Ok(Box::new(SSS {
                directory_path,
                selected_file: None,
                preview_texture: None,
                show_favorites: false,
                truepath: None,
                show_gallery: false,
                gallery_textures: HashMap::new(),
                show_about: false,
                show_profiles: false,
                profiles: load_profiles(),
                new_profile_game: None,
                new_profile_path: None,
            }))
        }),
    )
}


fn load_icon() -> IconData {
    // Example: Load and convert a PNG file to IconData
    let png_bytes = include_bytes!("../assets/icon.png").as_slice();
    eframe::icon_data::from_png_bytes(png_bytes)
        .expect("Failed to convert to IconData")
}
