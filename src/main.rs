use adw::prelude::*;
use adw::Application;
use gtk::graphene::Vec2;
use gtk::{
    glib, 
    Align, 
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
    Label,
    ScrolledWindow,
    Overlay,
    gdk::Rectangle,
    Picture,
    gio::File,
    gdk_pixbuf::Pixbuf,
    };

use std::rc::Rc;
use std::cell::RefCell;

const APP_ID: &str = "org.gtk_rs.Cliquemark";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    return app.run();
}

fn build_ui(app: &Application) {
    // master container
    let master_box = Box::builder()
        .orientation(Orientation::Horizontal)
        // .spacing(50)
        .build();

    // Create a window
    let main_window = Rc::new(ApplicationWindow::builder()
        .application(app)
        .title("Cliquemark")
        .child(&master_box)
        .build()
    );
    main_window.set_size_request(1100, 600);

    // settings container 
    let settings_box = Box::builder()
        .margin_start(50)
        .margin_bottom(50)
        .margin_top(50)
        .margin_end(50)
        .valign(Align::Center)
        .halign(Align::Center)
        .orientation(Orientation::Vertical)
        .spacing(12)
        .build();
    settings_box.set_hexpand(false);

    master_box.append(&settings_box);
    
    let selection_button_grid = Grid::builder()
    .valign(Align::Center)
    .build();
    selection_button_grid.set_row_spacing(12);
    selection_button_grid.set_column_spacing(12);    
    settings_box.append(&selection_button_grid);

    // folder directory chooser
    let choose_folder_button = Button::builder()
        .label("Select Folder")
        .build();

    let chosen_folder_text = Rc::new(Label::builder()
        .hexpand(true)
        .label("Nothing chosen")
        .build()
    );
    let folder_scrolled_container = ScrolledWindow::builder()
        .build();

    // Add the TextView to the ScrolledWindow
    folder_scrolled_container.set_child(Some(&*chosen_folder_text));

    selection_button_grid.attach(&choose_folder_button, 0, 0, 1, 1);
    selection_button_grid.attach(&folder_scrolled_container, 1,0,1,1);

    // watermark chooser
    let choose_watermark_button = Button::builder()
        .label("Select Watermark")
        .build();
    
    let chosen_watermark_text = Rc::new(Label::builder()
        .label("Nothing chosen")
        .build()
    );
    let watermark_scrolled_container = ScrolledWindow::builder()
        .build();
    watermark_scrolled_container.set_child(Some(&*chosen_watermark_text));
    selection_button_grid.attach(&choose_watermark_button, 0, 1, 1, 1);
    selection_button_grid.attach(&watermark_scrolled_container, 1,1,1,1);


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

    let preview_side_box = Box::builder()
        .margin_start(50)
        .margin_end(50)
        .margin_top(50)
        .margin_bottom(50)
        .orientation(Orientation::Vertical)
        .halign(Align::Center)
        .valign(Align::Fill)
        .hexpand(true)
        .vexpand(true)
        .build();
    master_box.append(&preview_side_box);
    
    let preview_side_sub_box = Box::builder()
        .orientation(Orientation::Vertical)
        .halign(Align::Fill)
        .valign(Align::Center)
        .hexpand(true)
        .vexpand(true)
        .build();
    preview_side_box.append(&preview_side_sub_box);

    let preview_widget = Overlay::builder()
        .build();
    preview_side_sub_box.append(&preview_widget);

    let image_preview = Rc::new(Picture::builder()
        .build()
    );
    preview_widget.set_child(Some(&*image_preview));

    let watermark_preview = Rc::new(Picture::builder()
        // .content_fit(ContentFit::Contain)
        .build()
    );
    
    // preview_widget.set_child(Some(&aspect_frame));
    preview_widget.add_overlay(&*watermark_preview);

    let image_preview_clone = Rc::clone(&image_preview);
    // let watermark_preview_clone = Rc::clone(&watermark_preview);
    
    let preview_image_dimensions: Rc<RefCell<[i32; 2]>> = Rc::new(RefCell::new([0, 0]));
    let preview_image_dimensions_clone = Rc::clone(&preview_image_dimensions);

    preview_widget.connect_get_child_position(move |_, _watermark_preview| {
            Picture::file(&image_preview_clone);
            // println!("{:?}", &preview_image_dimensions_clone.borrow());

            let x = (preview_image_dimensions_clone.borrow()[0] as f64 / 2.0).round() as i32;
            let y = (preview_image_dimensions_clone.borrow()[1] as f64 / 2.0).round() as i32;
            let width = 200;
            let height = 200;
            return Some(Rectangle::new(x, y, width, height));
    });    
    
    choose_folder_button.connect_clicked({
        let main_window_clone = Rc::clone(&main_window);
        let preview_image_dimensions_clone = Rc::clone(&preview_image_dimensions);

        move |_| {
            let folder_dialog = FileDialog::builder()
            .title("Select Folder")
            .build();

            let chosen_folder_text = Rc::clone(&chosen_folder_text);            
            let image_preview_clone = Rc::clone(&image_preview);
            let preview_image_dimensions_clone_clone = Rc::clone(&preview_image_dimensions_clone);

            folder_dialog.select_folder(Some(&*main_window_clone),None::<&gtk::gio::Cancellable>, 
            move |result| {
                match result {
                    Ok(folder) => {
                        let folder_path = &folder.path().unwrap();
                        chosen_folder_text.set_text(folder_path.to_str().unwrap());
                        
                        let preview_file_entry = std::fs::read_dir(folder_path).unwrap().next().unwrap().unwrap().path();
                        let preview_file_pixbuf = Pixbuf::from_file(&preview_file_entry).unwrap();

                        let mut dims = preview_image_dimensions_clone_clone.borrow_mut();
                        dims[0] = preview_file_pixbuf.width(); 
                        dims[1] = preview_file_pixbuf.height();  
                        
                        image_preview_clone.set_file(Some(&File::for_path( &preview_file_entry )));
                    }
                    Err(error) => {
                        println!("Error: {}", error);
                    }
                }
            });
        }
    });

    choose_watermark_button.connect_clicked({
        let main_window = Rc::clone(&main_window);
        move |_| {
            let file_dialog = FileDialog::builder()
            .title("Select Watermark")
            .build();

            let chosen_watermark_text: Rc<Label> = Rc::clone(&chosen_watermark_text); 
            let watermark_preview = Rc::clone(&watermark_preview);            
            file_dialog.open(Some(&*main_window),None::<&gtk::gio::Cancellable>, move |result| {
                match result {
                    Ok(file) => {
                        let file_path = &file.path().unwrap();
                        chosen_watermark_text.set_text(&file_path.file_name().unwrap().to_str().unwrap());
                        watermark_preview.set_file( Some(&File::for_path(&file_path)) );
                    }
                    Err(error) => {
                        println!("Error: {}", error);
                    }
                }
            });
        }
    });
    
    // Present window
    main_window.present();
}