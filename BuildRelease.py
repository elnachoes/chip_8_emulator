from os import system

destination_folder_name = "WindowsRelease"

release_includes = [
    # game dirs
    # NOTE : all of the games here were retrieved from https://johnearnest.github.io/chip8Archive/ please check them out! 
    "fullgames",
    
    # dll deps
    "dependancies",
    
    # the release executable
    "target\\release\\*.exe",
    
    # include the readme
    "README.md"
]

# NOTE : as of right now this script is designed to run on windows with just cmd

# run a build
system("cargo build -r")

# make a build folder
system(f"del /q {destination_folder_name}")

# make a build folder
system(f"mkdir {destination_folder_name}")

# move files to the new build folder
for i in release_includes :
    system(f"copy {i} {destination_folder_name}")

# zip up the build folder to make sharing it easy
system(f"7z a {destination_folder_name} {destination_folder_name}")