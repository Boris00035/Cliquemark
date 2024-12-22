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
    Separator};

const APP_ID: &str = "org.gtk_rs.HelloWorld1";

fn main() -> glib::ExitCode {

    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}


fn build_ui(app: &Application) {
    // folder directory chooser

    // watermark chooser

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

    bottom_left_check_box.set_active(true);

    let alignment_check_box_box = Box::builder()
        .valign(Align::Center)
        .halign(Align::Center)
        .orientation(Orientation::Horizontal)
        .build();

    alignment_check_box_box.append(&top_left_check_box);
    alignment_check_box_box.append(&top_right_check_box);
    alignment_check_box_box.append(&bottom_left_check_box);
    alignment_check_box_box.append(&bottom_right_check_box);
    
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

    // confirm button
    let confirm_button = Button::builder()
        .halign(Align::Center)
        .label("Watermark")
        .build();

    confirm_button.connect_clicked(|button| {
        button.set_label("Hello World!");
    });

    // settings container 
    let settings_box = Box::builder()
        .margin_start(50)
        .valign(Align::Center)
        .halign(Align::Center)
        .orientation(Orientation::Vertical)
        .spacing(12)
        .build();

    settings_box.append(&alignment_check_box_box);
    settings_box.append(&scale_slider);
    settings_box.append(&confirm_button);

    // master container
    let master_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(50)
        .build();

    let vertical_seperator = Separator::new(Orientation::Vertical);

    master_box.append(&settings_box);
    master_box.append(&vertical_seperator);
    // master_box.append(&result_preview);

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Cliquemark")
        .child(&master_box)
        .build();

    // Present window
    window.present();
}