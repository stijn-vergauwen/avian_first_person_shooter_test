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
- Add rotation
- Check if player input can be handled using observers, idk if Bevy sends observer events for this
- Have character entity hierarchy with root entity, neck / head entity, meshes and colliders as child entities
- Character camera at eye position, moving forward when looking down (looking down in front of you) and backwards when up 
- Add picking up of objects
- Allow items to configure in what orientation they should be held
- Add player jump
- Load in a Blender scene with test objects to move around
- Model a basic rifle
- Able to pick up weapon
- Able to shoot weapon
- Weapon shot calculates where it hits
- Put visual indication at bullet hitpoint


## Thoughts For Future Projects
