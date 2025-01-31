# Group13
The application provides a useful way to avoid data loss due to a non-working monitor, while you are still able to move your mouse. When a critical situation happens (or if you just want to back up some data), you can draw an easy gesture with your mouse—tracing a rectangle following the monitor edges—and then confirm with a swipe from left to right.

## Features
- You can choose which **source/destination** directories to use for copying data;
- After the first execution, the app will run automatically at every **system startup**, so you do not have to worry about manually starting it;
- The app runs in the **background**, ensuring it does not interfere with your workspace.
- Once started the app, if the main GUI is closed, you can open it by pressing Ctrl+T.

## More Features!
- **Usability**: The app is easy to use and control, thanks to a simple GUI and an easy gesture;
- **Security**: The app operates without any data leaks or malicious software;
- **Availability**: The app will always be on, as long as your PC is turned on;
- **Portability**: The app works on Windows, MacOS, and Linux.


## Installation
The app is portable, so you just need to use the appropriate executable, as listed below, according to your operating system.

### Windows
- **Execution**: run `release\windows\Group13.exe`;
- For any **modifications** to the code, uninstall the program, then run the script `win_build_release.bat` to rebuild the project;
- **Uninstall** the software: run `release\windows\uninstall.exe`.

### MacOS
- **Requirements**: `osascript` should be installed by default in the OS, if it doesn't work, check the installation.
- **Execution**: it is enough to run `release/macos/Group13`, since all files are already signed; if any troubles:
  - run `release/macos/uninstall`;
  - run `release/macos/spawn_gui`;
  - run `release/macos/spawn_popup`;
  - run `release/macos/uninstall`;
  - run `release/macos/Group13`;
- For any **modifications** to the code, uninstall the program, then run the script `macos_build_release.sh` to rebuild the project;
- **Uninstall** the software: run `release/macos/uninstall`;

### Linux
- **Requirement**: `pulseadio` need to be installed to run audio; you can install it by: 
  ```shell
  sudo apt-get update
  sudo apt-get install pulseaudio
  ```
- **Execution**: run `release/linux/Group13`;
- For any **modifications** to the code, uninstall the program, then run the script `lin_build_release.sh` to rebuild the project;
- **Uninstall** the software: run `release/linux/uninstall`.

# Screenshots

![GUI](./images/Backup_GUI.png)\
*GUI: source, destination and file type definition*

![GUI](./images/Backup_instruction_GUI.png)\
*GUI: instructions to use it*
