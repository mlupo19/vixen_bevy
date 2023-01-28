use noise::{NoiseFn, Fbm, Perlin, MultiFractal, Curve, Min, Clamp, Cache, ScaleBias};


pub fn simple_noise(seed: u32) -> impl NoiseFn<f64, 3> {
    let base_continent_def_fb0 = Fbm::<Perlin>::new(seed)
        .set_frequency(CONTINENT_FREQUENCY)
        .set_persistence(0.5)
        .set_lacunarity(CONTINENT_LACUNARITY)
        .set_octaves(6);

        // 2: [Continent-with-ranges module]: Next, a curve module modifies the
    // output value from the continent module so that very high values appear
    // near sea level. This defines the positions of the mountain ranges.
    let base_continent_def_cu = Curve::new(base_continent_def_fb0)
        .add_control_point(-2.0000 + SEA_LEVEL, -1.625 + SEA_LEVEL)
        .add_control_point(-1.0000 + SEA_LEVEL, -1.375 + SEA_LEVEL)
        .add_control_point(0.0000 + SEA_LEVEL, -0.375 + SEA_LEVEL)
        .add_control_point(0.0625 + SEA_LEVEL, 0.125 + SEA_LEVEL)
        .add_control_point(0.1250 + SEA_LEVEL, 0.250 + SEA_LEVEL)
        .add_control_point(0.2500 + SEA_LEVEL, 1.000 + SEA_LEVEL)
        .add_control_point(0.5000 + SEA_LEVEL, 0.250 + SEA_LEVEL)
        .add_control_point(0.7500 + SEA_LEVEL, 0.250 + SEA_LEVEL)
        .add_control_point(1.0000 + SEA_LEVEL, 0.500 + SEA_LEVEL)
        .add_control_point(2.0000 + SEA_LEVEL, 0.500 + SEA_LEVEL);

    // 3: [Carver module]: This higher-frequency BasicMulti module will be
    // used by subsequent noise functions to carve out chunks from the
    // mountain ranges within the continent-with-ranges module so that the
    // mountain ranges will not be completely impassible.
    let base_continent_def_fb1 = Fbm::<Perlin>::new(seed + 1)
        .set_frequency(CONTINENT_FREQUENCY * 4.34375)
        .set_persistence(0.5)
        .set_lacunarity(CONTINENT_LACUNARITY)
        .set_octaves(8);

    // 4: [Scaled-carver module]: This scale/bias module scales the output
    // value from the carver module such that it is usually near 1.0. This
    // is required for step 5.
    let base_continent_def_sb = ScaleBias::new(base_continent_def_fb1)
        .set_scale(0.375)
        .set_bias(0.625);

    // 5: [Carved-continent module]: This minimum-value module carves out
    // chunks from the continent-with-ranges module. it does this by ensuring
    // that only the minimum of the output values from the scaled-carver
    // module and the continent-with-ranges module contributes to the output
    // value of this subgroup. Most of the time, the minimum value module will
    // select the output value from the continent-with-ranges module since the
    // output value from the scaled-carver is usually near 1.0. Occasionally,
    // the output from the scaled-carver module will be less than the output
    // value from the continent-with-ranges module, so in this case, the output
    // value from the scaled-carver module is selected.
    let base_continent_def_mi = Min::new(base_continent_def_sb, base_continent_def_cu);

    // 6: [Clamped-continent module]: Finally, a clamp module modifies the
    // carved continent module to ensure that the output value of this subgroup
    // is between -1.0 and 1.0.
    let base_continent_def_cl = Clamp::new(base_continent_def_mi).set_bounds(-1.0, 1.0);

    // 7: [Base-continent-definition subgroup]: Caches the output value from
    // the clamped-continent module.
    let base_continent_def = Cache::new(base_continent_def_cl);
    base_continent_def
}

/// Frequency of the planet's continents. Higher frequency produces
/// smaller, more numerous continents. This value is measured in radians.
const CONTINENT_FREQUENCY: f64 = 1.0;

/// Lacunarity of the planet's continents. Changing this value produces
/// slightly different continents. For the best results, this value should
/// be random, but close to 2.0.
const CONTINENT_LACUNARITY: f64 = 2.208984375;

/// Lacunarity of the planet's mountains. Changing the value produces
/// slightly different mountains. For the best results, this value should
/// be random, but close to 2.0.
const MOUNTAIN_LACUNARITY: f64 = 2.142578125;

/// Lacunarity of the planet's hills. Changing this value produces
/// slightly different hills. For the best results, this value should be
/// random, but close to 2.0.
const HILLS_LACUNARITY: f64 = 2.162109375;

/// Lacunarity of the planet's plains. Changing this value produces
/// slightly different plains. For the best results, this value should be
/// random, but close to 2.0.
const PLAINS_LACUNARITY: f64 = 2.314453125;

/// Lacunarity of the planet's badlands. Changing this value produces
/// slightly different badlands. For the best results, this value should
/// be random, but close to 2.0.
const BADLANDS_LACUNARITY: f64 = 2.212890625;

/// Specifies the "twistiness" of the mountains.
const MOUNTAINS_TWIST: f64 = 1.0;

/// Specifies the "twistiness" of the hills.
const HILLS_TWIST: f64 = 1.0;

/// Specifies the "twistiness" of the badlands.
const BADLANDS_TWIST: f64 = 1.0;

/// Specifies the planet's sea level. This value must be between -1.0
/// (minimum planet elevation) and +1.0 (maximum planet elevation).
const SEA_LEVEL: f64 = 0.0;

/// Specifies the level on the planet in which continental shelves appear.
/// This value must be between -1.0 (minimum planet elevation) and +1.0
/// (maximum planet elevation), and must be less than `SEA_LEVEL`.
const SHELF_LEVEL: f64 = -0.375;

/// Determines the amount of mountainous terrain that appears on the
/// planet. Values range from 0.0 (no mountains) to 1.0 (all terrain is
/// covered in mountains). Mountains terrain will overlap hilly terrain.
/// Because the badlands terrain may overlap parts of the mountainous
/// terrain, setting `MOUNTAINS_AMOUNT` to 1.0 may not completely cover the
/// terrain in mountains.
const MOUNTAINS_AMOUNT: f64 = 0.5;

/// Determines the amount of hilly terrain that appears on the planet.
/// Values range from 0.0 (no hills) to 1.0 (all terrain is covered in
/// hills). This value must be less than `MOUNTAINS_AMOUNT`. Because the
/// mountains terrain will overlap parts of the hilly terrain, and the
/// badlands terrain may overlap parts of the hilly terrain, setting
/// `HILLS_AMOUNT` to 1.0 may not completely cover the terrain in hills.
const HILLS_AMOUNT: f64 = (1.0 + MOUNTAINS_AMOUNT) / 2.0;

/// Determines the amount of badlands terrain that covers the planet.
/// Values range from 0.0 (no badlands) to 1.0 (all terrain is covered in
/// badlands). Badlands terrain will overlap any other type of terrain.
const BADLANDS_AMOUNT: f64 = 0.3125;

/// Offset to apply to the terrain type definition. Low values (< 1.0)
/// cause the rough areas to appear only at high elevations. High values
/// (> 2.0) cause the rough areas to appear at any elevation. The
/// percentage of rough areas on the planet are independent of this value.
const TERRAIN_OFFSET: f64 = 1.0;

/// Specifies the amount of "glaciation" on the mountains. This value
/// should be close to 1.0 and greater than 1.0.
const MOUNTAIN_GLACIATION: f64 = 1.375;

/// Scaling to apply to the base continent elevations, in planetary
/// elevation units.
const CONTINENT_HEIGHT_SCALE: f64 = (1.0 - SEA_LEVEL) / 4.0;

/// Maximum depth of the rivers, in planetary elevation units.
const RIVER_DEPTH: f64 = 0.0234375;