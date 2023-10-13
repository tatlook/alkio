Alkio
===

It would be a open-source game engine later, if I have good luck and skills.
Now it is just a strange green window. I can't do any technical things, so I just use librarys.
Like SFML, Rhai.

## Why not Another Engine
Unity is now a very bad engine. It is too heavy and it costs. Unity's community is nearly broken
after a new contract you know. Godot is a good engine, it is open, free and light. 
But it is for artics, not for developers, like inheritance over combining.

## Idea
Godot uses Scenes, which makes it light. But everything like light, collision, spirit are in
same tree, which makes non-sense.
And Unity, as I remember, uses GameObject and it contains diffrent things, which is smart.
So Alkio (in my hope) will combine both Unity's and Godot's advantages, let we see how it will be done.

### Multiple Trees
There will be not only one game tree, where contains everything. There will be more than one
unique trees in a game. Like render tree, sound tree, logic tree, network tree etc.
When developer wants to do something with collisons, he/she just check into a collison tree.
In the collison tree there is nothing else than collison shapes. Same thing in other scenario.

### Node Peers
I got idea from Godot's peer system. A render node may have peers in sound tree and logic tree.
First, we create an enemy instance in logic tree, then it spawns in the render tree and show in
the screen, spawns in the sound tree and make deploy sound. You can always know what peers a
node have.

### Scene
Just copy the scene system from Godot.

## Development
I have actually no idea how to make an game engine from scratch. I'm also a begginer in Rust.
Do you think this is a good idea? If you say yes, then wellcome.

### Current State
A stub I say. And only I sometimes do some work in this project.

## License
GPLv3 (See LICENSE)
