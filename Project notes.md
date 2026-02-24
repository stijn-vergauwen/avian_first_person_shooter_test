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


## Backlog
- make standing target that freely turns horizontally
- make standing target that falls when shot, able to reset itself on event (revolute joint for pivot point? separate timer component that fires reset event after duration)
- add grid texture to world objects (already have sprites from Kenney)
- add bullet visual, traveling bullet should move super fast but be slightly visible
- remove firing direction gizmo
- eject casing when shooting
- bullet impact decal
- bullet impact particle effect
- setup saving & loading weapon config from ron file
- on startup, loop through files in weapon folder, load the data and spawn a new gun entity + its model for each file
- store weapon weight and recoil in config file
- add UI to inspector to configure shot origin position (at barrel end)
    - start shot raycast from origin
    - start muzzle flash at origin
    - apply shot recoil from origin
- add UI to inspector to configure aim down sight position
    - draw ray to visualize how player camera will be lined up
    - align player camera to ads position on ads (different per gun, should replace 'aim_down_sight' AnchorOffset)
- add UI to inspector to configure empty casing ejection position & direction
    - add config for spin along each axis, and how randomized it should be
    - eject casing according to config

Extra:
- make flying disc shooting area
- add toggle to shoot discs every few seconds
- shatter disc on hit


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