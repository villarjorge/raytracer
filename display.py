import imageio.v3 as iio
import matplotlib.pyplot as plt

# This script cannot read the final render. It ether gives an error or produces visual artifacts (see figure 12)
image = iio.imread("images/image.ppm")
plt.imshow(image)
plt.show()