use gtk::prelude::*;
use gtk::{
    glib, 
    Align, 
    Application, 
    ApplicationWindow, 
    Box, 
    Orientation, 
    CheckButton,
    Button,
    Scale,
    Adjustment,
    Separator,
    FileDialog,
    Grid,
    Label
    };
use std::rc::Rc;

const APP_ID: &str = "org.gtk_rs.Cliquemark";

fn main() -> glib::ExitCode {
    // Force dark mode, looks better, maybe remove this when using libadwaita
    gtk::init().expect("Failed to initialize GTK");
    if let Some(settings) = gtk::Settings::default() {
        settings.set_property("gtk-application-prefer-dark-theme", true);
    }

    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}


fn build_ui(app: &Application) {
    // master container
    let master_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(50)
        .build();

    // Create a window
    let main_window = Rc::new(ApplicationWindow::builder()
        .application(app)
        .title("Cliquemark")
        .child(&master_box)
        .build()
    );

    // settings container 
    let settings_box = Box::builder()
        .margin_start(50)
        .valign(Align::Center)
        .halign(Align::Center)
        .orientation(Orientation::Vertical)
        .spacing(12)
        .build();

    master_box.append(&settings_box);
    
    let selection_button_grid = Grid::builder()
    .valign(Align::Center)
    .halign(Align::Center)
    .build();
    selection_button_grid.set_row_spacing(12);
    selection_button_grid.set_column_spacing(12);
    
    settings_box.append(&selection_button_grid);

    // folder directory chooser
    let choose_folder_button = Button::builder()
        .label("Select Folder")
        .build();

    let chosen_folder_text = Rc::new(Label::builder()
        .label("Nothing chosen")
        .build()
    );
    selection_button_grid.attach(&choose_folder_button, 0, 0, 1, 1);
    selection_button_grid.attach(&*chosen_folder_text, 1,0,1,1);

    choose_folder_button.connect_clicked({
        let main_window = Rc::clone(&main_window);
        move |_| {
            let folder_dialog = FileDialog::builder()
            .title("Select Folder")
            .build();

            let chosen_folder_text = Rc::clone(&chosen_folder_text);
            folder_dialog.select_folder(Some(&*main_window),None::<&gtk::gio::Cancellable>, move |result| {
                match result {
                    Ok(folder) => {
                        chosen_folder_text.set_text(&folder.path().unwrap().file_name().unwrap().to_str().unwrap());
                    }
                    Err(error) => {
                        println!("Error: {}", error);
                    }
                }
            });
        }
    });

    // watermark chooser
    let choose_watermark = Button::builder()
        .label("Select Watermark")
        .build();
    
    let chosen_watermark_text = Rc::new(Label::builder()
    .label("Nothing chosen")
    .build()
    );
    selection_button_grid.attach(&choose_watermark, 0, 1, 1, 1);
    selection_button_grid.attach(&*chosen_watermark_text, 1,1,1,1);


    choose_watermark.connect_clicked({
        let main_window = Rc::clone(&main_window);
        move |_| {
            let file_dialog = FileDialog::builder()
            .title("Select Watermark")
            .build();

            let chosen_watermark_text: Rc<Label> = Rc::clone(&chosen_watermark_text);            
            file_dialog.open(Some(&*main_window),None::<&gtk::gio::Cancellable>, move |result| {
                match result {
                    Ok(file) => {
                        chosen_watermark_text.set_text(&file.path().unwrap().file_name().unwrap().to_str().unwrap());
                    }
                    Err(error) => {
                        println!("Error: {}", error);
                    }
                }
            });
        }
    });

    // Alignment check boxes
    let top_left_check_box = CheckButton::builder()
        .label("Top left")
        .build();
    let top_right_check_box = CheckButton::builder()
        .label("Top right")
        .group(&top_left_check_box)
        .build();
    let bottom_left_check_box = CheckButton::builder()
        .label("Bottom left")
        .group(&top_left_check_box)
        .build();
    let bottom_right_check_box = CheckButton::builder()
        .label("Bottom right")
        .group(&top_left_check_box)
        .build();
    bottom_right_check_box.set_active(true);

    let alignment_check_box_container = Box::builder()
        .valign(Align::Center)
        .halign(Align::Center)
        .orientation(Orientation::Horizontal)
        .build();

    alignment_check_box_container.append(&top_left_check_box);
    alignment_check_box_container.append(&top_right_check_box);
    alignment_check_box_container.append(&bottom_left_check_box);
    alignment_check_box_container.append(&bottom_right_check_box);
    settings_box.append(&alignment_check_box_container);
    
    // scale slider
    let adjustment = Adjustment::new(
        1.0,
        0.1, 
        1.0,
        0.1,
        0.0,
        0.0
    ); 
    let scale_slider = Scale::builder()
        .draw_value(true)
        .adjustment(&adjustment)
        .build();
    settings_box.append(&scale_slider);

    // confirm button
    let confirm_button = Button::builder()
        .halign(Align::Center)
        .label("Watermark")
        .build();

    confirm_button.connect_clicked(|button| {
        button.set_label("Hello World!");
    });
    settings_box.append(&confirm_button);

    let vertical_seperator = Separator::new(Orientation::Vertical);
    master_box.append(&vertical_seperator);

   
    // master_box.append(&result_preview);


    // Present window
    main_window.present();
}