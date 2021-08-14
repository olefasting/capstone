# Capstone - Twilight of the Archons

This is an action/run 'n' gun RPG, created in Rust using [macroquad](https://github.com/not-fl3/macroquad).
The future holds huge boss fights, and a mind-nuking story line, as you progress through the world, and your own mind, to battle yourself and the archons to break free of samsara and the materialist dystopia of our not too distant future.

## Contributing

If you want to contribute, pull requests are welcome. If you want to partake in the project, we are also open to this. The prototype currently use placehoder graphics leeched from ich.io so we would love to get a good pixel artist on board. Coders are also welcome to alleviate, but we more or less have that role fulfilled already. Send me a mail, on [simon@magus.no](mailto:simon@magus.no), if you are interested.

## License

The game is unlicensed because it has a planned commercial release, so  the game in its entirety, the IP (as it is fleshed out in the future), is ours and not to be copied.
Feel free to browse the code, however, and use anything you find for your own project(s). This means that you can copy and reuse implementation methods and bits of code, for commercial and non-commercial purposes, but you can't compile the game, with or without modifications, to distribute it.
You can, of course, compile the game and try it out, as long as it falls in under fair-use.

## Features

This is a work in progress but current features include (not an exhaustive list as we are adding features at a high pace):

- Composable actors, from code or by JSON [assets/actors.json](https://github.com/olefasting/capstone/blob/master/assets/actors.json)
- RPG mechanics, such as character stats and various abilities, currently contained in items, but a spell and feat system is in the works
- Composable items, from code of by JSON [assets/items.json](https://github.com/olefasting/capstone/blob/master/assets/items.json), character inventory and more
- Dynamic resource loading, so textures can be added by editing [assets/resources.json](https://github.com/olefasting/capstone/blob/master/assets/resources.json) and referenced by `texture_id` in actors and items, both in-code and in the corresponding json-files
- Tiled maps (to be replaced by a proprietary format, as soon as we create an in-game editor)

## Twilight of The Archons

The name 'Twilight of the Archons' has been inspired by the [documentary](https://www.youtube.com/watch?v=HsYTsdBCBdE) by the same name, made by Robert Bonomo of [Cactus Land Productions](http://www.thecactusland.com/). Though the documentary focus more on the worldly aspects of our current predicament and this game more on the spiritual aspects; I believe the title was such an excellent one that I sought Robert's blessing to steal it.
Blessing has been given and, even though he is not affiliated with the project in any way, I would like to give credit to him and his awesome work, here.

Copyright 2021 Ole A. Sjo Fasting and [Magus Interactive](https://magus.no)

UNLICENSED
