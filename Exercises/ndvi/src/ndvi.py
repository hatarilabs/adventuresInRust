import time
startTime = time.time()
#import required libraries
import rasterio
from rasterio import plot
import matplotlib.pyplot as plt
import numpy as np
import os

#import bands as separate 1 band raster
band4 = rasterio.open('data/HLS.L30.T30PYS.2024308T102014.v2.0.B04.tif') #red
band5 = rasterio.open('data/HLS.L30.T30PYS.2024308T102014.v2.0.B05.tif') #nir
#number of raster rows
print(band4.height, band4.width)
#type of raster byte
print(band4.dtypes[0])
#raster sytem of reference
print(band4.crs)
#raster transform parameters
print(band4.transform)
#raster values as matrix array
print(band4.read(1))
#multiple band representation
# fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(12, 6))
# plot.show(band4, ax=ax1, cmap='Blues') #red
# plot.show(band5, ax=ax2, cmap='Blues') #nir
# fig.tight_layout()
#generate nir and red objects as arrays in float64 format
red = band4.read(1).astype('float64')
nir = band5.read(1).astype('float64')

#ndvi calculation, empty cells or nodata cells are reported as 0
ndvi=np.where(
    (nir+red)==0., 
    0, 
    (nir-red)/(nir+red))
print(ndvi[:5,:5])
#export ndvi image
ndviImage = rasterio.open('data/ndvi_python.tiff','w',driver='Gtiff',
                          width=band4.width, 
                          height = band4.height, 
                          count=1, crs=band4.crs, 
                          transform=band4.transform, 
                          dtype='float32')
ndviImage.write(ndvi,1)
ndviImage.close()
# #plot ndvi
# ndvi = rasterio.open('../Output/ndviImage.tiff')
# fig = plt.figure(figsize=(18,12))
# plot.show(ndvi)

endTime = time.time()

elapsedTime = endTime - startTime
print("Elapsed time: %.4f seconds"%elapsedTime)