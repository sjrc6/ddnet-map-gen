use super::*;
use eyre::Result;
use ndarray::Array2;
use rand::Rng;
use pathfinding::prelude::Grid;

pub struct GoreGenerator;

fn place_rect(tiles: &mut ndarray::ArrayBase<ndarray::OwnedRepr<twmap::GameTile>, ndarray::Dim<[usize; 2]>>, 
    x: i64, y: i64, width: i64, height: i64, level_height: i64, level_width: i64,tile_type: u8){
    for i in x..x+width{
        for j in y..y+height{
            if i < 0 || j < 0 || i >= level_width || j >= level_height{
                continue;
            }
            tiles[(j as usize, i as usize)].id = tile_type;
        }
    }    
}
fn place_rect_center(tiles: &mut ndarray::ArrayBase<ndarray::OwnedRepr<twmap::GameTile>, ndarray::Dim<[usize; 2]>>, 
    x: i64, y: i64, width: i64, height: i64, level_height: i64, level_width: i64,tile_type: u8){

        place_rect(tiles,x-width/2,y-height/2,width,height,level_height,level_width,tile_type);
}
fn place_rect_border(tiles: &mut ndarray::ArrayBase<ndarray::OwnedRepr<twmap::GameTile>, ndarray::Dim<[usize; 2]>>, 
    x: i64, y: i64, width: i64, height: i64, level_height: i64, level_width: i64,border: i64, tile_type: u8){
    for i in x..x+width{
        for j in y..y+height{
            if i < 0 || j < 0 || i >= level_width || j >= level_height{
                continue;
            }
            if i >= x + border && j >= y + border && i < x+width-border && j < y+height-border{
                continue;
            }
            tiles[(j as usize, i as usize)].id = tile_type;
        }
    }    
}
fn place_rect_border_center(tiles: &mut ndarray::ArrayBase<ndarray::OwnedRepr<twmap::GameTile>, ndarray::Dim<[usize; 2]>>, 
    x: i64, y: i64, width: i64, height: i64, level_height: i64, level_width: i64,border: i64,tile_type: u8){

        place_rect_border(tiles,x-width/2,y-height/2,width,height,level_height,level_width,border,tile_type);
}

impl MapGenerator for GoreGenerator {

    fn generate<R: Rng + ?Sized>(rng: &mut R) -> Result<TwMap> {
        let mut map = create_initial_map()?;
        const HEIGHT: usize = 60;
        const WIDTH: usize = 350;
        const STARTBUFFER: usize = 10;
        const FINISHBUFFER: usize = 10;

        //TODO: write a loop that checks for hookables/freeze and adds them to the arrays required to draw sprites for those tiles (hookable_tiles, freeze_tiles)
        //that way you only need to set game tiles when constructing the map
        let mut tiles = Array2::from_shape_simple_fn((HEIGHT, WIDTH), || {
            GameTile::new(TILE_EMPTY, TileFlags::empty())
        });
        let mut front_tiles = Array2::from_shape_simple_fn((HEIGHT, WIDTH), || {
            GameTile::new(TILE_EMPTY, TileFlags::empty())
        });

        let mut hookable_tiles =
            Array2::from_shape_simple_fn((HEIGHT, WIDTH), || Tile::new(0, TileFlags::empty()));
        let mut freeze_tiles =
            Array2::from_shape_simple_fn((HEIGHT, WIDTH), || Tile::new(0, TileFlags::empty()));

        //Place Spawn
        tiles[(HEIGHT - 3, 5)].id = TILE_SPAWN;
        
        //Place start and finish tiles
        for y in 0..HEIGHT {
            front_tiles[(y, STARTBUFFER)].id = TILE_START;
            front_tiles[(y, WIDTH - FINISHBUFFER)].id = TILE_FINISH;
        }

        //Fill the middle with hookable
        for y in 0..HEIGHT{
            for x in STARTBUFFER+1..WIDTH - FINISHBUFFER{
                    tiles[(y, x)].id = TILE_HOOKABLE;
            }
        }

        //Place a bunch of hollow air rectangles
        for _i in 0..100{
            let x = rng.gen_range(STARTBUFFER..WIDTH - FINISHBUFFER) as i64;
            let y = rng.gen_range(0..HEIGHT) as i64;
            let width = rng.gen_range(6..30) as i64;
            let height = rng.gen_range(3..15) as i64;
            place_rect_border_center(&mut tiles, x, y, width, height, HEIGHT as i64, WIDTH as i64, 3, TILE_EMPTY);
            //height = std::cmp::max(height - 6, 0);
           // width = std::cmp::max(width - 6, 0);
            //place_rect_center(&mut tiles, x, y, width, height, HEIGHT as i64, WIDTH as i64, TILE_HOOKABLE);
        }

        //Place rectangles with air around them
        for _i in 0..30{
            let x = rng.gen_range(STARTBUFFER..WIDTH - FINISHBUFFER) as i64;
            let y = rng.gen_range(0..HEIGHT) as i64;
            let mut width = rng.gen_range(11..14) as i64;
            let mut height = rng.gen_range(17..28) as i64;
            place_rect_center(&mut tiles, x, y, width, height, HEIGHT as i64, WIDTH as i64, TILE_EMPTY);
            height = std::cmp::max(height - 7, 0);
            width = std::cmp::max(width - 7, 0);
            place_rect_center(&mut tiles, x, y, width, height, HEIGHT as i64, WIDTH as i64, TILE_HOOKABLE);
        }
        //Place horizontal rectangles with air around them
        for _i in 0..45{
            let x = rng.gen_range(STARTBUFFER..WIDTH - FINISHBUFFER) as i64;
            let y = rng.gen_range(0..HEIGHT) as i64;
            let mut width = rng.gen_range(17..28) as i64;
            let mut height = rng.gen_range(11..14) as i64;
            place_rect_center(&mut tiles, x, y, width, height, HEIGHT as i64, WIDTH as i64, TILE_EMPTY);
            height = std::cmp::max(height - 7, 0);
            width = std::cmp::max(width - 7, 0);
            place_rect_center(&mut tiles, x, y, width, height, HEIGHT as i64, WIDTH as i64, TILE_HOOKABLE);
        }
        //Place hookable around the border
        for y in 0..HEIGHT{
            for x in 0..WIDTH{
                if y == 0 || y == HEIGHT - 1 || x == 0 || x == WIDTH - 1{
                    tiles[(y, x)].id = TILE_HOOKABLE;
                    //hookable_tiles[(y, x)].id = 1;
                }
            }
        }

        //Cover Everything in freeze 
        for y in 1..HEIGHT-1{
            for x in STARTBUFFER..WIDTH-FINISHBUFFER{
                if tiles[(y, x)].id != TILE_EMPTY{
                    continue;
                }

                if 
                tiles[(y - 1, x + 1)].id == TILE_HOOKABLE ||
                tiles[(y - 1, x)].id == TILE_HOOKABLE ||
                tiles[(y - 1, x - 1)].id == TILE_HOOKABLE ||
                tiles[(y, x - 1)].id == TILE_HOOKABLE ||
                tiles[(y, x + 1)].id == TILE_HOOKABLE ||
                tiles[(y + 1, x + 1)].id == TILE_HOOKABLE ||
                tiles[(y + 1, x)].id == TILE_HOOKABLE ||
                tiles[(y + 1, x - 1)].id == TILE_HOOKABLE
                {
                    tiles[(y, x)].id = TILE_FREEZE;
                }
            }
        }




        let game_layer = GameLayer {
            tiles: CompressedData::Loaded(tiles),
        };

        let front_layer = FrontLayer {
            tiles: CompressedData::Loaded(front_tiles),
        };

        let mut hook_tiles_layer = TilesLayer::new((HEIGHT, WIDTH));
        hook_tiles_layer.image = Some(0);
        hook_tiles_layer.tiles = CompressedData::Loaded(hookable_tiles);

        let mut freeze_tiles_layer = TilesLayer::new((HEIGHT, WIDTH));
        freeze_tiles_layer.image = Some(1);
        freeze_tiles_layer.tiles = CompressedData::Loaded(freeze_tiles);
        freeze_tiles_layer.color = Color {
            r: 0,
            g: 0,
            b: 0,
            a: 200,
        };

        let mut physics = Group::physics();
        physics.layers.push(Layer::Game(game_layer));
        physics.layers.push(Layer::Front(front_layer));
        physics.layers.push(Layer::Tiles(hook_tiles_layer));
        physics.layers.push(Layer::Tiles(freeze_tiles_layer));

        map.groups.push(quads_sky());
        map.groups.push(physics);

        Ok(map)
    }
}
