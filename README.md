# 💣Minesweeper💣

## For what?
<p>
This repo exists to help people who want to commit into gamedev on rust.<br>
It is not rust native game engine as I'm using godot-rust here, but i didn't find any better option.<br>
</p>
<p>
godot-rust provides really bare minimum for creating something and <br>
a lot of things are not documented at all. I don't blame them.<br>
At the end of the day it's community driven and I'm really happy to be part of it.<br>
</p>
<p>
For this reason I created this repo. There are a lot of comments on things that were confusing to me or lacking docs.<br>
That should be enough to understand how this game works from start to finish without any problems.<br>
Project itself wasn't really hard, but I learned a lot coding it.<br>
Hope it will help you as well😊
</p>

## Features
### Flags system
<p>
Based mechanic of minesweeper. Implemented like in most games of this type
</p>

### Scaling formula for grid generation
<p>
Grid size and mines amount are scaled with difficulty level where 0 - easy, 1 - medium, 2 - hard.<br>
For now it doesn't matter i'd say, but if you want to add more levels, it would be handy
</p>

### Score system
<p>
Exponential-based formula to earn score based on time spent.
</p>

## How to play?
<p>
You need to build dynamic library of code I provided with:
`cargo build`
<br>
And after this you can preview or export the game with godot

</p>

## Todo
- fix UI (probably will never happen cuz i suck in UI)
