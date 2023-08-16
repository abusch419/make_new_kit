# Make new kits from sample packs automatically for the deluge with Rust!

Making new kits on the deluge when you get a new sample pack is tedious and time consuming.

This script allows you to extract the filenames out of a sample pack and then automaticallty create however many kits are needed to capture all of the files. 
XML files are generated and saved to your desktop under a name of your choosing. 

To run the script follow these steps:
1. Clone this repo. 
2. Download Rust. 
3. Navigate to the repo on your machine. 
4. Pop open a code editor. 
5. Hard code the global variables at the top of main.rs and read the comments carefully when doing do. 

Finally run cargo run and you should get a folder full of kit files to test on your deluge. 

I recommend running one folder of voices at a time to avoid confusing naming. The script isn't smart enough to name all the kits with the voices they contain - kit names are currently hard coded. This is because the script also isn't smart enough to create kits by voice. This would be difficult in some cases since all sample packs have different naming conventions. 

This is why I process voices by group - kick, snare, hi hat, cymbal, etc. 

When it come to loops - it might make more sense to process large groups of different loop types at once if the names are hard to categorize under one single identifier. 

Here's a tiny video of how I use it.

https://github.com/abusch419/make_new_kit/assets/48231024/40927f2e-008c-4026-bcb6-9dd03ac3575b



Made with 🖤 by Busch 
Deep Current / Folk Art / Vulture Stone
abusch419@gmail.com
