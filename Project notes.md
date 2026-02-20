# Project Notes
Info and documentation for development.


## Project Goals
- Player should control a first-person character, able to walk / run around in the world.
- Should be able to pick up weapons, look at them, aim and shoot.
- Should have an in-game way to configure weapons, their behaviour and stats and stuff like aim position.
- Shooting should feel satisfying and responsive, with recoil, sound, and some visual effects like muzzle flashes and hit marks.
- The in-game world should have interactable elements to test out weapons, like shooting targets and obstacles to move around.


## Code Refactoring & Improvements
- weapon should stay stable when aiming down sight and moving.
    - maybe use separate pd controller parameters while in ads? Will probably still be some movement lag but that's probably fine
    - maybe make it a child entity of the player while ads? I don't think this is a good solution, could cause problems with collisions or snapping, better to keep things in the physics sim
- weapon collides with player head when aiming down sight. Could temporarily filter out collisions between weapon & head while ads?


## Backlog
- bullet impact decal
- bullet impact particle effect


## Thoughts & Info For Future Projects
- Reworks for PID controller:
    - make single interface, allows splitting to more specific contrtoller implementations (quaternions, without initial response, etc)
        - I'm thinking a common PidController trait, maybe with a generic T for 'position' type and a associated type for velocity & acceleration that defaults to T
        - then multiple struct variants that implement this trait, a default one that copies what I already have, one for rotation that uses quaternion & scaled axis types, and one that leaves out initial response param so I can remove some of the calculations.
    - make generic components for controlling an entities position or rotation, both directly and through physics engine.
        - Idea is that you spawn in this controller component with it's parameters, and then you only have to interact with it to set the target, updating is handled within the plugin
- I think I just realised how entity observers work:
    - To spawn a global observer, add the observer in a Plugin (on app, together with adding systems), it will react to all events & entity events of the event type that is declared in the function's parameters (e.g. `On<Pointer<Drag>>`)
    - To spawn an observer that only reacts to specific entities, add the observer in a system using Commands. Either create an Observer component and add entities to it, or use the `observe` function on EntityCommands. This observer will only react to events of the given event type when the target entity is one of the entities stored in the list of that observer.
    - Global observers can also be manually spawned by spawning the Observer component without any antities added to it. But usually this will be done with the `.add_observer` function in a Plugin, since that is shorthand for spawning the component.
    - See the 'reset to default orientation' button logic, that's the first time I made an observer for a specific entity.