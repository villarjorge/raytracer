"""
A script that uses imageio and matplotlib to render a ppm. S
ince I wrote it, I have moved away from using ppm, so I don't need it anymore
"""

import imageio.v3 as iio
import matplotlib.pyplot as plt

# This script cannot read the final render. It ether gives an error or produces visual artifacts (see figure 12)
image = iio.imread("images/image.ppm")
plt.imshow(image)
plt.show()