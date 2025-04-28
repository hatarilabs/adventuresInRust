use gdal::DriverManager;
use gdal::raster::RasterBand;
use gdal::Dataset;
use gdal::raster::Buffer;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    // Open the GeoTIFF file
    let band4src = Dataset::open("data/HLS.L30.T30PYS.2024308T102014.v2.0.B04.tif")?;
    let band5src = Dataset::open("data/HLS.L30.T30PYS.2024308T102014.v2.0.B05.tif")?;
    let band4geo = band4src.geo_transform().unwrap();
    let band4prj = band4src.spatial_ref().unwrap();

    println!("Raster size: {:?}", band4src.raster_size());
    println!("Geo transform: {:?}", band4geo);
    println!("Raster src: {}", band4prj.to_wkt().unwrap());


    // // Let's say you want to read the first band
    let band4read: RasterBand = band4src.rasterband(1)?;
    let band5read: RasterBand = band5src.rasterband(1)?;

    let (size_x, size_y) = band4read.size();
    println!("Band 4 size: {}x{}", size_x, size_y);

    // // Create a buffer to hold the Float32 data
    let mut band4 = vec![0.0f32; (size_x * size_y) as usize];
    let mut band5 = vec![0.0f32; (size_x * size_y) as usize];

    // Read into buffer
    band4read.read_into_slice(
        (0, 0),           // window offset (top-left pixel)
        (size_x, size_y), // window size
        (size_x, size_y), // buffer size
        &mut band4,      // target buffer
        None,             // no resampling
    )?;

    // Read into buffer
    band5read.read_into_slice(
        (0, 0),           // window offset (top-left pixel)
        (size_x, size_y), // window size
        (size_x, size_y), // buffer size
        &mut band5,      // target buffer
        None,             // no resampling
    )?;

    println!("Band 4 First few pixel values: {:?}", &band4[0..10]);

    println!("Band 5 First few pixel values: {:?}", &band5[0..10]);

    let ndvi: Vec<f32> = band4.iter()
        .zip(band5.iter())
        .map(|(&band4,&band5)| {
            if band4 + band5 != 0.0 {
                // Some ((band5buffer - band4buffer)/(band5buffer + band4buffer))
                (band5 - band4)/(band5 + band4)
            } else {
                // None
                0.0
            }
        })
        .collect();

    println!("{:?}", &ndvi[0..10]);

    // Open a driver to create the raster
    // let driver = Driver::get_by_name("GTiff").expect("Driver not found");
    let driver = DriverManager::get_driver_by_name("Gtiff")?;

    // Create the raster dataset
    let mut ndvids = driver.create_with_band_type::<f32, _>(
        "data/ndvi_rust.tif", // output file path
        size_x,
        size_y,
        1, // number of bands
    )?;

    // Set the projection 
    let _ = ndvids.set_projection(&band4prj.to_wkt().unwrap());

    // Set the geo transform
    let _ = ndvids.set_geo_transform(&band4geo);

    // // Write data to the band (assuming you have one band)
    let mut band = ndvids.rasterband(1)?;

    // Now wrap it into a Buffer
    let mut ndvi_buffer = Buffer::<f32>::new((size_x, size_y), ndvi);
    band.write((0, 0), (size_x, size_y), &mut ndvi_buffer)?;

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);

    Ok(())

}
