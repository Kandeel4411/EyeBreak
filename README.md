# EyeBreak

[![GitHub license](https://img.shields.io/github/license/Naereen/StrapDown.js.svg)](https://github.com/Kandeel4411/EyeBreak/blob/master/LICENSE)

A simple reminder program that aims to enforce the 20/20/20 eye rule to help protect your eyes.  

***Every 20 minutes, look at something 20ft away for 20 seconds.***

## Installation

Windows and Linux are the only supported platforms currently. Go to [releases](https://github.com/kandeel4411/eyebreak/releases) page and download the zip file that corresponds to your platform.  
  
Run `eyebreak` and voila!  
*It may take a few seconds on Windows to start up.*

## Building from source

Requirements: Python, Rust/Cargo installed.  
Clone the repositery then open the terminal, navigate to it and run the following command:
```
cargo build --release
```
This should generate an executable file in the `target/release/` directory.  
In the same terminal session, install the project's python dependencies by running the following command:
```
pip install -r requirements.txt
```
Then to generate an executable file from the python script, run the following command:
```
pyinstaller alert.pyw --onefile
```
This should generate an executable file in the `dist` directory, copy said executable to the same directory where you stored the cargo generated executable and run the cargo executable and you should be good to go!
