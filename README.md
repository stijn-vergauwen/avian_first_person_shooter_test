![Project thumbnail image](<screenshots/Falling tower thumbnail.png>)
# Avian first person shooter test
A prototype built in the [Bevy](https://bevy.org/) game engine and using [Avian3D](https://github.com/avianphysics/avian) for physics. To try out a bunch of ideas I had around first-person shooter games.

Available [here on itch.io](https://kaihadev.itch.io/first-person-shooter-prototype), playable in browser!

To run this source code, you just need Rust installed.

## About the project

### What you can do in-game
- Move using the WASD keys and jump using spacebar.
- Grab most small objects using E, the small circle in the center of the screen will turn cyan when you can grab something.
- When holding an object, press T to enter 'inspector mode'. Here you can grab and rotate objects and configure weapons.
- When holding a weapon, shoot with left click and aim with right click.

### Motivation
I feel like shooting games could get cool new kinds of gameplay if they leaned more into physics. I'm looking for ways that you could give the player underlying systems that they can then experiment with themselves, instead of designing the game to be played a specific 'correct' way.  
I had fun playing around with this project however it looks like this quickly results in more of a 'simulator' than a game so I'm not yet sure how to balance that.

I for example made the weapon it's own rigidbody with weight, even when the player holds it. This means that you automatically get slowed down by carrying a heavy weapon as that's just how the physics get calculated.  
Any held object can be rotated, including guns, I like the idea of freely choosing in what orientation you hold your weapon, you could do things like shoot behind a low wall or obstacle, or around a corner that might be dangerous, without needing to stand in a vulnerable position.  
If implemented well I think these kinds of things add new skills to learn and tactics that feel more realistic. But again the current result feels more like a sim not a game, and there's stuff like movement delay when everything gets calculated through physics forces, which makes it less responsive and less fun.

### Future project ideas
- One game idea I've had for a while is a sort of sci-fi, first-person parkour & shooting game. Where you improvise your way through indoor rooms and corridors, killing enemies and collecting some sort of items. I think it would be cool to combine this kind of fast-paced running and shooting with procedurally generated worlds and roguelite meta-progression. This prototype gave me a better idea of possible issues or limitations you might have when simulating more parts of the gameplay through a physics engine, like the weapon lagging behind the player movement because it's not directly stuck to the player.
- I'm curious about how a 'gun creator' could work, an in-game way to design guns and configure how they work. I've layed some groundwork for this by loading the weapons in from configuration files and adding hot-reloading, but extending this further seemed out of the scope of this project.

## About the code
Implementation details, stuff I found interesting and stuff I learned.

- Grabbed objects follow the player using PID controller logic, one for position and one for rotation.
    - Each controller takes in the current position and velocity of whatever value it calculates, and returns the acceleration that should be applied in the next physics step. This way the physics engine simulates all the actual state and forces, while the PID controller calculates how to move towards its target using the parameters of that specific controller, allowing you to add 'character' to the animation while staying within the constraints of the physical environment.
    - One example of this is the 'update_grabbed_object_position' function in 'object_movement.rs'. The parameters of that controller are set when inserting the GrabbedObject resource.
    - took me a while to figure out how to connect PID controllers with the physics engine like this, and I'm pretty sure my code can be simplified much more. The most difficult part was getting this to work with Quaternions, how I understand it now is that Quaternions should only be used to describe a single rotation, changes in rotation (velocity & acceleration) should be calculated using a different format, Avian3D seems to use Scaled axes for this so that's what I ended up using in the Quaternion variant of my PID controller.

- The weapons are spawned from a folder with weapon config files, the idea was that these can be freely added and modified to add new weapons to the game.
    - The config file contains a path to the weapon model, so adding new models is also quite easy.
    - Further steps would be to make every field configurable in-game, be able to create new files or make copies, and a way to select the 3D model in-game.

- The inspector mode ui (inspector_mode_ui.rs) contains a lot of sliders for configuring the weapon, I made the SliderForWeaponConfig component as a way to make 'something that edits the weapon config' reusable.
    - SliderForWeaponConfig contains 2 closures, a getter and a setter for a WeaponConfig value.
        - The getter gets used to update the SliderValue whenever WeaponConfig changes.
        - The setter gets used to udpate WeaponConfig whenever the SliderValue changes.
    - This was my first time using closures to store logic inside components, while I still find it a bit confusing it also seems really powerful! Especially for UI and combined with events & observers like this. The code is quite messy here sorry about that, I was experimenting.

In Project notes.md there's also a list of different issues I had and things I learned during development.

## Credits / licencing
- Skybox: [Cozy Voxel Skybox (free) by ThomasGamboa](https://thomasgamboa.itch.io/cozy-voxel-skybox-free-voxel-style-textures) on itch.io
- Grid / prototype textures: [Prototype Textures by Kenney](https://kenney.nl/assets/prototype-textures) thanks Kenney!

The other assets I made myself! Using Blender and Krita. Feel free to do whatever you want with these, they're not that good haha.