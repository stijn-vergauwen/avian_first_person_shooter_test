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
- Add external camera that looks towards the player, for debugging, place this view in the corner of screen
- Make the test weapon a child of ToolAnchor, maybe on 'e' pressed
- Weapon should be visible while holding it, and point towards what player is aiming at with slight position offset
- Add object to shoot at
- Put visual indication at bullet hitpoint (e.g. small black sphere)
- Add rotation
- Check if player input can be handled using observers, idk if Bevy sends observer events for this
- Have character entity hierarchy with root entity, neck / head entity, meshes and colliders as child entities
- Character camera at eye position, moving forward when looking down (looking down in front of you) and backwards when up 
- Add picking up of objects when looking at them
- Add test object that can be picked up but isn't a weapon (like a radio)
- Allow items to configure in what orientation they should be held
- Able to pick up & drop objects
- Able to shoot weapon that is currently being held
- Add player jump


## Thoughts For Future Projects
