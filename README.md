[![Review Assignment Due Date](https://classroom.github.com/assets/deadline-readme-button-22041afd0340ce965d47ae6ef1cefeee28c7c493a6346c4f15d667ab976d596c.svg)](https://classroom.github.com/a/c2Cd-Xpe)
# Weather Visualizer

## Description

This application will be a command line app that makes an API call to give the user the weather in their area. Additionally, it will provide a visualization of the weather. App will use the OpenWeatherMap API to find the weather for the user's area, and will use Nannou to create a visualization of the weather. Specific cities will have special visualization, such as of important landmarks. The visualization will match up with the weather report, so a forecast of sunny skies will result in an image with a sunny sky. The program will automatically open a new window on the user's screen where the visualization will be.

## Installation

In order to install the project, the user should first clone the repository into a folder of their choosing.

The user will also need to obtain an API key from the OpenWeather API. You can sign up for the OpenWeather API [here](https://home.openweathermap.org/users/sign_up) to obtain an API key. After receiving the key, it should be placed in a .env file with the name API_KEY. 

After opening the project in an IDE, the user should run the command "cargo run" to start the program.


## How to use

After starting the program with "cargo run", the user will see prompts in the terminal window.

First, the user will see a welcome statement with some information about the application. Then, they will be asked to enter either 'y' or 'n'. If the user enters 'n', the program will quit. 

If the user enters 'y', the user will be prompted to enter the name of a city they would like to visualize the weather for.

After this, the user can choose to continue the simulation for another city by entering 'w' into the terminal or exit the program by entering 'x' into the simulation. If the user enters 'w' into the terminal, they will be reprompted for a new city and asked again if they would like to continue the simulation.
