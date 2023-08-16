# make_new_kit
make new kits on the deluge with rust 


Making new kits on the deluge when you get a new sample pack is tedious and time consuming.
This script allows you to extract the filenames out of a sample pack and then automaticallty create however many kits are needed to capture all of the files. 
XML files are generated and saved to your desktop under a name of your choosing. 

To run the script just clone this repo. Download Rust. Navigate to the repo on your machine. Pop open an IDE. Hard code the file path for your new sample pack in the main function in main.rs, and provide a better name than desired_folder_name and desired_file_name for the files and folder that will be generated, and also don't forget to hard code the path to your individual desktop. 

Finally run cargo run and you sohuld get a folder full of kit files to test on your deluge. 

Made with 🖤 by Busch 
Deep Current / Folk Art / Vulture Stone