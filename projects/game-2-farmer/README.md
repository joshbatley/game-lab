# Farmer

Throwing some random shit down, some might be duped but ideas of what i want
TODO: 
- [ ] Start screens along with things like pausing the game
- [ ] Timer on the plants, as well as the pause time
- [ ] Character Controller
  - [ ] Stamina 
  - [ ] Health?
- [ ] Debug tools
- [ ] Map loaders - Randomised? From a file or summit
- [ ] Inventory 
- [ ] Game states
- [ ] Save states


```
// Just external files not how it is in system
Map {
    tileset_meta: TilesetMeta
    layers: []Layers
    entity_tags: []string  // tag for entity to use
}

Layer_type {    
    // Complete grid for sprites 
    // 0 - X, -1 is none selected
    layer, = 0

    // numbered grids for meta, e.g. collisions
    // 0 is nothing, 1 >= is meta info decieded in engine 
    int_grid = 1

    // Sparse collections of entities 
    entity, = 2
}

TilesetMeta {
    out_size: int
    width: int,
    height: int
    background: color
    tilemaps: []string //tag for sprite map think file name
}

Layer {
    data: []ints, // Entity index id or sprite location or int
    type: enum Layer_type
    tileset_id: string // tilemap id for sprite
    name: string // meta name used for engine, e.g. collision layer
}

Entity {
    id: int,
    tag: string,
    ...options
}
```