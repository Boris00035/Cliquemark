use adw::prelude::*;
use adw::Application;

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
    gdk_pixbuf::{Pixbuf, PixbufRotation},
    SpinButton,
    gdk::Texture,
    Stack,
    };

use std::rc::Rc;
use std::cell::RefCell;
use exif;
use rand::prelude::IndexedRandom;


const APP_ID: &str = "org.gtk_rs.Cliquemark";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    return app.run();
}

fn calculate_watermark_position(
    preview_image_dimensions: &RefCell<[i32; 2]>,
    preview_watermark_dimensions: &RefCell<[i32; 2]>,
    image_preview: &Rc<Picture>,
    scale_slider_value: &f64,
    margin_value: i32,
    top_left_check_box: &Rc<CheckButton>,
    top_right_check_box: &Rc<CheckButton>,
    bottom_left_check_box: &Rc<CheckButton>,
    bottom_right_check_box: &Rc<CheckButton>,
) -> Rectangle 
{
    let mut width_ratio = 0.0;
    let mut height_ratio = 0.0;
    
    if preview_image_dimensions.borrow()[0] != 0 && preview_image_dimensions.borrow()[1] != 0 {
        width_ratio = preview_watermark_dimensions.borrow()[0] as f64 / preview_image_dimensions.borrow()[0] as f64;
        height_ratio = preview_watermark_dimensions.borrow()[1] as f64 / preview_image_dimensions.borrow()[1] as f64;
    }
    
    let width = (width_ratio * image_preview.width() as f64 * scale_slider_value).ceil() as i32;
    let height = (height_ratio * image_preview.height() as f64 * scale_slider_value).ceil() as i32;

    let x = (top_right_check_box.is_active() as i32 + bottom_right_check_box.is_active() as i32) * (image_preview.width() - width - margin_value)
                + (top_left_check_box.is_active() as i32 + bottom_left_check_box.is_active() as i32) * margin_value;
    let y = (bottom_left_check_box.is_active() as i32 + bottom_right_check_box.is_active() as i32) * (image_preview.height() - height - margin_value)
                + (top_left_check_box.is_active() as i32 + top_right_check_box.is_active() as i32) * margin_value;

    // println!("{:?}, {:?}, {:?}, {:?}", image_preview.width(), image_preview.height(), width, height);
    
    return Rectangle::new(x, y, width, height);
    // return Rectangle::new(0,0, width, height);
}


fn build_ui(app: &Application) {

    // master container
    let master_box = Box::builder()
        .orientation(Orientation::Horizontal)
        // .spacing(50)
        .build();

    let loader_window_box = Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    let main_stack = Stack::builder()
        .transition_type(gtk::StackTransitionType::Crossfade)
        // .interpolate_size(true)
        .vhomogeneous(true)
        .hhomogeneous(true)
        .build();   
    main_stack.add_named(&master_box, Some("main_window"));
    main_stack.add_named(&loader_window_box, Some("loader_window"));

    // Create a window
    let main_window = Rc::new(ApplicationWindow::builder()
        .application(app)
        .title("Cliquemark")
        .child(&main_stack)
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
    let top_left_check_box = Rc::new(CheckButton::builder()
        .label("Top left")
        .build());
    let top_right_check_box = Rc::new(CheckButton::builder()
        .label("Top right")
        .group(&*top_left_check_box)
        .build());
    let bottom_left_check_box = Rc::new(CheckButton::builder()
        .label("Bottom left")
        .group(&*top_left_check_box)
        .build());
    let bottom_right_check_box = Rc::new(CheckButton::builder()
        .label("Bottom right")
        .group(&*top_left_check_box)
        .build());
    top_left_check_box.set_active(true);

    let alignment_check_box_container = Box::builder()
        .valign(Align::Center)
        .halign(Align::Center)
        .orientation(Orientation::Horizontal)
        .build();

    alignment_check_box_container.append(&*top_left_check_box);
    alignment_check_box_container.append(&*top_right_check_box);
    alignment_check_box_container.append(&*bottom_left_check_box);
    alignment_check_box_container.append(&*bottom_right_check_box);
    settings_box.append(&alignment_check_box_container);
    
    // scale slider
    let scale_adjustment = Adjustment::new(
        1.0,
        0.01, 
        2.0,
        0.01,
        0.01,
        0.01
    ); 
    let scale_slider = Rc::new(Scale::builder()
        .digits(2)
        .draw_value(true)
        .adjustment(&scale_adjustment)
        .build()
    );
    settings_box.append(&*scale_slider);

    let adjustment = Adjustment::new(0.0, 0.0, 100.0, 1.0, 10.0, 0.0);
    let margin_input = Rc::new(SpinButton::builder()
        .adjustment(&adjustment)
        .climb_rate(1.0)
        .digits(0)
        .halign(Align::Center)
        .margin_end(100)
        .build()
    );

    // confirm button
    let confirm_button = Button::builder()
        .halign(Align::Center)
        .label("Watermark")
        .build();

    let margin_confirm_container = Box::builder()
        .orientation(Orientation::Horizontal)
        .halign(Align::Center)
        .build();

    margin_confirm_container.append(&*margin_input);
    margin_confirm_container.append(&confirm_button);

    settings_box.append(&margin_confirm_container);

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

    let preview_widget = Rc::new(Overlay::builder()
        .build()
    );
    preview_side_sub_box.append(&*preview_widget);

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
    
    let preview_image_dimensions: Rc<RefCell<[i32; 2]>> = Rc::new(RefCell::new([0, 0]));
    let preview_watermark_dimensions: Rc<RefCell<[i32; 2]>> = Rc::new(RefCell::new([0, 0]));

    preview_widget.connect_get_child_position(
    {
        let preview_image_dimensions = Rc::clone(&preview_image_dimensions);
        let preview_watermark_dimensions = Rc::clone(&preview_watermark_dimensions);
        let image_preview = Rc::clone(&image_preview);
        let scale_slider = Rc::clone(&scale_slider);
        let margin_input = Rc::clone(&margin_input);

        let top_left_check_box = Rc::clone(&top_left_check_box);
        let top_right_check_box = Rc::clone(&top_right_check_box);
        let bottom_left_check_box = Rc::clone(&bottom_left_check_box);
        let bottom_right_check_box = Rc::clone(&bottom_right_check_box);


        move |_, _watermark_preview| {
            let watermark_rectangle = calculate_watermark_position(
                &preview_image_dimensions,
                &preview_watermark_dimensions,
                &image_preview,
                &scale_slider.value(),
                margin_input.value() as i32,
                &top_left_check_box,
                &top_right_check_box,
                &bottom_left_check_box,
                &bottom_right_check_box,
            );
            return Some(watermark_rectangle);
        }
    });    


    top_left_check_box.connect_toggled( {
        let preview_widget = Rc::clone(&preview_widget);   
        move |_| {
            let _ = &preview_widget.queue_allocate();
        }
    });    
    top_right_check_box.connect_toggled({
        let preview_widget = Rc::clone(&preview_widget);        
        move |_| {
            let _ = &preview_widget.queue_allocate();
        }
    });
    bottom_left_check_box.connect_toggled({
        let preview_widget = Rc::clone(&preview_widget);        
        move |_| {
            let _ = &preview_widget.queue_allocate();
        }
    });    
    bottom_right_check_box.connect_toggled({
        let preview_widget = Rc::clone(&preview_widget);        
        move |_| {
            let _ = &preview_widget.queue_allocate();
        }
    });

    scale_slider.connect_value_changed({
        let preview_widget = Rc::clone(&preview_widget);        
        move |_| {
            let _ = &preview_widget.queue_allocate();
        }
    });

    margin_input.connect_value_changed({
        let preview_widget = Rc::clone(&preview_widget);        
        move |_| {
            let _ = &preview_widget.queue_allocate();
        }
    });
    

    choose_folder_button.connect_clicked({
        let main_window_clone = Rc::clone(&main_window);
        let preview_image_dimensions = Rc::clone(&preview_image_dimensions);

        move |_| {
            let folder_dialog = FileDialog::builder()
            .title("Select Folder")
            .build();

            let chosen_folder_text = Rc::clone(&chosen_folder_text);            
            let image_preview = Rc::clone(&image_preview);
            let preview_image_dimensions = Rc::clone(&preview_image_dimensions);

            folder_dialog.select_folder(Some(&*main_window_clone),None::<&gtk::gio::Cancellable>, 
            move |result| {
                match result {
                    Ok(folder) => {
                        let folder_path = &folder.path().unwrap();
                        chosen_folder_text.set_text(folder_path.to_str().unwrap());
                        
                        let random_preview_entry;
                        let entries = std::fs::read_dir(folder_path).unwrap()
                        .filter_map(|entry| {
                            let entry = entry.ok()?;
                            let path = entry.path();
                            if is_image_file(&path) {
                                Some(path)
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>();
                        
                        if entries.is_empty() {
                            println!("No image files found in the folder.");
                        }

                        let mut rng = rand::rng();
                        if let Some(random_image) = entries.choose(&mut rng) {
                            random_preview_entry = random_image;
                        } else {
                            todo!();
                        };
                        
                        let mut preview_image_pixbuf = Pixbuf::from_file(&random_preview_entry).unwrap();

                        preview_image_pixbuf = match random_preview_entry.extension().and_then(|ext| ext.to_str()) {
                            Some("png") => preview_image_pixbuf,
                            Some("PNG") => preview_image_pixbuf,
                            _ => apply_exif_rotation(random_preview_entry, preview_image_pixbuf),
                        };

                        let mut image_preview_dims = preview_image_dimensions.borrow_mut();

                        image_preview_dims[0] = preview_image_pixbuf.width(); 
                        image_preview_dims[1] = preview_image_pixbuf.height();  
                        
                        image_preview.set_paintable( Some(&Texture::for_pixbuf(&preview_image_pixbuf)) );
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
            let preview_watermark_dimensions = Rc::clone(&preview_watermark_dimensions);
            let preview_widget = Rc::clone(&preview_widget);
         
            file_dialog.open(Some(&*main_window),None::<&gtk::gio::Cancellable>, move |result| {
                match result {
                    Ok(file) => {
                        let file_path = &file.path().unwrap();
                        // let file_path: &std::path::Path = &file.path().unwrap();
                        chosen_watermark_text.set_text(&file_path.file_name().unwrap().to_str().unwrap());
                        
                        let mut preview_watermark_pixbuf = Pixbuf::from_file(&file_path).unwrap();

                        let mut watermark_preview_dims = preview_watermark_dimensions.borrow_mut();
                        watermark_preview_dims[0] = preview_watermark_pixbuf.width(); 
                        watermark_preview_dims[1] = preview_watermark_pixbuf.height();  
                        // watermark_preview.set_file( Some(&File::for_path(&file_path)) );

                        preview_watermark_pixbuf = match file_path.extension().and_then(|ext| ext.to_str()) {
                            Some("png") => preview_watermark_pixbuf,
                            Some("PNG") => preview_watermark_pixbuf,
                            _ => apply_exif_rotation(file_path, preview_watermark_pixbuf),
                        };
                        watermark_preview.set_paintable( Some(&Texture::for_pixbuf(&preview_watermark_pixbuf)) );
                    }
                    Err(error) => {
                        println!("Error: {}", error);
                    }
                }
            });
            let _ = &preview_widget.queue_allocate();
        }
    });
    

    confirm_button.connect_clicked(move |_| {
        
        apply_watermark(&main_stack);
    });

    // Present window
    main_window.present();
}

fn apply_watermark(main_stack: &Stack) {
    // todo!();
    let _ = &main_stack.set_visible_child_full("loader_window", gtk::StackTransitionType::Crossfade);
    // let _ = &main_stack.set_visible_child_full("main_window", gtk::StackTransitionType::Crossfade);
}


fn apply_exif_rotation(file_path: &std::path::Path, image_pixbuf: Pixbuf) -> Pixbuf {
    let file = std::fs::File::open(file_path).unwrap();
        let mut bufreader = std::io::BufReader::new(&file);
        let exifreader = exif::Reader::new();
        let exif: exif::Exif = match exifreader.read_from_container(&mut bufreader) {
            Ok(exif) => exif,
            Err(e) => {
                eprintln!("Failed to read EXIF data: {}", e);
                return image_pixbuf;
            }
        };
    
        let image_orientation = match exif.get_field(exif::Tag::Orientation, exif::In::PRIMARY) {
            Some(orientation) =>
                match orientation.value.get_uint(0) {
                    Some(v @ 1..=8) => v,
                    _ => panic!("Orientation value is broken, file:{}", file_path.to_str().unwrap()),
                },
            Option::None => 1,
        };

        let corrected_pixbuf = match image_orientation {
            1 => Some(image_pixbuf),
            2 => image_pixbuf.flip(true),
            3 => image_pixbuf.rotate_simple(PixbufRotation::Upsidedown),
            4 => image_pixbuf.flip(false),
            5 => image_pixbuf.flip(true).unwrap().rotate_simple(PixbufRotation::Counterclockwise),
            6 => image_pixbuf.rotate_simple(PixbufRotation::Clockwise),
            7 => image_pixbuf.flip(true).unwrap().rotate_simple(PixbufRotation::Clockwise),
            8 => image_pixbuf.rotate_simple(PixbufRotation::Counterclockwise),
            _ => panic!{"Hier gaat iets heel goed mis in de match logica van de exif match"},
        }.expect("Failed to apply exif transformation");

    return corrected_pixbuf;
}

fn is_image_file(path: &std::path::Path) -> bool {
    if let Some(extension) = path.extension() {
        let ext = extension.to_string_lossy().to_lowercase();
        matches!(ext.as_str(), "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp")
    } else {
        false
    }
}