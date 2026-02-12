# Project Notes
Info and documentation for development.


## Project Goals
- Player should control a first-person character, able to walk / run around in the world.
- Should be able to pick up weapons, look at them, aim and shoot.
- Should have an in-game way to configure weapons, their behaviour and stats and stuff like aim position.
- Shooting should feel satisfying and responsive, with recoil, sound, and some visual effects like muzzle flashes and hit marks.
- The in-game world should have interactable elements to test out weapons, like shooting targets and obstacles to move around.


## Code Refactoring & Improvements
- rework grabbed_object logic to have multiple 'anchor positions' relative to the player and switch between those
    - put GrabbedObject component on the player root entity
    - have position offsets for 'in front of player', 'in front of player head', and 'in primary hand'
    - calculate and store Isometry3D's for each position offset, each update
    - store 'preffered grab position' in grabbable objects
    - in inspector mode, move the object to 'in front of player head'
- make events for grabbing and dropping objects
- make events for entering and leaving 'inspector mode'
- check if interaction_target module can be replaced with bevy_picker (not sure because interaction_target casts ray along entity rotation instead of mouse position)


## Backlog
- refactor all grabbed object observers to separate functions instead of inline callbacks
- button to reset object to default orientation (value inside orientation component or quat::identity if component not added)
- when leaving inspector, keep the relative rotation of the object to the player as it had in the inspector
- Should now be able to pick up objects, inspect them, rotate them around, carry them in any orientation, and reset them to their default rotation anytime


## Thoughts For Future Projects
- Reworks for PID controller:
    - make single interface, allows splitting to more specific contrtoller implementations (quaternions, without initial response, etc)
        - I'm thinking a common PidController trait, maybe with a generic T for 'position' type and a associated type for velocity & acceleration that defaults to T
        - then multiple struct variants that implement this trait, a default one that copies what I already have, one for rotation that uses quaternion & scaled axis types, and one that leaves out initial response param so I can remove some of the calculations.
    - make generic components for controlling an entities position or rotation, both directly and through physics engine.
        - Idea is that you spawn in this controller component with it's parameters, and then you only have to interact with it to set the target, updating is handled within the plugin