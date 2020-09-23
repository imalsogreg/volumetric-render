# volumetric-render

## About

Hi! This is a toy project I'm using to play with rust, stats, and images. The
problem I wanted to solve - how can we visualize functions of R3, like a
[multivariate normal
distribution](https://en.wikipedia.org/wiki/Multivariate_normal_distribution)
(MVN), or a gaussian mixture model made of MVNs?

Here is an idea: place a camera at some location, an for each pixel of the camera, cast a ray through the distribution. Integrate the value of the function along that ray. I think this technique is called volumetric rendering?

The best way to numerically integrate a function is to know both the function itself and its first derivative - together these do a good job of estimating the area under the curve. So, I wanted to try also implementing [automatic differentiation](), so that we would get derivatives for free on any R3 function. Today, I'm still too confused about how AD works outside the context of functions from R1 to R1, so I don't end up relying on the derivatives during numerical integration. :)

# Results

Here are renderings of a gaussian mixture PDF, from a camera rotating around. 

![gaussian mixture model render](https://github.com/imalsogreg/volumetric-render/raw/master/images/pdf.gif)

![gaussian mixture with camera further away](https://github.com/imalsogreg/volumetric-render/raw/master/images/pdf_afar.gif)

I'm plotting the value linearly into Red and Green, and I plot the log of the value into Blue. I love this trick for highlighting the function's true value while allowing you to see the lower-value features that would otherwise be obscured by the larger values.

This final one is a rendering of the *gradient* of the PDF. The *x* derivitate is in Red, the *y* derivative is in Green and *z* is in Blue.

![gaussian mixture model gradient](https://github.com/imalsogreg/volumetric-render/raw/master/images/mvn_gradient_as_colors.gif)
