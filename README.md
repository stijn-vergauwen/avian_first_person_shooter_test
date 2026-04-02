TODO: add an image showcasing the entire project at the top of the readme
# Avian first person shooter test
TODO: link to bevy & avian
TODO: link to itch.io page
A prototype built in the [Bevy] game engine and using [Avian3D] for physics. To try out a bunch of ideas I had around first-person shooter games.

Available on [itch.io], playable in browser (if it is)

## About the prototype
what is the purpose of the project? what can you do as player? how do i want to build on top of this in future projects?
TODO: add sections for each of these questions, using the draft below as reference

About purpose:
- I feel like shooting games could get cool new kinds of gameplay if they leaned more into physics. In this project I wanted to make the basics of character movement and weapon handling as a foundation and experiment with weapon configuration in-game.
- I want to treat everything as physics objects, experimented with how that affects gun handling, I like the idea of being able to hold your weapon in any orientation but current result is not that accurate and not that easy to use


About what you can do:
- You can move using the wasd keys and jump using spacebar
- You can grab most smaller objects using 'e', the small circle in the center of the screen will turn cyan when you can grab something
- When holding an object, you can press 'T' to enter 'inspector mode', here you can grab and rotate the object and configure weapons
- When holding a weapon, you can shoot with left click and aim with right click.


About future project ideas:
- One game idea I've had for a while is a sort of sci-fi, first-person parkour & shooting game. Where you improvise your way through indoor rooms and corridors, killing enemies and collecting some sort of items. I think it would be cool to combine this kind of fast-paced running and shooting with procedurally generated worlds and roguelite meta-progression. This prototype gave me a better idea of possible issues or limitations you might have when simulating more parts of the gameplay through a physics engine, like the weapon lagging behind the player movement because it's not directly stuck to the player.
- I'm curious about how a 'gun creator' could work, an in-game way to design guns and configure how they work. I've layed some groundwork for this by loading the weapons in from configuration files and adding hot-reloading, but editing all this and adding new files in-game seemed out of the scope of this project.

## About the code
implementation details, stuff i found interesting and stuff i learned
TODO: write out a list of things using the topics below as reference


- part about the character controller, short explanation of the movement
- part about object grabbing, how I used pd controllers to link the held object to the player using different offsets
- part about the WeaponConfig asset, loading and saving and hot-reloading
- part about the SliderForWeaponConfig component in inspector_mode_ui, how this combines with events & observers and Bevy's feathers framework
    - this file is quite messy sorry about that, lot of experimenting

In Project notes.md you can see more details of different issues I had and things I learned during development.

## Credits / licencing
TODO: add links to credited assets

- Skybox:
- Grid / prototype texture: kenney

The rest I made myself! Using Blender and Krita. Feel free to do whatever you want with these.