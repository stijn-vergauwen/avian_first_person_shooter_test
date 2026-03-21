# Project Notes
Info and documentation for development.


## Project Goals
- Player should control a first-person character, able to walk / run around in the world.
- Should be able to pick up weapons, look at them, aim and shoot.
- Should have an in-game way to configure weapons, their behaviour and stats and stuff like aim position.
- Shooting should feel satisfying and responsive, with recoil, sound, and some visual effects like muzzle flashes and hit marks.
- The in-game world should have interactable elements to test out weapons, like shooting targets and obstacles to move around.


## Code Refactoring & Improvements

## Backlog
- remake shooting range area
    - spawn shooting targets in shooting range
    - make button model in Blender, spawn this in & make interactive to reset targets
- use grid / prototype texture on objects, with consistent scale
    - on ground
    - on yellow blocks
    - idk if it looks good on everything
- remake gym area
    - add blocks with their heights displayed to test jumping
    - add ramps with their slope displayed to test movement on slopes
    - add gaps of different widths to test how far the player can jump
    - make parkour course where you go over ramps and around corners and have a few spots where you can shoot targets
- bullet impact particle effect, dependent on hit angle & color of hit object
    - spawn small cubes on hit
    - start direction and speed should be dependent on bullet hit angle and force
    - despawn after some duration
    - give cubes the same color as the object that was hit
    - mix in a few bright orange / yellow cubes as a 'spark' effect
- add skybox


Extra:
- add tabs to inspector menu
    - shot origin tab
    - aim down sight tab
    - shell ejection tab
    - only show config elements of active tab
    - only draw gizmos of active tab
- add view bobbing, move player camera, maybe small downwards force based on movement speed and smooth return using pd controller
- sound effects for shooting, bullet hits, walking, object impacts
- make flying disc shooting area
- add toggle to shoot discs every few seconds
- shatter disc on hit


## Thoughts & Info For Future Projects
- Reworks for PID controller:
    - make single interface, allows splitting to more specific contrtoller implementations (quaternions, without initial response)
        - I'm thinking a common PidController trait, maybe with a generic T for 'position' type and a associated type for velocity & acceleration that defaults to T
        - then multiple struct variants that implement this trait, a default one that copies what I already have, one for rotation that uses quaternion & scaled axis types, and one that leaves out initial response param so I can remove some of the calculations.
    - make generic components for controlling an entities position or rotation, both directly and through physics engine.
        - Idea is that you spawn in this controller component with it's parameters, and then you only have to interact with it to set the target, updating is handled within the plugin
- I think I just realised how entity observers work:
    - To spawn a global observer, add the observer in a Plugin (on app, together with adding systems), it will react to all events & entity events of the event type that is declared in the function's parameters (e.g. `On<Pointer<Drag>>`)
    - To spawn an observer that only reacts to specific entities, add the observer in a system using Commands. Either create an Observer component and add entities to it, or use the `observe` function on EntityCommands. This observer will only react to events of the given event type when the target entity is one of the entities stored in the list of that observer.
    - Global observers can also be manually spawned by spawning the Observer component without any antities added to it. But usually this will be done with the `.add_observer` function in a Plugin, since that is shorthand for spawning the component.
    - See the 'reset to default orientation' button logic, that's the first time I made an observer for a specific entity.
- Transparent textures being displayed as dark areas:
    - I had this issue with the bullet impact decal
    - Reason is likely because 'AphaMode' value in StandardMaterial defaults to 'Opaque' which ignores alpha values, I set it to 'Mask' to fix it in this case.
- `Transform` and `GlobalTransform` should both be kept up-to-date with physics by Avian, there should never be a need to consider using Position or Rotation instead of transforms.
    - Avian3D runs Bevy's transform propagation in FixedUpdate.
        - under `impl Plugin for PhysicsTransformPlugin` in the 'physics_transform' module you can see Bevy's propagate systems are added to it's schedule.
        - `self.schedule` defaults to FixedPostUpdate, and the run condition 'config.propagate_before_physics' defaults to true.
    - I also now see that Transforms get updated immediately after physics calculations, so I should never need to use Position instead of Transform to get more up-to-date values. (as the avian docs already told me, but now I see the code..).
    - I was under the impression that when I switched from Transform to Position that the result did look more correct, but turns out this shouldn't be te case, need to verify again. Just using Transform & GlobalTransform everywhere does sound a lot simpler.